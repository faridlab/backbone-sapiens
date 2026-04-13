// GetUserPermission Query
// Query for retrieving a UserPermission by ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPermissionQuery {
    pub id: String,
}

impl GetUserPermissionQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPermissionResponse {
    pub success: bool,
    pub message: String,
    pub user_permission: Option<crate::application::commands::UserPermissionDto>,
}

impl GetUserPermissionResponse {
    pub fn success(user_permission: crate::application::commands::UserPermissionDto) -> Self {
        Self {
            success: true,
            message: "UserPermission retrieved successfully".to_string(),
            user_permission: Some(user_permission),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            success: false,
            message: format!("UserPermission with id '{}' not found", id),
            user_permission: None,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user_permission: None,
        }
    }
}