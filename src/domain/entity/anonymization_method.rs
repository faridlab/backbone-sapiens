use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "anonymization_method", rename_all = "snake_case")]
pub enum AnonymizationMethod {
    Full,
    Partial,
    Pseudonymization,
}

impl std::fmt::Display for AnonymizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full => write!(f, "full"),
            Self::Partial => write!(f, "partial"),
            Self::Pseudonymization => write!(f, "pseudonymization"),
        }
    }
}

impl FromStr for AnonymizationMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "full" => Ok(Self::Full),
            "partial" => Ok(Self::Partial),
            "pseudonymization" => Ok(Self::Pseudonymization),
            _ => Err(format!("Unknown AnonymizationMethod variant: {}", s)),
        }
    }
}

impl Default for AnonymizationMethod {
    fn default() -> Self {
        Self::Full
    }
}
