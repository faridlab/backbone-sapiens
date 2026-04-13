//! Authentication Routes
//!
//! Defines all authentication-related HTTP endpoints and their routing configuration.

use axum::{
    routing::{get, post},
    Router,
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;

use crate::handlers::auth::{
    login_handler, enhanced_login_handler, logout_handler, simple_logout_handler,
    register_handler, check_username_handler, check_email_handler,
    refresh_token_handler, validate_refresh_token_handler, revoke_refresh_token_handler,
    verify_email_handler, verify_email_get_handler, resend_verification_handler, check_verification_status_handler,
    forgot_password_handler, check_reset_token_handler, resend_reset_email_handler,
    reset_password_handler, validate_reset_token_handler,
    auth_status_handler, auth_status_by_user_handler, user_sessions_handler,
};
use crate::handlers::AppState;

/// Create authentication router
pub fn create_auth_router() -> Router<AppState> {
    Router::new()
        // Core authentication endpoints
        .route("/login", post(enhanced_login_handler))
        .route("/login/simple", post(login_handler))
        .route("/logout", post(simple_logout_handler))
        .route("/logout/:session_id", post(logout_handler))
        .route("/register", post(register_handler))
        .route("/refresh", post(refresh_token_handler))
        .route("/validate-token", post(validate_refresh_token_handler))
        .route("/revoke-token", post(revoke_refresh_token_handler))

        // Email verification endpoints
        .route("/verify-email", post(verify_email_handler))
        .route("/verify-email", get(verify_email_get_handler))
        .route("/resend-verification", post(resend_verification_handler))
        .route("/check-verification/:email", get(check_verification_status_handler))

        // Password reset endpoints
        .route("/forgot-password", post(forgot_password_handler))
        .route("/check-reset-token", post(check_reset_token_handler))
        .route("/resend-reset-email", post(resend_reset_email_handler))
        .route("/reset-password", post(reset_password_handler))
        .route("/validate-reset-token", post(validate_reset_token_handler))

        // Auth status endpoints
        .route("/status/:session_id", get(auth_status_handler))
        .route("/status/user/:user_id", get(auth_status_by_user_handler))
        .route("/sessions/:user_id", get(user_sessions_handler))

        // Registration validation endpoints
        .route("/check-username", post(check_username_handler))
        .route("/check-email", post(check_email_handler))

        // Health check
        .route("/health", get(auth_health_handler))
}

/// Authentication health check handler
async fn auth_health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "service": "authentication",
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": "1.0.0",
        "endpoints": {
            "core": [
                "POST /api/v1/auth/login",
                "POST /api/v1/auth/logout",
                "POST /api/v1/auth/register",
                "POST /api/v1/auth/refresh"
            ],
            "verification": [
                "POST /api/v1/auth/verify-email",
                "GET /api/v1/auth/verify-email",
                "POST /api/v1/auth/resend-verification",
                "GET /api/v1/auth/check-verification/:email"
            ],
            "password_reset": [
                "POST /api/v1/auth/forgot-password",
                "POST /api/v1/auth/reset-password",
                "POST /api/v1/auth/check-reset-token",
                "POST /api/v1/auth/resend-reset-email"
            ],
            "status": [
                "GET /api/v1/auth/status/:session_id",
                "GET /api/v1/auth/status/user/:user_id",
                "GET /api/v1/auth/sessions/:user_id"
            ],
            "validation": [
                "POST /api/v1/auth/check-username",
                "POST /api/v1/auth/check-email"
            ]
        }
    }))
}

/// Get all authentication routes with proper API versioning
pub fn get_auth_routes() -> Router<AppState> {
    Router::new()
        .nest("/api/v1/auth", create_auth_router())
}

/// Authentication middleware configuration
pub mod middleware {
    use axum::{
        extract::Request,
        http::{StatusCode, header},
        middleware::{self, Next},
        response::Response,
    };
    use std::sync::Arc;
    use tower::ServiceBuilder;
    use tower_http::cors::{CorsLayer, Any};

    /// CORS configuration for authentication endpoints
    pub fn cors_layer() -> CorsLayer {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_credentials(true)
    }

    /// Rate limiting middleware for authentication endpoints
    pub async fn rate_limit_middleware(
        request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        // TODO: Implement rate limiting
        // For now, just pass through
        Ok(next.run(request).await)
    }

    /// Security headers middleware
    pub async fn security_headers_middleware(
        request: Request,
        next: Next,
    ) -> Response {
        let mut response = next.run(request).await;

        let headers = response.headers_mut();
        headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
        headers.insert("X-Frame-Options", "DENY".parse().unwrap());
        headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
        headers.insert("Strict-Transport-Security", "max-age=31536000; includeSubDomains".parse().unwrap());

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, Method},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_auth_health_handler() {
        let response = auth_health_handler().await;
        let json = serde_json::to_string(&response.0).unwrap();

        assert!(json.contains("\"service\":\"authentication\""));
        assert!(json.contains("\"status\":\"healthy\""));
    }

    #[tokio::test]
    async fn test_routes_creation() {
        let router = create_auth_router();
        // Test that router can be created without panicking
        assert!(true);
    }

    #[tokio::test]
    async fn test_api_versioning() {
        let router = get_auth_routes();
        // Test that versioned router can be created without panicking
        assert!(true);
    }
}