// UpdateUser Command
// Command for updating existing User entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: Option<String>;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpdateUserCommand {
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
pub struct UpdateUserResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<super::UserDto>,
}

impl UpdateUserResponse {
    pub fn success(user: super::UserDto) -> Self {
        Self {
            success: true,
            message: "User updated successfully".to_string(),
            user: Some(user),
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