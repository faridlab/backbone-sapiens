use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::WorkflowExecutionStatus;
use super::AuditMetadata;

/// Strongly-typed ID for WorkflowExecution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkflowExecutionId(pub Uuid);

impl WorkflowExecutionId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for WorkflowExecutionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for WorkflowExecutionId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for WorkflowExecutionId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<WorkflowExecutionId> for Uuid {
    fn from(id: WorkflowExecutionId) -> Self { id.0 }
}

impl AsRef<Uuid> for WorkflowExecutionId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for WorkflowExecutionId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowExecution {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub workflow_definition_id: Uuid,
    pub status: WorkflowExecutionStatus,
    pub current_action: i32,
    pub total_actions: i32,
    pub progress_percentage: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_context: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl WorkflowExecution {
    /// Create a builder for WorkflowExecution
    pub fn builder() -> WorkflowExecutionBuilder {
        WorkflowExecutionBuilder::default()
    }

    /// Create a new WorkflowExecution with required fields
    pub fn new(workflow_id: Uuid, workflow_definition_id: Uuid, status: WorkflowExecutionStatus, current_action: i32, total_actions: i32, progress_percentage: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            workflow_id,
            workflow_definition_id,
            status,
            current_action,
            total_actions,
            progress_percentage,
            timeout_minutes: None,
            started_at: None,
            completed_at: None,
            failed_at: None,
            cancelled_at: None,
            error_message: None,
            execution_context: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> WorkflowExecutionId {
        WorkflowExecutionId(self.id)
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
    pub fn status(&self) -> &WorkflowExecutionStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the timeout_minutes field (chainable)
    pub fn with_timeout_minutes(mut self, value: i32) -> Self {
        self.timeout_minutes = Some(value);
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

    /// Set the cancelled_at field (chainable)
    pub fn with_cancelled_at(mut self, value: DateTime<Utc>) -> Self {
        self.cancelled_at = Some(value);
        self
    }

    /// Set the error_message field (chainable)
    pub fn with_error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    /// Set the execution_context field (chainable)
    pub fn with_execution_context(mut self, value: serde_json::Value) -> Self {
        self.execution_context = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "workflow_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.workflow_id = v; }
                }
                "workflow_definition_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.workflow_definition_id = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "current_action" => {
                    if let Ok(v) = serde_json::from_value(value) { self.current_action = v; }
                }
                "total_actions" => {
                    if let Ok(v) = serde_json::from_value(value) { self.total_actions = v; }
                }
                "progress_percentage" => {
                    if let Ok(v) = serde_json::from_value(value) { self.progress_percentage = v; }
                }
                "timeout_minutes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.timeout_minutes = v; }
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
                "cancelled_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cancelled_at = v; }
                }
                "error_message" => {
                    if let Ok(v) = serde_json::from_value(value) { self.error_message = v; }
                }
                "execution_context" => {
                    if let Ok(v) = serde_json::from_value(value) { self.execution_context = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for WorkflowExecution {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "WorkflowExecution"
    }
}

impl backbone_core::PersistentEntity for WorkflowExecution {
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

impl backbone_orm::EntityRepoMeta for WorkflowExecution {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("workflow_id".to_string(), "uuid".to_string());
        m.insert("workflow_definition_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "workflow_execution_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for WorkflowExecution entity
///
/// Provides a fluent API for constructing WorkflowExecution instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct WorkflowExecutionBuilder {
    workflow_id: Option<Uuid>,
    workflow_definition_id: Option<Uuid>,
    status: Option<WorkflowExecutionStatus>,
    current_action: Option<i32>,
    total_actions: Option<i32>,
    progress_percentage: Option<f64>,
    timeout_minutes: Option<i32>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    failed_at: Option<DateTime<Utc>>,
    cancelled_at: Option<DateTime<Utc>>,
    error_message: Option<String>,
    execution_context: Option<serde_json::Value>,
}

impl WorkflowExecutionBuilder {
    /// Set the workflow_id field (required)
    pub fn workflow_id(mut self, value: Uuid) -> Self {
        self.workflow_id = Some(value);
        self
    }

    /// Set the workflow_definition_id field (required)
    pub fn workflow_definition_id(mut self, value: Uuid) -> Self {
        self.workflow_definition_id = Some(value);
        self
    }

    /// Set the status field (default: `WorkflowExecutionStatus::default()`)
    pub fn status(mut self, value: WorkflowExecutionStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the current_action field (default: `0`)
    pub fn current_action(mut self, value: i32) -> Self {
        self.current_action = Some(value);
        self
    }

    /// Set the total_actions field (default: `0`)
    pub fn total_actions(mut self, value: i32) -> Self {
        self.total_actions = Some(value);
        self
    }

    /// Set the progress_percentage field (default: `0_f64`)
    pub fn progress_percentage(mut self, value: f64) -> Self {
        self.progress_percentage = Some(value);
        self
    }

    /// Set the timeout_minutes field (optional)
    pub fn timeout_minutes(mut self, value: i32) -> Self {
        self.timeout_minutes = Some(value);
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

    /// Set the cancelled_at field (optional)
    pub fn cancelled_at(mut self, value: DateTime<Utc>) -> Self {
        self.cancelled_at = Some(value);
        self
    }

    /// Set the error_message field (optional)
    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    /// Set the execution_context field (optional)
    pub fn execution_context(mut self, value: serde_json::Value) -> Self {
        self.execution_context = Some(value);
        self
    }

    /// Build the WorkflowExecution entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<WorkflowExecution, String> {
        let workflow_id = self.workflow_id.ok_or_else(|| "workflow_id is required".to_string())?;
        let workflow_definition_id = self.workflow_definition_id.ok_or_else(|| "workflow_definition_id is required".to_string())?;

        Ok(WorkflowExecution {
            id: Uuid::new_v4(),
            workflow_id,
            workflow_definition_id,
            status: self.status.unwrap_or(WorkflowExecutionStatus::default()),
            current_action: self.current_action.unwrap_or(0),
            total_actions: self.total_actions.unwrap_or(0),
            progress_percentage: self.progress_percentage.unwrap_or(0_f64),
            timeout_minutes: self.timeout_minutes,
            started_at: self.started_at,
            completed_at: self.completed_at,
            failed_at: self.failed_at,
            cancelled_at: self.cancelled_at,
            error_message: self.error_message,
            execution_context: self.execution_context,
            metadata: AuditMetadata::default(),
        })
    }
}
