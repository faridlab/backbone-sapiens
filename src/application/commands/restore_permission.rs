// RestorePermission Command
// Command for restoring soft-deleted Permission entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorePermissionCommand {
    pub id: String,
}

impl RestorePermissionCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorePermissionResponse {
    pub success: bool,
    pub message: String,
    pub permission: Option<super::PermissionDto>,
}

impl RestorePermissionResponse {
    pub fn success(permission: super::PermissionDto) -> Self {
        Self {
            success: true,
            message: "Permission restored successfully".to_string(),
            permission: Some(permission),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            permission: None,
        }
    }
}