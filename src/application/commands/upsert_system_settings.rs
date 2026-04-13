// UpsertSystemSettings Command
// Command for upserting SystemSettings entities (update or insert)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertSystemSettingsCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpsertSystemSettingsCommand {
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
pub struct UpsertSystemSettingsResponse {
    pub success: bool,
    pub message: String,
    pub system_settings: Option<super::SystemSettingsDto>,
    pub was_created: bool,
}

impl UpsertSystemSettingsResponse {
    pub fn created(system_settings: super::SystemSettingsDto) -> Self {
        Self {
            success: true,
            message: "SystemSettings created successfully".to_string(),
            system_settings: Some(system_settings),
            was_created: true,
        }
    }

    pub fn updated(system_settings: super::SystemSettingsDto) -> Self {
        Self {
            success: true,
            message: "SystemSettings updated successfully".to_string(),
            system_settings: Some(system_settings),
            was_created: false,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            system_settings: None,
            was_created: false,
        }
    }
}