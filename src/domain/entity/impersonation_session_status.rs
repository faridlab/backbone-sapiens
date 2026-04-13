use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "impersonation_session_status", rename_all = "snake_case")]
pub enum ImpersonationSessionStatus {
    Active,
    Ended,
    Expired,
    Terminated,
}

impl std::fmt::Display for ImpersonationSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Ended => write!(f, "ended"),
            Self::Expired => write!(f, "expired"),
            Self::Terminated => write!(f, "terminated"),
        }
    }
}

impl FromStr for ImpersonationSessionStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "ended" => Ok(Self::Ended),
            "expired" => Ok(Self::Expired),
            "terminated" => Ok(Self::Terminated),
            _ => Err(format!("Unknown ImpersonationSessionStatus variant: {}", s)),
        }
    }
}

impl Default for ImpersonationSessionStatus {
    fn default() -> Self {
        Self::Active
    }
}
