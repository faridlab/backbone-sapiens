use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "trigger_type", rename_all = "snake_case")]
pub enum TriggerType {
    Manual,
    Scheduled,
    EventBased,
    Webhook,
    ApiCall,
}

impl std::fmt::Display for TriggerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::EventBased => write!(f, "event_based"),
            Self::Webhook => write!(f, "webhook"),
            Self::ApiCall => write!(f, "api_call"),
        }
    }
}

impl FromStr for TriggerType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "manual" => Ok(Self::Manual),
            "scheduled" => Ok(Self::Scheduled),
            "event_based" => Ok(Self::EventBased),
            "webhook" => Ok(Self::Webhook),
            "api_call" => Ok(Self::ApiCall),
            _ => Err(format!("Unknown TriggerType variant: {}", s)),
        }
    }
}

impl Default for TriggerType {
    fn default() -> Self {
        Self::Manual
    }
}
