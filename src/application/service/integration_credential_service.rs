//! The credential store's verb surface (ADR-0024, minimal build).
//!
//! `issue` / `read_secret` / `rotate` / `revoke` / `describe` — no CRUD, no
//! delete, no secret in any response. Every verb runs inside
//! `with_company_scope(Some(company_id))` and on one transaction per verb, so
//! the strict RLS fence holds and multi-statement verbs (rotate: insert new +
//! CAS-revoke old) commit atomically.
//!
//! Fail-closed posture: with `CREDENTIAL_MASTER_KEY` unset the entire surface
//! refuses (issue cannot seal, read cannot open) rather than degrading to
//! plaintext storage. Reads of an expired credential CAS it to `expired` (the
//! lazy drift the lifecycle documents) and refuse; revoked credentials refuse.

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::domain::entity::{CredentialPurpose, CredentialStatus, IntegrationCredential};
use crate::infrastructure::persistence::{IntegrationCredentialRepository, NewCredentialRow};

use super::credential_crypto::{self, CredentialScope, CryptoError, ZeroizingSecret, CURRENT_KEY_ID};

#[derive(Debug, thiserror::Error)]
pub enum CredentialStoreError {
    #[error("credential master key is not configured — the credential store fails closed")]
    MissingMasterKey,
    #[error("no credential issued for this scope")]
    NotFound,
    #[error("the credential for this scope is {0} (terminal); issue or rotate a new one")]
    NotActive(CredentialStatus),
    #[error("the credential for this scope has expired; rotate it")]
    Expired,
    #[error("an active credential already exists for this scope; rotate it instead of issuing a second")]
    DuplicateActive,
    #[error("provider and account_ref must be non-empty slugs without ':' (scope encoding is colon-joined)")]
    InvalidScope,
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("crypto error: {0}")]
    Crypto(#[from] CryptoError),
}

/// Metadata-only view of a credential — the ONLY shape any HTTP response (or
/// log line) may carry. No ciphertext, no secret.
#[derive(Debug, serde::Serialize)]
pub struct CredentialDescriptor {
    pub id: Uuid,
    pub provider: String,
    pub account_ref: String,
    pub purpose: CredentialPurpose,
    pub key_id: String,
    pub status: CredentialStatus,
    /// The effective status at listing time (an active-but-past-expiry row
    /// reports `expired` without a write).
    pub effective_status: CredentialStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub rotated_from: Option<Uuid>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

impl CredentialDescriptor {
    fn from_row(row: &IntegrationCredential, now: DateTime<Utc>) -> Self {
        Self {
            id: row.id,
            provider: row.provider.clone(),
            account_ref: row.account_ref.clone(),
            purpose: row.purpose,
            key_id: row.key_id.clone(),
            status: row.status,
            effective_status: row.effective_status(now),
            expires_at: row.expires_at,
            rotated_from: row.rotated_from,
            last_used_at: row.last_used_at,
            created_at: row.metadata.created_at,
        }
    }
}

fn validate_scope_slug(s: &str, what: &'static str) -> Result<(), CredentialStoreError> {
    let ok = !s.is_empty()
        && s.len() <= 120
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.');
    if ok {
        Ok(())
    } else {
        tracing::warn!(field = what, "credential scope slug rejected");
        Err(CredentialStoreError::InvalidScope)
    }
}

fn is_unique_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db) if db.code().map(|c| c.as_ref() == "23505").unwrap_or(false))
}

pub struct IntegrationCredentialService {
    pool: PgPool,
    repository: IntegrationCredentialRepository,
}

impl IntegrationCredentialService {
    pub fn new(pool: PgPool) -> Self {
        let repository = IntegrationCredentialRepository::new(pool.clone());
        Self { pool, repository }
    }

    /// Issue the FIRST active credential for a scope (or re-issue after
    /// revocation). Seals the secret under the current KEK generation with the
    /// scope bound as AAD.
    pub async fn issue(
        &self,
        company_id: Uuid,
        provider: &str,
        account_ref: &str,
        purpose: CredentialPurpose,
        secret: Zeroizing<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<CredentialDescriptor, CredentialStoreError> {
        validate_scope_slug(provider, "provider")?;
        validate_scope_slug(account_ref, "account_ref")?;
        let master_key =
            credential_crypto::master_key_from_env().map_err(|_| CredentialStoreError::MissingMasterKey)?;
        let ciphertext = seal_for(&master_key, provider, account_ref, &purpose, secret.as_bytes())?;

        backbone_orm::company_scope::with_company_scope(Some(company_id), async {
            let mut tx = self.pool.begin().await?;
            backbone_orm::company_scope::bind_company_on(&mut tx, company_id).await?;

            let result = self
                .issue_on_tx(&mut tx, company_id, provider, account_ref, purpose, ciphertext, expires_at)
                .await;
            finish(tx, result).await
        })
        .await
    }

    /// The access-controlled read port: the ONLY path that opens a secret.
    /// Enforces expiry (lazily CASing `active → expired` on first observation)
    /// and revocation, stamps `last_used_at`, and returns the plaintext
    /// zeroized-on-drop. Callers: in-process seams (webhook verification,
    /// provider API clients) — never an HTTP response.
    pub async fn read_secret(
        &self,
        company_id: Uuid,
        provider: &str,
        account_ref: &str,
        purpose: CredentialPurpose,
    ) -> Result<ZeroizingSecret, CredentialStoreError> {
        let master_key =
            credential_crypto::master_key_from_env().map_err(|_| CredentialStoreError::MissingMasterKey)?;

        backbone_orm::company_scope::with_company_scope(Some(company_id), async {
            let mut tx = self.pool.begin().await?;
            backbone_orm::company_scope::bind_company_on(&mut tx, company_id).await?;

            let found = self
                .repository
                .find_active_by_scope_on(&mut tx, provider, account_ref, purpose)
                .await;
            let result = match found {
                Err(e) => Err(e.into()),
                // Honest error: never issued, or already terminal.
                Ok(None) => match self
                    .repository
                    .find_any_by_scope_on(&mut tx, provider, account_ref, purpose)
                    .await
                {
                    Err(e) => Err(e.into()),
                    Ok(Some(row)) => Err(CredentialStoreError::NotActive(row.status)),
                    Ok(None) => Err(CredentialStoreError::NotFound),
                },
                // Lazy expiry drift: the first read past expires_at flips the
                // row terminal, COMMITS the flip, then refuses the secret.
                // Commit-then-refuse is deliberate — routing this through
                // `finish` would roll the CAS back with the error, resurrecting
                // the drift for every future read; the flip must persist even
                // though this read refuses.
                Ok(Some(row)) if row.effective_status(Utc::now()) == CredentialStatus::Expired => {
                    return match self
                        .repository
                        .cas_status_on(&mut tx, row.id, CredentialStatus::Active, CredentialStatus::Expired)
                        .await
                    {
                        Err(e) => {
                            let _ = tx.rollback().await;
                            Err(e.into())
                        }
                        Ok(_) => match tx.commit().await {
                            Err(e) => Err(CredentialStoreError::from(e)),
                            Ok(()) => Err(CredentialStoreError::Expired),
                        },
                    };
                }
                Ok(Some(row)) => {
                    let secret = open_for(&master_key, &row, purpose);
                    match secret {
                        Err(e) => Err(e),
                        Ok(secret) => match self.repository.touch_last_used_on(&mut tx, row.id).await {
                            Err(e) => Err(e.into()),
                            Ok(()) => Ok(secret),
                        },
                    }
                }
            };
            finish(tx, result).await
        })
        .await
    }

    /// Replace the active credential atomically: the successor is inserted and
    /// the predecessor CAS-revoked in ONE transaction, so a crash can never
    /// leave two actives (the partial unique index backstops the fence) nor
    /// zero actives on a failure.
    pub async fn rotate(
        &self,
        company_id: Uuid,
        provider: &str,
        account_ref: &str,
        purpose: CredentialPurpose,
        new_secret: Zeroizing<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<CredentialDescriptor, CredentialStoreError> {
        validate_scope_slug(provider, "provider")?;
        validate_scope_slug(account_ref, "account_ref")?;
        let master_key =
            credential_crypto::master_key_from_env().map_err(|_| CredentialStoreError::MissingMasterKey)?;
        let ciphertext = seal_for(&master_key, provider, account_ref, &purpose, new_secret.as_bytes())?;

        backbone_orm::company_scope::with_company_scope(Some(company_id), async {
            let mut tx = self.pool.begin().await?;
            backbone_orm::company_scope::bind_company_on(&mut tx, company_id).await?;

            let result = self
                .rotate_on_tx(&mut tx, company_id, provider, account_ref, purpose, ciphertext, expires_at)
                .await;
            finish(tx, result).await
        })
        .await
    }

    /// Withdraw a credential by id. Revoking a non-active credential is a
    /// no-op success (idempotent), matching operator intent.
    pub async fn revoke(&self, company_id: Uuid, credential_id: Uuid) -> Result<(), CredentialStoreError> {
        backbone_orm::company_scope::with_company_scope(Some(company_id), async {
            let mut tx = self.pool.begin().await?;
            backbone_orm::company_scope::bind_company_on(&mut tx, company_id).await?;

            let result = self.revoke_on_tx(&mut tx, credential_id).await;
            finish(tx, result).await
        })
        .await
    }

    /// Metadata-only listing of a scope's credential lineage. Never exposes
    /// ciphertext.
    pub async fn describe(
        &self,
        company_id: Uuid,
        provider: &str,
        account_ref: &str,
    ) -> Result<Vec<CredentialDescriptor>, CredentialStoreError> {
        backbone_orm::company_scope::with_company_scope(Some(company_id), async {
            let mut tx = self.pool.begin().await?;
            backbone_orm::company_scope::bind_company_on(&mut tx, company_id).await?;

            let rows = sqlx::query_as::<_, IntegrationCredential>(
                r#"SELECT id, company_id, provider, account_ref, purpose, key_id, ciphertext,
                          status, expires_at, rotated_from, last_used_at, metadata
                     FROM sapiens.integration_credentials
                    WHERE company_id = NULLIF(current_setting('app.company_id', true), '')::uuid
                      AND provider = $1 AND account_ref = $2
                    ORDER BY (metadata->>'created_at') DESC NULLS LAST"#,
            )
            .bind(provider)
            .bind(account_ref)
            .fetch_all(&mut *tx)
            .await
            .map_err(CredentialStoreError::Database);
            finish(tx, rows).await
        })
        .await
        .map(|rows| {
            let now = Utc::now();
            rows.iter().map(|r| CredentialDescriptor::from_row(r, now)).collect()
        })
    }

    /// Metadata-only fetch of one credential by id (the rotate-by-id route
    /// resolves the scope triple through this). Never exposes ciphertext.
    pub async fn describe_by_id(
        &self,
        company_id: Uuid,
        credential_id: Uuid,
    ) -> Result<Option<CredentialDescriptor>, CredentialStoreError> {
        backbone_orm::company_scope::with_company_scope(Some(company_id), async {
            let mut tx = self.pool.begin().await?;
            backbone_orm::company_scope::bind_company_on(&mut tx, company_id).await?;

            let result = match self.repository.find_by_id_on(&mut tx, credential_id).await {
                Err(e) => Err(e.into()),
                Ok(None) => Ok(None),
                Ok(Some(row)) => {
                    let now = Utc::now();
                    Ok(Some(CredentialDescriptor::from_row(&row, now)))
                }
            };
            finish(tx, result).await
        })
        .await
    }

    // ── transaction bodies ────────────────────────────────────────────────────

    async fn issue_on_tx(
        &self,
        tx: &mut sqlx::PgConnection,
        company_id: Uuid,
        provider: &str,
        account_ref: &str,
        purpose: CredentialPurpose,
        ciphertext: String,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<CredentialDescriptor, CredentialStoreError> {
        // Refuse a silent second active credential: rotation is the only
        // sanctioned replacement path (it keeps lineage).
        if self
            .repository
            .find_active_by_scope_on(tx, provider, account_ref, purpose)
            .await?
            .is_some()
        {
            return Err(CredentialStoreError::DuplicateActive);
        }

        let row = NewCredentialRow {
            company_id,
            provider: provider.to_string(),
            account_ref: account_ref.to_string(),
            purpose,
            key_id: CURRENT_KEY_ID.to_string(),
            ciphertext,
            expires_at,
            rotated_from: None,
        };
        let id = self.repository.insert_on(tx, &row).await.map_err(|e| {
            if is_unique_violation(&e) {
                CredentialStoreError::DuplicateActive
            } else {
                e.into()
            }
        })?;
        let created = self
            .repository
            .find_by_id_on(tx, id)
            .await?
            .ok_or(CredentialStoreError::NotFound)?;
        Ok(CredentialDescriptor::from_row(&created, Utc::now()))
    }

    async fn rotate_on_tx(
        &self,
        tx: &mut sqlx::PgConnection,
        company_id: Uuid,
        provider: &str,
        account_ref: &str,
        purpose: CredentialPurpose,
        ciphertext: String,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<CredentialDescriptor, CredentialStoreError> {
        let predecessor = self
            .repository
            .find_active_by_scope_on(tx, provider, account_ref, purpose)
            .await?
            .ok_or(CredentialStoreError::NotFound)?;

        // An expired-but-still-active row is exactly what rotation repairs —
        // the CAS targets status 'active' because the expiry drift may not
        // have been observed yet.
        if self
            .repository
            .cas_status_on(tx, predecessor.id, CredentialStatus::Active, CredentialStatus::Revoked)
            .await?
            != 1
        {
            return Err(CredentialStoreError::NotActive(predecessor.status));
        }

        let row = NewCredentialRow {
            company_id,
            provider: provider.to_string(),
            account_ref: account_ref.to_string(),
            purpose,
            key_id: CURRENT_KEY_ID.to_string(),
            ciphertext,
            expires_at,
            rotated_from: Some(predecessor.id),
        };
        let id = self.repository.insert_on(tx, &row).await?;
        let created = self
            .repository
            .find_by_id_on(tx, id)
            .await?
            .ok_or(CredentialStoreError::NotFound)?;
        Ok(CredentialDescriptor::from_row(&created, Utc::now()))
    }

    async fn revoke_on_tx(
        &self,
        tx: &mut sqlx::PgConnection,
        credential_id: Uuid,
    ) -> Result<(), CredentialStoreError> {
        let row = self
            .repository
            .find_by_id_on(tx, credential_id)
            .await?
            .ok_or(CredentialStoreError::NotFound)?;

        if row.status == CredentialStatus::Active {
            self.repository
                .cas_status_on(tx, row.id, CredentialStatus::Active, CredentialStatus::Revoked)
                .await?;
        }
        Ok(())
    }
}

/// Commit on success, roll back on failure — the one exit path every verb
/// shares, so no error branch can leak a half-open transaction.
async fn finish<T, E>(
    tx: sqlx::Transaction<'_, sqlx::Postgres>,
    result: Result<T, E>,
) -> Result<T, E>
where
    E: From<sqlx::Error>,
{
    match result {
        Ok(v) => match tx.commit().await {
            Ok(()) => Ok(v),
            Err(e) => Err(e.into()),
        },
        Err(e) => {
            let _ = tx.rollback().await;
            Err(e)
        }
    }
}

fn seal_for(
    master_key: &[u8],
    provider: &str,
    account_ref: &str,
    purpose: &CredentialPurpose,
    secret: &[u8],
) -> Result<String, CredentialStoreError> {
    let scope = CredentialScope {
        provider: provider.to_string(),
        account_ref: account_ref.to_string(),
        purpose: purpose.to_string(),
    };
    credential_crypto::seal(master_key, &scope, secret).map_err(CredentialStoreError::Crypto)
}

fn open_for(
    master_key: &[u8],
    row: &IntegrationCredential,
    purpose: CredentialPurpose,
) -> Result<ZeroizingSecret, CredentialStoreError> {
    let scope = CredentialScope {
        provider: row.provider.clone(),
        account_ref: row.account_ref.clone(),
        purpose: purpose.to_string(),
    };
    credential_crypto::open(master_key, &row.key_id, &scope, &row.ciphertext)
        .map_err(CredentialStoreError::Crypto)
}
