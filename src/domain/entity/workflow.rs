use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::WorkflowType;
use super::WorkflowStatus;
use super::AuditMetadata;

/// Strongly-typed ID for Workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkflowId(pub Uuid);

impl WorkflowId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for WorkflowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for WorkflowId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for WorkflowId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<WorkflowId> for Uuid {
    fn from(id: WorkflowId) -> Self { id.0 }
}

impl AsRef<Uuid> for WorkflowId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for WorkflowId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Workflow {
    pub id: Uuid,
    pub workflow_type: WorkflowType,
    pub status: WorkflowStatus,
    pub initiator_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<Uuid>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    pub current_step: i32,
    pub total_steps: i32,
    pub progress_percentage: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    pub retry_count: i32,
    pub max_retries: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Workflow {
    /// Create a builder for Workflow
    pub fn builder() -> WorkflowBuilder {
        WorkflowBuilder::default()
    }

    /// Create a new Workflow with required fields
    pub fn new(workflow_type: WorkflowType, status: WorkflowStatus, initiator_id: Uuid, title: String, current_step: i32, total_steps: i32, progress_percentage: f64, retry_count: i32, max_retries: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            workflow_type,
            status,
            initiator_id,
            target_user_id: None,
            title,
            description: None,
            context: None,
            current_step,
            total_steps,
            progress_percentage,
            started_at: None,
            completed_at: None,
            failed_at: None,
            expires_at: None,
            retry_count,
            max_retries,
            error_message: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> WorkflowId {
        WorkflowId(self.id)
    }

    /// Get when this entity was created
    pub fn created_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.created_at.as_ref()
    }

    /// Get when this entity was last updated
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.updated_at.as_ref()
    }

    /// Check if this entity is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.metadata.deleted_at.is_some()
    }

    /// Check if this entity is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.metadata.deleted_at.is_none()
    }

    /// Get when this entity was deleted
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.deleted_at.as_ref()
    }

    /// Get who created this entity
    pub fn created_by(&self) -> Option<&Uuid> {
        self.metadata.created_by.as_ref()
    }

    /// Get who last updated this entity
    pub fn updated_by(&self) -> Option<&Uuid> {
        self.metadata.updated_by.as_ref()
    }

    /// Get who deleted this entity
    pub fn deleted_by(&self) -> Option<&Uuid> {
        self.metadata.deleted_by.as_ref()
    }

    /// Get the current status
    pub fn status(&self) -> &WorkflowStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the target_user_id field (chainable)
    pub fn with_target_user_id(mut self, value: Uuid) -> Self {
        self.target_user_id = Some(value);
        self
    }

    /// Set the description field (chainable)
    pub fn with_description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the context field (chainable)
    pub fn with_context(mut self, value: serde_json::Value) -> Self {
        self.context = Some(value);
        self
    }

    /// Set the started_at field (chainable)
    pub fn with_started_at(mut self, value: DateTime<Utc>) -> Self {
        self.started_at = Some(value);
        self
    }

    /// Set the completed_at field (chainable)
    pub fn with_completed_at(mut self, value: DateTime<Utc>) -> Self {
        self.completed_at = Some(value);
        self
    }

    /// Set the failed_at field (chainable)
    pub fn with_failed_at(mut self, value: DateTime<Utc>) -> Self {
        self.failed_at = Some(value);
        self
    }

    /// Set the expires_at field (chainable)
    pub fn with_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the error_message field (chainable)
    pub fn with_error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "workflow_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.workflow_type = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "initiator_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.initiator_id = v; }
                }
                "target_user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.target_user_id = v; }
                }
                "title" => {
                    if let Ok(v) = serde_json::from_value(value) { self.title = v; }
                }
                "description" => {
                    if let Ok(v) = serde_json::from_value(value) { self.description = v; }
                }
                "context" => {
                    if let Ok(v) = serde_json::from_value(value) { self.context = v; }
                }
                "current_step" => {
                    if let Ok(v) = serde_json::from_value(value) { self.current_step = v; }
                }
                "total_steps" => {
                    if let Ok(v) = serde_json::from_value(value) { self.total_steps = v; }
                }
                "progress_percentage" => {
                    if let Ok(v) = serde_json::from_value(value) { self.progress_percentage = v; }
                }
                "started_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.started_at = v; }
                }
                "completed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.completed_at = v; }
                }
                "failed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.failed_at = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "retry_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.retry_count = v; }
                }
                "max_retries" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_retries = v; }
                }
                "error_message" => {
                    if let Ok(v) = serde_json::from_value(value) { self.error_message = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Workflow {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Workflow"
    }
}

impl backbone_core::PersistentEntity for Workflow {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.created_at
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.created_at = Some(ts);
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.updated_at
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.updated_at = Some(ts);
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.deleted_at
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.metadata.deleted_at = ts;
    }
}

impl backbone_orm::EntityRepoMeta for Workflow {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("initiator_id".to_string(), "uuid".to_string());
        m.insert("target_user_id".to_string(), "uuid".to_string());
        m.insert("workflow_type".to_string(), "workflow_type".to_string());
        m.insert("status".to_string(), "workflow_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["title"]
    }
}

/// Builder for Workflow entity
///
/// Provides a fluent API for constructing Workflow instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct WorkflowBuilder {
    workflow_type: Option<WorkflowType>,
    status: Option<WorkflowStatus>,
    initiator_id: Option<Uuid>,
    target_user_id: Option<Uuid>,
    title: Option<String>,
    description: Option<String>,
    context: Option<serde_json::Value>,
    current_step: Option<i32>,
    total_steps: Option<i32>,
    progress_percentage: Option<f64>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    failed_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    retry_count: Option<i32>,
    max_retries: Option<i32>,
    error_message: Option<String>,
}

impl WorkflowBuilder {
    /// Set the workflow_type field (required)
    pub fn workflow_type(mut self, value: WorkflowType) -> Self {
        self.workflow_type = Some(value);
        self
    }

    /// Set the status field (default: `WorkflowStatus::default()`)
    pub fn status(mut self, value: WorkflowStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the initiator_id field (required)
    pub fn initiator_id(mut self, value: Uuid) -> Self {
        self.initiator_id = Some(value);
        self
    }

    /// Set the target_user_id field (optional)
    pub fn target_user_id(mut self, value: Uuid) -> Self {
        self.target_user_id = Some(value);
        self
    }

    /// Set the title field (required)
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    /// Set the description field (optional)
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the context field (optional)
    pub fn context(mut self, value: serde_json::Value) -> Self {
        self.context = Some(value);
        self
    }

    /// Set the current_step field (default: `0`)
    pub fn current_step(mut self, value: i32) -> Self {
        self.current_step = Some(value);
        self
    }

    /// Set the total_steps field (default: `0`)
    pub fn total_steps(mut self, value: i32) -> Self {
        self.total_steps = Some(value);
        self
    }

    /// Set the progress_percentage field (default: `0_f64`)
    pub fn progress_percentage(mut self, value: f64) -> Self {
        self.progress_percentage = Some(value);
        self
    }

    /// Set the started_at field (optional)
    pub fn started_at(mut self, value: DateTime<Utc>) -> Self {
        self.started_at = Some(value);
        self
    }

    /// Set the completed_at field (optional)
    pub fn completed_at(mut self, value: DateTime<Utc>) -> Self {
        self.completed_at = Some(value);
        self
    }

    /// Set the failed_at field (optional)
    pub fn failed_at(mut self, value: DateTime<Utc>) -> Self {
        self.failed_at = Some(value);
        self
    }

    /// Set the expires_at field (optional)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the retry_count field (default: `0`)
    pub fn retry_count(mut self, value: i32) -> Self {
        self.retry_count = Some(value);
        self
    }

    /// Set the max_retries field (default: `3`)
    pub fn max_retries(mut self, value: i32) -> Self {
        self.max_retries = Some(value);
        self
    }

    /// Set the error_message field (optional)
    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    /// Build the Workflow entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Workflow, String> {
        let workflow_type = self.workflow_type.ok_or_else(|| "workflow_type is required".to_string())?;
        let initiator_id = self.initiator_id.ok_or_else(|| "initiator_id is required".to_string())?;
        let title = self.title.ok_or_else(|| "title is required".to_string())?;

        Ok(Workflow {
            id: Uuid::new_v4(),
            workflow_type,
            status: self.status.unwrap_or(WorkflowStatus::default()),
            initiator_id,
            target_user_id: self.target_user_id,
            title,
            description: self.description,
            context: self.context,
            current_step: self.current_step.unwrap_or(0),
            total_steps: self.total_steps.unwrap_or(0),
            progress_percentage: self.progress_percentage.unwrap_or(0_f64),
            started_at: self.started_at,
            completed_at: self.completed_at,
            failed_at: self.failed_at,
            expires_at: self.expires_at,
            retry_count: self.retry_count.unwrap_or(0),
            max_retries: self.max_retries.unwrap_or(3),
            error_message: self.error_message,
            metadata: AuditMetadata::default(),
        })
    }
}
