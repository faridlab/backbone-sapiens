use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "security_event_type", rename_all = "snake_case")]
pub enum SecurityEventType {
    LoginFailed,
    SuspiciousLogin,
    PasswordReused,
    AccountLocked,
    PrivilegeEscalation,
    BruteForceAttack,
    MaliciousContent,
    ImpersonationStarted,
    ImpersonationEnded,
    MfaEnabled,
    MfaDisabled,
    DataExported,
    DataAnonymized,
    RateLimitExceeded,
    PasswordResetRequest,
    PasswordChanged,
}

impl std::fmt::Display for SecurityEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoginFailed => write!(f, "login_failed"),
            Self::SuspiciousLogin => write!(f, "suspicious_login"),
            Self::PasswordReused => write!(f, "password_reused"),
            Self::AccountLocked => write!(f, "account_locked"),
            Self::PrivilegeEscalation => write!(f, "privilege_escalation"),
            Self::BruteForceAttack => write!(f, "brute_force_attack"),
            Self::MaliciousContent => write!(f, "malicious_content"),
            Self::ImpersonationStarted => write!(f, "impersonation_started"),
            Self::ImpersonationEnded => write!(f, "impersonation_ended"),
            Self::MfaEnabled => write!(f, "mfa_enabled"),
            Self::MfaDisabled => write!(f, "mfa_disabled"),
            Self::DataExported => write!(f, "data_exported"),
            Self::DataAnonymized => write!(f, "data_anonymized"),
            Self::RateLimitExceeded => write!(f, "rate_limit_exceeded"),
            Self::PasswordResetRequest => write!(f, "password_reset_request"),
            Self::PasswordChanged => write!(f, "password_changed"),
        }
    }
}

impl FromStr for SecurityEventType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "login_failed" => Ok(Self::LoginFailed),
            "suspicious_login" => Ok(Self::SuspiciousLogin),
            "password_reused" => Ok(Self::PasswordReused),
            "account_locked" => Ok(Self::AccountLocked),
            "privilege_escalation" => Ok(Self::PrivilegeEscalation),
            "brute_force_attack" => Ok(Self::BruteForceAttack),
            "malicious_content" => Ok(Self::MaliciousContent),
            "impersonation_started" => Ok(Self::ImpersonationStarted),
            "impersonation_ended" => Ok(Self::ImpersonationEnded),
            "mfa_enabled" => Ok(Self::MfaEnabled),
            "mfa_disabled" => Ok(Self::MfaDisabled),
            "data_exported" => Ok(Self::DataExported),
            "data_anonymized" => Ok(Self::DataAnonymized),
            "rate_limit_exceeded" => Ok(Self::RateLimitExceeded),
            "password_reset_request" => Ok(Self::PasswordResetRequest),
            "password_changed" => Ok(Self::PasswordChanged),
            _ => Err(format!("Unknown SecurityEventType variant: {}", s)),
        }
    }
}

impl Default for SecurityEventType {
    fn default() -> Self {
        Self::LoginFailed
    }
}
