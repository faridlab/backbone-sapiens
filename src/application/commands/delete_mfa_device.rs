// DeleteMfaDevice Command
// Command for soft-deleting MfaDevice entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMfaDeviceCommand {
    pub id: String,
}

impl DeleteMfaDeviceCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMfaDeviceResponse {
    pub success: bool,
    pub message: String,
}

impl DeleteMfaDeviceResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "MfaDevice deleted successfully".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}