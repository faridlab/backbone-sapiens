use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::WorkflowStepType;
use super::WorkflowStepStatus;
use super::AuditMetadata;

/// Strongly-typed ID for WorkflowStep
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkflowStepId(pub Uuid);

impl WorkflowStepId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for WorkflowStepId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for WorkflowStepId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for WorkflowStepId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<WorkflowStepId> for Uuid {
    fn from(id: WorkflowStepId) -> Self { id.0 }
}

impl AsRef<Uuid> for WorkflowStepId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for WorkflowStepId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowStep {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub step_number: i32,
    pub step_type: WorkflowStepType,
    pub step_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: WorkflowStepStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
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

impl WorkflowStep {
    /// Create a builder for WorkflowStep
    pub fn builder() -> WorkflowStepBuilder {
        WorkflowStepBuilder::default()
    }

    /// Create a new WorkflowStep with required fields
    pub fn new(workflow_id: Uuid, step_number: i32, step_type: WorkflowStepType, step_name: String, status: WorkflowStepStatus, retry_count: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            workflow_id,
            step_number,
            step_type,
            step_name,
            description: None,
            status,
            parameters: None,
            result: None,
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
    pub fn typed_id(&self) -> WorkflowStepId {
        WorkflowStepId(self.id)
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
    pub fn status(&self) -> &WorkflowStepStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the description field (chainable)
    pub fn with_description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the parameters field (chainable)
    pub fn with_parameters(mut self, value: serde_json::Value) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Set the result field (chainable)
    pub fn with_result(mut self, value: serde_json::Value) -> Self {
        self.result = Some(value);
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
                "workflow_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.workflow_id = v; }
                }
                "step_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.step_number = v; }
                }
                "step_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.step_type = v; }
                }
                "step_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.step_name = v; }
                }
                "description" => {
                    if let Ok(v) = serde_json::from_value(value) { self.description = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "parameters" => {
                    if let Ok(v) = serde_json::from_value(value) { self.parameters = v; }
                }
                "result" => {
                    if let Ok(v) = serde_json::from_value(value) { self.result = v; }
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

impl super::Entity for WorkflowStep {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "WorkflowStep"
    }
}

impl backbone_core::PersistentEntity for WorkflowStep {
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

impl backbone_orm::EntityRepoMeta for WorkflowStep {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("workflow_id".to_string(), "uuid".to_string());
        m.insert("step_type".to_string(), "workflow_step_type".to_string());
        m.insert("status".to_string(), "workflow_step_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["step_name"]
    }
}

/// Builder for WorkflowStep entity
///
/// Provides a fluent API for constructing WorkflowStep instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct WorkflowStepBuilder {
    workflow_id: Option<Uuid>,
    step_number: Option<i32>,
    step_type: Option<WorkflowStepType>,
    step_name: Option<String>,
    description: Option<String>,
    status: Option<WorkflowStepStatus>,
    parameters: Option<serde_json::Value>,
    result: Option<serde_json::Value>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    failed_at: Option<DateTime<Utc>>,
    duration_ms: Option<i32>,
    error_message: Option<String>,
    retry_count: Option<i32>,
}

impl WorkflowStepBuilder {
    /// Set the workflow_id field (required)
    pub fn workflow_id(mut self, value: Uuid) -> Self {
        self.workflow_id = Some(value);
        self
    }

    /// Set the step_number field (required)
    pub fn step_number(mut self, value: i32) -> Self {
        self.step_number = Some(value);
        self
    }

    /// Set the step_type field (required)
    pub fn step_type(mut self, value: WorkflowStepType) -> Self {
        self.step_type = Some(value);
        self
    }

    /// Set the step_name field (required)
    pub fn step_name(mut self, value: String) -> Self {
        self.step_name = Some(value);
        self
    }

    /// Set the description field (optional)
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the status field (default: `WorkflowStepStatus::default()`)
    pub fn status(mut self, value: WorkflowStepStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the parameters field (optional)
    pub fn parameters(mut self, value: serde_json::Value) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Set the result field (optional)
    pub fn result(mut self, value: serde_json::Value) -> Self {
        self.result = Some(value);
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

    /// Build the WorkflowStep entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<WorkflowStep, String> {
        let workflow_id = self.workflow_id.ok_or_else(|| "workflow_id is required".to_string())?;
        let step_number = self.step_number.ok_or_else(|| "step_number is required".to_string())?;
        let step_type = self.step_type.ok_or_else(|| "step_type is required".to_string())?;
        let step_name = self.step_name.ok_or_else(|| "step_name is required".to_string())?;

        Ok(WorkflowStep {
            id: Uuid::new_v4(),
            workflow_id,
            step_number,
            step_type,
            step_name,
            description: self.description,
            status: self.status.unwrap_or(WorkflowStepStatus::default()),
            parameters: self.parameters,
            result: self.result,
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
