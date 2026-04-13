// DeleteUserRole Command
// Command for soft-deleting UserRole entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserRoleCommand {
    pub id: String,
}

impl DeleteUserRoleCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserRoleResponse {
    pub success: bool,
    pub message: String,
}

impl DeleteUserRoleResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "UserRole deleted successfully".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}