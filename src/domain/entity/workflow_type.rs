use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "workflow_type", rename_all = "snake_case")]
pub enum WorkflowType {
    UserRegistration,
    EmailVerification,
    PasswordReset,
    AccountSuspension,
    RoleAssignment,
    PermissionGrant,
    MfaSetup,
    EmailChange,
    AccountRecovery,
    BulkUserImport,
    OauthLinking,
    DeviceTrust,
    SecurityReview,
    ComplianceAudit,
}

impl std::fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserRegistration => write!(f, "user_registration"),
            Self::EmailVerification => write!(f, "email_verification"),
            Self::PasswordReset => write!(f, "password_reset"),
            Self::AccountSuspension => write!(f, "account_suspension"),
            Self::RoleAssignment => write!(f, "role_assignment"),
            Self::PermissionGrant => write!(f, "permission_grant"),
            Self::MfaSetup => write!(f, "mfa_setup"),
            Self::EmailChange => write!(f, "email_change"),
            Self::AccountRecovery => write!(f, "account_recovery"),
            Self::BulkUserImport => write!(f, "bulk_user_import"),
            Self::OauthLinking => write!(f, "oauth_linking"),
            Self::DeviceTrust => write!(f, "device_trust"),
            Self::SecurityReview => write!(f, "security_review"),
            Self::ComplianceAudit => write!(f, "compliance_audit"),
        }
    }
}

impl FromStr for WorkflowType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user_registration" => Ok(Self::UserRegistration),
            "email_verification" => Ok(Self::EmailVerification),
            "password_reset" => Ok(Self::PasswordReset),
            "account_suspension" => Ok(Self::AccountSuspension),
            "role_assignment" => Ok(Self::RoleAssignment),
            "permission_grant" => Ok(Self::PermissionGrant),
            "mfa_setup" => Ok(Self::MfaSetup),
            "email_change" => Ok(Self::EmailChange),
            "account_recovery" => Ok(Self::AccountRecovery),
            "bulk_user_import" => Ok(Self::BulkUserImport),
            "oauth_linking" => Ok(Self::OauthLinking),
            "device_trust" => Ok(Self::DeviceTrust),
            "security_review" => Ok(Self::SecurityReview),
            "compliance_audit" => Ok(Self::ComplianceAudit),
            _ => Err(format!("Unknown WorkflowType variant: {}", s)),
        }
    }
}

impl Default for WorkflowType {
    fn default() -> Self {
        Self::UserRegistration
    }
}
