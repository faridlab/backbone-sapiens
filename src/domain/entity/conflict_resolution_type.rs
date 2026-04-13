use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "conflict_resolution_type", rename_all = "snake_case")]
pub enum ConflictResolutionType {
    Priority,
    Latest,
    Manual,
    Escalation,
}

impl std::fmt::Display for ConflictResolutionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Priority => write!(f, "priority"),
            Self::Latest => write!(f, "latest"),
            Self::Manual => write!(f, "manual"),
            Self::Escalation => write!(f, "escalation"),
        }
    }
}

impl FromStr for ConflictResolutionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "priority" => Ok(Self::Priority),
            "latest" => Ok(Self::Latest),
            "manual" => Ok(Self::Manual),
            "escalation" => Ok(Self::Escalation),
            _ => Err(format!("Unknown ConflictResolutionType variant: {}", s)),
        }
    }
}

impl Default for ConflictResolutionType {
    fn default() -> Self {
        Self::Priority
    }
}
