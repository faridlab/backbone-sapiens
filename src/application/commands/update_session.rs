// UpdateSession Command
// Command for updating existing Session entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSessionCommand {
    pub id: String,
    // TODO: Add your command fields here based on entity proto
    // Example: pub name: Option<String>;
    // Example: pub description: Option<String>;

    // Generic fields for any custom data
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub updated_by: String,
}

impl UpdateSessionCommand {
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
pub struct UpdateSessionResponse {
    pub success: bool,
    pub message: String,
    pub session: Option<super::SessionDto>,
}

impl UpdateSessionResponse {
    pub fn success(session: super::SessionDto) -> Self {
        Self {
            success: true,
            message: "Session updated successfully".to_string(),
            session: Some(session),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            session: None,
        }
    }
}