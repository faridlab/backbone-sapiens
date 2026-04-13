// UpsertRole Command
// Command for upserting Role entities (update or insert)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertRoleCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpsertRoleCommand {
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
pub struct UpsertRoleResponse {
    pub success: bool,
    pub message: String,
    pub role: Option<super::RoleDto>,
    pub was_created: bool,
}

impl UpsertRoleResponse {
    pub fn created(role: super::RoleDto) -> Self {
        Self {
            success: true,
            message: "Role created successfully".to_string(),
            role: Some(role),
            was_created: true,
        }
    }

    pub fn updated(role: super::RoleDto) -> Self {
        Self {
            success: true,
            message: "Role updated successfully".to_string(),
            role: Some(role),
            was_created: false,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            role: None,
            was_created: false,
        }
    }
}