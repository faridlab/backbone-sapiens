// UpsertSession Command
// Command for upserting Session entities (update or insert)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertSessionCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: String;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpsertSessionCommand {
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
pub struct UpsertSessionResponse {
    pub success: bool,
    pub message: String,
    pub session: Option<super::SessionDto>,
    pub was_created: bool,
}

impl UpsertSessionResponse {
    pub fn created(session: super::SessionDto) -> Self {
        Self {
            success: true,
            message: "Session created successfully".to_string(),
            session: Some(session),
            was_created: true,
        }
    }

    pub fn updated(session: super::SessionDto) -> Self {
        Self {
            success: true,
            message: "Session updated successfully".to_string(),
            session: Some(session),
            was_created: false,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            session: None,
            was_created: false,
        }
    }
}