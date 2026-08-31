//! The `UserCreated` outbox staging seam — where a user creation becomes a durable event.
//!
//! Hand-authored (user-owned). Two producers feed this seam:
//!
//! 1. **Self-registration** (`AuthService::register`) — the event is staged INSIDE the
//!    registration transaction, right after the user row and the verification token are
//!    written, so the state change and the event commit atomically. A crash between the
//!    commit and any downstream delivery can never drop the event, and a rolled-back
//!    registration never emits a phantom one.
//! 2. **The admin/CRUD create path** (`POST /users`, `/users/bulk`, `/users/upsert` via
//!    `GenericCrudService`) — the generated service only exposes the post-commit
//!    `CrudEventPublisher` hook, so [`UserCreatedOutboxPublisher`] stages the same outbox
//!    row immediately after the repository insert commits. This is one transaction later
//!    than the registration path (documented in `docs/spec-sapiens.md` §7); the outbox
//!    row is identical either way.
//!
//! After staging, both producers ALSO publish `UserDomainEvent::Created` on the module's
//! typed in-process event bus when one is wired (the convenience delivery the
//! `SapiensIntegrationEventPublisher` translator turns into a cross-module
//! `sapiens.user.created` integration event). The outbox row is the durable backstop the
//! host relay drains; the in-process publish is immediate delivery only. When the bus is
//! not wired the in-process publish is **silently dropped** — the same semantics the
//! domain-layer `DefaultAuthenticationService::publish_event` has always had — while the
//! outbox row still lands and the relay still delivers it.
//!
//! ## Wire shape
//!
//! The outbox row for a user creation is:
//!
//! ```text
//! event_type     = "UserCreated"                    (UserDomainEvent::Created::event_type())
//! aggregate_type = "User"
//! aggregate_id   = <user uuid>
//! payload        = {"event_type":"Created","user_id":"<uuid>","occurred_at":"<rfc3339>"}
//! company_id     = 00000000-0000-0000-0000-000000000000   (nil sentinel; users are
//!                  platform-level and have no company dimension — never filter these
//!                  rows by company)
//! ```

use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

use backbone_messaging::crud_event::{CrudEvent, CrudEventPublisher};
use backbone_messaging::{DomainEvent, EventError};

use crate::domain::entity::{User, UserDomainEvent};

use super::EventBus;

/// The outbox schema this module stages into (module/schema name: `sapiens`).
pub const SAPIENS_OUTBOX_SCHEMA: &str = "sapiens";

/// The sentinel tenant stamped on every outbox row this module stages.
///
/// User accounts are platform-level: there is no company dimension on the `users` table,
/// so the NOT NULL `company_id` column carries the nil uuid instead. Consumers must key
/// on the event type and aggregate id, never on `company_id`. This is a CONSTANT, not a
/// config knob — a configurable "default company" would synthesize a tenant fence this
/// module's data model cannot back.
pub const SAPIENS_PLATFORM_COMPANY_ID: Uuid = Uuid::nil();

/// Stage a `UserDomainEvent::Created` into `sapiens.outbox_events` on the caller's OPEN
/// transaction, in-transaction with the user write that produced it.
///
/// Idempotent on the event id (`ON CONFLICT DO NOTHING`), per `backbone_outbox::outbox::stage`.
pub async fn stage_user_created_event(
    conn: &mut sqlx::PgConnection,
    user_id: Uuid,
    occurred_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    let event = UserDomainEvent::Created {
        user_id: user_id.to_string(),
        occurred_at,
    };
    let payload = serde_json::to_value(&event)
        .map_err(|e| sqlx::Error::InvalidArgument(format!("serialize UserDomainEvent::Created: {e}")))?;
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

/// Publishes `UserCreated` for every NEW user the CRUD service creates.
///
/// Attached to the `UserService` (`GenericCrudService<User, _>`) via
/// `with_event_publisher` in the module build. `GenericCrudService::create` fires
/// `CrudEvent::Created` exactly once per successfully inserted user row — `bulk_create`
/// and `upsert` both delegate to `create`, so every creation path through the CRUD router
/// lands here once. A create that fails (validation error) never fires the hook: one
/// committed user row ⇒ at most one outbox row, one in-process publish.
///
/// ## Re-fire idempotency
///
/// The `users` table's email uniqueness is soft-delete-aware — `UNIQUE (email,
/// (metadata->>'deleted_at'))` — and a Postgres unique index treats NULLs as distinct,
/// so it does NOT actually refuse a second LIVE row with the same email. A re-fired
/// admin create therefore inserts a second row (new user id) today. This publisher
/// suppresses the event when the created row is such a duplicate: if another live
/// (non-soft-deleted) user already holds the email, the account is not a new user in the
/// business sense and no `UserCreated` is staged or published. The original user's event
/// is untouched, so a re-fire can never double-publish for one email. A create of a
/// previously soft-deleted email still publishes (the live-row check ignores deleted
/// rows). The durable fix is a partial unique index on the schema
/// (`UNIQUE (email) WHERE metadata->>'deleted_at' IS NULL`); this guard is the
/// publish-seam half of the guarantee until then.
///
/// Non-`Created` variants are ignored (`Ok(())`) — this publisher owns only the
/// user-creation contract.
pub struct UserCreatedOutboxPublisher {
    pool: sqlx::PgPool,
    /// Optional typed in-process bus for immediate delivery. When `None` the in-process
    /// publish is silently dropped; the outbox row is still staged and the host relay
    /// still delivers it.
    domain_event_bus: Option<Arc<EventBus>>,
}

impl UserCreatedOutboxPublisher {
    pub fn new(pool: sqlx::PgPool, domain_event_bus: Option<Arc<EventBus>>) -> Self {
        Self {
            pool,
            domain_event_bus,
        }
    }

    /// True when another LIVE (non-soft-deleted) user row already holds this email —
    /// i.e. the create that produced `entity` re-fired an existing account.
    async fn duplicates_live_email(&self, entity: &User) -> bool {
        match sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM users \
             WHERE email = $1 AND id <> $2 AND metadata->>'deleted_at' IS NULL",
        )
        .bind(&entity.email)
        .bind(entity.id)
        .fetch_one(&self.pool)
        .await
        {
            Ok(n) => n > 0,
            // Fail open: without the check we cannot know this is a duplicate, and the
            // outbox row is the half of the contract that must not be lost.
            Err(e) => {
                log::warn!("UserCreated duplicate-email check failed, publishing anyway: {e}");
                false
            }
        }
    }

    /// Fire-and-forget in-process delivery, mirroring the domain service's
    /// `publish_event`: if no bus is configured the event is silently dropped.
    async fn publish_in_process(&self, event: UserDomainEvent) {
        if let Some(bus) = &self.domain_event_bus {
            let _ = bus.publish(event).await;
        }
    }
}

#[async_trait::async_trait]
impl CrudEventPublisher<User> for UserCreatedOutboxPublisher {
    async fn publish(&self, event: CrudEvent<User>) -> Result<(), EventError> {
        let entity = match event {
            CrudEvent::Created { entity, .. } => entity,
            // BulkCreated never reaches a GenericCrudService publisher (bulk_create loops
            // create()), and the other lifecycle events are out of this contract's scope.
            _ => return Ok(()),
        };

        if self.duplicates_live_email(&entity).await {
            return Ok(());
        }

        let occurred_at = Utc::now();
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| EventError::PublishError(format!("outbox tx begin: {e}")))?;
        stage_user_created_event(&mut tx, entity.id, occurred_at)
            .await
            .map_err(|e| EventError::PublishError(format!("UserCreated stage: {e}")))?;
        tx.commit()
            .await
            .map_err(|e| EventError::PublishError(format!("outbox tx commit: {e}")))?;

        self.publish_in_process(UserDomainEvent::Created {
            user_id: entity.id.to_string(),
            occurred_at,
        })
        .await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_company_sentinel_is_nil() {
        assert_eq!(SAPIENS_PLATFORM_COMPANY_ID, Uuid::nil());
        assert_eq!(SAPIENS_OUTBOX_SCHEMA, "sapiens");
    }
}
