//! Authentication middleware
//!
//! Provides authentication context and user extraction from JWT tokens.

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use uuid::Uuid;

/// Authenticated user context
#[derive(Debug, Clone)]
pub struct AuthContext {
    /// User ID
    pub user_id: Uuid,
    /// Username
    pub username: String,
    /// User's email
    pub email: String,
    /// User's roles (e.g., ["admin", "user"])
    pub roles: Vec<String>,
    /// User's direct permissions
    pub permissions: Vec<String>,
    /// Session ID
    pub session_id: Option<Uuid>,
    /// Whether the user's email is verified
    pub email_verified: bool,
}

impl AuthContext {
    /// Create a system context for internal operations
    pub fn system() -> Self {
        Self {
            user_id: Uuid::nil(),
            username: "system".to_string(),
            email: "system@internal".to_string(),
            roles: vec!["system".to_string()],
            permissions: vec!["*".to_string()],
            session_id: None,
            email_verified: true,
        }
    }

    /// Create an anonymous/guest context
    pub fn anonymous() -> Self {
        Self {
            user_id: Uuid::nil(),
            username: "anonymous".to_string(),
            email: "".to_string(),
            roles: vec!["guest".to_string()],
            permissions: vec![],
            session_id: None,
            email_verified: false,
        }
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role || r == "super_admin")
    }

    /// Check if user has any of the specified roles
    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|r| self.has_role(r))
    }

    /// Check if user has all of the specified roles
    pub fn has_all_roles(&self, roles: &[&str]) -> bool {
        roles.iter().all(|r| self.has_role(r))
    }

    /// Check if user has a specific permission
    pub fn has_permission(&self, permission: &str) -> bool {
        // Wildcard permission grants everything
        if self.permissions.contains(&"*".to_string()) {
            return true;
        }

        // Check direct permission
        if self.permissions.contains(&permission.to_string()) {
            return true;
        }

        // Check wildcard patterns (e.g., "users:*" matches "users:read")
        let parts: Vec<&str> = permission.split(':').collect();
        if parts.len() >= 2 {
            let wildcard = format!("{}:*", parts[0]);
            if self.permissions.contains(&wildcard) {
                return true;
            }
        }

        false
    }

    /// Check if user is authenticated (not anonymous)
    pub fn is_authenticated(&self) -> bool {
        self.user_id != Uuid::nil() && !self.roles.contains(&"guest".to_string())
    }

    /// Check if user is a super admin
    pub fn is_super_admin(&self) -> bool {
        self.has_role("super_admin")
    }

    /// Check if user is an admin (admin or super_admin)
    pub fn is_admin(&self) -> bool {
        self.has_role("admin") || self.is_super_admin()
    }

    /// Check if user owns a resource
    pub fn owns(&self, owner_id: &Uuid) -> bool {
        &self.user_id == owner_id
    }
}

/// Wrapper for the current authenticated user
#[derive(Debug, Clone)]
pub struct CurrentUser(pub AuthContext);

impl CurrentUser {
    pub fn inner(&self) -> &AuthContext {
        &self.0
    }
}

/// Trait for authentication providers
#[async_trait::async_trait]
pub trait AuthMiddleware: Send + Sync + 'static {
    /// Validate a token and return the auth context
    async fn validate_token(&self, token: &str) -> Result<AuthContext, AuthError>;

    /// Extract token from request headers
    fn extract_token(&self, headers: &axum::http::HeaderMap) -> Option<String> {
        headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .map(|s| s.to_string())
    }
}

/// Authentication error
#[derive(Debug, Clone, thiserror::Error)]
pub enum AuthError {
    #[error("Missing authentication token")]
    MissingToken,

    #[error("Invalid authentication token")]
    InvalidToken,

    #[error("Token has expired")]
    TokenExpired,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("User not found")]
    UserNotFound,

    #[error("Session invalidated")]
    SessionInvalidated,

    #[error("Account locked")]
    AccountLocked,

    #[error("Email not verified")]
    EmailNotVerified,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authentication token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authentication token"),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token has expired"),
            AuthError::InsufficientPermissions => (StatusCode::FORBIDDEN, "Insufficient permissions"),
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "User not found"),
            AuthError::SessionInvalidated => (StatusCode::UNAUTHORIZED, "Session has been invalidated"),
            AuthError::AccountLocked => (StatusCode::FORBIDDEN, "Account is locked"),
            AuthError::EmailNotVerified => (StatusCode::FORBIDDEN, "Email not verified"),
        };

        let body = serde_json::json!({
            "error": {
                "code": format!("{:?}", self).to_uppercase(),
                "message": message,
            }
        });

        (status, axum::Json(body)).into_response()
    }
}

/// Extractor for authentication context
#[derive(Debug, Clone)]
pub struct AuthExtractor(pub AuthContext);

impl std::ops::Deref for AuthExtractor {
    type Target = AuthContext;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for AuthExtractor
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthContext>()
            .cloned()
            .map(AuthExtractor)
            .ok_or(AuthError::MissingToken)
    }
}

/// Optional auth extractor - returns anonymous context if not authenticated
#[derive(Debug, Clone)]
pub struct OptionalAuth(pub AuthContext);

impl std::ops::Deref for OptionalAuth {
    type Target = AuthContext;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for OptionalAuth
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let ctx = parts
            .extensions
            .get::<AuthContext>()
            .cloned()
            .unwrap_or_else(AuthContext::anonymous);
        Ok(OptionalAuth(ctx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_role() {
        let ctx = AuthContext {
            user_id: Uuid::new_v4(),
            username: "admin".to_string(),
            email: "admin@test.com".to_string(),
            roles: vec!["admin".to_string(), "user".to_string()],
            permissions: vec![],
            session_id: None,
            email_verified: true,
        };

        assert!(ctx.has_role("admin"));
        assert!(ctx.has_role("user"));
        assert!(!ctx.has_role("super_admin"));
    }

    #[test]
    fn test_super_admin_has_all_roles() {
        let ctx = AuthContext {
            user_id: Uuid::new_v4(),
            username: "superadmin".to_string(),
            email: "super@test.com".to_string(),
            roles: vec!["super_admin".to_string()],
            permissions: vec![],
            session_id: None,
            email_verified: true,
        };

        assert!(ctx.has_role("admin")); // super_admin implies all roles
        assert!(ctx.has_role("user"));
    }

    #[test]
    fn test_has_permission() {
        let ctx = AuthContext {
            user_id: Uuid::new_v4(),
            username: "user".to_string(),
            email: "user@test.com".to_string(),
            roles: vec!["user".to_string()],
            permissions: vec!["users:read".to_string(), "profiles:*".to_string()],
            session_id: None,
            email_verified: true,
        };

        assert!(ctx.has_permission("users:read"));
        assert!(!ctx.has_permission("users:write"));
        assert!(ctx.has_permission("profiles:read")); // wildcard
        assert!(ctx.has_permission("profiles:write")); // wildcard
    }

    #[test]
    fn test_wildcard_permission() {
        let ctx = AuthContext {
            user_id: Uuid::new_v4(),
            username: "admin".to_string(),
            email: "admin@test.com".to_string(),
            roles: vec!["admin".to_string()],
            permissions: vec!["*".to_string()],
            session_id: None,
            email_verified: true,
        };

        assert!(ctx.has_permission("anything:here"));
        assert!(ctx.has_permission("users:delete"));
    }
}
