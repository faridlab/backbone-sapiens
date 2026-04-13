use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "device_trust_type", rename_all = "snake_case")]
pub enum DeviceTrustType {
    Desktop,
    Laptop,
    Mobile,
    Tablet,
    Browser,
    Application,
}

impl std::fmt::Display for DeviceTrustType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Desktop => write!(f, "desktop"),
            Self::Laptop => write!(f, "laptop"),
            Self::Mobile => write!(f, "mobile"),
            Self::Tablet => write!(f, "tablet"),
            Self::Browser => write!(f, "browser"),
            Self::Application => write!(f, "application"),
        }
    }
}

impl FromStr for DeviceTrustType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "desktop" => Ok(Self::Desktop),
            "laptop" => Ok(Self::Laptop),
            "mobile" => Ok(Self::Mobile),
            "tablet" => Ok(Self::Tablet),
            "browser" => Ok(Self::Browser),
            "application" => Ok(Self::Application),
            _ => Err(format!("Unknown DeviceTrustType variant: {}", s)),
        }
    }
}

impl Default for DeviceTrustType {
    fn default() -> Self {
        Self::Desktop
    }
}
