// BulkCreateMfaDevice Command
// Command for bulk creating multiple MfaDevice entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateMfaDeviceCommand {
    pub items: Vec<super::CreateMfaDeviceCommand>,
}

impl BulkCreateMfaDeviceCommand {
    pub fn new(items: Vec<super::CreateMfaDeviceCommand>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateMfaDeviceResponse {
    pub success: bool,
    pub message: String,
    pub created_count: usize,
    pub failed_count: usize,
    pub created_mfa_devices: Vec<super::MfaDeviceDto>,
    pub errors: Vec<String>,
}

impl BulkCreateMfaDeviceResponse {
    pub fn new(
        created_count: usize,
        failed_count: usize,
        created_mfa_devices: Vec<super::MfaDeviceDto>,
        errors: Vec<String>,
    ) -> Self {
        let total_count = created_count + failed_count;
        let success = failed_count == 0;

        Self {
            success,
            message: if success {
                format!("Successfully created {} mfa_devices", created_count)
            } else {
                format!("Created {} of {} mfa_devices ({} failed)", created_count, total_count, failed_count)
            },
            created_count,
            failed_count,
            created_mfa_devices,
            errors,
        }
    }
}