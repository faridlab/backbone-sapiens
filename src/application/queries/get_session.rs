// GetSession Query
// Query for retrieving a Session by ID

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSessionQuery {
    pub id: String,
}

impl GetSessionQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSessionResponse {
    pub success: bool,
    pub message: String,
    pub session: Option<crate::application::commands::SessionDto>,
}

impl GetSessionResponse {
    pub fn success(session: crate::application::commands::SessionDto) -> Self {
        Self {
            success: true,
            message: "Session retrieved successfully".to_string(),
            session: Some(session),
        }
    }

    pub fn not_found(id: &str) -> Self {
        Self {
            success: false,
            message: format!("Session with id '{}' not found", id),
            session: None,
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