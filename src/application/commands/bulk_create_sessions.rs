// BulkCreateSession Command
// Command for bulk creating multiple Session entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateSessionCommand {
    pub items: Vec<super::CreateSessionCommand>,
}

impl BulkCreateSessionCommand {
    pub fn new(items: Vec<super::CreateSessionCommand>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateSessionResponse {
    pub success: bool,
    pub message: String,
    pub created_count: usize,
    pub failed_count: usize,
    pub created_sessions: Vec<super::SessionDto>,
    pub errors: Vec<String>,
}

impl BulkCreateSessionResponse {
    pub fn new(
        created_count: usize,
        failed_count: usize,
        created_sessions: Vec<super::SessionDto>,
        errors: Vec<String>,
    ) -> Self {
        let total_count = created_count + failed_count;
        let success = failed_count == 0;

        Self {
            success,
            message: if success {
                format!("Successfully created {} sessions", created_count)
            } else {
                format!("Created {} of {} sessions ({} failed)", created_count, total_count, failed_count)
            },
            created_count,
            failed_count,
            created_sessions,
            errors,
        }
    }
}