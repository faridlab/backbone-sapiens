use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "analytics_event_type", rename_all = "snake_case")]
pub enum AnalyticsEventType {
    UserLogin,
    UserLogout,
    UserRegistration,
    PasswordChange,
    MfaSetup,
    MfaVerification,
    RoleAssignment,
    PermissionGrant,
    ApiAccess,
    PageView,
    FeatureUsage,
    SecurityEvent,
    SystemEvent,
}

impl std::fmt::Display for AnalyticsEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserLogin => write!(f, "user_login"),
            Self::UserLogout => write!(f, "user_logout"),
            Self::UserRegistration => write!(f, "user_registration"),
            Self::PasswordChange => write!(f, "password_change"),
            Self::MfaSetup => write!(f, "mfa_setup"),
            Self::MfaVerification => write!(f, "mfa_verification"),
            Self::RoleAssignment => write!(f, "role_assignment"),
            Self::PermissionGrant => write!(f, "permission_grant"),
            Self::ApiAccess => write!(f, "api_access"),
            Self::PageView => write!(f, "page_view"),
            Self::FeatureUsage => write!(f, "feature_usage"),
            Self::SecurityEvent => write!(f, "security_event"),
            Self::SystemEvent => write!(f, "system_event"),
        }
    }
}

impl FromStr for AnalyticsEventType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user_login" => Ok(Self::UserLogin),
            "user_logout" => Ok(Self::UserLogout),
            "user_registration" => Ok(Self::UserRegistration),
            "password_change" => Ok(Self::PasswordChange),
            "mfa_setup" => Ok(Self::MfaSetup),
            "mfa_verification" => Ok(Self::MfaVerification),
            "role_assignment" => Ok(Self::RoleAssignment),
            "permission_grant" => Ok(Self::PermissionGrant),
            "api_access" => Ok(Self::ApiAccess),
            "page_view" => Ok(Self::PageView),
            "feature_usage" => Ok(Self::FeatureUsage),
            "security_event" => Ok(Self::SecurityEvent),
            "system_event" => Ok(Self::SystemEvent),
            _ => Err(format!("Unknown AnalyticsEventType variant: {}", s)),
        }
    }
}

impl Default for AnalyticsEventType {
    fn default() -> Self {
        Self::UserLogin
    }
}
