use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "resource_permission_status", rename_all = "snake_case")]
pub enum ResourcePermissionStatus {
    Active,
    Expired,
    Revoked,
}

impl std::fmt::Display for ResourcePermissionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Expired => write!(f, "expired"),
            Self::Revoked => write!(f, "revoked"),
        }
    }
}

impl FromStr for ResourcePermissionStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "expired" => Ok(Self::Expired),
            "revoked" => Ok(Self::Revoked),
            _ => Err(format!("Unknown ResourcePermissionStatus variant: {}", s)),
        }
    }
}

impl Default for ResourcePermissionStatus {
    fn default() -> Self {
        Self::Active
    }
}
