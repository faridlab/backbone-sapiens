use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "conflict_resolution_status", rename_all = "snake_case")]
pub enum ConflictResolutionStatus {
    Pending,
    Resolved,
    Escalated,
    Ignored,
}

impl std::fmt::Display for ConflictResolutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Resolved => write!(f, "resolved"),
            Self::Escalated => write!(f, "escalated"),
            Self::Ignored => write!(f, "ignored"),
        }
    }
}

impl FromStr for ConflictResolutionStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "resolved" => Ok(Self::Resolved),
            "escalated" => Ok(Self::Escalated),
            "ignored" => Ok(Self::Ignored),
            _ => Err(format!("Unknown ConflictResolutionStatus variant: {}", s)),
        }
    }
}

impl Default for ConflictResolutionStatus {
    fn default() -> Self {
        Self::Pending
    }
}
