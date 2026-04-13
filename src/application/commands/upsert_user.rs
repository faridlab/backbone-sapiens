// UpsertUser Command
// Command for upserting User entities (update or insert)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertUserCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpsertUserCommand {
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
pub struct UpsertUserResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<super::UserDto>,
    pub was_created: bool,
}

impl UpsertUserResponse {
    pub fn created(user: super::UserDto) -> Self {
        Self {
            success: true,
            message: "User created successfully".to_string(),
            user: Some(user),
            was_created: true,
        }
    }

    pub fn updated(user: super::UserDto) -> Self {
        Self {
            success: true,
            message: "User updated successfully".to_string(),
            user: Some(user),
            was_created: false,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user: None,
            was_created: false,
        }
    }
}