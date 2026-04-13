use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "direct_permission_grant_status", rename_all = "snake_case")]
pub enum DirectPermissionGrantStatus {
    Active,
    Expired,
    Revoked,
    Suspended,
}

impl std::fmt::Display for DirectPermissionGrantStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Expired => write!(f, "expired"),
            Self::Revoked => write!(f, "revoked"),
            Self::Suspended => write!(f, "suspended"),
        }
    }
}

impl FromStr for DirectPermissionGrantStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "expired" => Ok(Self::Expired),
            "revoked" => Ok(Self::Revoked),
            "suspended" => Ok(Self::Suspended),
            _ => Err(format!("Unknown DirectPermissionGrantStatus variant: {}", s)),
        }
    }
}

impl Default for DirectPermissionGrantStatus {
    fn default() -> Self {
        Self::Active
    }
}
