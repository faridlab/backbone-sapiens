// GetSystemSettings Query
// Query for retrieving a SystemSettings by ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSystemSettingsQuery {
    pub id: String,
}

impl GetSystemSettingsQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSystemSettingsResponse {
    pub success: bool,
    pub message: String,
    pub system_settings: Option<crate::application::commands::SystemSettingsDto>,
}

impl GetSystemSettingsResponse {
    pub fn success(system_settings: crate::application::commands::SystemSettingsDto) -> Self {
        Self {
            success: true,
            message: "SystemSettings retrieved successfully".to_string(),
            system_settings: Some(system_settings),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            success: false,
            message: format!("SystemSettings with id '{}' not found", id),
            system_settings: None,
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