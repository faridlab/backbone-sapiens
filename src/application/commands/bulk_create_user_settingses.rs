// BulkCreateUserSettings Command
// Command for bulk creating multiple UserSettings entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateUserSettingsCommand {
    pub items: Vec<super::CreateUserSettingsCommand>,
}

impl BulkCreateUserSettingsCommand {
    pub fn new(items: Vec<super::CreateUserSettingsCommand>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateUserSettingsResponse {
    pub success: bool,
    pub message: String,
    pub created_count: usize,
    pub failed_count: usize,
    pub created_user_settingses: Vec<super::UserSettingsDto>,
    pub errors: Vec<String>,
}

impl BulkCreateUserSettingsResponse {
    pub fn new(
        created_count: usize,
        failed_count: usize,
        created_user_settingses: Vec<super::UserSettingsDto>,
        errors: Vec<String>,
    ) -> Self {
        let total_count = created_count + failed_count;
        let success = failed_count == 0;

        Self {
            success,
            message: if success {
                format!("Successfully created {} user_settingses", created_count)
            } else {
                format!("Created {} of {} user_settingses ({} failed)", created_count, total_count, failed_count)
            },
            created_count,
            failed_count,
            created_user_settingses,
            errors,
        }
    }
}