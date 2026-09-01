//! Trusted-device keys — scoped, expiring, audit-trailed credentials for
//! remembered devices.
//!
//! A trusted device is NOT a boolean on a fingerprint: it is a random key the
//! device presents, stored ONLY as a hash, with a declared SCOPE (what the key
//! may do), a hard EXPIRY, and a full audit trail (issued at, issued from,
//! last used). Keys are revoked wholesale on password change
//! ([`DeviceTrustKeyService::revoke_all_for_user`]), so a stolen remembered
//! device dies with the password rotation.
//!
//! Expiry is CLAMPED: whatever the caller asks for, the key never outlives the
//! MFA session timeout — a remembered device extends the MFA prompt window, it
//! does not create a second, longer-lived session class. When a dedicated
//! auth-timeout configuration lands, the clamp takes the configured MFA
//! timeout instead of this constant (the clamp point is the single
//! [`Self::max_trust_age`] function).

use chrono::{DateTime, TimeDelta, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::infrastructure::auth::crypto;

/// The MFA prompt window a remembered device may bridge. Matches the
/// session-management service's 30-minute session window.
pub const MFA_SESSION_TIMEOUT: TimeDelta = TimeDelta::minutes(30);

/// What an issued key may be used for. Scopes are explicit strings so a key
/// minted for one purpose never authorizes another.
pub const SCOPE_MFA_STEP_UP: &str = "mfa:step_up";

/// An issued trusted-device key: the plaintext key is returned ONCE at issue
/// time; only its hash is persisted.
#[derive(Debug)]
pub struct IssuedTrustKey {
    pub key_id: Uuid,
    pub key: String,
    pub scope: String,
    pub expires_at: DateTime<Utc>,
}

/// A key that verified on presentation.
#[derive(Debug)]
pub struct TrustedDevice {
    pub key_id: Uuid,
    pub user_id: Uuid,
    pub scope: String,
}

pub struct DeviceTrustKeyService {
    pool: PgPool,
}

impl DeviceTrustKeyService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// The longest a trusted-device key may live: the requested lifetime
    /// clamped to the MFA session timeout.
    pub fn max_trust_age() -> TimeDelta {
        MFA_SESSION_TIMEOUT
    }

    /// Issue a key for a user's device. `requested_ttl` is clamped to
    /// [`Self::max_trust_age`]. The audit trail (issued_at, issued_ip, scope,
    /// fingerprint) lives on the row; `reason` records why it was issued.
    pub async fn issue(
        &self,
        user_id: Uuid,
        device_fingerprint: Option<&str>,
        scope: &str,
        requested_ttl: TimeDelta,
        issued_ip: Option<&str>,
        reason: &str,
    ) -> Result<IssuedTrustKey, sqlx::Error> {
        let ttl = requested_ttl.min(Self::max_trust_age());
        let key = crypto::generate_refresh_token();
        let key_hash = crypto::hash_token(&key);
        let key_id = Uuid::new_v4();
        let expires_at = Utc::now() + ttl;
        let metadata = serde_json::json!({
            "issued_reason": reason,
            "requested_ttl_seconds": requested_ttl.num_seconds(),
            "granted_ttl_seconds": ttl.num_seconds(),
        });

        sqlx::query(
            "INSERT INTO sapiens.device_trust_keys \
             (id, user_id, key_hash, scope, device_fingerprint, expires_at, issued_ip, metadata) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8::jsonb)",
        )
        .bind(key_id)
        .bind(user_id)
        .bind(&key_hash)
        .bind(scope)
        .bind(device_fingerprint)
        .bind(expires_at)
        .bind(issued_ip)
        .bind(metadata.to_string())
        .execute(&self.pool)
        .await?;

        Ok(IssuedTrustKey { key_id, key, scope: scope.to_string(), expires_at })
    }

    /// Verify a presented key: unrevoked, unexpired, scope-matching. Updates
    /// `last_used_at` (the audit trail's usage half) on success.
    pub async fn verify(&self, presented_key: &str, scope: &str) -> Option<TrustedDevice> {
        let key_hash = crypto::hash_token(presented_key);
        let row: Option<(Uuid, Uuid, String)> = sqlx::query_as(
            "UPDATE sapiens.device_trust_keys \
             SET last_used_at = NOW() \
             WHERE key_hash = $1 AND scope = $2 \
               AND revoked_at IS NULL AND expires_at > NOW() \
             RETURNING id, user_id, scope",
        )
        .bind(&key_hash)
        .bind(scope)
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten();

        row.map(|(key_id, user_id, scope)| TrustedDevice { key_id, user_id, scope })
    }

    /// Revoke one key by id.
    pub async fn revoke(&self, key_id: Uuid) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(
            "UPDATE sapiens.device_trust_keys SET revoked_at = NOW() \
             WHERE id = $1 AND revoked_at IS NULL",
        )
        .bind(key_id)
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected())
    }

    /// Revoke every outstanding key for a user. Called from the password
    /// change and reset paths — a password rotation kills every remembered
    /// device, not just the one that asked.
    pub async fn revoke_all_for_user(&self, user_id: Uuid) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(
            "UPDATE sapiens.device_trust_keys SET revoked_at = NOW() \
             WHERE user_id = $1 AND revoked_at IS NULL",
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected())
    }
}
