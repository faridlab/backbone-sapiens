use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "user_saml_link_status", rename_all = "snake_case")]
pub enum UserSAMLLinkStatus {
    Active,
    Inactive,
    Revoked,
}

impl std::fmt::Display for UserSAMLLinkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
            Self::Revoked => write!(f, "revoked"),
        }
    }
}

impl FromStr for UserSAMLLinkStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "revoked" => Ok(Self::Revoked),
            _ => Err(format!("Unknown UserSAMLLinkStatus variant: {}", s)),
        }
    }
}

impl Default for UserSAMLLinkStatus {
    fn default() -> Self {
        Self::Active
    }
}
