use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "primary_auth_method", rename_all = "snake_case")]
pub enum PrimaryAuthMethod {
    Password,
    Sso,
    Oauth,
    Certificate,
    Biometric,
}

impl std::fmt::Display for PrimaryAuthMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Password => write!(f, "password"),
            Self::Sso => write!(f, "sso"),
            Self::Oauth => write!(f, "oauth"),
            Self::Certificate => write!(f, "certificate"),
            Self::Biometric => write!(f, "biometric"),
        }
    }
}

impl FromStr for PrimaryAuthMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "password" => Ok(Self::Password),
            "sso" => Ok(Self::Sso),
            "oauth" => Ok(Self::Oauth),
            "certificate" => Ok(Self::Certificate),
            "biometric" => Ok(Self::Biometric),
            _ => Err(format!("Unknown PrimaryAuthMethod variant: {}", s)),
        }
    }
}

impl Default for PrimaryAuthMethod {
    fn default() -> Self {
        Self::Password
    }
}
