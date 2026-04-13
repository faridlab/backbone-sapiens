use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "notification_type", rename_all = "snake_case")]
pub enum NotificationType {
    Welcome,
    EmailVerification,
    PasswordReset,
    SecurityAlert,
    AccountSuspended,
    RoleAssigned,
    PermissionGranted,
    MfaEnabled,
    LoginAlert,
    SystemMaintenance,
    General,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Welcome => write!(f, "welcome"),
            Self::EmailVerification => write!(f, "email_verification"),
            Self::PasswordReset => write!(f, "password_reset"),
            Self::SecurityAlert => write!(f, "security_alert"),
            Self::AccountSuspended => write!(f, "account_suspended"),
            Self::RoleAssigned => write!(f, "role_assigned"),
            Self::PermissionGranted => write!(f, "permission_granted"),
            Self::MfaEnabled => write!(f, "mfa_enabled"),
            Self::LoginAlert => write!(f, "login_alert"),
            Self::SystemMaintenance => write!(f, "system_maintenance"),
            Self::General => write!(f, "general"),
        }
    }
}

impl FromStr for NotificationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "welcome" => Ok(Self::Welcome),
            "email_verification" => Ok(Self::EmailVerification),
            "password_reset" => Ok(Self::PasswordReset),
            "security_alert" => Ok(Self::SecurityAlert),
            "account_suspended" => Ok(Self::AccountSuspended),
            "role_assigned" => Ok(Self::RoleAssigned),
            "permission_granted" => Ok(Self::PermissionGranted),
            "mfa_enabled" => Ok(Self::MfaEnabled),
            "login_alert" => Ok(Self::LoginAlert),
            "system_maintenance" => Ok(Self::SystemMaintenance),
            "general" => Ok(Self::General),
            _ => Err(format!("Unknown NotificationType variant: {}", s)),
        }
    }
}

impl Default for NotificationType {
    fn default() -> Self {
        Self::Welcome
    }
}
