// BulkCreateSystemSettings Command
// Command for bulk creating multiple SystemSettings entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateSystemSettingsCommand {
    pub items: Vec<super::CreateSystemSettingsCommand>,
}

impl BulkCreateSystemSettingsCommand {
    pub fn new(items: Vec<super::CreateSystemSettingsCommand>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateSystemSettingsResponse {
    pub success: bool,
    pub message: String,
    pub created_count: usize,
    pub failed_count: usize,
    pub created_system_settingses: Vec<super::SystemSettingsDto>,
    pub errors: Vec<String>,
}

impl BulkCreateSystemSettingsResponse {
    pub fn new(
        created_count: usize,
        failed_count: usize,
        created_system_settingses: Vec<super::SystemSettingsDto>,
        errors: Vec<String>,
    ) -> Self {
        let total_count = created_count + failed_count;
        let success = failed_count == 0;

        Self {
            success,
            message: if success {
                format!("Successfully created {} system_settingses", created_count)
            } else {
                format!("Created {} of {} system_settingses ({} failed)", created_count, total_count, failed_count)
            },
            created_count,
            failed_count,
            created_system_settingses,
            errors,
        }
    }
}