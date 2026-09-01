//! The GATED public auth router — sapiens' only intended public auth surface.
//!
//! ## Mounting contract (binding on every host)
//!
//! This router is **exported, never mounted**. `SapiensModule::routes()` does
//! not include it and must never grow it: whether (and where) the public auth
//! forms answer is a HOST decision, taken deliberately per deployment.
//! Mounting it requires the host to:
//!
//! 1. mount it at a base it chose, verified free of the host's own session
//!    bridge (a host-owned `/api/v1/auth` login/me/refresh bridge already
//!    answers on some deployments — double-registering that base panics the
//!    router at boot);
//! 2. front it with the reverse proxy (Caddy) so client addresses arrive in
//!    `x-forwarded-for` / `x-real-ip` — the per-IP throttle keys on those
//!    headers and degrades to one shared bucket without them;
//! 3. accept the declared posture: signup is a kill-switchable POLICY
//!    (default OFF — a fresh database has no policy row and refuses every
//!    registration), forms are throttled per-identity AND per-IP, and every
//!    reply is shaped so an anonymous requester cannot learn whether an
//!    identity exists.
//!
//! ## What is deliberately absent
//!
//! - No `check-username` / `check-email` style endpoints — direct identity
//!   enumeration oracles; removed with the router they lived in.
//! - No GET path that sends or verifies anything (the historical
//!   `GET /verify-email` is gone: verification codes are submitted, never
//!   carried on a link someone else can fire).
//! - No registration result detail: the register reply is the SAME status,
//!   body and shape whether the address was newly registered or already
//!   existed (`AuthService::register_via_public_form`).

use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use chrono::TimeDelta;
use serde::Deserialize;

use crate::application::service::auth_throttle::{ThrottleDecision, ThrottleLimit};
use crate::application::service::signup_policy::{InvitationVerifier, SignupMode};
use crate::application::service::{
    AuthService, AuthError, AuthThrottleService, DeviceTrustKeyService, RegisterInput,
    SignupPolicyService,
};

// ── Declared throttle postures (Tier B) ─────────────────────────────────────
//
// Per-identity buckets key on the SUBMITTED address (normalized), per-IP
// buckets on the client address. Both are durable fixed windows — they hold
// across restarts and replicas. Change the posture here, not in handlers.

/// Self-service registration: generous for a human, useless for enumeration.
const REGISTER_IDENTITY_LIMIT: ThrottleLimit =
    ThrottleLimit::identity_const(5, TimeDelta::hours(1));
const REGISTER_IP_LIMIT: ThrottleLimit = ThrottleLimit::ip_const(20, TimeDelta::hours(1));
/// Login: tight enough to blunt credential stuffing.
const LOGIN_IDENTITY_LIMIT: ThrottleLimit =
    ThrottleLimit::identity_const(10, TimeDelta::minutes(15));
const LOGIN_IP_LIMIT: ThrottleLimit = ThrottleLimit::ip_const(30, TimeDelta::minutes(15));
/// Password-reset request (an email-sending form): the tightest of all.
const FORGOT_IDENTITY_LIMIT: ThrottleLimit =
    ThrottleLimit::identity_const(3, TimeDelta::hours(1));
const FORGOT_IP_LIMIT: ThrottleLimit = ThrottleLimit::ip_const(20, TimeDelta::hours(1));
/// Verification-code submission.
const VERIFY_IDENTITY_LIMIT: ThrottleLimit =
    ThrottleLimit::identity_const(10, TimeDelta::hours(1));
const VERIFY_IP_LIMIT: ThrottleLimit = ThrottleLimit::ip_const(60, TimeDelta::hours(1));

/// Everything the gated handlers need, wired from the module build. The
/// invitation verifier is optional: with none wired, `InvitationOnly` mode
/// refuses every registration (fail-closed by construction) — the portal
/// module (the invitation issuer) or the host wires the real one.
#[derive(Clone)]
pub struct PublicAuthState {
    auth: Arc<AuthService>,
    signup_policy: Arc<SignupPolicyService>,
    throttle: Arc<AuthThrottleService>,
    #[allow(dead_code)] // reserved for the step-up flow the host mounts later
    device_trust_keys: Arc<DeviceTrustKeyService>,
    invitation_verifier: Option<Arc<dyn InvitationVerifier>>,
}

impl PublicAuthState {
    pub fn new(
        auth: Arc<AuthService>,
        signup_policy: Arc<SignupPolicyService>,
        throttle: Arc<AuthThrottleService>,
        device_trust_keys: Arc<DeviceTrustKeyService>,
    ) -> Self {
        Self {
            auth,
            signup_policy,
            throttle,
            device_trust_keys,
            invitation_verifier: None,
        }
    }

    /// Wire the invitation verifier (the portal module's Tier A credential
    /// check). Without this, invitation-only mode refuses everything.
    pub fn with_invitation_verifier(
        mut self,
        verifier: Arc<dyn InvitationVerifier>,
    ) -> Self {
        self.invitation_verifier = Some(verifier);
        self
    }
}

/// Build the gated public auth router. NOT part of `SapiensModule::routes()`;
/// see the module docs for the mounting contract.
pub fn create_public_auth_routes(state: PublicAuthState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/verify-email", post(verify_email))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
        .route("/change-password", post(change_password))
        .with_state(state)
}

// ── helpers ─────────────────────────────────────────────────────────────────

fn json_body(status: StatusCode, key: &str, message: &str) -> axum::response::Response {
    (status, Json(serde_json::json!({ key: message }))).into_response()
}

fn accepted(message: &str) -> axum::response::Response {
    json_body(StatusCode::OK, "message", message)
}

fn bad_request(message: &str) -> axum::response::Response {
    json_body(StatusCode::BAD_REQUEST, "error", message)
}

fn refused(message: &str) -> axum::response::Response {
    json_body(StatusCode::FORBIDDEN, "error", message)
}

fn unauthorized(message: &str) -> axum::response::Response {
    json_body(StatusCode::UNAUTHORIZED, "error", message)
}

fn throttled() -> axum::response::Response {
    json_body(
        StatusCode::TOO_MANY_REQUESTS,
        "error",
        "Too many attempts. Please try again later.",
    )
}

/// The client address the per-IP buckets key on. Behind the reverse proxy the
/// proxy-set `x-forwarded-for` (first hop) or `x-real-ip` carries it; without
/// either header every caller shares one bucket — degrading closed, never open.
fn client_ip(headers: &HeaderMap) -> String {
    if let Some(xff) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = xff.split(',').next() {
            let ip = first.trim();
            if !ip.is_empty() {
                return ip.to_string();
            }
        }
    }
    if let Some(real) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        if !real.trim().is_empty() {
            return real.trim().to_string();
        }
    }
    "unknown".to_string()
}

/// Run both throttle dimensions for a form submission. Every refusal is the
/// same 429 body regardless of which dimension tripped.
async fn throttle_gate(
    svc: &AuthThrottleService,
    headers: &HeaderMap,
    identity: &str,
    identity_limit: ThrottleLimit,
    ip_limit: ThrottleLimit,
) -> Option<axum::response::Response> {
    if svc.check_and_increment(identity_limit, identity).await == ThrottleDecision::Refused {
        return Some(throttled());
    }
    if svc.check_and_increment(ip_limit, &client_ip(headers)).await == ThrottleDecision::Refused {
        return Some(throttled());
    }
    None
}

// ── request payloads ────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
    confirm_password: String,
    #[serde(default)]
    first_name: Option<String>,
    #[serde(default)]
    last_name: Option<String>,
    #[serde(default)]
    accept_terms: bool,
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    invitation_token: Option<String>,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

#[derive(Deserialize)]
struct VerifyEmailRequest {
    email: String,
    code: String,
}

#[derive(Deserialize)]
struct ForgotPasswordRequest {
    email: String,
}

#[derive(Deserialize)]
struct ResetPasswordRequest {
    token: String,
    new_password: String,
    confirm_password: String,
}

#[derive(Deserialize)]
struct ChangePasswordRequest {
    refresh_token: String,
    current_password: String,
    new_password: String,
    confirm_password: String,
}

// ── handlers ────────────────────────────────────────────────────────────────

/// POST /register — the policy-gated, throttled, de-oracled signup form.
///
/// Order: throttle → policy → (invitation) → de-oracled registration. Every
/// refusal carries a uniform body that reveals nothing about the account
/// table; the success reply is IDENTICAL for a newly-registered and an
/// already-existing address.
async fn register(
    State(state): State<PublicAuthState>,
    headers: HeaderMap,
    Json(req): Json<RegisterRequest>,
) -> axum::response::Response {
    if let Some(response) = throttle_gate(
        &state.throttle,
        &headers,
        &req.email,
        REGISTER_IDENTITY_LIMIT,
        REGISTER_IP_LIMIT,
    )
    .await
    {
        return response;
    }

    match state.signup_policy.signup_mode().await {
        SignupMode::Off => {
            return refused("Registration is currently disabled");
        }
        SignupMode::InvitationOnly => {
            let Some(verifier) = state.invitation_verifier.clone() else {
                // No verifier wired: invitation mode cannot verify anything,
                // so it refuses everything. Fail-closed by construction.
                return refused("Registration is currently disabled");
            };
            let Some(token) = req.invitation_token.as_deref() else {
                return bad_request("An invitation is required to register");
            };
            // An unreadable verifier outcome and a revoked credential are the
            // SAME refusal: "invalid or revoked invitation" must not separate
            // them, or the revocation list becomes an oracle.
            let verified = match verifier.verify(token).await {
                Some(inv) => !state.signup_policy.invitation_is_revoked(&inv.credential_id).await,
                None => false,
            };
            if !verified {
                return refused("Invalid or revoked invitation");
            }
        }
        SignupMode::Open => {}
    }

    let input = RegisterInput {
        email: req.email,
        password: req.password,
        confirm_password: req.confirm_password,
        first_name: req.first_name,
        last_name: req.last_name,
        accept_terms: req.accept_terms,
        username: req.username,
    };

    match state.auth.register_via_public_form(input).await {
        Ok(_) => (
            StatusCode::ACCEPTED,
            Json(serde_json::json!({
                "message": "If the submission is valid, your account has been created and a verification email sent where required."
            })),
        )
            .into_response(),
        Err(AuthError::Validation(msg)) => bad_request(&msg),
        Err(_) => {
            // Internal failures get the generic 500 — never a detail leak.
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// POST /login — throttled both dimensions; unknown identity, wrong password,
/// locked and inactive accounts all answer the SAME 401 body (the service
/// equalizes verification cost so they are timing-indistinguishable too).
async fn login(
    State(state): State<PublicAuthState>,
    headers: HeaderMap,
    Json(req): Json<LoginRequest>,
) -> axum::response::Response {
    if let Some(response) = throttle_gate(
        &state.throttle,
        &headers,
        &req.email,
        LOGIN_IDENTITY_LIMIT,
        LOGIN_IP_LIMIT,
    )
    .await
    {
        return response;
    }

    match state.auth.login(&req.email, &req.password).await {
        Ok(result) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "access_token": result.access_token,
                "refresh_token": result.refresh_token,
                "expires_in": result.expires_in,
            })),
        )
            .into_response(),
        Err(AuthError::InvalidCredentials) => unauthorized("Invalid email or password"),
        // A locked or unverified account is still an authentication refusal;
        // the distinct message helps the legitimate owner without revealing
        // anything to a stranger (the stranger already submitted the right
        // password to reach these states).
        Err(AuthError::AccountLocked) => unauthorized("Invalid email or password"),
        Err(AuthError::EmailNotVerified) => unauthorized("Invalid email or password"),
        Err(AuthError::AccountInactive(_)) => unauthorized("Invalid email or password"),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// POST /refresh — rotate the refresh token. The absolute and idle session
/// postures (`SESSION_TIMEOUT_POLICY`) are enforced here in the service: an
/// idle-expired session is revoked, not refreshed.
async fn refresh(
    State(state): State<PublicAuthState>,
    Json(req): Json<RefreshRequest>,
) -> axum::response::Response {
    match state.auth.refresh_token(&req.refresh_token).await {
        Ok(result) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "access_token": result.access_token,
                "refresh_token": result.refresh_token,
                "expires_in": result.expires_in,
            })),
        )
            .into_response(),
        Err(AuthError::Validation(msg)) => bad_request(&msg),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// POST /logout — revoke every session of the token's owner.
async fn logout(
    State(state): State<PublicAuthState>,
    Json(req): Json<RefreshRequest>,
) -> axum::response::Response {
    match state.auth.resolve_refresh_token_owner(&req.refresh_token).await {
        Some(owner) => match state.auth.logout(owner.user_id).await {
            Ok(_) => accepted("Logged out"),
            Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
        // An unknown token logs "out" successfully — logout is idempotent and
        // must not confirm which tokens exist.
        None => accepted("Logged out"),
    }
}

/// POST /verify-email — submit a verification code. Unknown address and wrong
/// code are the SAME 400 (the service enforces it); there is no GET twin.
async fn verify_email(
    State(state): State<PublicAuthState>,
    headers: HeaderMap,
    Json(req): Json<VerifyEmailRequest>,
) -> axum::response::Response {
    if let Some(response) = throttle_gate(
        &state.throttle,
        &headers,
        &req.email,
        VERIFY_IDENTITY_LIMIT,
        VERIFY_IP_LIMIT,
    )
    .await
    {
        return response;
    }

    match state.auth.verify_email(&req.email, &req.code).await {
        Ok(_) => accepted("Email verified"),
        Err(AuthError::Validation(msg)) => bad_request(&msg),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// POST /forgot-password — request a reset email. Always the same 200 (the
/// service performs identical work whether the address exists or not and
/// sends nothing for an existing-but-unregistered address).
async fn forgot_password(
    State(state): State<PublicAuthState>,
    headers: HeaderMap,
    Json(req): Json<ForgotPasswordRequest>,
) -> axum::response::Response {
    if let Some(response) = throttle_gate(
        &state.throttle,
        &headers,
        &req.email,
        FORGOT_IDENTITY_LIMIT,
        FORGOT_IP_LIMIT,
    )
    .await
    {
        return response;
    }

    match state.auth.forgot_password(&req.email).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "If the address is registered, a password reset email has been sent."
            })),
        )
            .into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// POST /reset-password — consume a reset code. Resets revoke every session
/// and every trusted-device key for the account (service-side).
async fn reset_password(
    State(state): State<PublicAuthState>,
    Json(req): Json<ResetPasswordRequest>,
) -> axum::response::Response {
    match state
        .auth
        .reset_password(&req.token, &req.new_password, &req.confirm_password)
        .await
    {
        Ok(_) => accepted("Password has been reset"),
        Err(AuthError::Validation(msg)) => bad_request(&msg),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// POST /change-password — prove the CURRENT password and present the refresh
/// token of the session to keep. Every OTHER session and every outstanding
/// trusted-device key is revoked (service-side, one transaction); the old
/// password is the proof, the token only names the surviving session.
async fn change_password(
    State(state): State<PublicAuthState>,
    Json(req): Json<ChangePasswordRequest>,
) -> axum::response::Response {
    let Some(owner) = state.auth.resolve_refresh_token_owner(&req.refresh_token).await else {
        return unauthorized("Invalid email or password");
    };

    match state
        .auth
        .change_password(
            owner.user_id,
            &owner.email,
            &req.current_password,
            &req.new_password,
            &req.confirm_password,
            Some(owner.session_id),
        )
        .await
    {
        Ok(_) => accepted("Password has been changed"),
        Err(AuthError::Validation(msg)) => bad_request(&msg),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
