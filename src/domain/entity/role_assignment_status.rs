use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "role_assignment_status", rename_all = "snake_case")]
pub enum RoleAssignmentStatus {
    Active,
    Inactive,
    Expired,
    Suspended,
    PendingApproval,
}

impl std::fmt::Display for RoleAssignmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
            Self::Expired => write!(f, "expired"),
            Self::Suspended => write!(f, "suspended"),
            Self::PendingApproval => write!(f, "pending_approval"),
        }
    }
}

impl FromStr for RoleAssignmentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "expired" => Ok(Self::Expired),
            "suspended" => Ok(Self::Suspended),
            "pending_approval" => Ok(Self::PendingApproval),
            _ => Err(format!("Unknown RoleAssignmentStatus variant: {}", s)),
        }
    }
}

impl Default for RoleAssignmentStatus {
    fn default() -> Self {
        Self::Active
    }
}
