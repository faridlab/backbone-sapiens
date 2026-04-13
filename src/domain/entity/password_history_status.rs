use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "password_history_status", rename_all = "snake_case")]
pub enum PasswordHistoryStatus {
    Active,
    Expired,
    Compromised,
    Replaced,
    Rotated,
}

impl std::fmt::Display for PasswordHistoryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Expired => write!(f, "expired"),
            Self::Compromised => write!(f, "compromised"),
            Self::Replaced => write!(f, "replaced"),
            Self::Rotated => write!(f, "rotated"),
        }
    }
}

impl FromStr for PasswordHistoryStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "expired" => Ok(Self::Expired),
            "compromised" => Ok(Self::Compromised),
            "replaced" => Ok(Self::Replaced),
            "rotated" => Ok(Self::Rotated),
            _ => Err(format!("Unknown PasswordHistoryStatus variant: {}", s)),
        }
    }
}

impl Default for PasswordHistoryStatus {
    fn default() -> Self {
        Self::Active
    }
}
