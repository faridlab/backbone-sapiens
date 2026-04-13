// UpdateUserPermission Command
// Command for updating existing UserPermission entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserPermissionCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: Option<String>;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpdateUserPermissionCommand {
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
pub struct UpdateUserPermissionResponse {
    pub success: bool,
    pub message: String,
    pub user_permission: Option<super::UserPermissionDto>,
}

impl UpdateUserPermissionResponse {
    pub fn success(user_permission: super::UserPermissionDto) -> Self {
        Self {
            success: true,
            message: "UserPermission updated successfully".to_string(),
            user_permission: Some(user_permission),
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