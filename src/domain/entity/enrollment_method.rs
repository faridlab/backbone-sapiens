use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "enrollment_method", rename_all = "snake_case")]
pub enum EnrollmentMethod {
    SelfService,
    AdminEnforced,
    Automatic,
    Emergency,
}

impl std::fmt::Display for EnrollmentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SelfService => write!(f, "self_service"),
            Self::AdminEnforced => write!(f, "admin_enforced"),
            Self::Automatic => write!(f, "automatic"),
            Self::Emergency => write!(f, "emergency"),
        }
    }
}

impl FromStr for EnrollmentMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "self_service" => Ok(Self::SelfService),
            "admin_enforced" => Ok(Self::AdminEnforced),
            "automatic" => Ok(Self::Automatic),
            "emergency" => Ok(Self::Emergency),
            _ => Err(format!("Unknown EnrollmentMethod variant: {}", s)),
        }
    }
}

impl Default for EnrollmentMethod {
    fn default() -> Self {
        Self::SelfService
    }
}
