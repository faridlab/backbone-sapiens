// GetPermission Query
// Query for retrieving a Permission by ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionQuery {
    pub id: String,
}

impl GetPermissionQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionResponse {
    pub success: bool,
    pub message: String,
    pub permission: Option<crate::application::commands::PermissionDto>,
}

impl GetPermissionResponse {
    pub fn success(permission: crate::application::commands::PermissionDto) -> Self {
        Self {
            success: true,
            message: "Permission retrieved successfully".to_string(),
            permission: Some(permission),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            success: false,
            message: format!("Permission with id '{}' not found", id),
            permission: None,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            permission: None,
        }
    }
}