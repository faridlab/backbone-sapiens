//! HTTP surface for the credential store — verbs only, metadata only.
//!
//! Routes (composed by the host behind its own gates; NOT part of
//! `SapiensModule::routes()`'s CRUD surface):
//!
//!   GET  /integration-credentials?provider=&account_ref=   → lineage listing
//!   POST /integration-credentials                           → issue
//!   POST /integration-credentials/:id/rotate                → rotate (by id)
//!   POST /integration-credentials/:id/revoke                → revoke
//!
//! There is deliberately NO route that returns a secret: `read_secret` is an
//! in-process port for seams (webhook verification, provider API clients).
//! Tenancy (ADR-0029): the routes must be mounted inside the host's org
//! request scope — the ambient org scope (`backbone_orm::org_scope`) is the
//! request identity these verbs ride, and a missing scope is 401 — fail-closed.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::application::service::integration_credential_service::{
    CredentialDescriptor, CredentialStoreError, IntegrationCredentialService,
};
use crate::domain::entity::CredentialPurpose;

#[derive(Clone)]
pub struct IntegrationCredentialAppState {
    pub service: Arc<IntegrationCredentialService>,
}

#[derive(Debug, Deserialize)]
pub struct DescribeQuery {
    pub provider: String,
    pub account_ref: String,
}

#[derive(Debug, Deserialize)]
pub struct IssueRequest {
    pub provider: String,
    pub account_ref: String,
    pub purpose: CredentialPurpose,
    /// The secret itself — write-only; it appears in no response.
    pub secret: String,
    /// Honest expiry from the provider; omit for genuinely non-expiring secrets.
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct RotateRequest {
    pub secret: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

fn error_status(err: &CredentialStoreError) -> StatusCode {
    match err {
        CredentialStoreError::MissingMasterKey => StatusCode::SERVICE_UNAVAILABLE,
        CredentialStoreError::NotFound => StatusCode::NOT_FOUND,
        CredentialStoreError::NotActive(_) | CredentialStoreError::Expired => StatusCode::CONFLICT,
        CredentialStoreError::DuplicateActive => StatusCode::CONFLICT,
        CredentialStoreError::InvalidScope => StatusCode::UNPROCESSABLE_ENTITY,
        CredentialStoreError::Database(_) | CredentialStoreError::Crypto(_) => {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

fn error_response(err: CredentialStoreError) -> Response {
    (error_status(&err), err.to_string()).into_response()
}

/// Refuse the request when no ambient org scope is bound — the host mounted
/// these routes without its org identity middleware (a configuration error —
/// fail closed). The verbs relay the scope themselves; the gate only decides
/// whether the request may proceed.
fn require_org_scope() -> Result<(), Response> {
    if backbone_orm::org_scope::current_org_scope().is_some() {
        Ok(())
    } else {
        Err((StatusCode::UNAUTHORIZED, "no org scope on request").into_response())
    }
}

/// Issue the first active credential for a scope.
pub async fn issue_credential(
    State(state): State<IntegrationCredentialAppState>,
    Json(body): Json<IssueRequest>,
) -> Response {
    if let Err(resp) = require_org_scope() {
        return resp;
    }
    let secret = Zeroizing::new(body.secret);
    match state
        .service
        .issue(&body.provider, &body.account_ref, body.purpose, secret, body.expires_at)
        .await
    {
        Ok(descriptor) => (StatusCode::CREATED, Json(descriptor)).into_response(),
        Err(err) => error_response(err),
    }
}

/// List a scope's credential lineage — metadata only, newest first.
pub async fn describe_credentials(
    State(state): State<IntegrationCredentialAppState>,
    Query(query): Query<DescribeQuery>,
) -> Response {
    if let Err(resp) = require_org_scope() {
        return resp;
    }
    match state.service.describe(&query.provider, &query.account_ref).await {
        Ok(descriptors) => (StatusCode::OK, Json(descriptors)).into_response(),
        Err(err) => error_response(err),
    }
}

/// Rotate: insert the successor + CAS-revoke the predecessor, atomically.
pub async fn rotate_credential(
    State(state): State<IntegrationCredentialAppState>,
    Path(credential_id): Path<Uuid>,
    Json(body): Json<RotateRequest>,
) -> Response {
    if let Err(resp) = require_org_scope() {
        return resp;
    }
    // The id pins the lineage; provider/account_ref/purpose come from the
    // row being replaced.
    let scope = match state.service.describe_by_id(credential_id).await {
        Ok(Some(d)) => (d.provider, d.account_ref, d.purpose),
        Ok(None) => return error_response(CredentialStoreError::NotFound),
        Err(err) => return error_response(err),
    };
    let secret = Zeroizing::new(body.secret);
    match state
        .service
        .rotate(&scope.0, &scope.1, scope.2, secret, body.expires_at)
        .await
    {
        Ok(descriptor) => (StatusCode::OK, Json(descriptor)).into_response(),
        Err(err) => error_response(err),
    }
}

/// Revoke by id. Idempotent for already-terminal credentials.
pub async fn revoke_credential(
    State(state): State<IntegrationCredentialAppState>,
    Path(credential_id): Path<Uuid>,
) -> Response {
    if let Err(resp) = require_org_scope() {
        return resp;
    }
    match state.service.revoke(credential_id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => error_response(err),
    }
}

/// Verb routes for the credential store. The host mounts these behind its own
/// role gate + org request scope (they are NOT merged into the module's CRUD
/// router).
pub fn create_integration_credential_routes(service: Arc<IntegrationCredentialService>) -> axum::Router {
    let state = IntegrationCredentialAppState { service };
    axum::Router::new()
        .route("/integration-credentials", axum::routing::post(issue_credential))
        .route("/integration-credentials", axum::routing::get(describe_credentials))
        .route("/integration-credentials/:id/rotate", axum::routing::post(rotate_credential))
        .route("/integration-credentials/:id/revoke", axum::routing::post(revoke_credential))
        .with_state(state)
}
