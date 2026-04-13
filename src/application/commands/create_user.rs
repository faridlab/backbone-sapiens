// CreateUser Command
// Command for creating new User entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserCommand {
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub created_by: String,
}

impl CreateUserCommand {
    pub fn new(
        custom_fields: HashMap<String, serde_json::Value>,
        created_by: String,
    ) -> Self {
        Self {
            custom_fields,
            created_by,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<UserDto>,
}

impl CreateUserResponse {
    pub fn success(user: UserDto) -> Self {
        Self {
            success: true,
            message: "User created successfully".to_string(),
            user: Some(user),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub id: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
    // TODO: Add your DTO fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    #[serde(flatten)]
    pub custom_fields: HashMap<String, serde_json::Value>,
}

impl UserDto {
    pub fn new(
        id: String,
        custom_fields: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            id,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            custom_fields,
        }
    }

    pub fn with_timestamps(
        mut self,
        created_at: Option<String>,
        updated_at: Option<String>,
        deleted_at: Option<String>,
    ) -> Self {
        self.created_at = created_at;
        self.updated_at = updated_at;
        self.deleted_at = deleted_at;
        self
    }
}