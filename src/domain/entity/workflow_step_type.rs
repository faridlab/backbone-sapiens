use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "workflow_step_type", rename_all = "snake_case")]
pub enum WorkflowStepType {
    Validation,
    DatabaseOperation,
    ExternalService,
    Notification,
    Approval,
    EmailSend,
    SecurityCheck,
    AuditLog,
    UserInteraction,
    BackgroundJob,
}

impl std::fmt::Display for WorkflowStepType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation => write!(f, "validation"),
            Self::DatabaseOperation => write!(f, "database_operation"),
            Self::ExternalService => write!(f, "external_service"),
            Self::Notification => write!(f, "notification"),
            Self::Approval => write!(f, "approval"),
            Self::EmailSend => write!(f, "email_send"),
            Self::SecurityCheck => write!(f, "security_check"),
            Self::AuditLog => write!(f, "audit_log"),
            Self::UserInteraction => write!(f, "user_interaction"),
            Self::BackgroundJob => write!(f, "background_job"),
        }
    }
}

impl FromStr for WorkflowStepType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "validation" => Ok(Self::Validation),
            "database_operation" => Ok(Self::DatabaseOperation),
            "external_service" => Ok(Self::ExternalService),
            "notification" => Ok(Self::Notification),
            "approval" => Ok(Self::Approval),
            "email_send" => Ok(Self::EmailSend),
            "security_check" => Ok(Self::SecurityCheck),
            "audit_log" => Ok(Self::AuditLog),
            "user_interaction" => Ok(Self::UserInteraction),
            "background_job" => Ok(Self::BackgroundJob),
            _ => Err(format!("Unknown WorkflowStepType variant: {}", s)),
        }
    }
}

impl Default for WorkflowStepType {
    fn default() -> Self {
        Self::Validation
    }
}
