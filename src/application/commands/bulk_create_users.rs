// BulkCreateUser Command
// Command for bulk creating multiple User entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateUserCommand {
    pub items: Vec<super::CreateUserCommand>,
}

impl BulkCreateUserCommand {
    pub fn new(items: Vec<super::CreateUserCommand>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateUserResponse {
    pub success: bool,
    pub message: String,
    pub created_count: usize,
    pub failed_count: usize,
    pub created_users: Vec<super::UserDto>,
    pub errors: Vec<String>,
}

impl BulkCreateUserResponse {
    pub fn new(
        created_count: usize,
        failed_count: usize,
        created_users: Vec<super::UserDto>,
        errors: Vec<String>,
    ) -> Self {
        let total_count = created_count + failed_count;
        let success = failed_count == 0;

        Self {
            success,
            message: if success {
                format!("Successfully created {} users", created_count)
            } else {
                format!("Created {} of {} users ({} failed)", created_count, total_count, failed_count)
            },
            created_count,
            failed_count,
            created_users,
            errors,
        }
    }
}