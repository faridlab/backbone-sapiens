// BulkCreateAuditLog Command
// Command for bulk creating multiple AuditLog entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateAuditLogCommand {
    pub items: Vec<super::CreateAuditLogCommand>,
}

impl BulkCreateAuditLogCommand {
    pub fn new(items: Vec<super::CreateAuditLogCommand>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateAuditLogResponse {
    pub success: bool,
    pub message: String,
    pub created_count: usize,
    pub failed_count: usize,
    pub created_audit_logs: Vec<super::AuditLogDto>,
    pub errors: Vec<String>,
}

impl BulkCreateAuditLogResponse {
    pub fn new(
        created_count: usize,
        failed_count: usize,
        created_audit_logs: Vec<super::AuditLogDto>,
        errors: Vec<String>,
    ) -> Self {
        let total_count = created_count + failed_count;
        let success = failed_count == 0;

        Self {
            success,
            message: if success {
                format!("Successfully created {} audit_logs", created_count)
            } else {
                format!("Created {} of {} audit_logs ({} failed)", created_count, total_count, failed_count)
            },
            created_count,
            failed_count,
            created_audit_logs,
            errors,
        }
    }
}