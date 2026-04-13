use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "analytics_metric_type", rename_all = "snake_case")]
pub enum AnalyticsMetricType {
    Counter,
    Gauge,
    Histogram,
    Timer,
}

impl std::fmt::Display for AnalyticsMetricType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Counter => write!(f, "counter"),
            Self::Gauge => write!(f, "gauge"),
            Self::Histogram => write!(f, "histogram"),
            Self::Timer => write!(f, "timer"),
        }
    }
}

impl FromStr for AnalyticsMetricType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "counter" => Ok(Self::Counter),
            "gauge" => Ok(Self::Gauge),
            "histogram" => Ok(Self::Histogram),
            "timer" => Ok(Self::Timer),
            _ => Err(format!("Unknown AnalyticsMetricType variant: {}", s)),
        }
    }
}

impl Default for AnalyticsMetricType {
    fn default() -> Self {
        Self::Counter
    }
}
