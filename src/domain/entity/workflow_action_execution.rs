use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::WorkflowActionExecutionStatus;
use super::AuditMetadata;

/// Strongly-typed ID for WorkflowActionExecution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkflowActionExecutionId(pub Uuid);

impl WorkflowActionExecutionId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for WorkflowActionExecutionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for WorkflowActionExecutionId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for WorkflowActionExecutionId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<WorkflowActionExecutionId> for Uuid {
    fn from(id: WorkflowActionExecutionId) -> Self { id.0 }
}

impl AsRef<Uuid> for WorkflowActionExecutionId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for WorkflowActionExecutionId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowActionExecution {
    pub id: Uuid,
    pub workflow_execution_id: Uuid,
    pub workflow_action_id: Uuid,
    pub action_order: i32,
    pub status: WorkflowActionExecutionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    pub retry_count: i32,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl WorkflowActionExecution {
    /// Create a builder for WorkflowActionExecution
    pub fn builder() -> WorkflowActionExecutionBuilder {
        WorkflowActionExecutionBuilder::default()
    }

    /// Create a new WorkflowActionExecution with required fields
    pub fn new(workflow_execution_id: Uuid, workflow_action_id: Uuid, action_order: i32, status: WorkflowActionExecutionStatus, retry_count: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            workflow_execution_id,
            workflow_action_id,
            action_order,
            status,
            input_data: None,
            output_data: None,
            started_at: None,
            completed_at: None,
            failed_at: None,
            duration_ms: None,
            error_message: None,
            retry_count,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> WorkflowActionExecutionId {
        WorkflowActionExecutionId(self.id)
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
    pub fn status(&self) -> &WorkflowActionExecutionStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the input_data field (chainable)
    pub fn with_input_data(mut self, value: serde_json::Value) -> Self {
        self.input_data = Some(value);
        self
    }

    /// Set the output_data field (chainable)
    pub fn with_output_data(mut self, value: serde_json::Value) -> Self {
        self.output_data = Some(value);
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

    /// Set the duration_ms field (chainable)
    pub fn with_duration_ms(mut self, value: i32) -> Self {
        self.duration_ms = Some(value);
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
                "workflow_execution_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.workflow_execution_id = v; }
                }
                "workflow_action_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.workflow_action_id = v; }
                }
                "action_order" => {
                    if let Ok(v) = serde_json::from_value(value) { self.action_order = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "input_data" => {
                    if let Ok(v) = serde_json::from_value(value) { self.input_data = v; }
                }
                "output_data" => {
                    if let Ok(v) = serde_json::from_value(value) { self.output_data = v; }
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
                "duration_ms" => {
                    if let Ok(v) = serde_json::from_value(value) { self.duration_ms = v; }
                }
                "error_message" => {
                    if let Ok(v) = serde_json::from_value(value) { self.error_message = v; }
                }
                "retry_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.retry_count = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for WorkflowActionExecution {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "WorkflowActionExecution"
    }
}

impl backbone_core::PersistentEntity for WorkflowActionExecution {
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

impl backbone_orm::EntityRepoMeta for WorkflowActionExecution {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("workflow_execution_id".to_string(), "uuid".to_string());
        m.insert("workflow_action_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "workflow_action_execution_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for WorkflowActionExecution entity
///
/// Provides a fluent API for constructing WorkflowActionExecution instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct WorkflowActionExecutionBuilder {
    workflow_execution_id: Option<Uuid>,
    workflow_action_id: Option<Uuid>,
    action_order: Option<i32>,
    status: Option<WorkflowActionExecutionStatus>,
    input_data: Option<serde_json::Value>,
    output_data: Option<serde_json::Value>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    failed_at: Option<DateTime<Utc>>,
    duration_ms: Option<i32>,
    error_message: Option<String>,
    retry_count: Option<i32>,
}

impl WorkflowActionExecutionBuilder {
    /// Set the workflow_execution_id field (required)
    pub fn workflow_execution_id(mut self, value: Uuid) -> Self {
        self.workflow_execution_id = Some(value);
        self
    }

    /// Set the workflow_action_id field (required)
    pub fn workflow_action_id(mut self, value: Uuid) -> Self {
        self.workflow_action_id = Some(value);
        self
    }

    /// Set the action_order field (required)
    pub fn action_order(mut self, value: i32) -> Self {
        self.action_order = Some(value);
        self
    }

    /// Set the status field (default: `WorkflowActionExecutionStatus::default()`)
    pub fn status(mut self, value: WorkflowActionExecutionStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the input_data field (optional)
    pub fn input_data(mut self, value: serde_json::Value) -> Self {
        self.input_data = Some(value);
        self
    }

    /// Set the output_data field (optional)
    pub fn output_data(mut self, value: serde_json::Value) -> Self {
        self.output_data = Some(value);
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

    /// Set the duration_ms field (optional)
    pub fn duration_ms(mut self, value: i32) -> Self {
        self.duration_ms = Some(value);
        self
    }

    /// Set the error_message field (optional)
    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    /// Set the retry_count field (default: `0`)
    pub fn retry_count(mut self, value: i32) -> Self {
        self.retry_count = Some(value);
        self
    }

    /// Build the WorkflowActionExecution entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<WorkflowActionExecution, String> {
        let workflow_execution_id = self.workflow_execution_id.ok_or_else(|| "workflow_execution_id is required".to_string())?;
        let workflow_action_id = self.workflow_action_id.ok_or_else(|| "workflow_action_id is required".to_string())?;
        let action_order = self.action_order.ok_or_else(|| "action_order is required".to_string())?;

        Ok(WorkflowActionExecution {
            id: Uuid::new_v4(),
            workflow_execution_id,
            workflow_action_id,
            action_order,
            status: self.status.unwrap_or(WorkflowActionExecutionStatus::default()),
            input_data: self.input_data,
            output_data: self.output_data,
            started_at: self.started_at,
            completed_at: self.completed_at,
            failed_at: self.failed_at,
            duration_ms: self.duration_ms,
            error_message: self.error_message,
            retry_count: self.retry_count.unwrap_or(0),
            metadata: AuditMetadata::default(),
        })
    }
}
