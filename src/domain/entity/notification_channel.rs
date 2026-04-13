use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "notification_channel", rename_all = "snake_case")]
pub enum NotificationChannel {
    InApp,
    Email,
    Sms,
    Push,
}

impl std::fmt::Display for NotificationChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InApp => write!(f, "in_app"),
            Self::Email => write!(f, "email"),
            Self::Sms => write!(f, "sms"),
            Self::Push => write!(f, "push"),
        }
    }
}

impl FromStr for NotificationChannel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "in_app" => Ok(Self::InApp),
            "email" => Ok(Self::Email),
            "sms" => Ok(Self::Sms),
            "push" => Ok(Self::Push),
            _ => Err(format!("Unknown NotificationChannel variant: {}", s)),
        }
    }
}

impl Default for NotificationChannel {
    fn default() -> Self {
        Self::InApp
    }
}
