//! The internal-user predicate — the module's single definition of "internal user".
//!
//! Hand-authored (user-owned). A user is an **internal user** when they hold an
//! **ACTIVE `organization_users` membership** — a row in `sapiens.organization_users`
//! with `status = 'active'` that is not soft-deleted (`metadata->>'deleted_at' IS NULL`).
//! `pending`, `inactive`, and `suspended` memberships do NOT make a user internal, and a
//! user with no membership at all is external by definition.
//!
//! This is the definition downstream growth/consumer loops (e.g. a digest that reacts to
//! `UserCreated`) must apply when deciding whether a freshly created account belongs to
//! the organization. It is intentionally a QUERY, not a flag carried on the
//! `UserDomainEvent::Created` payload: membership is point-in-time state that can be
//! granted or revoked after creation, so consumers evaluate it at consumption time via
//! these helpers instead of trusting a value frozen at publish time.
//!
//! Scope note: the `organization_users` model itself carries a PENDING disposition
//! (see `schema/models/organization_user.model.yaml`) that forbids wiring login,
//! provisioning, or scope claims to it until that decision lands. These read-only
//! predicates add no such wiring — they only define the term for event consumers.

use sqlx::PgPool;
use uuid::Uuid;

/// Does `user_id` hold an ACTIVE (non-deleted) `organization_users` membership?
///
/// This is THE definition of "internal user". See the module documentation above.
pub async fn is_internal_user(pool: &PgPool, user_id: Uuid) -> sqlx::Result<bool> {
    let is_internal: bool = sqlx::query_scalar(
        r#"SELECT EXISTS (
             SELECT 1 FROM sapiens.organization_users
             WHERE user_id = $1
               AND status = 'active'
               AND metadata->>'deleted_at' IS NULL
           )"#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    Ok(is_internal)
}

/// Batch form of [`is_internal_user`] — returns the subset of `user_ids` that are
/// internal users. Consumption loops that process many `UserCreated` events in one pass
/// should use this instead of per-user queries.
pub async fn internal_user_ids(pool: &PgPool, user_ids: &[Uuid]) -> sqlx::Result<Vec<Uuid>> {
    let rows: Vec<(Uuid,)> = sqlx::query_as(
        r#"SELECT DISTINCT user_id FROM sapiens.organization_users
           WHERE user_id = ANY($1)
             AND status = 'active'
             AND metadata->>'deleted_at' IS NULL"#,
    )
    .bind(user_ids)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}
