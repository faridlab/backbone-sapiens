// DeletePasswordResetToken Command
// Command for soft-deleting PasswordResetToken entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePasswordResetTokenCommand {
    pub id: String,
}

impl DeletePasswordResetTokenCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePasswordResetTokenResponse {
    pub success: bool,
    pub message: String,
}

impl DeletePasswordResetTokenResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "PasswordResetToken deleted successfully".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}