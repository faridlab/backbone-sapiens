// UpsertUserRole Command
// Command for upserting UserRole entities (update or insert)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertUserRoleCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpsertUserRoleCommand {
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
pub struct UpsertUserRoleResponse {
    pub success: bool,
    pub message: String,
    pub user_role: Option<super::UserRoleDto>,
    pub was_created: bool,
}

impl UpsertUserRoleResponse {
    pub fn created(user_role: super::UserRoleDto) -> Self {
        Self {
            success: true,
            message: "UserRole created successfully".to_string(),
            user_role: Some(user_role),
            was_created: true,
        }
    }

    pub fn updated(user_role: super::UserRoleDto) -> Self {
        Self {
            success: true,
            message: "UserRole updated successfully".to_string(),
            user_role: Some(user_role),
            was_created: false,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user_role: None,
            was_created: false,
        }
    }
}