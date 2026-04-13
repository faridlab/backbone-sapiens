// UpdateUserSettings Command
// Command for updating existing UserSettings entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserSettingsCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: Option<String>;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpdateUserSettingsCommand {
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
pub struct UpdateUserSettingsResponse {
    pub success: bool,
    pub message: String,
    pub user_settings: Option<super::UserSettingsDto>,
}

impl UpdateUserSettingsResponse {
    pub fn success(user_settings: super::UserSettingsDto) -> Self {
        Self {
            success: true,
            message: "UserSettings updated successfully".to_string(),
            user_settings: Some(user_settings),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user_settings: None,
        }
    }
}