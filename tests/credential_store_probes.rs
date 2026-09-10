//! Credential store probes (ADR-0024, minimal build) — the fail-closed
//! lifecycle against a REAL Postgres. Requires DATABASE_URL pointing at a DB
//! with the `sapiens` schema migrated (`metaphor migration` / the .up.sql
//! files in order). The fence probe needs the connecting user to be a
//! superuser (it mints a restricted probe role and installs the decorator
//! fence shape itself).
//!
//! CSP-1 issue → read_secret roundtrip (and `last_used_at` stamped).
//! CSP-2 the HTTP surface returns metadata only — no ciphertext, no secret.
//! CSP-3 an expired credential refuses AND the read CASes the row terminal.
//! CSP-4 a revoked credential refuses with an honest terminal error.
//! CSP-5 rotate links lineage and kills the predecessor's secret.
//! CSP-6 a row whose key_id was corrupted fails closed (GCM authentication).
//! CSP-7 the org fence twin (ADR-0029): with the composing decorator's fence
//!      installed, a scoped session sees only its unit's rows; an unscoped or
//!      foreign-scoped session sees zero rows (fail-closed).
//! CSP-8 a second active credential for one scope is refused.

use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use zeroize::Zeroizing;

use backbone_orm::org_scope::OrgScope;
use backbone_sapiens::application::service::integration_credential_service::{
    CredentialStoreError, IntegrationCredentialService,
};
use backbone_sapiens::domain::entity::CredentialPurpose;
use backbone_sapiens::presentation::http::integration_credential_handler::create_integration_credential_routes;
use backbone_sapiens::CredentialDescriptor;

fn d(s: &str) -> String {
    format!("{}-{}", s, &Uuid::new_v4().simple().to_string()[..8])
}

async fn pool() -> sqlx::PgPool {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:postgres@localhost:5433/backbone_sapiens".to_string()
    });
    PgPool::connect(&url).await.expect("connect DB")
}

/// The database URL rewritten to connect as a specific probe role — the
/// restricted-identity pool the fence probe drives the service with.
fn role_url(role: &str, password: &str) -> String {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:postgres@localhost:5433/backbone_sapiens".to_string()
    });
    let after_at = url.rsplit('@').next().expect("url has an authority").to_string();
    format!("postgresql://{role}:{password}@{after_at}")
}

fn master_key_env() {
    // A fixed 32-byte test key; tests that need it absent remove it.
    std::env::set_var("CREDENTIAL_MASTER_KEY", "MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=");
}

fn secret(s: &str) -> Zeroizing<String> {
    Zeroizing::new(s.to_string())
}

/// `Result::expect_err` needs `T: Debug`; `ZeroizingSecret` deliberately has
/// no Debug so a secret can never drift into a log line.
fn expect_read_err(
    res: Result<backbone_sapiens::application::service::credential_crypto::ZeroizingSecret, CredentialStoreError>,
    msg: &str,
) -> CredentialStoreError {
    match res {
        Err(e) => e,
        Ok(_) => panic!("{}", msg),
    }
}

#[tokio::test]
async fn csp1_issue_and_read_secret_roundtrip() {
    master_key_env();
    let pool = pool().await;
    let service = IntegrationCredentialService::new(pool.clone());
    let (provider, account) = (d("doku"), d("prov"));

    let descriptor = service
        .issue(&provider, &account, CredentialPurpose::WebhookVerify, secret("sk-live-abc"), None)
        .await
        .expect("issue");
    assert_eq!(descriptor.status.to_string(), "active");
    assert!(descriptor.expires_at.is_none());

    let read = service
        .read_secret(&provider, &account, CredentialPurpose::WebhookVerify)
        .await
        .expect("read_secret");
    assert_eq!(read.as_string(), "sk-live-abc");

    // last_used_at stamped by the read
    let after = service.describe(&provider, &account).await.unwrap();
    assert!(after.iter().any(|c: &CredentialDescriptor| c.last_used_at.is_some()));
}

#[tokio::test]
async fn csp2_http_surface_returns_metadata_only() {
    master_key_env();
    let pool = pool().await;
    let service = std::sync::Arc::new(IntegrationCredentialService::new(pool.clone()));
    let (provider, account) = (d("midtrans"), d("prov"));

    let router = create_integration_credential_routes(service.clone());

    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt; // Router::oneshot — tower/util is enabled via axum

    let body = serde_json::json!({
        "provider": provider,
        "account_ref": account,
        "purpose": "webhook_verify",
        "secret": "sk-http-secret",
    })
    .to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/integration-credentials")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();

    // The routes ride the ambient org scope: wrap the request in an org
    // request scope the way the composing service's middleware does (a
    // single-unit scope; the probe DB has no org spine to resolve a real one).
    let scope = OrgScope::for_company_unit(Uuid::new_v4());
    let resp = backbone_orm::org_scope::with_org_request_scope(&pool, scope, router.oneshot(req))
        .await
        .expect("org-scoped oneshot")
        .expect("oneshot");
    assert_eq!(resp.status(), 201);

    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(json.get("id").is_some(), "descriptor carries id");
    assert!(json.get("ciphertext").is_none(), "response must not carry ciphertext");
    assert!(json.get("secret").is_none(), "response must not carry the secret");

    // describe over HTTP is metadata-only too
    let req = Request::builder()
        .method("GET")
        .uri(format!("/integration-credentials?provider={}&account_ref={}", provider, account))
        .body(Body::empty())
        .unwrap();
    let router = create_integration_credential_routes(service.clone());
    let scope = OrgScope::for_company_unit(Uuid::new_v4());
    let resp = backbone_orm::org_scope::with_org_request_scope(&pool, scope, router.oneshot(req))
        .await
        .expect("org-scoped describe")
        .expect("describe oneshot");
    assert_eq!(resp.status(), 200);
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let list: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let arr = list.as_array().expect("describe returns a list");
    assert_eq!(arr.len(), 1);
    assert!(arr[0].get("ciphertext").is_none());
}

#[tokio::test]
async fn csp2b_http_surface_refuses_without_org_scope() {
    master_key_env();
    let pool = pool().await;
    let service = std::sync::Arc::new(IntegrationCredentialService::new(pool.clone()));
    let router = create_integration_credential_routes(service);

    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    // No ambient org scope bound → 401, fail-closed (the host mounted the
    // routes without its org identity middleware).
    let req = Request::builder()
        .method("POST")
        .uri("/integration-credentials")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({
                "provider": "doku",
                "account_ref": "prov",
                "purpose": "webhook_verify",
                "secret": "sk-should-not-seal",
            })
            .to_string(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.expect("oneshot");
    assert_eq!(resp.status(), 401, "missing org scope must be 401");
}

#[tokio::test]
async fn csp3_expired_read_fails_closed_and_flips_status() {
    master_key_env();
    let pool = pool().await;
    let service = IntegrationCredentialService::new(pool.clone());
    let (provider, account) = (d("doku"), d("prov"));

    service
        .issue(
            &provider,
            &account,
            CredentialPurpose::WebhookVerify,
            secret("sk-expiring"),
            Some(Utc::now() - Duration::hours(1)),
        )
        .await
        .expect("issue past-expiry credential");

    // The read refuses...
    let err = expect_read_err(
        service
            .read_secret(&provider, &account, CredentialPurpose::WebhookVerify)
            .await,
        "expired read must fail closed",
    );
    assert!(matches!(err, CredentialStoreError::Expired), "got {:?}", err);

    // ...and the drift flip PERSISTED: the row is now terminal 'expired'.
    let listed = service.describe(&provider, &account).await.unwrap();
    assert_eq!(listed[0].status.to_string(), "expired");

    // A re-read reports the honest terminal state, not a fresh drift.
    let err = expect_read_err(
        service
            .read_secret(&provider, &account, CredentialPurpose::WebhookVerify)
            .await,
        "re-read of expired",
    );
    assert!(matches!(err, CredentialStoreError::NotActive(_)), "got {:?}", err);
}

#[tokio::test]
async fn csp4_revoked_read_fails_closed() {
    master_key_env();
    let pool = pool().await;
    let service = IntegrationCredentialService::new(pool.clone());
    let (provider, account) = (d("xendit"), d("prov"));

    let descriptor = service
        .issue(&provider, &account, CredentialPurpose::ApiRead, secret("sk-rev"), None)
        .await
        .unwrap();
    service.revoke(descriptor.id).await.expect("revoke");

    let err = expect_read_err(
        service
            .read_secret(&provider, &account, CredentialPurpose::ApiRead)
            .await,
        "revoked read must fail closed",
    );
    match &err {
        CredentialStoreError::NotActive(s) => assert_eq!(s.to_string(), "revoked"),
        other => panic!("expected NotActive(revoked), got {:?}", other),
    }

    // Idempotent revoke: a second call still succeeds.
    service.revoke(descriptor.id).await.expect("re-revoke noop");
}

#[tokio::test]
async fn csp5_rotate_links_lineage_and_kills_old_secret() {
    master_key_env();
    let pool = pool().await;
    let service = IntegrationCredentialService::new(pool.clone());
    let (provider, account) = (d("midtrans"), d("prov"));

    let first = service
        .issue(&provider, &account, CredentialPurpose::WebhookVerify, secret("sk-old"), None)
        .await
        .unwrap();

    let second = service
        .rotate(&provider, &account, CredentialPurpose::WebhookVerify, secret("sk-new"), None)
        .await
        .expect("rotate");
    assert_eq!(second.rotated_from, Some(first.id));

    // The successor reads; the predecessor's secret is gone (the active-row
    // lookup finds only the successor — the predecessor was CAS-revoked).
    let read = service
        .read_secret(&provider, &account, CredentialPurpose::WebhookVerify)
        .await
        .expect("read successor");
    assert_eq!(read.as_string(), "sk-new");

    let listed = service.describe(&provider, &account).await.unwrap();
    assert_eq!(listed.len(), 2, "lineage keeps both rows");
    assert!(listed.iter().any(|c| c.id == first.id && c.status.to_string() == "revoked"));
    assert!(listed.iter().any(|c| c.id == second.id && c.status.to_string() == "active"));
}

#[tokio::test]
async fn csp6_wrong_key_id_fails_closed() {
    master_key_env();
    let pool = pool().await;
    let service = IntegrationCredentialService::new(pool.clone());
    let (provider, account) = (d("doku"), d("prov"));

    service
        .issue(&provider, &account, CredentialPurpose::WebhookVerify, secret("sk-real"), None)
        .await
        .unwrap();

    // Corrupt the KEK generation on the row (simulates a lost/rotated master
    // key generation with the row left behind).
    sqlx::query("UPDATE sapiens.integration_credentials SET key_id = 'k-lost' WHERE provider = $1 AND account_ref = $2")
        .bind(&provider)
        .bind(&account)
        .execute(&pool)
        .await
        .unwrap();

    let err = expect_read_err(
        service
            .read_secret(&provider, &account, CredentialPurpose::WebhookVerify)
            .await,
        "wrong key_id must fail closed",
    );
    assert!(matches!(err, CredentialStoreError::Crypto(_)), "got {:?}", err);
}

/// Install the fence shape a composing service's tenancy decorator puts on
/// this table (ADR-0029): the `org_unit_id` column (its DEFAULT resolves the
/// acting unit for inserts that omit it), row-level security forced on, and
/// the entitlement-union policy. Every step is re-runnable and torn down at
/// the end, so the scratch database returns to the module's bare shape.
async fn install_decorator_fence(pool: &PgPool) {
    sqlx::query("DROP POLICY IF EXISTS integration_credentials_org_isolation ON sapiens.integration_credentials")
        .execute(pool).await.unwrap();
    sqlx::query(
        "ALTER TABLE sapiens.integration_credentials DROP COLUMN IF EXISTS org_unit_id",
    )
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        "ALTER TABLE sapiens.integration_credentials \
         ADD COLUMN org_unit_id uuid DEFAULT nullif(current_setting('app.acting_unit_id', true), '')::uuid",
    )
    .execute(pool)
    .await
    .unwrap();
    sqlx::query("ALTER TABLE sapiens.integration_credentials ENABLE ROW LEVEL SECURITY")
        .execute(pool).await.unwrap();
    sqlx::query("ALTER TABLE sapiens.integration_credentials FORCE  ROW LEVEL SECURITY")
        .execute(pool).await.unwrap();
    sqlx::query(
        "CREATE POLICY integration_credentials_org_isolation ON sapiens.integration_credentials \
         FOR ALL \
         USING      (org_unit_id = ANY(string_to_array(current_setting('app.scope_unit_ids', true), ',')::uuid[])) \
         WITH CHECK (org_unit_id = ANY(string_to_array(current_setting('app.scope_unit_ids', true), ',')::uuid[]))",
    )
    .execute(pool)
    .await
    .unwrap();
}

async fn remove_decorator_fence(pool: &PgPool) {
    sqlx::query("DROP POLICY IF EXISTS integration_credentials_org_isolation ON sapiens.integration_credentials")
        .execute(pool).await.ok();
    sqlx::query(
        "ALTER TABLE sapiens.integration_credentials DROP COLUMN IF EXISTS org_unit_id",
    )
    .execute(pool)
    .await
    .ok();
}

#[tokio::test]
async fn csp7_org_fence_scopes_isolation() {
    master_key_env();
    let admin = pool().await;
    let service = IntegrationCredentialService::new(admin.clone());
    let (provider, account) = (d("doku"), d("prov"));

    const PROBE_ROLE: &str = "csp_org_fence_probe";
    const PROBE_PW: &str = "csp-org-fence-pw";

    install_decorator_fence(&admin).await;

    // Restricted-role pool: RLS applies to it even under FORCE, unlike the
    // superuser admin pool (a composing service connects its pools with a
    // restricted role — the fence is real only for that path).
    sqlx::query("DROP OWNED BY csp_org_fence_probe").execute(&admin).await.ok();
    sqlx::query("DROP ROLE IF EXISTS csp_org_fence_probe").execute(&admin).await.ok();
    sqlx::query(&format!("CREATE ROLE {PROBE_ROLE} LOGIN PASSWORD '{PROBE_PW}'"))
        .execute(&admin)
        .await
        .unwrap();
    sqlx::query(&format!("GRANT USAGE ON SCHEMA sapiens TO {PROBE_ROLE}"))
        .execute(&admin)
        .await
        .unwrap();
    sqlx::query(&format!(
        "GRANT SELECT, INSERT, UPDATE ON sapiens.integration_credentials TO {PROBE_ROLE}"
    ))
    .execute(&admin)
    .await
    .unwrap();
    let app = PgPool::connect(&role_url(PROBE_ROLE, PROBE_PW)).await.expect("connect probe role");

    let unit_a = Uuid::new_v4();
    let unit_b = Uuid::new_v4();

    // Issue under unit A's org scope (superuser pool): the relayed scope's
    // acting unit lands in org_unit_id via the decorated column DEFAULT.
    {
        let scope = OrgScope::for_company_unit(unit_a);
        let issued = backbone_orm::org_scope::with_org_request_scope(&admin, scope, async {
            service.issue(&provider, &account, CredentialPurpose::WebhookVerify, secret("sk-a"), None).await
        })
        .await
        .expect("scoped issue")
        .expect("issue");
        assert_eq!(issued.status.to_string(), "active");
    }

    let landed_unit: Option<Uuid> = sqlx::query_scalar(
        "SELECT org_unit_id FROM sapiens.integration_credentials WHERE provider = $1 AND account_ref = $2",
    )
    .bind(&provider)
    .bind(&account)
    .fetch_one(&admin)
    .await
    .unwrap();
    assert_eq!(landed_unit, Some(unit_a), "decorated DEFAULT resolved the acting unit");

    // The scoped app-role session reads its own row end-to-end (the verb's
    // transaction relays the ambient scope onto the fence).
    let app_service = IntegrationCredentialService::new(app.clone());
    {
        let scope = OrgScope::for_company_unit(unit_a);
        let read = backbone_orm::org_scope::with_org_request_scope(&app, scope, async {
            app_service.read_secret(&provider, &account, CredentialPurpose::WebhookVerify).await
        })
        .await
        .expect("scoped read")
        .expect("own-unit read must see the row");
        assert_eq!(read.as_string(), "sk-a");
    }

    // A foreign unit's scope sees NOTHING — not even the row's existence.
    {
        let scope = OrgScope::for_company_unit(unit_b);
        let err = expect_read_err(
            backbone_orm::org_scope::with_org_request_scope(&app, scope, async {
                app_service.read_secret(&provider, &account, CredentialPurpose::WebhookVerify).await
            })
            .await
            .expect("foreign-scoped read"),
            "cross-unit read",
        );
        assert!(matches!(err, CredentialStoreError::NotFound), "got {:?}", err);
    }

    // No scope at all sees zero rows (fail-closed): the unscoped transaction
    // leaves the fence variables unset and the policy matches nothing.
    {
        let err = expect_read_err(
            app_service.read_secret(&provider, &account, CredentialPurpose::WebhookVerify).await,
            "unscoped read must see nothing",
        );
        assert!(matches!(err, CredentialStoreError::NotFound), "got {:?}", err);
    }

    app.close().await;
    sqlx::query("DROP OWNED BY csp_org_fence_probe").execute(&admin).await.ok();
    sqlx::query("DROP ROLE IF EXISTS csp_org_fence_probe").execute(&admin).await.ok();
    remove_decorator_fence(&admin).await;
}

#[tokio::test]
async fn csp8_unique_active_scope_per_purpose() {
    master_key_env();
    let pool = pool().await;
    let service = IntegrationCredentialService::new(pool.clone());
    let (provider, account) = (d("xendit"), d("prov"));

    service
        .issue(&provider, &account, CredentialPurpose::WebhookVerify, secret("sk-1"), None)
        .await
        .expect("first issue");

    let err = service
        .issue(&provider, &account, CredentialPurpose::WebhookVerify, secret("sk-2"), None)
        .await
        .expect_err("second active issue must be refused");
    assert!(matches!(err, CredentialStoreError::DuplicateActive), "got {:?}", err);

    // A DIFFERENT purpose may hold its own active credential — the uniqueness
    // is per purpose, not per account.
    service
        .issue(&provider, &account, CredentialPurpose::ApiRead, secret("sk-read"), None)
        .await
        .expect("issue for another purpose");
}
