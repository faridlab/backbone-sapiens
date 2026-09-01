//! User lifecycle outbox staging — the deactivate / anonymize / delete seam.
//!
//! Hand-authored (user-owned). Mirrors the [`UserCreated`][super::user_created_outbox]
//! contract exactly (same `sapiens.outbox_events` rows, same nil-company
//! sentinel, same dual delivery: durable outbox row + optional in-process
//! typed-bus publish), so a downstream subscription written against
//! `UserCreated` consumes these without any new machinery.
//!
//! Wire shape (all three events):
//!
//! ```text
//! event_type     = "UserDeactivated" | "UserAnonymized" | "UserDeleted"
//! aggregate_type = "User"
//! aggregate_id   = <user uuid>
//! payload        = serialized UserDomainEvent variant
//! company_id     = 00000000-0000-0000-0000-000000000000   (platform-level)
//! ```
//!
//! Producers wired in the module build:
//!
//! 1. **Deactivation** — [`UserDeactivationLifecycle`] watches every user
//!    update through the CRUD service (the mounted `PATCH /users/:id` and the
//!    `POST /users/:id/transitions/deactivate` verb both land there) and
//!    emits `UserDeactivated` on the first transition into `Inactive`.
//! 2. **Anonymization** — [`AnonymizationRecordOutboxPublisher`] emits
//!    `UserAnonymized` when an anonymization record is created for a user
//!    (the GDPR erasure write).
//! 3. **Deletion** — the user CRUD soft-delete verb fires
//!    `CrudEvent::SoftDeleted`; the user event publisher (in
//!    [super::user_created_outbox]) stages `UserDeleted` from it.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use backbone_messaging::crud_event::{CrudEvent, CrudEventPublisher};
use backbone_messaging::{DomainEvent, EventError};

use crate::domain::entity::{AnonymizationRecord, User, UserDomainEvent, UserStatus};

use super::user_created_outbox::{SAPIENS_OUTBOX_SCHEMA, SAPIENS_PLATFORM_COMPANY_ID};
use super::EventBus;

/// Stage one lifecycle event on the caller's OPEN transaction (in-transaction
/// with the write that produced it). Idempotent on the event id.
async fn stage_lifecycle_event(
    conn: &mut sqlx::PgConnection,
    event: UserDomainEvent,
    user_id: Uuid,
    occurred_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    let payload = serde_json::to_value(&event).map_err(|e| {
        sqlx::Error::InvalidArgument(format!("serialize lifecycle event: {e}"))
    })?;
    let record = backbone_outbox::OutboxRecord::new(
        event.event_type(),
        "User",
        user_id.to_string(),
        SAPIENS_PLATFORM_COMPANY_ID,
        payload,
        occurred_at,
    );
    backbone_outbox::outbox::stage(conn, SAPIENS_OUTBOX_SCHEMA, &record)
        .await
        .map_err(|e| sqlx::Error::InvalidArgument(format!("outbox stage: {e}")))?;
    Ok(())
}

/// Stage a `UserDeactivated` event on the caller's open transaction.
pub async fn stage_user_deactivated_event(
    conn: &mut sqlx::PgConnection,
    user_id: Uuid,
    reason: &str,
    occurred_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    stage_lifecycle_event(
        conn,
        UserDomainEvent::Deactivated {
            user_id: user_id.to_string(),
            reason: reason.to_string(),
            occurred_at,
        },
        user_id,
        occurred_at,
    )
    .await
}

/// Stage a `UserAnonymized` event on the caller's open transaction.
pub async fn stage_user_anonymized_event(
    conn: &mut sqlx::PgConnection,
    user_id: Uuid,
    occurred_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    stage_lifecycle_event(
        conn,
        UserDomainEvent::Anonymized {
            user_id: user_id.to_string(),
            occurred_at,
        },
        user_id,
        occurred_at,
    )
    .await
}

/// Stage a `UserDeleted` event on the caller's open transaction.
pub async fn stage_user_deleted_event(
    conn: &mut sqlx::PgConnection,
    user_id: Uuid,
    occurred_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    stage_lifecycle_event(
        conn,
        UserDomainEvent::Deleted {
            user_id: user_id.to_string(),
            occurred_at,
        },
        user_id,
        occurred_at,
    )
    .await
}

/// Post-commit emit: stage the event in its own transaction, then mirror it on
/// the typed in-process bus when one is wired (silently dropped otherwise —
/// the outbox row is the durable record the relay drains).
async fn emit_post_commit(
    pool: &sqlx::PgPool,
    bus: Option<&Arc<EventBus>>,
    event: UserDomainEvent,
    user_id: Uuid,
    occurred_at: DateTime<Utc>,
) {
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            log::warn!("lifecycle outbox tx begin failed for user {user_id}: {e}");
            return;
        }
    };
    let staged = match &event {
        UserDomainEvent::Deactivated { reason, .. } => {
            stage_user_deactivated_event(&mut tx, user_id, reason, occurred_at).await
        }
        UserDomainEvent::Anonymized { .. } => {
            stage_user_anonymized_event(&mut tx, user_id, occurred_at).await
        }
        UserDomainEvent::Deleted { .. } => {
            stage_user_deleted_event(&mut tx, user_id, occurred_at).await
        }
        _ => Ok(()),
    };
    if let Err(e) = staged {
        log::warn!("lifecycle outbox stage failed for user {user_id}: {e}");
        return;
    }
    if let Err(e) = tx.commit().await {
        log::warn!("lifecycle outbox commit failed for user {user_id}: {e}");
        return;
    }
    if let Some(bus) = bus {
        let _ = bus.publish(event).await;
    }
}

/// Lifecycle hook that publishes `UserDeactivated` when a user's status first
/// transitions to `Inactive` through the CRUD service.
///
/// Detection is BEFORE/AFTER the write: `before_update` fetches the prior row
/// and marks the user when the write will deactivate them; `after_update`
/// consumes the mark only if the saved row really is `Inactive` (a failed
/// update leaves a stale mark, which the next save of that user consumes or
/// discards by the same status check — a mark can never fire twice).
pub struct UserDeactivationLifecycle {
    pool: sqlx::PgPool,
    domain_event_bus: Option<Arc<EventBus>>,
    pending: std::sync::Mutex<std::collections::HashSet<Uuid>>,
}

impl UserDeactivationLifecycle {
    pub fn new(pool: sqlx::PgPool, domain_event_bus: Option<Arc<EventBus>>) -> Self {
        Self {
            pool,
            domain_event_bus,
            pending: std::sync::Mutex::new(std::collections::HashSet::new()),
        }
    }

    fn mark(&self, user_id: Uuid) {
        let mut guard = self.pending.lock().unwrap_or_else(|p| p.into_inner());
        guard.insert(user_id);
    }

    fn take_mark(&self, user_id: Uuid) -> bool {
        let mut guard = self.pending.lock().unwrap_or_else(|p| p.into_inner());
        guard.remove(&user_id)
    }

    fn drop_mark(&self, user_id: Uuid) {
        let mut guard = self.pending.lock().unwrap_or_else(|p| p.into_inner());
        guard.remove(&user_id);
    }
}

#[async_trait::async_trait]
impl backbone_core::ServiceLifecycle<User> for UserDeactivationLifecycle {
    async fn before_update(&self, entity: &mut User) -> backbone_core::ServiceResult<()> {
        if entity.status != UserStatus::Inactive {
            return Ok(());
        }
        // The write WILL deactivate — was the user active before? (Rows that
        // are already Inactive must not re-fire the event.)
        let prior_status: Option<String> = sqlx::query_scalar(
            "SELECT status::text FROM users WHERE id = $1",
        )
        .bind(entity.id)
        .fetch_optional(&self.pool)
        .await
        .unwrap_or(None);

        match prior_status.as_deref() {
            // No prior row (insert race) or already inactive: nothing to mark.
            None | Some("inactive") => {}
            _ => self.mark(entity.id),
        }
        Ok(())
    }

    async fn after_update(&self, entity: &User) -> backbone_core::ServiceResult<()> {
        if entity.status == UserStatus::Inactive {
            if self.take_mark(entity.id) {
                let occurred_at = Utc::now();
                emit_post_commit(
                    &self.pool,
                    self.domain_event_bus.as_ref(),
                    UserDomainEvent::Deactivated {
                        user_id: entity.id.to_string(),
                        reason: "status set to inactive".to_string(),
                        occurred_at,
                    },
                    entity.id,
                    occurred_at,
                )
                .await;
            }
        } else {
            // A different update consumed the slot: discard any stale mark.
            self.drop_mark(entity.id);
        }
        Ok(())
    }
}

/// Publishes `UserAnonymized` when an anonymization record is created for a
/// user — the GDPR-erasure moment downstream subscriptions revoke on.
pub struct AnonymizationRecordOutboxPublisher {
    pool: sqlx::PgPool,
    domain_event_bus: Option<Arc<EventBus>>,
}

impl AnonymizationRecordOutboxPublisher {
    pub fn new(pool: sqlx::PgPool, domain_event_bus: Option<Arc<EventBus>>) -> Self {
        Self { pool, domain_event_bus }
    }
}

#[async_trait::async_trait]
impl CrudEventPublisher<AnonymizationRecord> for AnonymizationRecordOutboxPublisher {
    async fn publish(&self, event: CrudEvent<AnonymizationRecord>) -> Result<(), EventError> {
        let entity = match event {
            CrudEvent::Created { entity, .. } => entity,
            _ => return Ok(()),
        };
        let occurred_at = Utc::now();
        emit_post_commit(
            &self.pool,
            self.domain_event_bus.as_ref(),
            UserDomainEvent::Anonymized {
                user_id: entity.user_id.to_string(),
                occurred_at,
            },
            entity.user_id,
            occurred_at,
        )
        .await;
        Ok(())
    }
}
