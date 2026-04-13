// BulkCreatePasswordResetToken Command
// Command for bulk creating multiple PasswordResetToken entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreatePasswordResetTokenCommand {
    pub items: Vec<super::CreatePasswordResetTokenCommand>,
}

impl BulkCreatePasswordResetTokenCommand {
    pub fn new(items: Vec<super::CreatePasswordResetTokenCommand>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreatePasswordResetTokenResponse {
    pub success: bool,
    pub message: String,
    pub created_count: usize,
    pub failed_count: usize,
    pub created_password_reset_tokens: Vec<super::PasswordResetTokenDto>,
    pub errors: Vec<String>,
}

impl BulkCreatePasswordResetTokenResponse {
    pub fn new(
        created_count: usize,
        failed_count: usize,
        created_password_reset_tokens: Vec<super::PasswordResetTokenDto>,
        errors: Vec<String>,
    ) -> Self {
        let total_count = created_count + failed_count;
        let success = failed_count == 0;

        Self {
            success,
            message: if success {
                format!("Successfully created {} password_reset_tokens", created_count)
            } else {
                format!("Created {} of {} password_reset_tokens ({} failed)", created_count, total_count, failed_count)
            },
            created_count,
            failed_count,
            created_password_reset_tokens,
            errors,
        }
    }
}