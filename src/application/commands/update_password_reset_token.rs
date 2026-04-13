// UpdatePasswordResetToken Command
// Command for updating existing PasswordResetToken entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasswordResetTokenCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: Option<String>;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpdatePasswordResetTokenCommand {
    pub fn new(
        id: String,
        custom_fields: HashMap<String, serde_json::Value>,
        updated_by: String,
    ) -> Self {
        Self {
            id,
            custom_fields,
            updated_by,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasswordResetTokenResponse {
    pub success: bool,
    pub message: String,
    pub password_reset_token: Option<super::PasswordResetTokenDto>,
}

impl UpdatePasswordResetTokenResponse {
    pub fn success(password_reset_token: super::PasswordResetTokenDto) -> Self {
        Self {
            success: true,
            message: "PasswordResetToken updated successfully".to_string(),
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