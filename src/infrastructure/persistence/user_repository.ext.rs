// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

//! The extension of `user_repository.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

use chrono::{DateTime, Utc};
use uuid::Uuid;
impl UserRepository {
    /// Find a user by email (for authentication purposes).
    ///
    /// Returns the full user row including password_hash.
    pub async fn find_by_email_for_auth(&self, email: &str) -> Result<Option<User>> {
        let result = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1 AND metadata->>'deleted_at' IS NULL",
        )
        .bind(email)
        .fetch_optional(self.pool())
        .await?;
        Ok(result)
    }
    /// Check whether an email address is already registered.
    pub async fn email_exists(&self, email: &str) -> Result<bool> {
        let row: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 AND metadata->>'deleted_at' IS NULL)",
        )
        .bind(email)
        .fetch_one(self.pool())
        .await?;
        Ok(row.0)
    }
    /// Increment the failed login counter and optionally set a lock expiry.
    pub async fn increment_failed_attempts(
        &self,
        user_id: Uuid,
        lock_until: Option<DateTime<Utc>>,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE users SET failed_login_attempts = failed_login_attempts + 1, locked_until = $2 WHERE id = $1",
        )
        .bind(user_id)
        .bind(lock_until)
        .execute(self.pool())
        .await?;
        Ok(())
    }
    /// Reset the failed-login counter and record the login timestamp.
    pub async fn record_successful_login(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE users SET failed_login_attempts = 0, last_login = NOW(), locked_until = NULL WHERE id = $1",
        )
        .bind(user_id)
        .execute(self.pool())
        .await?;
        Ok(())
    }
    /// Return the role names assigned to a user (via user_roles → roles join).
    pub async fn lookup_user_roles(&self, user_id: Uuid) -> Vec<String> {
        let result: Result<Vec<(String,)>, _> = sqlx::query_as(
            // A revoked grant (soft-deleted user_role) or a deleted role grants nothing.
            "SELECT r.name FROM roles r JOIN user_roles ur ON r.id = ur.role_id \
             WHERE ur.user_id = $1 AND (ur.metadata->>'deleted_at') IS NULL AND (r.metadata->>'deleted_at') IS NULL",
        )
        .bind(user_id)
        .fetch_all(self.pool())
        .await;
        result.unwrap_or_default().into_iter().map(|(n,)| n).collect()
    }
    /// Retrieve the raw metadata JSON blob for a user.
    ///
    /// Returns an empty object `{}` if the user has no metadata or is not found.
    pub async fn get_user_metadata_raw(&self, user_id: Uuid) -> Result<serde_json::Value> {
        let row: Option<(serde_json::Value,)> =
            sqlx::query_as("SELECT metadata FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_optional(self.pool())
                .await?;
        Ok(row.map(|(v,)| v).unwrap_or_else(|| serde_json::json!({})))
    }
    /// Update a user's username.
    pub async fn update_username(&self, user_id: Uuid, username: &str) -> Result<()> {
        sqlx::query("UPDATE users SET username = $2 WHERE id = $1")
            .bind(user_id)
            .bind(username)
            .execute(self.pool())
            .await?;
        Ok(())
    }
    /// Merge (shallow-patch) arbitrary JSON into the user's metadata column.
    pub async fn merge_metadata(&self, user_id: Uuid, metadata: &serde_json::Value) -> Result<()> {
        sqlx::query(
            "UPDATE users SET metadata = COALESCE(metadata, '{}'::jsonb) || $2 WHERE id = $1",
        )
        .bind(user_id)
        .bind(metadata)
        .execute(self.pool())
        .await?;
        Ok(())
    }
}
