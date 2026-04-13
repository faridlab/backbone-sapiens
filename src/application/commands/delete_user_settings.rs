// DeleteUserSettings Command
// Command for soft-deleting UserSettings entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserSettingsCommand {
    pub id: String,
}

impl DeleteUserSettingsCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserSettingsResponse {
    pub success: bool,
    pub message: String,
}

impl DeleteUserSettingsResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "UserSettings deleted successfully".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}