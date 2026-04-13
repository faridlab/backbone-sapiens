use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "action_type", rename_all = "snake_case")]
pub enum ActionType {
    SendEmail,
    CreateTask,
    UpdateRecord,
    CallApi,
    Approve,
    Reject,
    NotifyUser,
    ScheduleJob,
}

impl std::fmt::Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SendEmail => write!(f, "send_email"),
            Self::CreateTask => write!(f, "create_task"),
            Self::UpdateRecord => write!(f, "update_record"),
            Self::CallApi => write!(f, "call_api"),
            Self::Approve => write!(f, "approve"),
            Self::Reject => write!(f, "reject"),
            Self::NotifyUser => write!(f, "notify_user"),
            Self::ScheduleJob => write!(f, "schedule_job"),
        }
    }
}

impl FromStr for ActionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "send_email" => Ok(Self::SendEmail),
            "create_task" => Ok(Self::CreateTask),
            "update_record" => Ok(Self::UpdateRecord),
            "call_api" => Ok(Self::CallApi),
            "approve" => Ok(Self::Approve),
            "reject" => Ok(Self::Reject),
            "notify_user" => Ok(Self::NotifyUser),
            "schedule_job" => Ok(Self::ScheduleJob),
            _ => Err(format!("Unknown ActionType variant: {}", s)),
        }
    }
}

impl Default for ActionType {
    fn default() -> Self {
        Self::SendEmail
    }
}
