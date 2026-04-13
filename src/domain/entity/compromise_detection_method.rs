use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "compromise_detection_method", rename_all = "snake_case")]
pub enum CompromiseDetectionMethod {
    FailedAttempts,
    UnusualUsage,
    AdminReport,
    AutomaticScan,
    UserReport,
}

impl std::fmt::Display for CompromiseDetectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedAttempts => write!(f, "failed_attempts"),
            Self::UnusualUsage => write!(f, "unusual_usage"),
            Self::AdminReport => write!(f, "admin_report"),
            Self::AutomaticScan => write!(f, "automatic_scan"),
            Self::UserReport => write!(f, "user_report"),
        }
    }
}

impl FromStr for CompromiseDetectionMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "failed_attempts" => Ok(Self::FailedAttempts),
            "unusual_usage" => Ok(Self::UnusualUsage),
            "admin_report" => Ok(Self::AdminReport),
            "automatic_scan" => Ok(Self::AutomaticScan),
            "user_report" => Ok(Self::UserReport),
            _ => Err(format!("Unknown CompromiseDetectionMethod variant: {}", s)),
        }
    }
}

impl Default for CompromiseDetectionMethod {
    fn default() -> Self {
        Self::FailedAttempts
    }
}
