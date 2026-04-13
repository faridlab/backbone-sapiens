use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::BulkOperationType;
use super::BulkOperationStatus;
use super::AuditMetadata;

/// Strongly-typed ID for BulkOperation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BulkOperationId(pub Uuid);

impl BulkOperationId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for BulkOperationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for BulkOperationId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for BulkOperationId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<BulkOperationId> for Uuid {
    fn from(id: BulkOperationId) -> Self { id.0 }
}

impl AsRef<Uuid> for BulkOperationId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for BulkOperationId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BulkOperation {
    pub id: Uuid,
    pub operation_type: BulkOperationType,
    pub status: BulkOperationStatus,
    pub created_by: Uuid,
    pub total_records: i32,
    pub processed_records: i32,
    pub successful_records: i32,
    pub failed_records: i32,
    pub skipped_records: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    pub progress_percentage: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_completion: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub max_retries: i32,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl BulkOperation {
    /// Create a builder for BulkOperation
    pub fn builder() -> BulkOperationBuilder {
        BulkOperationBuilder::default()
    }

    /// Create a new BulkOperation with required fields
    pub fn new(operation_type: BulkOperationType, status: BulkOperationStatus, total_records: i32, processed_records: i32, successful_records: i32, failed_records: i32, skipped_records: i32, progress_percentage: f64, retry_count: i32, max_retries: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            operation_type,
            status,
            created_by: Default::default(),
            total_records,
            processed_records,
            successful_records,
            failed_records,
            skipped_records,
            input_file_path: None,
            output_file_path: None,
            error_file_path: None,
            parameters: None,
            progress_percentage,
            started_at: None,
            completed_at: None,
            estimated_completion: None,
            error_message: None,
            retry_count,
            max_retries,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> BulkOperationId {
        BulkOperationId(self.id)
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
    pub fn status(&self) -> &BulkOperationStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the input_file_path field (chainable)
    pub fn with_input_file_path(mut self, value: String) -> Self {
        self.input_file_path = Some(value);
        self
    }

    /// Set the output_file_path field (chainable)
    pub fn with_output_file_path(mut self, value: String) -> Self {
        self.output_file_path = Some(value);
        self
    }

    /// Set the error_file_path field (chainable)
    pub fn with_error_file_path(mut self, value: String) -> Self {
        self.error_file_path = Some(value);
        self
    }

    /// Set the parameters field (chainable)
    pub fn with_parameters(mut self, value: serde_json::Value) -> Self {
        self.parameters = Some(value);
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

    /// Set the estimated_completion field (chainable)
    pub fn with_estimated_completion(mut self, value: DateTime<Utc>) -> Self {
        self.estimated_completion = Some(value);
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
                "operation_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.operation_type = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "total_records" => {
                    if let Ok(v) = serde_json::from_value(value) { self.total_records = v; }
                }
                "processed_records" => {
                    if let Ok(v) = serde_json::from_value(value) { self.processed_records = v; }
                }
                "successful_records" => {
                    if let Ok(v) = serde_json::from_value(value) { self.successful_records = v; }
                }
                "failed_records" => {
                    if let Ok(v) = serde_json::from_value(value) { self.failed_records = v; }
                }
                "skipped_records" => {
                    if let Ok(v) = serde_json::from_value(value) { self.skipped_records = v; }
                }
                "input_file_path" => {
                    if let Ok(v) = serde_json::from_value(value) { self.input_file_path = v; }
                }
                "output_file_path" => {
                    if let Ok(v) = serde_json::from_value(value) { self.output_file_path = v; }
                }
                "error_file_path" => {
                    if let Ok(v) = serde_json::from_value(value) { self.error_file_path = v; }
                }
                "parameters" => {
                    if let Ok(v) = serde_json::from_value(value) { self.parameters = v; }
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
                "estimated_completion" => {
                    if let Ok(v) = serde_json::from_value(value) { self.estimated_completion = v; }
                }
                "error_message" => {
                    if let Ok(v) = serde_json::from_value(value) { self.error_message = v; }
                }
                "retry_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.retry_count = v; }
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

impl super::Entity for BulkOperation {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "BulkOperation"
    }
}

impl backbone_core::PersistentEntity for BulkOperation {
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

impl backbone_orm::EntityRepoMeta for BulkOperation {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("operation_type".to_string(), "bulk_operation_type".to_string());
        m.insert("status".to_string(), "bulk_operation_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for BulkOperation entity
///
/// Provides a fluent API for constructing BulkOperation instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct BulkOperationBuilder {
    operation_type: Option<BulkOperationType>,
    status: Option<BulkOperationStatus>,
    total_records: Option<i32>,
    processed_records: Option<i32>,
    successful_records: Option<i32>,
    failed_records: Option<i32>,
    skipped_records: Option<i32>,
    input_file_path: Option<String>,
    output_file_path: Option<String>,
    error_file_path: Option<String>,
    parameters: Option<serde_json::Value>,
    progress_percentage: Option<f64>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    estimated_completion: Option<DateTime<Utc>>,
    error_message: Option<String>,
    retry_count: Option<i32>,
    max_retries: Option<i32>,
}

impl BulkOperationBuilder {
    /// Set the operation_type field (required)
    pub fn operation_type(mut self, value: BulkOperationType) -> Self {
        self.operation_type = Some(value);
        self
    }

    /// Set the status field (default: `BulkOperationStatus::default()`)
    pub fn status(mut self, value: BulkOperationStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the total_records field (required)
    pub fn total_records(mut self, value: i32) -> Self {
        self.total_records = Some(value);
        self
    }

    /// Set the processed_records field (default: `0`)
    pub fn processed_records(mut self, value: i32) -> Self {
        self.processed_records = Some(value);
        self
    }

    /// Set the successful_records field (default: `0`)
    pub fn successful_records(mut self, value: i32) -> Self {
        self.successful_records = Some(value);
        self
    }

    /// Set the failed_records field (default: `0`)
    pub fn failed_records(mut self, value: i32) -> Self {
        self.failed_records = Some(value);
        self
    }

    /// Set the skipped_records field (default: `0`)
    pub fn skipped_records(mut self, value: i32) -> Self {
        self.skipped_records = Some(value);
        self
    }

    /// Set the input_file_path field (optional)
    pub fn input_file_path(mut self, value: String) -> Self {
        self.input_file_path = Some(value);
        self
    }

    /// Set the output_file_path field (optional)
    pub fn output_file_path(mut self, value: String) -> Self {
        self.output_file_path = Some(value);
        self
    }

    /// Set the error_file_path field (optional)
    pub fn error_file_path(mut self, value: String) -> Self {
        self.error_file_path = Some(value);
        self
    }

    /// Set the parameters field (optional)
    pub fn parameters(mut self, value: serde_json::Value) -> Self {
        self.parameters = Some(value);
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

    /// Set the estimated_completion field (optional)
    pub fn estimated_completion(mut self, value: DateTime<Utc>) -> Self {
        self.estimated_completion = Some(value);
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

    /// Set the max_retries field (default: `3`)
    pub fn max_retries(mut self, value: i32) -> Self {
        self.max_retries = Some(value);
        self
    }

    /// Build the BulkOperation entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<BulkOperation, String> {
        let operation_type = self.operation_type.ok_or_else(|| "operation_type is required".to_string())?;
        let total_records = self.total_records.ok_or_else(|| "total_records is required".to_string())?;

        Ok(BulkOperation {
            id: Uuid::new_v4(),
            operation_type,
            status: self.status.unwrap_or(BulkOperationStatus::default()),
            created_by: Default::default(),
            total_records,
            processed_records: self.processed_records.unwrap_or(0),
            successful_records: self.successful_records.unwrap_or(0),
            failed_records: self.failed_records.unwrap_or(0),
            skipped_records: self.skipped_records.unwrap_or(0),
            input_file_path: self.input_file_path,
            output_file_path: self.output_file_path,
            error_file_path: self.error_file_path,
            parameters: self.parameters,
            progress_percentage: self.progress_percentage.unwrap_or(0_f64),
            started_at: self.started_at,
            completed_at: self.completed_at,
            estimated_completion: self.estimated_completion,
            error_message: self.error_message,
            retry_count: self.retry_count.unwrap_or(0),
            max_retries: self.max_retries.unwrap_or(3),
            metadata: AuditMetadata::default(),
        })
    }
}
