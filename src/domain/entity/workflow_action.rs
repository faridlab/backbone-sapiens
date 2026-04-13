use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::ActionType;
use super::AuditMetadata;

/// Strongly-typed ID for WorkflowAction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkflowActionId(pub Uuid);

impl WorkflowActionId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for WorkflowActionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for WorkflowActionId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for WorkflowActionId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<WorkflowActionId> for Uuid {
    fn from(id: WorkflowActionId) -> Self { id.0 }
}

impl AsRef<Uuid> for WorkflowActionId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for WorkflowActionId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowAction {
    pub id: Uuid,
    pub workflow_definition_id: Uuid,
    pub action_type: ActionType,
    pub action_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub step_order: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<serde_json::Value>,
    pub is_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<i32>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl WorkflowAction {
    /// Create a builder for WorkflowAction
    pub fn builder() -> WorkflowActionBuilder {
        WorkflowActionBuilder::default()
    }

    /// Create a new WorkflowAction with required fields
    pub fn new(workflow_definition_id: Uuid, action_type: ActionType, action_name: String, step_order: i32, is_required: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            workflow_definition_id,
            action_type,
            action_name,
            description: None,
            step_order,
            parameters: None,
            conditions: None,
            is_required,
            timeout_minutes: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> WorkflowActionId {
        WorkflowActionId(self.id)
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

    /// Set the conditions field (chainable)
    pub fn with_conditions(mut self, value: serde_json::Value) -> Self {
        self.conditions = Some(value);
        self
    }

    /// Set the timeout_minutes field (chainable)
    pub fn with_timeout_minutes(mut self, value: i32) -> Self {
        self.timeout_minutes = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "workflow_definition_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.workflow_definition_id = v; }
                }
                "action_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.action_type = v; }
                }
                "action_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.action_name = v; }
                }
                "description" => {
                    if let Ok(v) = serde_json::from_value(value) { self.description = v; }
                }
                "step_order" => {
                    if let Ok(v) = serde_json::from_value(value) { self.step_order = v; }
                }
                "parameters" => {
                    if let Ok(v) = serde_json::from_value(value) { self.parameters = v; }
                }
                "conditions" => {
                    if let Ok(v) = serde_json::from_value(value) { self.conditions = v; }
                }
                "is_required" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_required = v; }
                }
                "timeout_minutes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.timeout_minutes = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for WorkflowAction {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "WorkflowAction"
    }
}

impl backbone_core::PersistentEntity for WorkflowAction {
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

impl backbone_orm::EntityRepoMeta for WorkflowAction {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("workflow_definition_id".to_string(), "uuid".to_string());
        m.insert("action_type".to_string(), "action_type".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["action_name"]
    }
}

/// Builder for WorkflowAction entity
///
/// Provides a fluent API for constructing WorkflowAction instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct WorkflowActionBuilder {
    workflow_definition_id: Option<Uuid>,
    action_type: Option<ActionType>,
    action_name: Option<String>,
    description: Option<String>,
    step_order: Option<i32>,
    parameters: Option<serde_json::Value>,
    conditions: Option<serde_json::Value>,
    is_required: Option<bool>,
    timeout_minutes: Option<i32>,
}

impl WorkflowActionBuilder {
    /// Set the workflow_definition_id field (required)
    pub fn workflow_definition_id(mut self, value: Uuid) -> Self {
        self.workflow_definition_id = Some(value);
        self
    }

    /// Set the action_type field (required)
    pub fn action_type(mut self, value: ActionType) -> Self {
        self.action_type = Some(value);
        self
    }

    /// Set the action_name field (required)
    pub fn action_name(mut self, value: String) -> Self {
        self.action_name = Some(value);
        self
    }

    /// Set the description field (optional)
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the step_order field (required)
    pub fn step_order(mut self, value: i32) -> Self {
        self.step_order = Some(value);
        self
    }

    /// Set the parameters field (optional)
    pub fn parameters(mut self, value: serde_json::Value) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Set the conditions field (optional)
    pub fn conditions(mut self, value: serde_json::Value) -> Self {
        self.conditions = Some(value);
        self
    }

    /// Set the is_required field (default: `true`)
    pub fn is_required(mut self, value: bool) -> Self {
        self.is_required = Some(value);
        self
    }

    /// Set the timeout_minutes field (optional)
    pub fn timeout_minutes(mut self, value: i32) -> Self {
        self.timeout_minutes = Some(value);
        self
    }

    /// Build the WorkflowAction entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<WorkflowAction, String> {
        let workflow_definition_id = self.workflow_definition_id.ok_or_else(|| "workflow_definition_id is required".to_string())?;
        let action_type = self.action_type.ok_or_else(|| "action_type is required".to_string())?;
        let action_name = self.action_name.ok_or_else(|| "action_name is required".to_string())?;
        let step_order = self.step_order.ok_or_else(|| "step_order is required".to_string())?;

        Ok(WorkflowAction {
            id: Uuid::new_v4(),
            workflow_definition_id,
            action_type,
            action_name,
            description: self.description,
            step_order,
            parameters: self.parameters,
            conditions: self.conditions,
            is_required: self.is_required.unwrap_or(true),
            timeout_minutes: self.timeout_minutes,
            metadata: AuditMetadata::default(),
        })
    }
}
