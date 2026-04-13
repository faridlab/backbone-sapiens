use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "o_auth_provider_type", rename_all = "snake_case")]
pub enum OAuthProviderType {
    Google,
    Github,
    Microsoft,
    Facebook,
    Apple,
}

impl std::fmt::Display for OAuthProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Google => write!(f, "google"),
            Self::Github => write!(f, "github"),
            Self::Microsoft => write!(f, "microsoft"),
            Self::Facebook => write!(f, "facebook"),
            Self::Apple => write!(f, "apple"),
        }
    }
}

impl FromStr for OAuthProviderType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "google" => Ok(Self::Google),
            "github" => Ok(Self::Github),
            "microsoft" => Ok(Self::Microsoft),
            "facebook" => Ok(Self::Facebook),
            "apple" => Ok(Self::Apple),
            _ => Err(format!("Unknown OAuthProviderType variant: {}", s)),
        }
    }
}

impl Default for OAuthProviderType {
    fn default() -> Self {
        Self::Google
    }
}
