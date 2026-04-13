use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "password_creation_method", rename_all = "snake_case")]
pub enum PasswordCreationMethod {
    Registration,
    PasswordChange,
    ResetByAdmin,
    SelfReset,
    ForcedChange,
}

impl std::fmt::Display for PasswordCreationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Registration => write!(f, "registration"),
            Self::PasswordChange => write!(f, "password_change"),
            Self::ResetByAdmin => write!(f, "reset_by_admin"),
            Self::SelfReset => write!(f, "self_reset"),
            Self::ForcedChange => write!(f, "forced_change"),
        }
    }
}

impl FromStr for PasswordCreationMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "registration" => Ok(Self::Registration),
            "password_change" => Ok(Self::PasswordChange),
            "reset_by_admin" => Ok(Self::ResetByAdmin),
            "self_reset" => Ok(Self::SelfReset),
            "forced_change" => Ok(Self::ForcedChange),
            _ => Err(format!("Unknown PasswordCreationMethod variant: {}", s)),
        }
    }
}

impl Default for PasswordCreationMethod {
    fn default() -> Self {
        Self::Registration
    }
}
