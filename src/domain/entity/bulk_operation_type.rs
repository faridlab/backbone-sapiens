use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "bulk_operation_type", rename_all = "snake_case")]
pub enum BulkOperationType {
    UserImport,
    UserExport,
    RoleAssignment,
    PermissionGrant,
    UserUpdate,
    UserSuspension,
    UserDeletion,
}

impl std::fmt::Display for BulkOperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserImport => write!(f, "user_import"),
            Self::UserExport => write!(f, "user_export"),
            Self::RoleAssignment => write!(f, "role_assignment"),
            Self::PermissionGrant => write!(f, "permission_grant"),
            Self::UserUpdate => write!(f, "user_update"),
            Self::UserSuspension => write!(f, "user_suspension"),
            Self::UserDeletion => write!(f, "user_deletion"),
        }
    }
}

impl FromStr for BulkOperationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user_import" => Ok(Self::UserImport),
            "user_export" => Ok(Self::UserExport),
            "role_assignment" => Ok(Self::RoleAssignment),
            "permission_grant" => Ok(Self::PermissionGrant),
            "user_update" => Ok(Self::UserUpdate),
            "user_suspension" => Ok(Self::UserSuspension),
            "user_deletion" => Ok(Self::UserDeletion),
            _ => Err(format!("Unknown BulkOperationType variant: {}", s)),
        }
    }
}

impl Default for BulkOperationType {
    fn default() -> Self {
        Self::UserImport
    }
}
