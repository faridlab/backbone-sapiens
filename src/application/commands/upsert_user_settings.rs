// UpsertUserSettings Command
// Command for upserting UserSettings entities (update or insert)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertUserSettingsCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpsertUserSettingsCommand {
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
pub struct UpsertUserSettingsResponse {
    pub success: bool,
    pub message: String,
    pub user_settings: Option<super::UserSettingsDto>,
    pub was_created: bool,
}

impl UpsertUserSettingsResponse {
    pub fn created(user_settings: super::UserSettingsDto) -> Self {
        Self {
            success: true,
            message: "UserSettings created successfully".to_string(),
            user_settings: Some(user_settings),
            was_created: true,
        }
    }

    pub fn updated(user_settings: super::UserSettingsDto) -> Self {
        Self {
            success: true,
            message: "UserSettings updated successfully".to_string(),
            user_settings: Some(user_settings),
            was_created: false,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user_settings: None,
            was_created: false,
        }
    }
}