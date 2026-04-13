// DeleteAuditLog Command
// Command for soft-deleting AuditLog entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAuditLogCommand {
    pub id: String,
}

impl DeleteAuditLogCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAuditLogResponse {
    pub success: bool,
    pub message: String,
}

impl DeleteAuditLogResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "AuditLog deleted successfully".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}