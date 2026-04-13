// GetRole Query
// Query for retrieving a Role by ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleQuery {
    pub id: String,
}

impl GetRoleQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleResponse {
    pub success: bool,
    pub message: String,
    pub role: Option<crate::application::commands::RoleDto>,
}

impl GetRoleResponse {
    pub fn success(role: crate::application::commands::RoleDto) -> Self {
        Self {
            success: true,
            message: "Role retrieved successfully".to_string(),
            role: Some(role),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            success: false,
            message: format!("Role with id '{}' not found", id),
            role: None,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            role: None,
        }
    }
}