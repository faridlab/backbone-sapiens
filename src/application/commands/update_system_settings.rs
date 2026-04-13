// UpdateSystemSettings Command
// Command for updating existing SystemSettings entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSystemSettingsCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: Option<String>;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpdateSystemSettingsCommand {
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
pub struct UpdateSystemSettingsResponse {
    pub success: bool,
    pub message: String,
    pub system_settings: Option<super::SystemSettingsDto>,
}

impl UpdateSystemSettingsResponse {
    pub fn success(system_settings: super::SystemSettingsDto) -> Self {
        Self {
            success: true,
            message: "SystemSettings updated successfully".to_string(),
            system_settings: Some(system_settings),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            system_settings: None,
        }
    }
}