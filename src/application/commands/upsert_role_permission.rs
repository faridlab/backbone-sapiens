// UpsertRolePermission Command
// Command for upserting RolePermission entities (update or insert)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertRolePermissionCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpsertRolePermissionCommand {
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
pub struct UpsertRolePermissionResponse {
    pub success: bool,
    pub message: String,
    pub role_permission: Option<super::RolePermissionDto>,
    pub was_created: bool,
}

impl UpsertRolePermissionResponse {
    pub fn created(role_permission: super::RolePermissionDto) -> Self {
        Self {
            success: true,
            message: "RolePermission created successfully".to_string(),
            role_permission: Some(role_permission),
            was_created: true,
        }
    }

    pub fn updated(role_permission: super::RolePermissionDto) -> Self {
        Self {
            success: true,
            message: "RolePermission updated successfully".to_string(),
            role_permission: Some(role_permission),
            was_created: false,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            role_permission: None,
            was_created: false,
        }
    }
}