use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::BulkOperationResultStatus;
use super::AuditMetadata;

/// Strongly-typed ID for BulkOperationResult
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BulkOperationResultId(pub Uuid);

impl BulkOperationResultId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for BulkOperationResultId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for BulkOperationResultId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for BulkOperationResultId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<BulkOperationResultId> for Uuid {
    fn from(id: BulkOperationResultId) -> Self { id.0 }
}

impl AsRef<Uuid> for BulkOperationResultId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for BulkOperationResultId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BulkOperationResult {
    pub id: Uuid,
    pub bulk_operation_id: Uuid,
    pub row_number: i32,
    pub record_type: String,
    pub status: BulkOperationResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_duration_ms: Option<i32>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl BulkOperationResult {
    /// Create a builder for BulkOperationResult
    pub fn builder() -> BulkOperationResultBuilder {
        BulkOperationResultBuilder::default()
    }

    /// Create a new BulkOperationResult with required fields
    pub fn new(bulk_operation_id: Uuid, row_number: i32, record_type: String, status: BulkOperationResultStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            bulk_operation_id,
            row_number,
            record_type,
            status,
            input_data: None,
            output_data: None,
            error_message: None,
            warning_message: None,
            processed_at: None,
            processing_duration_ms: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> BulkOperationResultId {
        BulkOperationResultId(self.id)
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
    pub fn status(&self) -> &BulkOperationResultStatus {
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

    /// Set the error_message field (chainable)
    pub fn with_error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    /// Set the warning_message field (chainable)
    pub fn with_warning_message(mut self, value: String) -> Self {
        self.warning_message = Some(value);
        self
    }

    /// Set the processed_at field (chainable)
    pub fn with_processed_at(mut self, value: DateTime<Utc>) -> Self {
        self.processed_at = Some(value);
        self
    }

    /// Set the processing_duration_ms field (chainable)
    pub fn with_processing_duration_ms(mut self, value: i32) -> Self {
        self.processing_duration_ms = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "bulk_operation_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bulk_operation_id = v; }
                }
                "row_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.row_number = v; }
                }
                "record_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.record_type = v; }
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
                "error_message" => {
                    if let Ok(v) = serde_json::from_value(value) { self.error_message = v; }
                }
                "warning_message" => {
                    if let Ok(v) = serde_json::from_value(value) { self.warning_message = v; }
                }
                "processed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.processed_at = v; }
                }
                "processing_duration_ms" => {
                    if let Ok(v) = serde_json::from_value(value) { self.processing_duration_ms = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for BulkOperationResult {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "BulkOperationResult"
    }
}

impl backbone_core::PersistentEntity for BulkOperationResult {
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

impl backbone_orm::EntityRepoMeta for BulkOperationResult {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("bulk_operation_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "bulk_operation_result_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["record_type"]
    }
}

/// Builder for BulkOperationResult entity
///
/// Provides a fluent API for constructing BulkOperationResult instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct BulkOperationResultBuilder {
    bulk_operation_id: Option<Uuid>,
    row_number: Option<i32>,
    record_type: Option<String>,
    status: Option<BulkOperationResultStatus>,
    input_data: Option<serde_json::Value>,
    output_data: Option<serde_json::Value>,
    error_message: Option<String>,
    warning_message: Option<String>,
    processed_at: Option<DateTime<Utc>>,
    processing_duration_ms: Option<i32>,
}

impl BulkOperationResultBuilder {
    /// Set the bulk_operation_id field (required)
    pub fn bulk_operation_id(mut self, value: Uuid) -> Self {
        self.bulk_operation_id = Some(value);
        self
    }

    /// Set the row_number field (required)
    pub fn row_number(mut self, value: i32) -> Self {
        self.row_number = Some(value);
        self
    }

    /// Set the record_type field (required)
    pub fn record_type(mut self, value: String) -> Self {
        self.record_type = Some(value);
        self
    }

    /// Set the status field (required)
    pub fn status(mut self, value: BulkOperationResultStatus) -> Self {
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

    /// Set the error_message field (optional)
    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    /// Set the warning_message field (optional)
    pub fn warning_message(mut self, value: String) -> Self {
        self.warning_message = Some(value);
        self
    }

    /// Set the processed_at field (optional)
    pub fn processed_at(mut self, value: DateTime<Utc>) -> Self {
        self.processed_at = Some(value);
        self
    }

    /// Set the processing_duration_ms field (optional)
    pub fn processing_duration_ms(mut self, value: i32) -> Self {
        self.processing_duration_ms = Some(value);
        self
    }

    /// Build the BulkOperationResult entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<BulkOperationResult, String> {
        let bulk_operation_id = self.bulk_operation_id.ok_or_else(|| "bulk_operation_id is required".to_string())?;
        let row_number = self.row_number.ok_or_else(|| "row_number is required".to_string())?;
        let record_type = self.record_type.ok_or_else(|| "record_type is required".to_string())?;
        let status = self.status.ok_or_else(|| "status is required".to_string())?;

        Ok(BulkOperationResult {
            id: Uuid::new_v4(),
            bulk_operation_id,
            row_number,
            record_type,
            status,
            input_data: self.input_data,
            output_data: self.output_data,
            error_message: self.error_message,
            warning_message: self.warning_message,
            processed_at: self.processed_at,
            processing_duration_ms: self.processing_duration_ms,
            metadata: AuditMetadata::default(),
        })
    }
}
