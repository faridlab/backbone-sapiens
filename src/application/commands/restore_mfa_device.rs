// RestoreMfaDevice Command
// Command for restoring soft-deleted MfaDevice entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreMfaDeviceCommand {
    pub id: String,
}

impl RestoreMfaDeviceCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreMfaDeviceResponse {
    pub success: bool,
    pub message: String,
    pub mfa_device: Option<super::MfaDeviceDto>,
}

impl RestoreMfaDeviceResponse {
    pub fn success(mfa_device: super::MfaDeviceDto) -> Self {
        Self {
            success: true,
            message: "MfaDevice restored successfully".to_string(),
            mfa_device: Some(mfa_device),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            mfa_device: None,
        }
    }
}