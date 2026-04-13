// DeleteUser Command
// Command for soft-deleting User entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserCommand {
    pub id: String,
}

impl DeleteUserCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserResponse {
    pub success: bool,
    pub message: String,
}

impl DeleteUserResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "User deleted successfully".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}