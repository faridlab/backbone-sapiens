// GetUser Query
// Query for retrieving a User by ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserQuery {
    pub id: String,
}

impl GetUserQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<crate::application::commands::UserDto>,
}

impl GetUserResponse {
    pub fn success(user: crate::application::commands::UserDto) -> Self {
        Self {
            success: true,
            message: "User retrieved successfully".to_string(),
            user: Some(user),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            success: false,
            message: format!("User with id '{}' not found", id),
            user: None,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user: None,
        }
    }
}