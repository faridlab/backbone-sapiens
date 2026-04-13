// GetAuditLog Query
// Query for retrieving a AuditLog by ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuditLogQuery {
    pub id: String,
}

impl GetAuditLogQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuditLogResponse {
    pub success: bool,
    pub message: String,
    pub audit_log: Option<crate::application::commands::AuditLogDto>,
}

impl GetAuditLogResponse {
    pub fn success(audit_log: crate::application::commands::AuditLogDto) -> Self {
        Self {
            success: true,
            message: "AuditLog retrieved successfully".to_string(),
            audit_log: Some(audit_log),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            success: false,
            message: format!("AuditLog with id '{}' not found", id),
            audit_log: None,
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