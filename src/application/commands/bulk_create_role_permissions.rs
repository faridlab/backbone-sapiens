// BulkCreateRolePermission Command
// Command for bulk creating multiple RolePermission entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateRolePermissionCommand {
    pub items: Vec<super::CreateRolePermissionCommand>,
}

impl BulkCreateRolePermissionCommand {
    pub fn new(items: Vec<super::CreateRolePermissionCommand>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateRolePermissionResponse {
    pub success: bool,
    pub message: String,
    pub created_count: usize,
    pub failed_count: usize,
    pub created_role_permissions: Vec<super::RolePermissionDto>,
    pub errors: Vec<String>,
}

impl BulkCreateRolePermissionResponse {
    pub fn new(
        created_count: usize,
        failed_count: usize,
        created_role_permissions: Vec<super::RolePermissionDto>,
        errors: Vec<String>,
    ) -> Self {
        let total_count = created_count + failed_count;
        let success = failed_count == 0;

        Self {
            success,
            message: if success {
                format!("Successfully created {} role_permissions", created_count)
            } else {
                format!("Created {} of {} role_permissions ({} failed)", created_count, total_count, failed_count)
            },
            created_count,
            failed_count,
            created_role_permissions,
            errors,
        }
    }
}