// RestoreUserPermission Command
// Command for restoring soft-deleted UserPermission entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreUserPermissionCommand {
    pub id: String,
}

impl RestoreUserPermissionCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreUserPermissionResponse {
    pub success: bool,
    pub message: String,
    pub user_permission: Option<super::UserPermissionDto>,
}

impl RestoreUserPermissionResponse {
    pub fn success(user_permission: super::UserPermissionDto) -> Self {
        Self {
            success: true,
            message: "UserPermission restored successfully".to_string(),
            user_permission: Some(user_permission),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user_permission: None,
        }
    }
}