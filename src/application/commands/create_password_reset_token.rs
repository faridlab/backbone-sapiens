// CreatePasswordResetToken Command
// Command for creating new PasswordResetToken entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePasswordResetTokenCommand {
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub created_by: String,
}

impl CreatePasswordResetTokenCommand {
    pub fn new(
        custom_fields: HashMap<String, serde_json::Value>,
        created_by: String,
    ) -> Self {
        Self {
            custom_fields,
            created_by,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePasswordResetTokenResponse {
    pub success: bool,
    pub message: String,
    pub password_reset_token: Option<PasswordResetTokenDto>,
}

impl CreatePasswordResetTokenResponse {
    pub fn success(password_reset_token: PasswordResetTokenDto) -> Self {
        Self {
            success: true,
            message: "PasswordResetToken created successfully".to_string(),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetTokenDto {
    pub id: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
    // TODO: Add your DTO fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    #[serde(flatten)]
    pub custom_fields: HashMap<String, serde_json::Value>,
}

impl PasswordResetTokenDto {
    pub fn new(
        id: String,
        custom_fields: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            id,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            custom_fields,
        }
    }

    pub fn with_timestamps(
        mut self,
        created_at: Option<String>,
        updated_at: Option<String>,
        deleted_at: Option<String>,
    ) -> Self {
        self.created_at = created_at;
        self.updated_at = updated_at;
        self.deleted_at = deleted_at;
        self
    }
}