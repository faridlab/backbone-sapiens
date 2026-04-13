// BulkCreateRole Command
// Command for bulk creating multiple Role entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateRoleCommand {
    pub items: Vec<super::CreateRoleCommand>,
}

impl BulkCreateRoleCommand {
    pub fn new(items: Vec<super::CreateRoleCommand>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateRoleResponse {
    pub success: bool,
    pub message: String,
    pub created_count: usize,
    pub failed_count: usize,
    pub created_roles: Vec<super::RoleDto>,
    pub errors: Vec<String>,
}

impl BulkCreateRoleResponse {
    pub fn new(
        created_count: usize,
        failed_count: usize,
        created_roles: Vec<super::RoleDto>,
        errors: Vec<String>,
    ) -> Self {
        let total_count = created_count + failed_count;
        let success = failed_count == 0;

        Self {
            success,
            message: if success {
                format!("Successfully created {} roles", created_count)
            } else {
                format!("Created {} of {} roles ({} failed)", created_count, total_count, failed_count)
            },
            created_count,
            failed_count,
            created_roles,
            errors,
        }
    }
}