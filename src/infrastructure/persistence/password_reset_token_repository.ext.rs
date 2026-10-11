// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

//! The extension of `password_reset_token_repository.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

use chrono::{DateTime, Utc};
use uuid::Uuid;
impl PasswordResetTokenRepository {
    /// Insert a new password-reset token row.
    ///
    /// Arguments: `user_id`, `token_hash` (hashed OTP), `expires_at`, and raw
    /// `metadata` JSON value.
    pub async fn create_reset_token(
        &self,
        user_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
        metadata: &serde_json::Value,
    ) -> Result<PasswordResetToken> {
        let id = Uuid::new_v4();
        let sql = format!(
            "INSERT INTO {TABLE_NAME} \
             (id, user_id, token_hash, expires_at, is_used, metadata) \
             VALUES ($1, $2, $3, $4, false, $5) \
             RETURNING *"
        );
        let result = sqlx::query_as::<_, PasswordResetToken>(&sql)
        .bind(id)
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .bind(metadata)
        .fetch_one(self.pool())
        .await?;
        Ok(result)
    }
    /// Find a valid (unused and not expired) password-reset token by its hash.
    pub async fn find_valid_by_hash(&self, token_hash: &str) -> Result<Option<PasswordResetToken>> {
        let sql = format!(
            "SELECT * FROM {TABLE_NAME} \
             WHERE token_hash = $1 AND is_used = false AND expires_at > NOW() \
             LIMIT 1"
        );
        let result = sqlx::query_as::<_, PasswordResetToken>(&sql)
        .bind(token_hash)
        .fetch_optional(self.pool())
        .await?;
        Ok(result)
    }
}
