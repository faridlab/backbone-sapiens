// GetUserRole Query
// Query for retrieving a UserRole by ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserRoleQuery {
    pub id: String,
}

impl GetUserRoleQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserRoleResponse {
    pub success: bool,
    pub message: String,
    pub user_role: Option<crate::application::commands::UserRoleDto>,
}

impl GetUserRoleResponse {
    pub fn success(user_role: crate::application::commands::UserRoleDto) -> Self {
        Self {
            success: true,
            message: "UserRole retrieved successfully".to_string(),
            user_role: Some(user_role),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            success: false,
            message: format!("UserRole with id '{}' not found", id),
            user_role: None,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user_role: None,
        }
    }
}