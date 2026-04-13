// UpsertPermission Command
// Command for upserting Permission entities (update or insert)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertPermissionCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpsertPermissionCommand {
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
pub struct UpsertPermissionResponse {
    pub success: bool,
    pub message: String,
    pub permission: Option<super::PermissionDto>,
    pub was_created: bool,
}

impl UpsertPermissionResponse {
    pub fn created(permission: super::PermissionDto) -> Self {
        Self {
            success: true,
            message: "Permission created successfully".to_string(),
            permission: Some(permission),
            was_created: true,
        }
    }

    pub fn updated(permission: super::PermissionDto) -> Self {
        Self {
            success: true,
            message: "Permission updated successfully".to_string(),
            permission: Some(permission),
            was_created: false,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            permission: None,
            was_created: false,
        }
    }
}