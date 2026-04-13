// UpsertAuditLog Command
// Command for upserting AuditLog entities (update or insert)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertAuditLogCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpsertAuditLogCommand {
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
pub struct UpsertAuditLogResponse {
    pub success: bool,
    pub message: String,
    pub audit_log: Option<super::AuditLogDto>,
    pub was_created: bool,
}

impl UpsertAuditLogResponse {
    pub fn created(audit_log: super::AuditLogDto) -> Self {
        Self {
            success: true,
            message: "AuditLog created successfully".to_string(),
            audit_log: Some(audit_log),
            was_created: true,
        }
    }

    pub fn updated(audit_log: super::AuditLogDto) -> Self {
        Self {
            success: true,
            message: "AuditLog updated successfully".to_string(),
            audit_log: Some(audit_log),
            was_created: false,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            audit_log: None,
            was_created: false,
        }
    }
}