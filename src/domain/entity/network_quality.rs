use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "network_quality", rename_all = "snake_case")]
pub enum NetworkQuality {
    Excellent,
    Good,
    Fair,
    Poor,
    Unknown,
}

impl std::fmt::Display for NetworkQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Excellent => write!(f, "excellent"),
            Self::Good => write!(f, "good"),
            Self::Fair => write!(f, "fair"),
            Self::Poor => write!(f, "poor"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl FromStr for NetworkQuality {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "excellent" => Ok(Self::Excellent),
            "good" => Ok(Self::Good),
            "fair" => Ok(Self::Fair),
            "poor" => Ok(Self::Poor),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown NetworkQuality variant: {}", s)),
        }
    }
}

impl Default for NetworkQuality {
    fn default() -> Self {
        Self::Unknown
    }
}
