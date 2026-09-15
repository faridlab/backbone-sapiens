//! The signed-in person's in-app notification tray.
//!
//! The generated CRUD router for this entity is deliberately NOT mounted. The
//! table has no org column — its only fence is `user_id` — so a generic mount
//! would let any authenticated caller list, and mark read, every user's
//! notifications across the whole holding. That is the reason this file exists:
//! the recipient is taken from the verified token and never from the request.
//!
//! Delivery state and attention state stay separate. The delivery fields
//! (`sent_at`, `delivered_at`) say the notification reached a channel;
//! `is_read` / `read_at` say a human looked at it. A notification can be
//! delivered and unread for a week, and conflating the two makes both
//! unqueryable.
//!
//! Marking read is an explicit verb, not a PATCH on the row, so a client cannot
//! rewrite a notification's contents on its way to setting one flag.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use backbone_auth::AuthContext;
use serde::Deserialize;
use sqlx::{PgPool, Row};

#[derive(Clone)]
pub struct NotificationInboxState {
    pub pool: PgPool,
}

#[derive(Debug, Deserialize)]
pub struct InboxQuery {
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub limit: Option<u32>,
    /// `true` returns only what has not been read yet.
    #[serde(default)]
    pub unread: Option<bool>,
}

/// The tray, the unread badge, and the two ways to clear it.
///
/// Mount behind whatever authenticates the request: every handler here reads
/// `AuthContext` and refuses without it, so an unauthenticated mount serves 401
/// rather than someone else's inbox.
pub fn create_notification_inbox_routes(state: NotificationInboxState) -> Router {
    Router::new()
        .route("/notifications/me", get(list_mine))
        .route("/notifications/me/unread-count", get(unread_count))
        .route("/notifications/me/read-all", post(read_all))
        .route("/notifications/:id/read", post(read_one))
        .with_state(Arc::new(state))
}

/// Rows a tray should show: this recipient's, in-app, not soft-deleted, and not
/// past their own expiry — `expires_at` exists so a notification can stop being
/// worth showing without being deleted.
const TRAY_SCOPE: &str = "user_id = $1::uuid
     AND channel = 'in_app'
     AND (metadata->>'deleted_at') IS NULL
     AND (expires_at IS NULL OR expires_at > NOW())";

fn unauthorized() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({"success": false, "error": "authentication required"})),
    )
}

fn failed(e: impl std::fmt::Display) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({"success": false, "error": e.to_string()})),
    )
}

async fn list_mine(
    State(state): State<Arc<NotificationInboxState>>,
    auth: Option<Extension<AuthContext>>,
    Query(q): Query<InboxQuery>,
) -> impl IntoResponse {
    let Some(Extension(auth)) = auth else { return unauthorized() };

    let limit = q.limit.unwrap_or(20).clamp(1, 100) as i64;
    let offset = (q.page.unwrap_or(1).max(1) - 1) as i64 * limit;
    let unread_only = if q.unread.unwrap_or(false) { " AND is_read = false" } else { "" };

    let sql = format!(
        "SELECT id, notification_type, title, message, data, is_read, read_at,
                priority, action_url, action_text, category,
                metadata->>'created_at' AS created_at
           FROM sapiens.notifications
          WHERE {TRAY_SCOPE}{unread_only}
          ORDER BY (metadata->>'created_at') DESC NULLS LAST
          LIMIT {limit} OFFSET {offset}"
    );

    let rows = match backbone_orm::company_scope::fetch_all_rows_scoped(
        &state.pool,
        sqlx::query(&sql).bind(&auth.user_id),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => return failed(e),
    };

    let items: Vec<serde_json::Value> = rows
        .iter()
        .map(|r| {
            serde_json::json!({
                "id": r.try_get::<uuid::Uuid, _>("id").map(|v| v.to_string()).unwrap_or_default(),
                "type": r.try_get::<String, _>("notification_type").unwrap_or_default(),
                "title": r.try_get::<String, _>("title").unwrap_or_default(),
                "message": r.try_get::<String, _>("message").unwrap_or_default(),
                "data": r.try_get::<Option<serde_json::Value>, _>("data").ok().flatten(),
                "isRead": r.try_get::<bool, _>("is_read").unwrap_or(false),
                "readAt": r.try_get::<Option<chrono::DateTime<chrono::Utc>>, _>("read_at")
                    .ok().flatten().map(|t| t.to_rfc3339()),
                // The severity the UI colours a row by. A real schema field,
                // not a presentation invention — low | normal | high | urgent.
                "priority": r.try_get::<String, _>("priority").unwrap_or_else(|_| "normal".into()),
                "actionUrl": r.try_get::<Option<String>, _>("action_url").ok().flatten(),
                "actionText": r.try_get::<Option<String>, _>("action_text").ok().flatten(),
                "category": r.try_get::<Option<String>, _>("category").ok().flatten(),
                "createdAt": r.try_get::<Option<String>, _>("created_at").ok().flatten(),
            })
        })
        .collect();

    (
        StatusCode::OK,
        Json(serde_json::json!({"success": true, "data": items})),
    )
}

async fn unread_count(
    State(state): State<Arc<NotificationInboxState>>,
    auth: Option<Extension<AuthContext>>,
) -> impl IntoResponse {
    let Some(Extension(auth)) = auth else { return unauthorized() };

    let sql = format!(
        "SELECT COUNT(*) FROM sapiens.notifications WHERE {TRAY_SCOPE} AND is_read = false"
    );
    match backbone_orm::company_scope::fetch_one_scalar_scoped(
        &state.pool,
        sqlx::query_scalar::<_, i64>(&sql).bind(&auth.user_id),
    )
    .await
    {
        Ok(n) => (
            StatusCode::OK,
            Json(serde_json::json!({"success": true, "data": {"count": n}})),
        ),
        Err(e) => failed(e),
    }
}

async fn read_one(
    State(state): State<Arc<NotificationInboxState>>,
    auth: Option<Extension<AuthContext>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Some(Extension(auth)) = auth else { return unauthorized() };

    // The recipient predicate is part of the UPDATE, not a check before it:
    // another user's notification matches zero rows rather than being marked.
    // `is_read` and `read_at` are set together so they can never disagree about
    // whether this was read.
    let sql = format!(
        "UPDATE sapiens.notifications
            SET is_read = true, read_at = COALESCE(read_at, NOW())
          WHERE id = $2::uuid AND {TRAY_SCOPE}
        RETURNING id"
    );
    match backbone_orm::company_scope::fetch_optional_row_scoped(
        &state.pool,
        sqlx::query(&sql).bind(&auth.user_id).bind(&id),
    )
    .await
    {
        // Not found and not-yours are the same answer on purpose: telling them
        // apart would confirm that someone else's notification id exists.
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"success": false, "error": "notification not found"})),
        ),
        Ok(Some(_)) => (
            StatusCode::OK,
            Json(serde_json::json!({"success": true, "data": {"id": id, "isRead": true}})),
        ),
        Err(e) => failed(e),
    }
}

async fn read_all(
    State(state): State<Arc<NotificationInboxState>>,
    auth: Option<Extension<AuthContext>>,
) -> impl IntoResponse {
    let Some(Extension(auth)) = auth else { return unauthorized() };

    let sql = format!(
        "WITH marked AS (
            UPDATE sapiens.notifications
               SET is_read = true, read_at = COALESCE(read_at, NOW())
             WHERE {TRAY_SCOPE} AND is_read = false
         RETURNING 1
         ) SELECT COUNT(*) FROM marked"
    );
    match backbone_orm::company_scope::fetch_one_scalar_scoped(
        &state.pool,
        sqlx::query_scalar::<_, i64>(&sql).bind(&auth.user_id),
    )
    .await
    {
        Ok(n) => (
            StatusCode::OK,
            Json(serde_json::json!({"success": true, "data": {"marked": n}})),
        ),
        Err(e) => failed(e),
    }
}
