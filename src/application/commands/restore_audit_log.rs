// RestoreAuditLog Command
// Command for restoring soft-deleted AuditLog entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreAuditLogCommand {
    pub id: String,
}

impl RestoreAuditLogCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreAuditLogResponse {
    pub success: bool,
    pub message: String,
    pub audit_log: Option<super::AuditLogDto>,
}

impl RestoreAuditLogResponse {
    pub fn success(audit_log: super::AuditLogDto) -> Self {
        Self {
            success: true,
            message: "AuditLog restored successfully".to_string(),
            audit_log: Some(audit_log),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            audit_log: None,
        }
    }
}