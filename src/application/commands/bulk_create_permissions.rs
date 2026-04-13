// BulkCreatePermission Command
// Command for bulk creating multiple Permission entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreatePermissionCommand {
    pub items: Vec<super::CreatePermissionCommand>,
}

impl BulkCreatePermissionCommand {
    pub fn new(items: Vec<super::CreatePermissionCommand>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreatePermissionResponse {
    pub success: bool,
    pub message: String,
    pub created_count: usize,
    pub failed_count: usize,
    pub created_permissions: Vec<super::PermissionDto>,
    pub errors: Vec<String>,
}

impl BulkCreatePermissionResponse {
    pub fn new(
        created_count: usize,
        failed_count: usize,
        created_permissions: Vec<super::PermissionDto>,
        errors: Vec<String>,
    ) -> Self {
        let total_count = created_count + failed_count;
        let success = failed_count == 0;

        Self {
            success,
            message: if success {
                format!("Successfully created {} permissions", created_count)
            } else {
                format!("Created {} of {} permissions ({} failed)", created_count, total_count, failed_count)
            },
            created_count,
            failed_count,
            created_permissions,
            errors,
        }
    }
}