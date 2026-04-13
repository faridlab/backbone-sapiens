//! Authentication HTTP Handlers
//!
//! Custom authentication handlers for registration, login, logout, and password management.

use axum::{
    extract::{State, Path, Query},
    response::{IntoResponse, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::AppState;
use crate::application::service::{AuthService, RegisterInput};

/// Registration request from mobile app.
///
/// Note: first_name and last_name are intentionally NOT included here.
/// The mobile app may collect these, but for this registration flow,
/// we do NOT store them in user metadata. User profile can be updated
/// later via the profile update endpoint if needed.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

/// Registration response
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Handler for user registration from mobile app.
///
/// Registration flow:
/// 1. Email + password are required
/// 2. Full email is used as username (e.g., "user@example.com")
/// 3. No first_name/last_name stored in metadata (can be added later via profile update)
/// 4. Email verification token is sent immediately
/// 5. User status is "pending_verification" until email is verified
pub async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> impl IntoResponse {
    // Keep email reference for logging after moving req fields
    let email = &req.email;

    let result = state.auth_service.register(RegisterInput {
        email: req.email,
        password: req.password,
        confirm_password: req.password.clone(),
        first_name: None,  // Not stored - use profile update endpoint later
        last_name: None,   // Not stored - use profile update endpoint later
        accept_terms: true,  // Mobile app handles terms acceptance before API call
        username: Some(email.clone()),  // Use full email as username
    }).await;

    match result {
        Ok(result) => {
            tracing::info!("User registered successfully: {}", email);
            (
                StatusCode::CREATED,
                Json(RegisterResponse {
                    success: true,
                    message: "Registration successful. Please check your email for verification code.".to_string(),
                    user_id: Some(result.user_id),
                })
            )
        }
        Err(e) => {
            tracing::error!("Registration failed for {}: {:?}", email, e);
            let status = match &e {
                crate::application::service::AuthError::Conflict(_) => StatusCode::CONFLICT,
                crate::application::service::AuthError::Validation(_) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            let message = e.to_string();
            (
                status,
                Json(RegisterResponse {
                    success: false,
                    message,
                    user_id: None,
                })
            )
        }
    }
}

// Placeholder handlers for other auth endpoints
// TODO: Implement these handlers

#[allow(dead_code)]
pub async fn login_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Login handler not implemented")
}

#[allow(dead_code)]
pub async fn enhanced_login_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Enhanced login handler not implemented")
}

#[allow(dead_code)]
pub async fn logout_handler(
    State(_state): State<AppState>,
    Path(_session_id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Logout handler not implemented")
}

#[allow(dead_code)]
pub async fn simple_logout_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Simple logout handler not implemented")
}

#[allow(dead_code)]
pub async fn check_username_handler(
    State(_state): State<AppState>,
    Query(_params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Check username handler not implemented")
}

#[allow(dead_code)]
pub async fn check_email_handler(
    State(_state): State<AppState>,
    Query(_params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Check email handler not implemented")
}

#[allow(dead_code)]
pub async fn refresh_token_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Refresh token handler not implemented")
}

#[allow(dead_code)]
pub async fn validate_refresh_token_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Validate refresh token handler not implemented")
}

#[allow(dead_code)]
pub async fn revoke_refresh_token_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Revoke refresh token handler not implemented")
}

#[allow(dead_code)]
pub async fn verify_email_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Verify email handler not implemented")
}

#[allow(dead_code)]
pub async fn verify_email_get_handler(
    State(_state): State<AppState>,
    Query(_params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Verify email GET handler not implemented")
}

#[allow(dead_code)]
pub async fn resend_verification_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Resend verification handler not implemented")
}

#[allow(dead_code)]
pub async fn check_verification_status_handler(
    State(_state): State<AppState>,
    Path(_email): Path<String>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Check verification status handler not implemented")
}

#[allow(dead_code)]
pub async fn forgot_password_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Forgot password handler not implemented")
}

#[allow(dead_code)]
pub async fn check_reset_token_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Check reset token handler not implemented")
}

#[allow(dead_code)]
pub async fn resend_reset_email_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Resend reset email handler not implemented")
}

#[allow(dead_code)]
pub async fn reset_password_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Reset password handler not implemented")
}

#[allow(dead_code)]
pub async fn validate_reset_token_handler(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Validate reset token handler not implemented")
}

#[allow(dead_code)]
pub async fn auth_status_handler(
    State(_state): State<AppState>,
    Path(_session_id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Auth status handler not implemented")
}

#[allow(dead_code)]
pub async fn auth_status_by_user_handler(
    State(_state): State<AppState>,
    Path(_user_id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Auth status by user handler not implemented")
}

#[allow(dead_code)]
pub async fn user_sessions_handler(
    State(_state): State<AppState>,
    Path(_user_id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "User sessions handler not implemented")
}
