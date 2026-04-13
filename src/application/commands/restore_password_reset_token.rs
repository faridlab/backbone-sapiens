// RestorePasswordResetToken Command
// Command for restoring soft-deleted PasswordResetToken entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorePasswordResetTokenCommand {
    pub id: String,
}

impl RestorePasswordResetTokenCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorePasswordResetTokenResponse {
    pub success: bool,
    pub message: String,
    pub password_reset_token: Option<super::PasswordResetTokenDto>,
}

impl RestorePasswordResetTokenResponse {
    pub fn success(password_reset_token: super::PasswordResetTokenDto) -> Self {
        Self {
            success: true,
            message: "PasswordResetToken restored successfully".to_string(),
            password_reset_token: Some(password_reset_token),
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