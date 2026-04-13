use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "password_strength", rename_all = "snake_case")]
pub enum PasswordStrength {
    VeryWeak,
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}

impl std::fmt::Display for PasswordStrength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VeryWeak => write!(f, "very_weak"),
            Self::Weak => write!(f, "weak"),
            Self::Moderate => write!(f, "moderate"),
            Self::Strong => write!(f, "strong"),
            Self::VeryStrong => write!(f, "very_strong"),
        }
    }
}

impl FromStr for PasswordStrength {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "very_weak" => Ok(Self::VeryWeak),
            "weak" => Ok(Self::Weak),
            "moderate" => Ok(Self::Moderate),
            "strong" => Ok(Self::Strong),
            "very_strong" => Ok(Self::VeryStrong),
            _ => Err(format!("Unknown PasswordStrength variant: {}", s)),
        }
    }
}

impl Default for PasswordStrength {
    fn default() -> Self {
        Self::Weak
    }
}
