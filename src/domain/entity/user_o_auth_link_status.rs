use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "user_o_auth_link_status", rename_all = "snake_case")]
pub enum UserOAuthLinkStatus {
    PendingVerification,
    Active,
    Expired,
    Revoked,
}

impl std::fmt::Display for UserOAuthLinkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PendingVerification => write!(f, "pending_verification"),
            Self::Active => write!(f, "active"),
            Self::Expired => write!(f, "expired"),
            Self::Revoked => write!(f, "revoked"),
        }
    }
}

impl FromStr for UserOAuthLinkStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending_verification" => Ok(Self::PendingVerification),
            "active" => Ok(Self::Active),
            "expired" => Ok(Self::Expired),
            "revoked" => Ok(Self::Revoked),
            _ => Err(format!("Unknown UserOAuthLinkStatus variant: {}", s)),
        }
    }
}

impl Default for UserOAuthLinkStatus {
    fn default() -> Self {
        Self::PendingVerification
    }
}
