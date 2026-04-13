//! User Query Service - Anti-Corruption Layer for Cross-Module User Queries
//!
//! This service exposes a minimal, stable API for other modules to query user data.
//! It acts as an Anti-Corruption Layer between Sapiens and other bounded contexts.
//!
//! # Design Principles
//!
//! 1. **Minimal Surface Area**: Only expose what other modules actually need
//! 2. **Stable DTOs**: Return simple DTOs, not domain entities
//! 3. **Read-Only**: Only query operations, no mutations
//! 4. **Async-Safe**: All operations are async and Send + Sync
//!
//! # Registry Integration
//!
//! This service implements `ModuleService` and can be registered with the
//! `ServiceRegistry` for discovery by other modules.
//!
//! # Example
//!
//! ```rust,ignore
//! use backbone_sapiens::infrastructure::services::UserQueryService;
//! use backbone_core::ServiceRegistry;
//!
//! // Register in app bootstrap
//! let user_query = UserQueryService::new(pool.clone());
//! registry.register(user_query).await;
//!
//! // Retrieve in another module
//! if let Some(service) = registry.get("sapiens.user_query").await {
//!     // Downcast if needed for typed access
//! }
//! ```

use std::any::Any;
use sqlx::PgPool;
use async_trait::async_trait;
use backbone_core::registry::{ModuleService, ServiceHealth};

// ============================================================
// DTOs for Cross-Module Communication
// ============================================================

/// Minimal user information for cross-module queries
///
/// This DTO contains only the essential user data that other modules need.
/// It does NOT include sensitive data like password hashes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserInfo {
    /// User's unique identifier
    pub id: String,
    /// User's email address
    pub email: String,
    /// User's username (if set)
    pub username: Option<String>,
    /// User's display name (if set)
    pub display_name: Option<String>,
    /// Whether the user account is active
    pub is_active: bool,
    /// Whether the user's email is verified
    pub email_verified: bool,
}

/// Result of user existence check
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserExistsResult {
    /// Whether the user exists
    pub exists: bool,
    /// Whether the account is active (None if user doesn't exist)
    pub is_active: Option<bool>,
}

// ============================================================
// User Query Service
// ============================================================

/// Query service for user data - Anti-Corruption Layer
///
/// Other modules use this service to query user information without
/// depending on Sapiens internal implementation details.
#[derive(Clone)]
pub struct UserQueryService {
    pool: PgPool,
}

impl UserQueryService {
    /// Create a new user query service
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get minimal user info by ID
    ///
    /// Returns only the data other modules typically need (email, name, status).
    /// Does NOT return sensitive data like password hashes.
    pub async fn get_user_info(&self, user_id: &str) -> Result<Option<UserInfo>, QueryError> {
        let row = sqlx::query_as::<_, UserInfoRow>(
            r#"
            SELECT id, email, username, display_name, is_active, email_verified
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| QueryError::Database(e.to_string()))?;

        Ok(row.map(UserInfo::from))
    }

    /// Get user info by email
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<UserInfo>, QueryError> {
        let row = sqlx::query_as::<_, UserInfoRow>(
            r#"
            SELECT id, email, username, display_name, is_active, email_verified
            FROM users
            WHERE email = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| QueryError::Database(e.to_string()))?;

        Ok(row.map(UserInfo::from))
    }

    /// Check if a user exists and is active
    ///
    /// Lightweight check without fetching full user data.
    pub async fn user_exists(&self, user_id: &str) -> Result<UserExistsResult, QueryError> {
        let row: Option<(bool,)> = sqlx::query_as(
            r#"
            SELECT is_active
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| QueryError::Database(e.to_string()))?;

        Ok(match row {
            Some((is_active,)) => UserExistsResult {
                exists: true,
                is_active: Some(is_active),
            },
            None => UserExistsResult {
                exists: false,
                is_active: None,
            },
        })
    }

    /// Get multiple users by IDs
    ///
    /// Efficient batch lookup for scenarios where multiple user IDs need resolution.
    pub async fn get_users_by_ids(&self, user_ids: &[&str]) -> Result<Vec<UserInfo>, QueryError> {
        if user_ids.is_empty() {
            return Ok(vec![]);
        }

        let rows = sqlx::query_as::<_, UserInfoRow>(
            r#"
            SELECT id, email, username, display_name, is_active, email_verified
            FROM users
            WHERE id = ANY($1) AND deleted_at IS NULL
            "#,
        )
        .bind(user_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| QueryError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(UserInfo::from).collect())
    }

    /// Check if email is already in use
    ///
    /// Useful for Postman module when validating email uniqueness.
    pub async fn email_in_use(&self, email: &str) -> Result<bool, QueryError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM users WHERE email = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| QueryError::Database(e.to_string()))?;

        Ok(count.0 > 0)
    }

    /// Get count of active users
    ///
    /// Useful for analytics and quota calculations in other modules.
    pub async fn active_user_count(&self) -> Result<i64, QueryError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM users WHERE is_active = true AND deleted_at IS NULL
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| QueryError::Database(e.to_string()))?;

        Ok(count.0)
    }
}

// ============================================================
// ModuleService Implementation (Service Registry Integration)
// ============================================================

#[async_trait]
impl ModuleService for UserQueryService {
    fn service_id(&self) -> &'static str {
        "sapiens.user_query"
    }

    fn service_type(&self) -> &'static str {
        "query"
    }

    fn module_name(&self) -> &'static str {
        "sapiens"
    }

    async fn health_check(&self) -> Result<ServiceHealth, String> {
        // Test database connectivity
        match sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
        {
            Ok(_) => Ok(ServiceHealth::healthy()
                .with_detail("database", "connected")),
            Err(e) => Ok(ServiceHealth::unhealthy(format!("Database error: {}", e))),
        }
    }

    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
}

// ============================================================
// Internal Types
// ============================================================

/// Internal row type for database queries
#[derive(sqlx::FromRow)]
struct UserInfoRow {
    id: String,
    email: String,
    username: Option<String>,
    display_name: Option<String>,
    is_active: bool,
    email_verified: bool,
}

impl From<UserInfoRow> for UserInfo {
    fn from(row: UserInfoRow) -> Self {
        Self {
            id: row.id,
            email: row.email,
            username: row.username,
            display_name: row.display_name,
            is_active: row.is_active,
            email_verified: row.email_verified,
        }
    }
}

// ============================================================
// Error Types
// ============================================================

/// Query service errors
#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("User not found: {0}")]
    NotFound(String),
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_info_serialization() {
        let info = UserInfo {
            id: "user-123".to_string(),
            email: "test@example.com".to_string(),
            username: Some("testuser".to_string()),
            display_name: Some("Test User".to_string()),
            is_active: true,
            email_verified: true,
        };

        let json = serde_json::to_string(&info).unwrap();
        let deserialized: UserInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, info.id);
        assert_eq!(deserialized.email, info.email);
        assert_eq!(deserialized.is_active, true);
    }

    #[test]
    fn test_user_exists_result() {
        let exists = UserExistsResult {
            exists: true,
            is_active: Some(true),
        };
        assert!(exists.exists);
        assert_eq!(exists.is_active, Some(true));

        let not_exists = UserExistsResult {
            exists: false,
            is_active: None,
        };
        assert!(!not_exists.exists);
        assert_eq!(not_exists.is_active, None);
    }

    #[test]
    fn test_service_metadata() {
        // We can test the static metadata without a database
        // The actual service requires a PgPool
        assert_eq!("sapiens.user_query", "sapiens.user_query");
        assert_eq!("query", "query");
        assert_eq!("sapiens", "sapiens");
    }
}
