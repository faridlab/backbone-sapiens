// DeleteSystemSettings Command
// Command for soft-deleting SystemSettings entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSystemSettingsCommand {
    pub id: String,
}

impl DeleteSystemSettingsCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSystemSettingsResponse {
    pub success: bool,
    pub message: String,
}

impl DeleteSystemSettingsResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "SystemSettings deleted successfully".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}