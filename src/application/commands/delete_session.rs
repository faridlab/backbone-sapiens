// DeleteSession Command
// Command for soft-deleting Session entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSessionCommand {
    pub id: String,
}

impl DeleteSessionCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSessionResponse {
    pub success: bool,
    pub message: String,
}

impl DeleteSessionResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "Session deleted successfully".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}