// UpdateRole Command
// Command for updating existing Role entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: Option<String>;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpdateRoleCommand {
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
pub struct UpdateRoleResponse {
    pub success: bool,
    pub message: String,
    pub role: Option<super::RoleDto>,
}

impl UpdateRoleResponse {
    pub fn success(role: super::RoleDto) -> Self {
        Self {
            success: true,
            message: "Role updated successfully".to_string(),
            role: Some(role),
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