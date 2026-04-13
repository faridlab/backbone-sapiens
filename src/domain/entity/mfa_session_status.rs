use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "mfa_session_status", rename_all = "snake_case")]
pub enum MFASessionStatus {
    Initiated,
    PendingVerification,
    Verified,
    Failed,
    Expired,
    Terminated,
}

impl std::fmt::Display for MFASessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initiated => write!(f, "initiated"),
            Self::PendingVerification => write!(f, "pending_verification"),
            Self::Verified => write!(f, "verified"),
            Self::Failed => write!(f, "failed"),
            Self::Expired => write!(f, "expired"),
            Self::Terminated => write!(f, "terminated"),
        }
    }
}

impl FromStr for MFASessionStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "initiated" => Ok(Self::Initiated),
            "pending_verification" => Ok(Self::PendingVerification),
            "verified" => Ok(Self::Verified),
            "failed" => Ok(Self::Failed),
            "expired" => Ok(Self::Expired),
            "terminated" => Ok(Self::Terminated),
            _ => Err(format!("Unknown MFASessionStatus variant: {}", s)),
        }
    }
}

impl Default for MFASessionStatus {
    fn default() -> Self {
        Self::Initiated
    }
}
