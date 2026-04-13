// RestoreRole Command
// Command for restoring soft-deleted Role entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreRoleCommand {
    pub id: String,
}

impl RestoreRoleCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreRoleResponse {
    pub success: bool,
    pub message: String,
    pub role: Option<super::RoleDto>,
}

impl RestoreRoleResponse {
    pub fn success(role: super::RoleDto) -> Self {
        Self {
            success: true,
            message: "Role restored successfully".to_string(),
            role: Some(role),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            role: None,
        }
    }
}