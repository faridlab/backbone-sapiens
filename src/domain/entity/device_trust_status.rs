use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "device_trust_status", rename_all = "snake_case")]
pub enum DeviceTrustStatus {
    Trusted,
    Untrusted,
    NewDevice,
    Compromised,
}

impl std::fmt::Display for DeviceTrustStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trusted => write!(f, "trusted"),
            Self::Untrusted => write!(f, "untrusted"),
            Self::NewDevice => write!(f, "new_device"),
            Self::Compromised => write!(f, "compromised"),
        }
    }
}

impl FromStr for DeviceTrustStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trusted" => Ok(Self::Trusted),
            "untrusted" => Ok(Self::Untrusted),
            "new_device" => Ok(Self::NewDevice),
            "compromised" => Ok(Self::Compromised),
            _ => Err(format!("Unknown DeviceTrustStatus variant: {}", s)),
        }
    }
}

impl Default for DeviceTrustStatus {
    fn default() -> Self {
        Self::Untrusted
    }
}
