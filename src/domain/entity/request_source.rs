use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "request_source", rename_all = "snake_case")]
pub enum RequestSource {
    Web,
    Mobile,
    Api,
    Admin,
    Cli,
    Webhook,
    Service,
}

impl std::fmt::Display for RequestSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Web => write!(f, "web"),
            Self::Mobile => write!(f, "mobile"),
            Self::Api => write!(f, "api"),
            Self::Admin => write!(f, "admin"),
            Self::Cli => write!(f, "cli"),
            Self::Webhook => write!(f, "webhook"),
            Self::Service => write!(f, "service"),
        }
    }
}

impl FromStr for RequestSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "web" => Ok(Self::Web),
            "mobile" => Ok(Self::Mobile),
            "api" => Ok(Self::Api),
            "admin" => Ok(Self::Admin),
            "cli" => Ok(Self::Cli),
            "webhook" => Ok(Self::Webhook),
            "service" => Ok(Self::Service),
            _ => Err(format!("Unknown RequestSource variant: {}", s)),
        }
    }
}

impl Default for RequestSource {
    fn default() -> Self {
        Self::Web
    }
}
