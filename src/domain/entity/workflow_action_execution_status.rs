use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "workflow_action_execution_status", rename_all = "snake_case")]
pub enum WorkflowActionExecutionStatus {
    Pending,
    Processing,
    Success,
    Failed,
    Skipped,
    Warning,
}

impl std::fmt::Display for WorkflowActionExecutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Processing => write!(f, "processing"),
            Self::Success => write!(f, "success"),
            Self::Failed => write!(f, "failed"),
            Self::Skipped => write!(f, "skipped"),
            Self::Warning => write!(f, "warning"),
        }
    }
}

impl FromStr for WorkflowActionExecutionStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "processing" => Ok(Self::Processing),
            "success" => Ok(Self::Success),
            "failed" => Ok(Self::Failed),
            "skipped" => Ok(Self::Skipped),
            "warning" => Ok(Self::Warning),
            _ => Err(format!("Unknown WorkflowActionExecutionStatus variant: {}", s)),
        }
    }
}

impl Default for WorkflowActionExecutionStatus {
    fn default() -> Self {
        Self::Pending
    }
}
