use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "generation_method", rename_all = "snake_case")]
pub enum GenerationMethod {
    UserInitiated,
    AdminEnforced,
    Automatic,
    DeviceEnrollment,
    Emergency,
}

impl std::fmt::Display for GenerationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserInitiated => write!(f, "user_initiated"),
            Self::AdminEnforced => write!(f, "admin_enforced"),
            Self::Automatic => write!(f, "automatic"),
            Self::DeviceEnrollment => write!(f, "device_enrollment"),
            Self::Emergency => write!(f, "emergency"),
        }
    }
}

impl FromStr for GenerationMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user_initiated" => Ok(Self::UserInitiated),
            "admin_enforced" => Ok(Self::AdminEnforced),
            "automatic" => Ok(Self::Automatic),
            "device_enrollment" => Ok(Self::DeviceEnrollment),
            "emergency" => Ok(Self::Emergency),
            _ => Err(format!("Unknown GenerationMethod variant: {}", s)),
        }
    }
}

impl Default for GenerationMethod {
    fn default() -> Self {
        Self::UserInitiated
    }
}
