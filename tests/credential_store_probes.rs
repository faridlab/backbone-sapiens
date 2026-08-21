//! Credential store probes (ADR-0024, minimal build) — the fail-closed
//! lifecycle against a REAL Postgres. Requires DATABASE_URL pointing at a DB
//! with the `sapiens` schema migrated (`metaphor migration` / the .up.sql
//! files in order). The connecting user must be able to `SET ROLE` for the
//! fence probe (the dev container's bootstrap user can).
//!
//! CSP-1 issue → read_secret roundtrip (and `last_used_at` stamped).
//! CSP-2 the HTTP surface returns metadata only — no ciphertext, no secret.
//! CSP-3 an expired credential refuses AND the read CASes the row terminal.
//! CSP-4 a revoked credential refuses with an honest terminal error.
//! CSP-5 rotate links lineage and kills the predecessor's secret.
//! CSP-6 a row whose key_id was corrupted fails closed (GCM authentication).
//! CSP-7 the strict RLS fence: another company's read sees nothing; a
//!      non-superuser role sees only its own company's rows.
//! CSP-8 a second active credential for one scope is refused.

use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use zeroize::Zeroizing;

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
    let company = Uuid::new_v4();
    let (provider, account) = (d("doku"), d("prov"));

    let descriptor = service
        .issue(company, &provider, &account, CredentialPurpose::WebhookVerify, secret("sk-live-abc"), None)
        .await
        .expect("issue");
    assert_eq!(descriptor.status.to_string(), "active");
    assert!(descriptor.expires_at.is_none());

    let read = service
        .read_secret(company, &provider, &account, CredentialPurpose::WebhookVerify)
        .await
        .expect("read_secret");
    assert_eq!(read.as_string(), "sk-live-abc");

    // last_used_at stamped by the read
    let after = service.describe(company, &provider, &account).await.unwrap();
    assert!(after.iter().any(|c: &CredentialDescriptor| c.last_used_at.is_some()));
}

#[tokio::test]
async fn csp2_http_surface_returns_metadata_only() {
    master_key_env();
    let pool = pool().await;
    let service = std::sync::Arc::new(IntegrationCredentialService::new(pool.clone()));
    let company = Uuid::new_v4();
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

    let resp = backbone_orm::company_scope::with_company_scope(
        Some(company),
        router.oneshot(req),
    )
    .await
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
    let resp = backbone_orm::company_scope::with_company_scope(
        Some(company),
        router.oneshot(req),
    )
    .await
    .expect("oneshot describe");
    assert_eq!(resp.status(), 200);
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let list: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let arr = list.as_array().expect("describe returns a list");
    assert_eq!(arr.len(), 1);
    assert!(arr[0].get("ciphertext").is_none());
}

#[tokio::test]
async fn csp3_expired_read_fails_closed_and_flips_status() {
    master_key_env();
    let pool = pool().await;
    let service = IntegrationCredentialService::new(pool.clone());
    let company = Uuid::new_v4();
    let (provider, account) = (d("doku"), d("prov"));

    service
        .issue(
            company,
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
            .read_secret(company, &provider, &account, CredentialPurpose::WebhookVerify)
            .await,
        "expired read must fail closed",
    );
    assert!(matches!(err, CredentialStoreError::Expired), "got {:?}", err);

    // ...and the drift flip PERSISTED: the row is now terminal 'expired'.
    let listed = service.describe(company, &provider, &account).await.unwrap();
    assert_eq!(listed[0].status.to_string(), "expired");

    // A re-read reports the honest terminal state, not a fresh drift.
    let err = expect_read_err(
        service
            .read_secret(company, &provider, &account, CredentialPurpose::WebhookVerify)
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
    let company = Uuid::new_v4();
    let (provider, account) = (d("xendit"), d("prov"));

    let descriptor = service
        .issue(company, &provider, &account, CredentialPurpose::ApiRead, secret("sk-rev"), None)
        .await
        .unwrap();
    service.revoke(company, descriptor.id).await.expect("revoke");

    let err = expect_read_err(
        service
            .read_secret(company, &provider, &account, CredentialPurpose::ApiRead)
            .await,
        "revoked read must fail closed",
    );
    match &err {
        CredentialStoreError::NotActive(s) => assert_eq!(s.to_string(), "revoked"),
        other => panic!("expected NotActive(revoked), got {:?}", other),
    }

    // Idempotent revoke: a second call still succeeds.
    service.revoke(company, descriptor.id).await.expect("re-revoke noop");
}

#[tokio::test]
async fn csp5_rotate_links_lineage_and_kills_old_secret() {
    master_key_env();
    let pool = pool().await;
    let service = IntegrationCredentialService::new(pool.clone());
    let company = Uuid::new_v4();
    let (provider, account) = (d("midtrans"), d("prov"));

    let first = service
        .issue(company, &provider, &account, CredentialPurpose::WebhookVerify, secret("sk-old"), None)
        .await
        .unwrap();

    let second = service
        .rotate(company, &provider, &account, CredentialPurpose::WebhookVerify, secret("sk-new"), None)
        .await
        .expect("rotate");
    assert_eq!(second.rotated_from, Some(first.id));

    // The successor reads; the predecessor's secret is gone (the active-row
    // lookup finds only the successor — the predecessor was CAS-revoked).
    let read = service
        .read_secret(company, &provider, &account, CredentialPurpose::WebhookVerify)
        .await
        .expect("read successor");
    assert_eq!(read.as_string(), "sk-new");

    let listed = service.describe(company, &provider, &account).await.unwrap();
    assert_eq!(listed.len(), 2, "lineage keeps both rows");
    assert!(listed.iter().any(|c| c.id == first.id && c.status.to_string() == "revoked"));
    assert!(listed.iter().any(|c| c.id == second.id && c.status.to_string() == "active"));
}

#[tokio::test]
async fn csp6_wrong_key_id_fails_closed() {
    master_key_env();
    let pool = pool().await;
    let service = IntegrationCredentialService::new(pool.clone());
    let company = Uuid::new_v4();
    let (provider, account) = (d("doku"), d("prov"));

    service
        .issue(company, &provider, &account, CredentialPurpose::WebhookVerify, secret("sk-real"), None)
        .await
        .unwrap();

    // Corrupt the KEK generation on the row (simulates a lost/rotated master
    // key generation with the row left behind).
    sqlx::query("UPDATE sapiens.integration_credentials SET key_id = 'k-lost' WHERE company_id = $1 AND provider = $2")
        .bind(company)
        .bind(&provider)
        .execute(&pool)
        .await
        .unwrap();

    let err = expect_read_err(
        service
            .read_secret(company, &provider, &account, CredentialPurpose::WebhookVerify)
            .await,
        "wrong key_id must fail closed",
    );
    assert!(matches!(err, CredentialStoreError::Crypto(_)), "got {:?}", err);
}

#[tokio::test]
async fn csp7_rls_cross_company_read_invisible() {
    master_key_env();
    let pool = pool().await;
    let service = IntegrationCredentialService::new(pool.clone());
    let company_a = Uuid::new_v4();
    let company_b = Uuid::new_v4();
    let (provider, account) = (d("doku"), d("prov"));

    service
        .issue(company_a, &provider, &account, CredentialPurpose::WebhookVerify, secret("sk-a"), None)
        .await
        .unwrap();

    // Service level: company B's scoped read sees NOTHING — not even the
    // existence of A's credential.
    let err = expect_read_err(
        service
            .read_secret(company_b, &provider, &account, CredentialPurpose::WebhookVerify)
            .await,
        "cross-company read",
    );
    assert!(matches!(err, CredentialStoreError::NotFound), "got {:?}", err);

    // Fence level: as a NON-SUPERUSER role, the strict policy shows company A's
    // row only when app.company_id is A — and zero rows when unset.
    sqlx::query("DROP ROLE IF EXISTS csp_fence_probe").execute(&pool).await.ok();
    sqlx::query("CREATE ROLE csp_fence_probe NOLOGIN").execute(&pool).await.unwrap();
    sqlx::query("GRANT USAGE ON SCHEMA sapiens TO csp_fence_probe").execute(&pool).await.unwrap();
    sqlx::query("GRANT SELECT ON sapiens.integration_credentials TO csp_fence_probe").execute(&pool).await.unwrap();

    let count_as = async {
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("SET LOCAL ROLE csp_fence_probe").execute(&mut *tx).await.unwrap();
        sqlx::query("SELECT set_config('app.company_id', $1, true)")
            .bind(company_a.to_string())
            .execute(&mut *tx)
            .await
            .unwrap();
        let n: i64 = sqlx::query_scalar("SELECT count(*) FROM sapiens.integration_credentials")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        n
    }
    .await;
    assert_eq!(count_as, 1, "role sees its own company's row");

    let count_unset = async {
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("SET LOCAL ROLE csp_fence_probe").execute(&mut *tx).await.unwrap();
        let n: i64 = sqlx::query_scalar("SELECT count(*) FROM sapiens.integration_credentials")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        n
    }
    .await;
    assert_eq!(count_unset, 0, "unset app.company_id sees zero rows (fail-closed)");

    let count_other = async {
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("SET LOCAL ROLE csp_fence_probe").execute(&mut *tx).await.unwrap();
        sqlx::query("SELECT set_config('app.company_id', $1, true)")
            .bind(company_b.to_string())
            .execute(&mut *tx)
            .await
            .unwrap();
        let n: i64 = sqlx::query_scalar("SELECT count(*) FROM sapiens.integration_credentials")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        n
    }
    .await;
    assert_eq!(count_other, 0, "another company's scope sees zero rows");

    sqlx::query("DROP ROLE IF EXISTS csp_fence_probe").execute(&pool).await.ok();
}

#[tokio::test]
async fn csp8_unique_active_scope_per_purpose() {
    master_key_env();
    let pool = pool().await;
    let service = IntegrationCredentialService::new(pool.clone());
    let company = Uuid::new_v4();
    let (provider, account) = (d("xendit"), d("prov"));

    service
        .issue(company, &provider, &account, CredentialPurpose::WebhookVerify, secret("sk-1"), None)
        .await
        .expect("first issue");

    let err = service
        .issue(company, &provider, &account, CredentialPurpose::WebhookVerify, secret("sk-2"), None)
        .await
        .expect_err("second active issue must be refused");
    assert!(matches!(err, CredentialStoreError::DuplicateActive), "got {:?}", err);

    // A DIFFERENT purpose may hold its own active credential — the uniqueness
    // is per purpose, not per account.
    service
        .issue(company, &provider, &account, CredentialPurpose::ApiRead, secret("sk-read"), None)
        .await
        .expect("issue for another purpose");
}
