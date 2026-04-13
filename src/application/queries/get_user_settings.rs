// GetUserSettings Query
// Query for retrieving a UserSettings by ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSettingsQuery {
    pub id: String,
}

impl GetUserSettingsQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSettingsResponse {
    pub success: bool,
    pub message: String,
    pub user_settings: Option<crate::application::commands::UserSettingsDto>,
}

impl GetUserSettingsResponse {
    pub fn success(user_settings: crate::application::commands::UserSettingsDto) -> Self {
        Self {
            success: true,
            message: "UserSettings retrieved successfully".to_string(),
            user_settings: Some(user_settings),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            success: false,
            message: format!("UserSettings with id '{}' not found", id),
            user_settings: None,
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