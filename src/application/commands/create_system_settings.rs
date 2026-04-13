// CreateSystemSettings Command
// Command for creating new SystemSettings entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSystemSettingsCommand {
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub created_by: String,
}

impl CreateSystemSettingsCommand {
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
pub struct CreateSystemSettingsResponse {
    pub success: bool,
    pub message: String,
    pub system_settings: Option<SystemSettingsDto>,
}

impl CreateSystemSettingsResponse {
    pub fn success(system_settings: SystemSettingsDto) -> Self {
        Self {
            success: true,
            message: "SystemSettings created successfully".to_string(),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSettingsDto {
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

impl SystemSettingsDto {
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