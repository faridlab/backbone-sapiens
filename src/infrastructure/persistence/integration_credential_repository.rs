//! Persistence for the credential store. All statements run on a CALLER-BOUND
//! connection (`*_on`) so a verb's reads and CAS transitions commit as one unit
//! under one `app.company_id` (bind via `company_scope::bind_company_on` after
//! `begin()`; pool-only reads ride `with_company_scope`). Enum parameters are
//! cast in the SQL (`$n::credential_purpose`) — runtime-bound Postgres
//! parameters arrive untyped, so the target enum must be named.
//!
//! Every statement carries the company fence as an explicit predicate (the same
//! expression as the table's RLS policy). The policy alone would be enough for
//! restricted roles, but superuser connections bypass RLS even under FORCE —
//! the predicate keeps the fence true regardless of the connecting role.

use sqlx::PgConnection;
use uuid::Uuid;

use crate::domain::entity::{CredentialPurpose, CredentialStatus, IntegrationCredential};

/// What `issue`/`rotate` insert. `ciphertext` is already sealed; the row is
/// born `active` (the partial unique index enforces one active per scope).
pub struct NewCredentialRow {
    pub company_id: Uuid,
    pub provider: String,
    pub account_ref: String,
    pub purpose: CredentialPurpose,
    pub key_id: String,
    pub ciphertext: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub rotated_from: Option<Uuid>,
}

pub struct IntegrationCredentialRepository {
    pool: sqlx::PgPool,
}

impl IntegrationCredentialRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &sqlx::PgPool {
        &self.pool
    }

    /// Insert a credential row. The caller's fence (WITH CHECK) rejects a
    /// cross-company insert server-side.
    pub async fn insert_on(&self, conn: &mut PgConnection, row: &NewCredentialRow) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query(
            r#"INSERT INTO sapiens.integration_credentials
                 (id, company_id, provider, account_ref, purpose, key_id, ciphertext,
                  status, expires_at, rotated_from)
               VALUES ($1, $2, $3, $4, $5::credential_purpose, $6, $7,
                       'active'::credential_status, $8, $9)"#,
        )
        .bind(id)
        .bind(row.company_id)
        .bind(&row.provider)
        .bind(&row.account_ref)
        .bind(row.purpose.to_string())
        .bind(&row.key_id)
        .bind(&row.ciphertext)
        .bind(row.expires_at)
        .bind(row.rotated_from)
        .execute(conn)
        .await?;
        Ok(id)
    }

    /// The ACTIVE credential for a scope, if any. Lineage rows (revoked /
    /// expired predecessors) are invisible here on purpose — reads serve only
    /// the live secret.
    pub async fn find_active_by_scope_on(
        &self,
        conn: &mut PgConnection,
        provider: &str,
        account_ref: &str,
        purpose: CredentialPurpose,
    ) -> Result<Option<IntegrationCredential>, sqlx::Error> {
        sqlx::query_as::<_, IntegrationCredential>(
            r#"SELECT id, company_id, provider, account_ref, purpose, key_id, ciphertext,
                      status, expires_at, rotated_from, last_used_at, metadata
                 FROM sapiens.integration_credentials
                WHERE company_id = NULLIF(current_setting('app.company_id', true), '')::uuid
                  AND provider = $1 AND account_ref = $2 AND purpose = $3::credential_purpose
                  AND status = 'active'::credential_status"#,
        )
        .bind(provider)
        .bind(account_ref)
        .bind(purpose.to_string())
        .fetch_optional(conn)
        .await
    }

    pub async fn find_by_id_on(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<IntegrationCredential>, sqlx::Error> {
        sqlx::query_as::<_, IntegrationCredential>(
            r#"SELECT id, company_id, provider, account_ref, purpose, key_id, ciphertext,
                      status, expires_at, rotated_from, last_used_at, metadata
                 FROM sapiens.integration_credentials
                WHERE company_id = NULLIF(current_setting('app.company_id', true), '')::uuid
                  AND id = $1"#,
        )
        .bind(id)
        .fetch_optional(conn)
        .await
    }

    /// Any credential for the scope regardless of status — lets read_secret
    /// distinguish "never issued" (NotFound) from "issued but terminal"
    /// (NotActive) so operators get an honest error.
    pub async fn find_any_by_scope_on(
        &self,
        conn: &mut PgConnection,
        provider: &str,
        account_ref: &str,
        purpose: CredentialPurpose,
    ) -> Result<Option<IntegrationCredential>, sqlx::Error> {
        sqlx::query_as::<_, IntegrationCredential>(
            r#"SELECT id, company_id, provider, account_ref, purpose, key_id, ciphertext,
                      status, expires_at, rotated_from, last_used_at, metadata
                 FROM sapiens.integration_credentials
                WHERE company_id = NULLIF(current_setting('app.company_id', true), '')::uuid
                  AND provider = $1 AND account_ref = $2 AND purpose = $3::credential_purpose"#,
        )
        .bind(provider)
        .bind(account_ref)
        .bind(purpose.to_string())
        .fetch_optional(conn)
        .await
    }

    /// Compare-and-set a lifecycle transition. Returns rows affected — 0 means
    /// the row was not in `from` (a concurrent transition won); callers decide
    /// whether that is an error or an idempotent noop.
    pub async fn cas_status_on(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        from: CredentialStatus,
        to: CredentialStatus,
    ) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(
            r#"UPDATE sapiens.integration_credentials
                  SET status = $3::credential_status
                WHERE company_id = NULLIF(current_setting('app.company_id', true), '')::uuid
                  AND id = $1 AND status = $2::credential_status"#,
        )
        .bind(id)
        .bind(from.to_string())
        .bind(to.to_string())
        .execute(conn)
        .await?;
        Ok(res.rows_affected())
    }

    pub async fn touch_last_used_on(&self, conn: &mut PgConnection, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE sapiens.integration_credentials SET last_used_at = now() \
             WHERE company_id = NULLIF(current_setting('app.company_id', true), '')::uuid AND id = $1",
        )
            .bind(id)
            .execute(conn)
            .await?;
        Ok(())
    }
}
