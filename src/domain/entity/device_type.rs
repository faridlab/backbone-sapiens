use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "device_type", rename_all = "snake_case")]
pub enum DeviceType {
    Web,
    Mobile,
    Tablet,
    Desktop,
    Unknown,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Web => write!(f, "web"),
            Self::Mobile => write!(f, "mobile"),
            Self::Tablet => write!(f, "tablet"),
            Self::Desktop => write!(f, "desktop"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl FromStr for DeviceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "web" => Ok(Self::Web),
            "mobile" => Ok(Self::Mobile),
            "tablet" => Ok(Self::Tablet),
            "desktop" => Ok(Self::Desktop),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown DeviceType variant: {}", s)),
        }
    }
}

impl Default for DeviceType {
    fn default() -> Self {
        Self::Unknown
    }
}
