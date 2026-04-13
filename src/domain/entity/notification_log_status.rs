use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "notification_log_status", rename_all = "snake_case")]
pub enum NotificationLogStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
    Bounced,
}

impl std::fmt::Display for NotificationLogStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Sent => write!(f, "sent"),
            Self::Delivered => write!(f, "delivered"),
            Self::Failed => write!(f, "failed"),
            Self::Bounced => write!(f, "bounced"),
        }
    }
}

impl FromStr for NotificationLogStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "sent" => Ok(Self::Sent),
            "delivered" => Ok(Self::Delivered),
            "failed" => Ok(Self::Failed),
            "bounced" => Ok(Self::Bounced),
            _ => Err(format!("Unknown NotificationLogStatus variant: {}", s)),
        }
    }
}

impl Default for NotificationLogStatus {
    fn default() -> Self {
        Self::Pending
    }
}
