// RestoreSystemSettings Command
// Command for restoring soft-deleted SystemSettings entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreSystemSettingsCommand {
    pub id: String,
}

impl RestoreSystemSettingsCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreSystemSettingsResponse {
    pub success: bool,
    pub message: String,
    pub system_settings: Option<super::SystemSettingsDto>,
}

impl RestoreSystemSettingsResponse {
    pub fn success(system_settings: super::SystemSettingsDto) -> Self {
        Self {
            success: true,
            message: "SystemSettings restored successfully".to_string(),
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