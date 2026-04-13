// UpdateUserRole Command
// Command for updating existing UserRole entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRoleCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: Option<String>;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpdateUserRoleCommand {
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
pub struct UpdateUserRoleResponse {
    pub success: bool,
    pub message: String,
    pub user_role: Option<super::UserRoleDto>,
}

impl UpdateUserRoleResponse {
    pub fn success(user_role: super::UserRoleDto) -> Self {
        Self {
            success: true,
            message: "UserRole updated successfully".to_string(),
            user_role: Some(user_role),
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