use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "mfa_device_status", rename_all = "snake_case")]
pub enum MFADeviceStatus {
    Active,
    Inactive,
    Suspended,
    Compromised,
    Expired,
    Revoked,
}

impl std::fmt::Display for MFADeviceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
            Self::Suspended => write!(f, "suspended"),
            Self::Compromised => write!(f, "compromised"),
            Self::Expired => write!(f, "expired"),
            Self::Revoked => write!(f, "revoked"),
        }
    }
}

impl FromStr for MFADeviceStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "suspended" => Ok(Self::Suspended),
            "compromised" => Ok(Self::Compromised),
            "expired" => Ok(Self::Expired),
            "revoked" => Ok(Self::Revoked),
            _ => Err(format!("Unknown MFADeviceStatus variant: {}", s)),
        }
    }
}

impl Default for MFADeviceStatus {
    fn default() -> Self {
        Self::Active
    }
}
