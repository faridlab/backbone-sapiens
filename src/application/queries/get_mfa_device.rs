// GetMfaDevice Query
// Query for retrieving a MfaDevice by ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMfaDeviceQuery {
    pub id: String,
}

impl GetMfaDeviceQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMfaDeviceResponse {
    pub success: bool,
    pub message: String,
    pub mfa_device: Option<crate::application::commands::MfaDeviceDto>,
}

impl GetMfaDeviceResponse {
    pub fn success(mfa_device: crate::application::commands::MfaDeviceDto) -> Self {
        Self {
            success: true,
            message: "MfaDevice retrieved successfully".to_string(),
            mfa_device: Some(mfa_device),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            success: false,
            message: format!("MfaDevice with id '{}' not found", id),
            mfa_device: None,
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