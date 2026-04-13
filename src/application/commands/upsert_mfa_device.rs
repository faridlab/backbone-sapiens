// UpsertMfaDevice Command
// Command for upserting MfaDevice entities (update or insert)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertMfaDeviceCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpsertMfaDeviceCommand {
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
pub struct UpsertMfaDeviceResponse {
    pub success: bool,
    pub message: String,
    pub mfa_device: Option<super::MfaDeviceDto>,
    pub was_created: bool,
}

impl UpsertMfaDeviceResponse {
    pub fn created(mfa_device: super::MfaDeviceDto) -> Self {
        Self {
            success: true,
            message: "MfaDevice created successfully".to_string(),
            mfa_device: Some(mfa_device),
            was_created: true,
        }
    }

    pub fn updated(mfa_device: super::MfaDeviceDto) -> Self {
        Self {
            success: true,
            message: "MfaDevice updated successfully".to_string(),
            mfa_device: Some(mfa_device),
            was_created: false,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            mfa_device: None,
            was_created: false,
        }
    }
}