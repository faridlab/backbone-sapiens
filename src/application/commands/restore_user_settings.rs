// RestoreUserSettings Command
// Command for restoring soft-deleted UserSettings entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreUserSettingsCommand {
    pub id: String,
}

impl RestoreUserSettingsCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreUserSettingsResponse {
    pub success: bool,
    pub message: String,
    pub user_settings: Option<super::UserSettingsDto>,
}

impl RestoreUserSettingsResponse {
    pub fn success(user_settings: super::UserSettingsDto) -> Self {
        Self {
            success: true,
            message: "UserSettings restored successfully".to_string(),
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