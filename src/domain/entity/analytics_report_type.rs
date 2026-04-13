use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "analytics_report_type", rename_all = "snake_case")]
pub enum AnalyticsReportType {
    UserActivity,
    LoginTrends,
    FeatureUsage,
    SecurityEvents,
    PerformanceMetrics,
    ComplianceAudit,
    Custom,
}

impl std::fmt::Display for AnalyticsReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserActivity => write!(f, "user_activity"),
            Self::LoginTrends => write!(f, "login_trends"),
            Self::FeatureUsage => write!(f, "feature_usage"),
            Self::SecurityEvents => write!(f, "security_events"),
            Self::PerformanceMetrics => write!(f, "performance_metrics"),
            Self::ComplianceAudit => write!(f, "compliance_audit"),
            Self::Custom => write!(f, "custom"),
        }
    }
}

impl FromStr for AnalyticsReportType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user_activity" => Ok(Self::UserActivity),
            "login_trends" => Ok(Self::LoginTrends),
            "feature_usage" => Ok(Self::FeatureUsage),
            "security_events" => Ok(Self::SecurityEvents),
            "performance_metrics" => Ok(Self::PerformanceMetrics),
            "compliance_audit" => Ok(Self::ComplianceAudit),
            "custom" => Ok(Self::Custom),
            _ => Err(format!("Unknown AnalyticsReportType variant: {}", s)),
        }
    }
}

impl Default for AnalyticsReportType {
    fn default() -> Self {
        Self::UserActivity
    }
}
