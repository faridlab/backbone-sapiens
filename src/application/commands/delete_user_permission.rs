// DeleteUserPermission Command
// Command for soft-deleting UserPermission entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserPermissionCommand {
    pub id: String,
}

impl DeleteUserPermissionCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserPermissionResponse {
    pub success: bool,
    pub message: String,
}

impl DeleteUserPermissionResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "UserPermission deleted successfully".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}