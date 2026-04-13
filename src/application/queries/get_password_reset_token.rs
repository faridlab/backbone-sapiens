// GetPasswordResetToken Query
// Query for retrieving a PasswordResetToken by ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPasswordResetTokenQuery {
    pub id: String,
}

impl GetPasswordResetTokenQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPasswordResetTokenResponse {
    pub success: bool,
    pub message: String,
    pub password_reset_token: Option<crate::application::commands::PasswordResetTokenDto>,
}

impl GetPasswordResetTokenResponse {
    pub fn success(password_reset_token: crate::application::commands::PasswordResetTokenDto) -> Self {
        Self {
            success: true,
            message: "PasswordResetToken retrieved successfully".to_string(),
            password_reset_token: Some(password_reset_token),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            success: false,
            message: format!("PasswordResetToken with id '{}' not found", id),
            password_reset_token: None,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            password_reset_token: None,
        }
    }
}