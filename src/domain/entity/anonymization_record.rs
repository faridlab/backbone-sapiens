use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AnonymizationMethod;
use super::AnonymizationStatus;
use super::AuditMetadata;

/// Strongly-typed ID for AnonymizationRecord
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnonymizationRecordId(pub Uuid);

impl AnonymizationRecordId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for AnonymizationRecordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for AnonymizationRecordId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for AnonymizationRecordId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<AnonymizationRecordId> for Uuid {
    fn from(id: AnonymizationRecordId) -> Self { id.0 }
}

impl AsRef<Uuid> for AnonymizationRecordId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for AnonymizationRecordId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AnonymizationRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub original_email: String,
    pub original_username: String,
    pub anonymized_by: Uuid,
    pub anonymized_at: DateTime<Utc>,
    pub reason: String,
    pub method: AnonymizationMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_days: Option<i32>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
    pub status: AnonymizationStatus,
}

impl AnonymizationRecord {
    /// Create a builder for AnonymizationRecord
    pub fn builder() -> AnonymizationRecordBuilder {
        AnonymizationRecordBuilder::default()
    }

    /// Create a new AnonymizationRecord with required fields
    pub fn new(user_id: Uuid, original_email: String, original_username: String, anonymized_by: Uuid, anonymized_at: DateTime<Utc>, reason: String, method: AnonymizationMethod, status: AnonymizationStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            original_email,
            original_username,
            anonymized_by,
            anonymized_at,
            reason,
            method,
            retention_period_days: None,
            metadata: AuditMetadata::default(),
            status,
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> AnonymizationRecordId {
        AnonymizationRecordId(self.id)
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
    pub fn status(&self) -> &AnonymizationStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the retention_period_days field (chainable)
    pub fn with_retention_period_days(mut self, value: i32) -> Self {
        self.retention_period_days = Some(value);
        self
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
                "original_email" => {
                    if let Ok(v) = serde_json::from_value(value) { self.original_email = v; }
                }
                "original_username" => {
                    if let Ok(v) = serde_json::from_value(value) { self.original_username = v; }
                }
                "anonymized_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.anonymized_by = v; }
                }
                "anonymized_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.anonymized_at = v; }
                }
                "reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.reason = v; }
                }
                "method" => {
                    if let Ok(v) = serde_json::from_value(value) { self.method = v; }
                }
                "retention_period_days" => {
                    if let Ok(v) = serde_json::from_value(value) { self.retention_period_days = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for AnonymizationRecord {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "AnonymizationRecord"
    }
}

impl backbone_core::PersistentEntity for AnonymizationRecord {
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

impl backbone_orm::EntityRepoMeta for AnonymizationRecord {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("method".to_string(), "anonymization_method".to_string());
        m.insert("status".to_string(), "anonymization_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["original_email", "original_username", "reason"]
    }
}

/// Builder for AnonymizationRecord entity
///
/// Provides a fluent API for constructing AnonymizationRecord instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct AnonymizationRecordBuilder {
    user_id: Option<Uuid>,
    original_email: Option<String>,
    original_username: Option<String>,
    anonymized_by: Option<Uuid>,
    anonymized_at: Option<DateTime<Utc>>,
    reason: Option<String>,
    method: Option<AnonymizationMethod>,
    retention_period_days: Option<i32>,
    status: Option<AnonymizationStatus>,
}

impl AnonymizationRecordBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the original_email field (required)
    pub fn original_email(mut self, value: String) -> Self {
        self.original_email = Some(value);
        self
    }

    /// Set the original_username field (required)
    pub fn original_username(mut self, value: String) -> Self {
        self.original_username = Some(value);
        self
    }

    /// Set the anonymized_by field (required)
    pub fn anonymized_by(mut self, value: Uuid) -> Self {
        self.anonymized_by = Some(value);
        self
    }

    /// Set the anonymized_at field (default: `Utc::now()`)
    pub fn anonymized_at(mut self, value: DateTime<Utc>) -> Self {
        self.anonymized_at = Some(value);
        self
    }

    /// Set the reason field (required)
    pub fn reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }

    /// Set the method field (default: `AnonymizationMethod::default()`)
    pub fn method(mut self, value: AnonymizationMethod) -> Self {
        self.method = Some(value);
        self
    }

    /// Set the retention_period_days field (optional)
    pub fn retention_period_days(mut self, value: i32) -> Self {
        self.retention_period_days = Some(value);
        self
    }

    /// Set the status field (default: `AnonymizationStatus::default()`)
    pub fn status(mut self, value: AnonymizationStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the AnonymizationRecord entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<AnonymizationRecord, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let original_email = self.original_email.ok_or_else(|| "original_email is required".to_string())?;
        let original_username = self.original_username.ok_or_else(|| "original_username is required".to_string())?;
        let anonymized_by = self.anonymized_by.ok_or_else(|| "anonymized_by is required".to_string())?;
        let reason = self.reason.ok_or_else(|| "reason is required".to_string())?;

        Ok(AnonymizationRecord {
            id: Uuid::new_v4(),
            user_id,
            original_email,
            original_username,
            anonymized_by,
            anonymized_at: self.anonymized_at.unwrap_or(Utc::now()),
            reason,
            method: self.method.unwrap_or(AnonymizationMethod::default()),
            retention_period_days: self.retention_period_days,
            metadata: AuditMetadata::default(),
            status: self.status.unwrap_or(AnonymizationStatus::default()),
        })
    }
}
