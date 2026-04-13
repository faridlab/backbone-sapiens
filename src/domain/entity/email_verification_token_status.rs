use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "email_verification_token_status", rename_all = "snake_case")]
pub enum EmailVerificationTokenStatus {
    Pending,
    Verified,
    Expired,
    Revoked,
}

impl std::fmt::Display for EmailVerificationTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Verified => write!(f, "verified"),
            Self::Expired => write!(f, "expired"),
            Self::Revoked => write!(f, "revoked"),
        }
    }
}

impl FromStr for EmailVerificationTokenStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "verified" => Ok(Self::Verified),
            "expired" => Ok(Self::Expired),
            "revoked" => Ok(Self::Revoked),
            _ => Err(format!("Unknown EmailVerificationTokenStatus variant: {}", s)),
        }
    }
}

impl Default for EmailVerificationTokenStatus {
    fn default() -> Self {
        Self::Pending
    }
}
