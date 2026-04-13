// UpdateRolePermission Command
// Command for updating existing RolePermission entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRolePermissionCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: Option<String>;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpdateRolePermissionCommand {
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
pub struct UpdateRolePermissionResponse {
    pub success: bool,
    pub message: String,
    pub role_permission: Option<super::RolePermissionDto>,
}

impl UpdateRolePermissionResponse {
    pub fn success(role_permission: super::RolePermissionDto) -> Self {
        Self {
            success: true,
            message: "RolePermission updated successfully".to_string(),
            role_permission: Some(role_permission),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            role_permission: None,
        }
    }
}