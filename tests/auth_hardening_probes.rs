//! Auth hardening probes — the public-form surface against a REAL Postgres.
//!
//! Requires DATABASE_URL pointing at a DB with the `sapiens` schema migrated
//! (all .up.sql files in order, including `auth_public_form_hardening`). The
//! connecting user needs INSERT/UPDATE on users, sapiens.sessions,
//! sapiens.email_verification_tokens, sapiens.outbox_events and the three
//! hardening tables.
//!
//! Probe families (one wire-shape test + service-level tests; every test uses
//! its own unique identities so the families never share rows):
//!
//! GATED-ROUTER (one sequential test — the signup policy row is global state
//! and the scenarios must not race each other):
//! - KS-1  off-by-default: a database with NO policy row refuses registration.
//! - KS-2  explicit Off refuses identically.
//! - KS-3  InvitationOnly with no verifier wired refuses EVERYTHING (fail-closed).
//! - KS-4  InvitationOnly: missing invitation -> 400; bad invitation -> 403;
//!         revoked credential -> the SAME 403 (the revocation list is not an
//!         oracle); a good invitation registers.
//! - ENUM-1 register: existing address and nonexistent address answer with
//!         IDENTICAL status + body.
//! - ENUM-2 login: unknown identity, wrong password, unverified account and
//!         locked shape all answer the SAME 401 body.
//! - THR-1 per-identity throttle: same submitted address under a rotating IP
//!         trips its own budget.
//! - THR-2 per-IP throttle: distinct addresses under one IP trip the IP budget.
//!
//! SERVICE-LEVEL:
//! - REV-1 password change revokes every OTHER session and every trusted-device
//!         key; the session that proved the old password survives.
//! - REV-2 password reset kills trusted-device keys too.
//! - ROT-1 refresh rotates the bearer (the old refresh token is dead) WITHOUT
//!         breaking the independent trusted-device key (it still verifies).
//! - TMO-1 the idle posture: an unexpired session idle beyond the declared
//!         window is refused AND revoked at refresh.
//! - LC-1  deactivate (status write) stages UserDeactivated on the outbox;
//!         anonymization-record create stages UserAnonymized; soft-delete
//!         stages UserDeleted.

use std::sync::Arc;

use axum::http::{Request, StatusCode};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

use backbone_sapiens::application::service::signup_policy::{
    InvitationVerifier, SignupMode, SignupPolicyService, VerifiedInvitation,
};
use backbone_sapiens::application::service::{AuthService, RegisterInput};
use backbone_sapiens::application::service::{
    AuthThrottleService, DeviceTrustKeyService, SCOPE_MFA_STEP_UP,
};
use backbone_sapiens::domain::entity::UserStatus;
use backbone_sapiens::infrastructure::auth::email::AuthEmailService;
use backbone_sapiens::infrastructure::auth::jwt::{JwtConfig, JwtService};
use backbone_sapiens::infrastructure::persistence::{
    EmailVerificationTokenRepository, PasswordResetTokenRepository, SessionRepository,
    UserRepository,
};
use backbone_sapiens::presentation::http::public_auth_routes::{
    create_public_auth_routes, PublicAuthState,
};

fn tag() -> String {
    Uuid::new_v4().simple().to_string()[..8].to_string()
}

async fn pool() -> PgPool {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:postgres@localhost:5433/backbone_sapiens".to_string()
    });
    PgPool::connect(&url).await.expect("connect DB")
}

async fn auth_service(pool: &PgPool) -> Arc<AuthService> {
    Arc::new(AuthService::new(
        Arc::new(UserRepository::new(pool.clone())),
        Arc::new(SessionRepository::new(pool.clone())),
        Arc::new(EmailVerificationTokenRepository::new(pool.clone())),
        Arc::new(PasswordResetTokenRepository::new(pool.clone())),
        Arc::new(JwtService::new(JwtConfig::from_env())),
        Arc::new(AuthEmailService::from_env()),
        pool.clone(),
    ))
}

/// Drive one request through the gated router and return (status, body).
/// Takes the router by value (axum's oneshot consumes it); pass a clone.
async fn send(
    app: axum::Router,
    ip: &str,
    path: &str,
    body: serde_json::Value,
) -> (StatusCode, String) {
    let req = Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .header("x-forwarded-for", ip)
        .body(axum::body::Body::from(body.to_string()))
        .expect("build request");
    let resp = app.oneshot(req).await.expect("oneshot");
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("read body");
    (status, String::from_utf8_lossy(&bytes).to_string())
}

fn router(state: PublicAuthState) -> axum::Router {
    create_public_auth_routes(state)
}

fn base_state(pool: &PgPool, auth: Arc<AuthService>) -> PublicAuthState {
    PublicAuthState::new(
        auth,
        Arc::new(SignupPolicyService::new(pool.clone())),
        Arc::new(AuthThrottleService::new(pool.clone())),
        Arc::new(DeviceTrustKeyService::new(pool.clone())),
    )
}

fn register_body(email: &str) -> serde_json::Value {
    json!({
        "email": email,
        "password": "Passw0rd-long",
        "confirm_password": "Passw0rd-long",
        "accept_terms": true,
    })
}

/// An invitation verifier that accepts exactly one token and maps it to one
/// credential id — enough to drive every KS-4 branch.
struct FixedVerifier {
    token: String,
    credential_id: String,
}

#[async_trait::async_trait]
impl InvitationVerifier for FixedVerifier {
    async fn verify(&self, token: &str) -> Option<VerifiedInvitation> {
        (token == self.token).then(|| VerifiedInvitation {
            credential_id: self.credential_id.clone(),
            email: None,
        })
    }
}

async fn clear_signup_policy(pool: &PgPool) {
    sqlx::query("DELETE FROM sapiens.auth_policy WHERE key = 'signup'")
        .execute(pool)
        .await
        .expect("clear signup policy");
}

async fn set_signup_mode(pool: &PgPool, mode: SignupMode) {
    SignupPolicyService::new(pool.clone())
        .set_signup_mode(mode)
        .await
        .expect("set signup mode");
}

// ─────────────────────────────────────────────────────────────────────────────
// GATED-ROUTER wire-shape probes (sequential: the policy row is global)
// ─────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn gated_router_wire_probes() {
    let pool = pool().await;
    let auth = auth_service(&pool).await;

    // ── KS-1: off-by-default. NO policy row: a fresh database refuses signup.
    clear_signup_policy(&pool).await;
    let state = base_state(&pool, auth.clone());
    let app = router(state);
    let email = format!("ks1-{}@probe.example", tag());
    let (status, body) = send(app.clone(), "10.7.0.1", "/register", register_body(&email)).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "no policy row must refuse");
    assert_eq!(
        body,
        json!({"error": "Registration is currently disabled"}).to_string(),
        "KS-1 refusal body"
    );

    // ── KS-2: explicit Off is the same refusal, byte-identical.
    set_signup_mode(&pool, SignupMode::Off).await;
    let email2 = format!("ks2-{}@probe.example", tag());
    let (status2, body2) = send(app.clone(), "10.7.0.2", "/register", register_body(&email2)).await;
    assert_eq!(status2, StatusCode::FORBIDDEN);
    assert_eq!(body2, body, "explicit Off is indistinguishable from default");

    // ── KS-3: InvitationOnly with NO verifier wired refuses everything.
    set_signup_mode(&pool, SignupMode::InvitationOnly).await;
    let email3 = format!("ks3-{}@probe.example", tag());
    let (status3, body3) = send(app.clone(), "10.7.0.3", "/register", register_body(&email3)).await;
    assert_eq!(status3, StatusCode::FORBIDDEN, "unwired invitation mode is closed");
    assert_eq!(body3, body, "same refusal body as Off");

    // ── KS-4: with a verifier wired, the invitation branches separate cleanly.
    let verifier = Arc::new(FixedVerifier {
        token: format!("invite-{}", tag()),
        credential_id: format!("cred-{}", tag()),
    });
    let state = base_state(&pool, auth.clone()).with_invitation_verifier(verifier.clone());
    let app = router(state);

    // missing invitation -> 400 (a shape problem, not an identity reveal)
    let email4 = format!("ks4a-{}@probe.example", tag());
    let (status4, body4) = send(app.clone(), "10.7.0.4", "/register", register_body(&email4)).await;
    assert_eq!(status4, StatusCode::BAD_REQUEST);
    assert_eq!(
        body4,
        json!({"error": "An invitation is required to register"}).to_string()
    );

    // bad invitation -> 403
    let mut bad = register_body(&format!("ks4b-{}@probe.example", tag()));
    bad["invitation_token"] = json!("definitely-not-the-invitation");
    let (status5, body5) = send(app.clone(), "10.7.0.5", "/register", bad).await;
    assert_eq!(status5, StatusCode::FORBIDDEN);
    assert_eq!(
        body5,
        json!({"error": "Invalid or revoked invitation"}).to_string()
    );

    // good invitation -> accepted, uniform 202
    let mut good = register_body(&format!("ks4c-{}@probe.example", tag()));
    good["invitation_token"] = json!(verifier.token);
    let (status6, body6) = send(app.clone(), "10.7.0.6", "/register", good).await;
    assert_eq!(status6, StatusCode::ACCEPTED, "good invitation registers");

    // REVOKED credential -> the SAME 403 as a bad invitation (the revocation
    // list must not become a credential-validity oracle)
    SignupPolicyService::new(pool.clone())
        .revoke_invitation(&verifier.credential_id, "probe revocation")
        .await
        .expect("revoke invitation");
    let mut revoked = register_body(&format!("ks4d-{}@probe.example", tag()));
    revoked["invitation_token"] = json!(verifier.token);
    let (status7, body7) = send(app.clone(), "10.7.0.7", "/register", revoked).await;
    assert_eq!(status7, StatusCode::FORBIDDEN, "revoked credential refuses");
    assert_eq!(body7, body5, "revoked == invalid invitation, byte-identical");

    // ── ENUM-1: register — existing vs nonexistent address are indistinguishable.
    set_signup_mode(&pool, SignupMode::Open).await;
    let state = base_state(&pool, auth.clone());
    let app = router(state);

    let fresh = format!("enum-new-{}@probe.example", tag());
    let (s_new, b_new) = send(app.clone(), "10.7.1.1", "/register", register_body(&fresh)).await;
    assert_eq!(s_new, StatusCode::ACCEPTED);
    // the SAME address again — it exists now; the reply must not say so
    let (s_dup, b_dup) = send(app.clone(), "10.7.1.2", "/register", register_body(&fresh)).await;
    assert_eq!(
        s_dup, s_new,
        "existing address answers the same status as a new one"
    );
    assert_eq!(b_dup, b_new, "existing address answers the SAME body");
    // and a third, never-seen address has the same reply too
    let other = format!("enum-other-{}@probe.example", tag());
    let (s_other, b_other) = send(app.clone(), "10.7.1.3", "/register", register_body(&other)).await;
    assert_eq!(s_other, s_new);
    assert_eq!(b_other, b_new);

    // ── ENUM-2: login — every refusal is the same 401 body.
    // (the `fresh` account exists but is unverified: same refusal as unknown)
    let (s_unv, b_unv) = send(
        app.clone(),
        "10.7.2.1",
        "/login",
        json!({"email": fresh, "password": "Passw0rd-long"}),
    )
    .await;
    let (s_unk, b_unk) = send(
        app.clone(),
        "10.7.2.2",
        "/login",
        json!({"email": format!("nobody-{}@probe.example", tag()), "password": "Passw0rd-long"}),
    )
    .await;
    // wrong password for a REAL verified account
    sqlx::query("UPDATE users SET email_verified = true, status = 'active' WHERE email = $1")
        .bind(&fresh)
        .execute(&pool)
        .await
        .expect("verify probe user");
    let (s_wrong, b_wrong) = send(
        app.clone(),
        "10.7.2.3",
        "/login",
        json!({"email": fresh, "password": "WrongPassw0rd"}),
    )
    .await;
    assert_eq!(s_unv, StatusCode::UNAUTHORIZED);
    assert_eq!(s_unk, StatusCode::UNAUTHORIZED);
    assert_eq!(s_wrong, StatusCode::UNAUTHORIZED);
    assert_eq!(b_unv, b_unk, "unverified and unknown are the same body");
    assert_eq!(b_unk, b_wrong, "unknown and wrong-password are the same body");
    assert_eq!(
        b_unk,
        json!({"error": "Invalid email or password"}).to_string()
    );

    // a CORRECT login succeeds and returns the token pair
    let (s_ok, b_ok) = send(
        app.clone(),
        "10.7.2.4",
        "/login",
        json!({"email": fresh, "password": "Passw0rd-long"}),
    )
    .await;
    assert_eq!(s_ok, StatusCode::OK, "correct login succeeds: {b_ok}");
    assert!(b_ok.contains("refresh_token"), "login returns a refresh token: {b_ok}");

    // ── THR-1: per-IDENTITY throttle (same submitted address, rotating IPs).
    //      The register identity budget is 5/hour: attempts 1..5 consume, 6 trips.
    clear_signup_policy(&pool).await; // policy Off: attempts still spend throttle budget
    let state = base_state(&pool, auth.clone());
    let app = router(state);
    let thrashed = format!("thr-id-{}@probe.example", tag());
    let mut seen = Vec::new();
    for i in 0..6u32 {
        let (status, body) = send(
            app.clone(),
            &format!("10.8.0.{i}"), // distinct IP every time: ONLY the identity budget can trip
            "/register",
            register_body(&thrashed),
        )
        .await;
        seen.push((status, body));
    }
    assert_eq!(
        seen[0].0,
        StatusCode::FORBIDDEN,
        "policy off still refuses (and spends budget)"
    );
    assert_eq!(
        seen[4].0, StatusCode::FORBIDDEN,
        "attempt 5 is the last within the identity budget"
    );
    assert_eq!(seen[5].0, StatusCode::TOO_MANY_REQUESTS, "identity budget trips on attempt 6");
    assert_eq!(
        seen[5].1,
        json!({"error": "Too many attempts. Please try again later."}).to_string()
    );

    // ── THR-2: per-IP throttle (distinct addresses, ONE IP).
    //      The register IP budget is 20/hour: 20 distinct emails pass, 21 trips.
    let mut last = (StatusCode::OK, String::new());
    for i in 0..21u32 {
        let email = format!("thr-ip-{}-{i}@probe.example", tag());
        let outcome = send(app.clone(), "10.9.9.9", "/register", register_body(&email)).await;
        last = outcome;
    }
    assert_eq!(
        last.0,
        StatusCode::TOO_MANY_REQUESTS,
        "IP budget trips after its own limit of distinct identities"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// SERVICE-LEVEL probes
// ─────────────────────────────────────────────────────────────────────────────

/// Create a registered + verified + active probe account, return (user_id, email).
async fn verified_user(pool: &PgPool, auth: &AuthService, prefix: &str) -> (Uuid, String) {
    let email = format!("{prefix}-{}@probe.example", tag());
    auth.register(RegisterInput {
        email: email.clone(),
        password: "Passw0rd-long".into(),
        confirm_password: "Passw0rd-long".into(),
        first_name: None,
        last_name: None,
        accept_terms: true,
        username: None,
    })
    .await
    .expect("register probe user");
    let id: Uuid = sqlx::query_scalar(
        "UPDATE users SET email_verified = true, status = 'active' WHERE email = $1 RETURNING id",
    )
    .bind(&email)
    .fetch_one(pool)
    .await
    .expect("verify probe user");
    (id, email)
}

#[tokio::test]
async fn password_change_revokes_other_sessions_and_device_keys() {
    let pool = pool().await;
    let auth = auth_service(&pool).await;
    let keys = DeviceTrustKeyService::new(pool.clone());

    let (user_id, email) = verified_user(&pool, &auth, "rev1").await;

    // two devices log in; one of them also trusts the device
    let dev_a = auth.login(&email, "Passw0rd-long").await.expect("login A");
    let dev_b = auth.login(&email, "Passw0rd-long").await.expect("login B");
    let trusted = keys
        .issue(user_id, Some("fp-probe"), SCOPE_MFA_STEP_UP, chrono::TimeDelta::days(365), Some("10.0.0.1"), "probe")
        .await
        .expect("issue trust key");
    // requested 365d but the clamp is the MFA window: the grant must be shorter
    assert!(
        trusted.expires_at <= chrono::Utc::now() + chrono::TimeDelta::minutes(30) + chrono::TimeDelta::seconds(5),
        "trust-key age is clamped to the MFA session timeout"
    );

    // device B changes its password (device B's own session survives)
    let keep = auth.session_id_for_refresh_token(user_id, &dev_b.refresh_token).await;
    assert!(keep.is_some(), "the caller's session resolves");
    auth.change_password(
        user_id,
        &email,
        "Passw0rd-long",
        "NewPassw0rd-2",
        "NewPassw0rd-2",
        keep,
    )
    .await
    .expect("change password");

    // device A's session is DEAD
    match auth.refresh_token(&dev_a.refresh_token).await {
        Err(backbone_sapiens::application::service::AuthError::Validation(_)) => {}
        Ok(_) => panic!("stolen/other session must be refused, got a fresh token pair"),
        Err(e) => panic!("stolen/other session must be refused as Validation, got {e}"),
    }
    // device B's session still refreshes
    auth.refresh_token(&dev_b.refresh_token)
        .await
        .expect("the password-changer's own session survives");
    // the trusted-device key is revoked with the password change
    assert!(
        keys.verify(&trusted.key, SCOPE_MFA_STEP_UP).await.is_none(),
        "password change revokes trusted-device keys"
    );
}

#[tokio::test]
async fn password_reset_revokes_device_keys() {
    let pool = pool().await;
    let auth = auth_service(&pool).await;
    let keys = DeviceTrustKeyService::new(pool.clone());

    let (user_id, email) = verified_user(&pool, &auth, "rev2").await;
    let trusted = keys
        .issue(user_id, None, SCOPE_MFA_STEP_UP, chrono::TimeDelta::minutes(10), None, "probe")
        .await
        .expect("issue trust key");

    // forgot_password stores the OTP hashed; read the raw code the email would
    // carry straight from the probe path instead: the service accepts the token
    // argument to reset_password, so mint one through the repo contract by
    // hashing a known code the same way the service does.
    let code = "1234567890abcdef";
    let hash = backbone_sapiens::infrastructure::auth::crypto::hash_token(code);
    sqlx::query(
        "INSERT INTO sapiens.password_reset_tokens \
         (id, user_id, token_hash, expires_at, used_at, ip_address, metadata) \
         VALUES (gen_random_uuid(), $1, $2, NOW() + INTERVAL '30 minutes', NULL, NULL, \
                 '{\"created_at\":null,\"updated_at\":null,\"deleted_at\":null,\"created_by\":null,\"updated_by\":null,\"deleted_by\":null}'::jsonb)",
    )
    .bind(user_id)
    .bind(&hash)
    .execute(&pool)
    .await
    .expect("seed reset token");

    auth.reset_password(code, "ResetPassw0rd-3", "ResetPassw0rd-3")
        .await
        .expect("reset password");

    assert!(
        keys.verify(&trusted.key, SCOPE_MFA_STEP_UP).await.is_none(),
        "password reset revokes trusted-device keys"
    );
    // and the account logs in with the NEW password
    auth.login(&email, "ResetPassw0rd-3")
        .await
        .expect("login after reset");
}

#[tokio::test]
async fn bearer_rotation_keeps_independent_credential() {
    let pool = pool().await;
    let auth = auth_service(&pool).await;
    let keys = DeviceTrustKeyService::new(pool.clone());

    let (user_id, email) = verified_user(&pool, &auth, "rot1").await;
    let session = auth.login(&email, "Passw0rd-long").await.expect("login");
    let trusted = keys
        .issue(user_id, None, SCOPE_MFA_STEP_UP, chrono::TimeDelta::minutes(10), None, "probe")
        .await
        .expect("issue trust key");

    // rotate the bearer
    let rotated = auth
        .refresh_token(&session.refresh_token)
        .await
        .expect("rotate refresh token");

    // the OLD bearer is dead
    match auth.refresh_token(&session.refresh_token).await {
        Err(backbone_sapiens::application::service::AuthError::Validation(_)) => {}
        Ok(_) => panic!("rotated-out token must be refused, got a fresh token pair"),
        Err(e) => panic!("rotated-out token must be refused as Validation, got {e}"),
    }
    // the NEW bearer works
    auth.refresh_token(&rotated.refresh_token)
        .await
        .expect("rotated bearer works");
    // the INDEPENDENT credential (the trusted-device key) is untouched by the
    // bearer rotation — either rotates without breaking the other
    assert!(
        keys.verify(&trusted.key, SCOPE_MFA_STEP_UP).await.is_some(),
        "bearer rotation must not revoke the trusted-device key"
    );
}

#[tokio::test]
async fn idle_timeout_posture_refuses_and_revokes() {
    let pool = pool().await;
    let auth = auth_service(&pool).await;

    let (user_id, email) = verified_user(&pool, &auth, "tmo1").await;
    let session = auth.login(&email, "Passw0rd-long").await.expect("login");

    // Age the session PAST the declared idle window but keep it absolutely
    // unexpired: only the idle posture can refuse it.
    sqlx::query(
        "UPDATE sapiens.sessions SET last_activity = NOW() - INTERVAL '8 days', \
         expires_at = NOW() + INTERVAL '1 day' WHERE user_id = $1 AND status = 'active'",
    )
    .bind(user_id)
    .execute(&pool)
    .await
    .expect("age session");

    match auth.refresh_token(&session.refresh_token).await {
        Err(backbone_sapiens::application::service::AuthError::Validation(msg))
            if msg.contains("inactivity") => {}
        Ok(_) => panic!("idle-expired session must be refused, got a fresh token pair"),
        Err(e) => panic!("idle-expired session must be refused as inactivity, got {e}"),
    }
    // and the refusal REVOKED it (not just refused once)
    let revoked: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM sapiens.sessions WHERE user_id = $1 AND status = 'revoked'",
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .expect("count revoked sessions");
    assert!(revoked >= 1, "idle expiry revokes the session");
}

#[tokio::test]
async fn lifecycle_outbox_stages_deactivated_anonymized_deleted() {
    use backbone_sapiens::infrastructure::messaging::AnonymizationRecordOutboxPublisher;
    use backbone_sapiens::infrastructure::messaging::UserDeactivationLifecycle;
    use backbone_sapiens::infrastructure::messaging::UserLifecycleOutboxPublisher;
    use backbone_sapiens::application::service::{AnonymizationRecordService, UserService};
    use backbone_sapiens::infrastructure::persistence::{
        AnonymizationRecordRepository, UserRepository,
    };
    use backbone_sapiens::presentation::dto::CreateUserDto;

    let pool = pool().await;

    // wire the services EXACTLY like the module build wires them
    let users = Arc::new(UserService::new(
        Arc::new(UserRepository::new(pool.clone())),
        Arc::new(UserDeactivationLifecycle::new(pool.clone(), None)),
        Arc::new(UserLifecycleOutboxPublisher::new(pool.clone(), None)),
    ));
    let anon = Arc::new(
        AnonymizationRecordService::with_repository(Arc::new(AnonymizationRecordRepository::new(
            pool.clone(),
        )))
        .with_event_publisher(Arc::new(AnonymizationRecordOutboxPublisher::new(
            pool.clone(),
            None,
        ))),
    );

    let email = format!("lc1-{}@probe.example", tag());
    let user = users
        .create(CreateUserDto {
            username: format!("lcuser{}", tag()),
            email: email.clone(),
            password_hash: "probe-hash-only".repeat(6),
            status: UserStatus::Active,
            email_verified: false,
            failed_login_attempts: 0,
            locked_until: None,
            last_login: None,
        })
        .await
        .expect("create user");

    // LC-1a: a status write into Inactive stages UserDeactivated
    let mut fields = std::collections::HashMap::new();
    fields.insert("status".to_string(), serde_json::json!("inactive"));
    users
        .partial_update(&user.id.to_string(), fields)
        .await
        .expect("deactivate user");
    let deactivated: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM sapiens.outbox_events \
         WHERE aggregate_id = $1 AND event_type = 'UserDeactivated'",
    )
    .bind(user.id.to_string())
    .fetch_one(&pool)
    .await
    .expect("count deactivated rows");
    assert_eq!(deactivated, 1, "one UserDeactivated row per deactivation");

    // a SECOND inactive write must not re-fire
    let mut again = std::collections::HashMap::new();
    again.insert("status".to_string(), serde_json::json!("inactive"));
    // (partial_update of an already-inactive row: the lifecycle sees prior
    // inactive and does not mark)
    users
        .partial_update(&user.id.to_string(), again)
        .await
        .expect("re-write inactive");
    let deactivated2: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM sapiens.outbox_events \
         WHERE aggregate_id = $1 AND event_type = 'UserDeactivated'",
    )
    .bind(user.id.to_string())
    .fetch_one(&pool)
    .await
    .expect("recount deactivated rows");
    assert_eq!(deactivated2, 1, "re-writing inactive does not re-fire");

    // LC-1b: creating the anonymization record stages UserAnonymized
    // (the record's own shape is generated; insert through the service)
    use backbone_sapiens::presentation::dto::CreateAnonymizationRecordDto;
    let dto = serde_json::from_value::<CreateAnonymizationRecordDto>(serde_json::json!({
        "user_id": user.id.to_string(),
        "original_email": email,
        "original_username": "lcuser-probe",
        "anonymized_by": user.id.to_string(),
        "anonymized_at": chrono::Utc::now().to_rfc3339(),
        "reason": "probe gdpr erasure",
        "method": "full",
        "status": "completed",
    }))
    .expect("build anonymization dto");
    anon.create(dto).await.expect("create anonymization record");
    let anonymized: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM sapiens.outbox_events \
         WHERE aggregate_id = $1 AND event_type = 'UserAnonymized'",
    )
    .bind(user.id.to_string())
    .fetch_one(&pool)
    .await
    .expect("count anonymized rows");
    assert_eq!(anonymized, 1, "one UserAnonymized row per erasure write");

    // LC-1c: soft-delete stages UserDeleted
    users.soft_delete(&user.id.to_string()).await.expect("soft delete");
    let deleted: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM sapiens.outbox_events \
         WHERE aggregate_id = $1 AND event_type = 'UserDeleted'",
    )
    .bind(user.id.to_string())
    .fetch_one(&pool)
    .await
    .expect("count deleted rows");
    assert_eq!(deleted, 1, "one UserDeleted row per soft-delete");
}
