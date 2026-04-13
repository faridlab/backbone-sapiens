// UpdateMfaDevice Command
// Command for updating existing MfaDevice entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMfaDeviceCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: Option<String>;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpdateMfaDeviceCommand {
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
pub struct UpdateMfaDeviceResponse {
    pub success: bool,
    pub message: String,
    pub mfa_device: Option<super::MfaDeviceDto>,
}

impl UpdateMfaDeviceResponse {
    pub fn success(mfa_device: super::MfaDeviceDto) -> Self {
        Self {
            success: true,
            message: "MfaDevice updated successfully".to_string(),
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