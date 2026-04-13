// UpsertUserPermission Command
// Command for upserting UserPermission entities (update or insert)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertUserPermissionCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpsertUserPermissionCommand {
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
pub struct UpsertUserPermissionResponse {
    pub success: bool,
    pub message: String,
    pub user_permission: Option<super::UserPermissionDto>,
    pub was_created: bool,
}

impl UpsertUserPermissionResponse {
    pub fn created(user_permission: super::UserPermissionDto) -> Self {
        Self {
            success: true,
            message: "UserPermission created successfully".to_string(),
            user_permission: Some(user_permission),
            was_created: true,
        }
    }

    pub fn updated(user_permission: super::UserPermissionDto) -> Self {
        Self {
            success: true,
            message: "UserPermission updated successfully".to_string(),
            user_permission: Some(user_permission),
            was_created: false,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user_permission: None,
            was_created: false,
        }
    }
}