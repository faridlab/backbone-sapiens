// DeleteRole Command
// Command for soft-deleting Role entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoleCommand {
    pub id: String,
}

impl DeleteRoleCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoleResponse {
    pub success: bool,
    pub message: String,
}

impl DeleteRoleResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "Role deleted successfully".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}