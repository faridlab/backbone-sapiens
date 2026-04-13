// RestoreUserRole Command
// Command for restoring soft-deleted UserRole entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreUserRoleCommand {
    pub id: String,
}

impl RestoreUserRoleCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreUserRoleResponse {
    pub success: bool,
    pub message: String,
    pub user_role: Option<super::UserRoleDto>,
}

impl RestoreUserRoleResponse {
    pub fn success(user_role: super::UserRoleDto) -> Self {
        Self {
            success: true,
            message: "UserRole restored successfully".to_string(),
            user_role: Some(user_role),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user_role: None,
        }
    }
}