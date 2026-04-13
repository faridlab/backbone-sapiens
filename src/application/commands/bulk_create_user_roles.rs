// BulkCreateUserRole Command
// Command for bulk creating multiple UserRole entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateUserRoleCommand {
    pub items: Vec<super::CreateUserRoleCommand>,
}

impl BulkCreateUserRoleCommand {
    pub fn new(items: Vec<super::CreateUserRoleCommand>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateUserRoleResponse {
    pub success: bool,
    pub message: String,
    pub created_count: usize,
    pub failed_count: usize,
    pub created_user_roles: Vec<super::UserRoleDto>,
    pub errors: Vec<String>,
}

impl BulkCreateUserRoleResponse {
    pub fn new(
        created_count: usize,
        failed_count: usize,
        created_user_roles: Vec<super::UserRoleDto>,
        errors: Vec<String>,
    ) -> Self {
        let total_count = created_count + failed_count;
        let success = failed_count == 0;

        Self {
            success,
            message: if success {
                format!("Successfully created {} user_roles", created_count)
            } else {
                format!("Created {} of {} user_roles ({} failed)", created_count, total_count, failed_count)
            },
            created_count,
            failed_count,
            created_user_roles,
            errors,
        }
    }
}