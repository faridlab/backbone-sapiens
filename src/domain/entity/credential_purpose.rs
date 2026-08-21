use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "credential_purpose", rename_all = "snake_case")]
pub enum CredentialPurpose {
    WebhookVerify,
    ApiRead,
    ApiWrite,
    OauthToken,
}

impl std::fmt::Display for CredentialPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WebhookVerify => write!(f, "webhook_verify"),
            Self::ApiRead => write!(f, "api_read"),
            Self::ApiWrite => write!(f, "api_write"),
            Self::OauthToken => write!(f, "oauth_token"),
        }
    }
}

impl FromStr for CredentialPurpose {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "webhook_verify" => Ok(Self::WebhookVerify),
            "api_read" => Ok(Self::ApiRead),
            "api_write" => Ok(Self::ApiWrite),
            "oauth_token" => Ok(Self::OauthToken),
            _ => Err(format!("Unknown CredentialPurpose variant: {}", s)),
        }
    }
}
