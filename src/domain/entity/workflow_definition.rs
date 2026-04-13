use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::WorkflowType;
use super::TriggerType;
use super::AuditMetadata;

/// Strongly-typed ID for WorkflowDefinition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkflowDefinitionId(pub Uuid);

impl WorkflowDefinitionId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for WorkflowDefinitionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for WorkflowDefinitionId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for WorkflowDefinitionId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<WorkflowDefinitionId> for Uuid {
    fn from(id: WorkflowDefinitionId) -> Self { id.0 }
}

impl AsRef<Uuid> for WorkflowDefinitionId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for WorkflowDefinitionId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowDefinition {
    pub id: Uuid,
    pub name: String,
    pub workflow_type: WorkflowType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub trigger_type: TriggerType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_config: Option<serde_json::Value>,
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<i32>,
    pub max_retries: i32,
    pub created_by: Uuid,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl WorkflowDefinition {
    /// Create a builder for WorkflowDefinition
    pub fn builder() -> WorkflowDefinitionBuilder {
        WorkflowDefinitionBuilder::default()
    }

    /// Create a new WorkflowDefinition with required fields
    pub fn new(name: String, workflow_type: WorkflowType, trigger_type: TriggerType, is_active: bool, max_retries: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            workflow_type,
            description: None,
            trigger_type,
            trigger_config: None,
            is_active,
            timeout_minutes: None,
            max_retries,
            created_by: Default::default(),
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> WorkflowDefinitionId {
        WorkflowDefinitionId(self.id)
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

    /// Set the trigger_config field (chainable)
    pub fn with_trigger_config(mut self, value: serde_json::Value) -> Self {
        self.trigger_config = Some(value);
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
                "name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.name = v; }
                }
                "workflow_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.workflow_type = v; }
                }
                "description" => {
                    if let Ok(v) = serde_json::from_value(value) { self.description = v; }
                }
                "trigger_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.trigger_type = v; }
                }
                "trigger_config" => {
                    if let Ok(v) = serde_json::from_value(value) { self.trigger_config = v; }
                }
                "is_active" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_active = v; }
                }
                "timeout_minutes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.timeout_minutes = v; }
                }
                "max_retries" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_retries = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for WorkflowDefinition {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "WorkflowDefinition"
    }
}

impl backbone_core::PersistentEntity for WorkflowDefinition {
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

impl backbone_orm::EntityRepoMeta for WorkflowDefinition {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("workflow_type".to_string(), "workflow_type".to_string());
        m.insert("trigger_type".to_string(), "trigger_type".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["name"]
    }
}

/// Builder for WorkflowDefinition entity
///
/// Provides a fluent API for constructing WorkflowDefinition instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct WorkflowDefinitionBuilder {
    name: Option<String>,
    workflow_type: Option<WorkflowType>,
    description: Option<String>,
    trigger_type: Option<TriggerType>,
    trigger_config: Option<serde_json::Value>,
    is_active: Option<bool>,
    timeout_minutes: Option<i32>,
    max_retries: Option<i32>,
}

impl WorkflowDefinitionBuilder {
    /// Set the name field (required)
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the workflow_type field (required)
    pub fn workflow_type(mut self, value: WorkflowType) -> Self {
        self.workflow_type = Some(value);
        self
    }

    /// Set the description field (optional)
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the trigger_type field (required)
    pub fn trigger_type(mut self, value: TriggerType) -> Self {
        self.trigger_type = Some(value);
        self
    }

    /// Set the trigger_config field (optional)
    pub fn trigger_config(mut self, value: serde_json::Value) -> Self {
        self.trigger_config = Some(value);
        self
    }

    /// Set the is_active field (default: `true`)
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Set the timeout_minutes field (optional)
    pub fn timeout_minutes(mut self, value: i32) -> Self {
        self.timeout_minutes = Some(value);
        self
    }

    /// Set the max_retries field (default: `3`)
    pub fn max_retries(mut self, value: i32) -> Self {
        self.max_retries = Some(value);
        self
    }

    /// Build the WorkflowDefinition entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<WorkflowDefinition, String> {
        let name = self.name.ok_or_else(|| "name is required".to_string())?;
        let workflow_type = self.workflow_type.ok_or_else(|| "workflow_type is required".to_string())?;
        let trigger_type = self.trigger_type.ok_or_else(|| "trigger_type is required".to_string())?;

        Ok(WorkflowDefinition {
            id: Uuid::new_v4(),
            name,
            workflow_type,
            description: self.description,
            trigger_type,
            trigger_config: self.trigger_config,
            is_active: self.is_active.unwrap_or(true),
            timeout_minutes: self.timeout_minutes,
            max_retries: self.max_retries.unwrap_or(3),
            created_by: Default::default(),
            metadata: AuditMetadata::default(),
        })
    }
}
