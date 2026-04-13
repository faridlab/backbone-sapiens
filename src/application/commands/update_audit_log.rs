// UpdateAuditLog Command
// Command for updating existing AuditLog entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuditLogCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: Option<String>;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpdateAuditLogCommand {
    pub fn new(
        id: String,
        custom_fields: HashMap<String, serde_json::Value>,
        updated_by: String,
    ) -> Self {
        Self {
            id,
            custom_fields,
            updated_by,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuditLogResponse {
    pub success: bool,
    pub message: String,
    pub audit_log: Option<super::AuditLogDto>,
}

impl UpdateAuditLogResponse {
    pub fn success(audit_log: super::AuditLogDto) -> Self {
        Self {
            success: true,
            message: "AuditLog updated successfully".to_string(),
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