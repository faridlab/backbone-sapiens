// GetRolePermission Query
// Query for retrieving a RolePermission by ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRolePermissionQuery {
    pub id: String,
}

impl GetRolePermissionQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRolePermissionResponse {
    pub success: bool,
    pub message: String,
    pub role_permission: Option<crate::application::commands::RolePermissionDto>,
}

impl GetRolePermissionResponse {
    pub fn success(role_permission: crate::application::commands::RolePermissionDto) -> Self {
        Self {
            success: true,
            message: "RolePermission retrieved successfully".to_string(),
            role_permission: Some(role_permission),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            success: false,
            message: format!("RolePermission with id '{}' not found", id),
            role_permission: None,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            role_permission: None,
        }
    }
}