// RestoreRolePermission Command
// Command for restoring soft-deleted RolePermission entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreRolePermissionCommand {
    pub id: String,
}

impl RestoreRolePermissionCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreRolePermissionResponse {
    pub success: bool,
    pub message: String,
    pub role_permission: Option<super::RolePermissionDto>,
}

impl RestoreRolePermissionResponse {
    pub fn success(role_permission: super::RolePermissionDto) -> Self {
        Self {
            success: true,
            message: "RolePermission restored successfully".to_string(),
            role_permission: Some(role_permission),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            role_permission: None,
        }
    }
}