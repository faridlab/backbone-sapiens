// DeletePermission Command
// Command for soft-deleting Permission entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionCommand {
    pub id: String,
}

impl DeletePermissionCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionResponse {
    pub success: bool,
    pub message: String,
}

impl DeletePermissionResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "Permission deleted successfully".to_string(),
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
        }
    }
}