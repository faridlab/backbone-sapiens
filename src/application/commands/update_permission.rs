// UpdatePermission Command
// Command for updating existing Permission entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: Option<String>;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpdatePermissionCommand {
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
pub struct UpdatePermissionResponse {
    pub success: bool,
    pub message: String,
    pub permission: Option<super::PermissionDto>,
}

impl UpdatePermissionResponse {
    pub fn success(permission: super::PermissionDto) -> Self {
        Self {
            success: true,
            message: "Permission updated successfully".to_string(),
            permission: Some(permission),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            permission: None,
        }
    }
}