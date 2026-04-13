use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "report_format", rename_all = "snake_case")]
pub enum ReportFormat {
    Json,
    Csv,
    Pdf,
    Excel,
}

impl std::fmt::Display for ReportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Csv => write!(f, "csv"),
            Self::Pdf => write!(f, "pdf"),
            Self::Excel => write!(f, "excel"),
        }
    }
}

impl FromStr for ReportFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "csv" => Ok(Self::Csv),
            "pdf" => Ok(Self::Pdf),
            "excel" => Ok(Self::Excel),
            _ => Err(format!("Unknown ReportFormat variant: {}", s)),
        }
    }
}

impl Default for ReportFormat {
    fn default() -> Self {
        Self::Json
    }
}
