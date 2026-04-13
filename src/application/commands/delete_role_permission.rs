// DeleteRolePermission Command
// Command for soft-deleting RolePermission entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRolePermissionCommand {
    pub id: String,
}

impl DeleteRolePermissionCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRolePermissionResponse {
    pub success: bool,
    pub message: String,
}

impl DeleteRolePermissionResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "RolePermission deleted successfully".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}