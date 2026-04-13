// RestoreUser Command
// Command for restoring soft-deleted User entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreUserCommand {
    pub id: String,
}

impl RestoreUserCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreUserResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<super::UserDto>,
}

impl RestoreUserResponse {
    pub fn success(user: super::UserDto) -> Self {
        Self {
            success: true,
            message: "User restored successfully".to_string(),
            user: Some(user),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user: None,
        }
    }
}