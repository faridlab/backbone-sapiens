use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "data_export_format", rename_all = "snake_case")]
pub enum DataExportFormat {
    Json,
    Csv,
    Pdf,
    Xml,
}

impl std::fmt::Display for DataExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Csv => write!(f, "csv"),
            Self::Pdf => write!(f, "pdf"),
            Self::Xml => write!(f, "xml"),
        }
    }
}

impl FromStr for DataExportFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "csv" => Ok(Self::Csv),
            "pdf" => Ok(Self::Pdf),
            "xml" => Ok(Self::Xml),
            _ => Err(format!("Unknown DataExportFormat variant: {}", s)),
        }
    }
}

impl Default for DataExportFormat {
    fn default() -> Self {
        Self::Json
    }
}
