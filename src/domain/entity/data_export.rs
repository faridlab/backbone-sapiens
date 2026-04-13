use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::DataExportStatus;
use super::DataExportFormat;
use super::AuditMetadata;

use crate::domain::state_machine::{DataExportStateMachine, DataExportState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for DataExport
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataExportId(pub Uuid);

impl DataExportId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for DataExportId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for DataExportId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for DataExportId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<DataExportId> for Uuid {
    fn from(id: DataExportId) -> Self { id.0 }
}

impl AsRef<Uuid> for DataExportId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for DataExportId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DataExport {
    pub id: Uuid,
    pub user_id: Uuid,
    pub requested_by: Uuid,
    pub requested_at: DateTime<Utc>,
    pub(crate) status: DataExportStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    pub expires_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_count: Option<i32>,
    pub format: DataExportFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl DataExport {
    /// Create a builder for DataExport
    pub fn builder() -> DataExportBuilder {
        DataExportBuilder::default()
    }

    /// Create a new DataExport with required fields
    pub fn new(user_id: Uuid, requested_by: Uuid, requested_at: DateTime<Utc>, status: DataExportStatus, expires_at: DateTime<Utc>, format: DataExportFormat) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            requested_by,
            requested_at,
            status,
            file_path: None,
            file_url: None,
            expires_at,
            completed_at: None,
            record_count: None,
            format,
            justification: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> DataExportId {
        DataExportId(self.id)
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
    pub fn status(&self) -> &DataExportStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the file_path field (chainable)
    pub fn with_file_path(mut self, value: String) -> Self {
        self.file_path = Some(value);
        self
    }

    /// Set the file_url field (chainable)
    pub fn with_file_url(mut self, value: String) -> Self {
        self.file_url = Some(value);
        self
    }

    /// Set the completed_at field (chainable)
    pub fn with_completed_at(mut self, value: DateTime<Utc>) -> Self {
        self.completed_at = Some(value);
        self
    }

    /// Set the record_count field (chainable)
    pub fn with_record_count(mut self, value: i32) -> Self {
        self.record_count = Some(value);
        self
    }

    /// Set the justification field (chainable)
    pub fn with_justification(mut self, value: String) -> Self {
        self.justification = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: DataExportState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<DataExportState>()?;
        let mut sm = DataExportStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<DataExportStatus>()
            .map_err(|e| StateMachineError::InvalidState(e.to_string()))?;
        Ok(())
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_id = v; }
                }
                "requested_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.requested_by = v; }
                }
                "requested_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.requested_at = v; }
                }
                "file_path" => {
                    if let Ok(v) = serde_json::from_value(value) { self.file_path = v; }
                }
                "file_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.file_url = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "completed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.completed_at = v; }
                }
                "record_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.record_count = v; }
                }
                "format" => {
                    if let Ok(v) = serde_json::from_value(value) { self.format = v; }
                }
                "justification" => {
                    if let Ok(v) = serde_json::from_value(value) { self.justification = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for DataExport {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "DataExport"
    }
}

impl backbone_core::PersistentEntity for DataExport {
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

impl backbone_orm::EntityRepoMeta for DataExport {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "data_export_status".to_string());
        m.insert("format".to_string(), "data_export_format".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for DataExport entity
///
/// Provides a fluent API for constructing DataExport instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct DataExportBuilder {
    user_id: Option<Uuid>,
    requested_by: Option<Uuid>,
    requested_at: Option<DateTime<Utc>>,
    status: Option<DataExportStatus>,
    file_path: Option<String>,
    file_url: Option<String>,
    expires_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    record_count: Option<i32>,
    format: Option<DataExportFormat>,
    justification: Option<String>,
}

impl DataExportBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the requested_by field (required)
    pub fn requested_by(mut self, value: Uuid) -> Self {
        self.requested_by = Some(value);
        self
    }

    /// Set the requested_at field (default: `Utc::now()`)
    pub fn requested_at(mut self, value: DateTime<Utc>) -> Self {
        self.requested_at = Some(value);
        self
    }

    /// Set the status field (default: `DataExportStatus::default()`)
    pub fn status(mut self, value: DataExportStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the file_path field (optional)
    pub fn file_path(mut self, value: String) -> Self {
        self.file_path = Some(value);
        self
    }

    /// Set the file_url field (optional)
    pub fn file_url(mut self, value: String) -> Self {
        self.file_url = Some(value);
        self
    }

    /// Set the expires_at field (required)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the completed_at field (optional)
    pub fn completed_at(mut self, value: DateTime<Utc>) -> Self {
        self.completed_at = Some(value);
        self
    }

    /// Set the record_count field (optional)
    pub fn record_count(mut self, value: i32) -> Self {
        self.record_count = Some(value);
        self
    }

    /// Set the format field (default: `DataExportFormat::default()`)
    pub fn format(mut self, value: DataExportFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Set the justification field (optional)
    pub fn justification(mut self, value: String) -> Self {
        self.justification = Some(value);
        self
    }

    /// Build the DataExport entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<DataExport, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let requested_by = self.requested_by.ok_or_else(|| "requested_by is required".to_string())?;
        let expires_at = self.expires_at.ok_or_else(|| "expires_at is required".to_string())?;

        Ok(DataExport {
            id: Uuid::new_v4(),
            user_id,
            requested_by,
            requested_at: self.requested_at.unwrap_or(Utc::now()),
            status: self.status.unwrap_or(DataExportStatus::default()),
            file_path: self.file_path,
            file_url: self.file_url,
            expires_at,
            completed_at: self.completed_at,
            record_count: self.record_count,
            format: self.format.unwrap_or(DataExportFormat::default()),
            justification: self.justification,
            metadata: AuditMetadata::default(),
        })
    }
}
