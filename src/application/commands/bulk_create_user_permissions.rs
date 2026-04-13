// BulkCreateUserPermission Command
// Command for bulk creating multiple UserPermission entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateUserPermissionCommand {
    pub items: Vec<super::CreateUserPermissionCommand>,
}

impl BulkCreateUserPermissionCommand {
    pub fn new(items: Vec<super::CreateUserPermissionCommand>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateUserPermissionResponse {
    pub success: bool,
    pub message: String,
    pub created_count: usize,
    pub failed_count: usize,
    pub created_user_permissions: Vec<super::UserPermissionDto>,
    pub errors: Vec<String>,
}

impl BulkCreateUserPermissionResponse {
    pub fn new(
        created_count: usize,
        failed_count: usize,
        created_user_permissions: Vec<super::UserPermissionDto>,
        errors: Vec<String>,
    ) -> Self {
        let total_count = created_count + failed_count;
        let success = failed_count == 0;

        Self {
            success,
            message: if success {
                format!("Successfully created {} user_permissions", created_count)
            } else {
                format!("Created {} of {} user_permissions ({} failed)", created_count, total_count, failed_count)
            },
            created_count,
            failed_count,
            created_user_permissions,
            errors,
        }
    }
}