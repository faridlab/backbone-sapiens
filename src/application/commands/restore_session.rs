// RestoreSession Command
// Command for restoring soft-deleted Session entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreSessionCommand {
    pub id: String,
}

impl RestoreSessionCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreSessionResponse {
    pub success: bool,
    pub message: String,
    pub session: Option<super::SessionDto>,
}

impl RestoreSessionResponse {
    pub fn success(session: super::SessionDto) -> Self {
        Self {
            success: true,
            message: "Session restored successfully".to_string(),
            session: Some(session),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            session: None,
        }
    }
}