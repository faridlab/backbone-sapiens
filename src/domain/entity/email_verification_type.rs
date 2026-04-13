use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "email_verification_type", rename_all = "snake_case")]
pub enum EmailVerificationType {
    AccountCreation,
    EmailChange,
    PasswordReset,
    AccountReactivation,
}

impl std::fmt::Display for EmailVerificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccountCreation => write!(f, "account_creation"),
            Self::EmailChange => write!(f, "email_change"),
            Self::PasswordReset => write!(f, "password_reset"),
            Self::AccountReactivation => write!(f, "account_reactivation"),
        }
    }
}

impl FromStr for EmailVerificationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "account_creation" => Ok(Self::AccountCreation),
            "email_change" => Ok(Self::EmailChange),
            "password_reset" => Ok(Self::PasswordReset),
            "account_reactivation" => Ok(Self::AccountReactivation),
            _ => Err(format!("Unknown EmailVerificationType variant: {}", s)),
        }
    }
}

impl Default for EmailVerificationType {
    fn default() -> Self {
        Self::AccountCreation
    }
}
