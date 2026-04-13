use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AuditLogSeverity;
use super::AuditMetadata;

/// Strongly-typed ID for AuditLog
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AuditLogId(pub Uuid);

impl AuditLogId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for AuditLogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for AuditLogId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for AuditLogId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<AuditLogId> for Uuid {
    fn from(id: AuditLogId) -> Self { id.0 }
}

impl AsRef<Uuid> for AuditLogId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for AuditLogId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    pub severity: AuditLogSeverity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub archived: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_location: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl AuditLog {
    /// Create a builder for AuditLog
    pub fn builder() -> AuditLogBuilder {
        AuditLogBuilder::default()
    }

    /// Create a new AuditLog with required fields
    pub fn new(action: String, severity: AuditLogSeverity, timestamp: DateTime<Utc>, archived: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: None,
            action,
            details: None,
            severity,
            ip_address: None,
            user_agent: None,
            session_id: None,
            resource_type: None,
            resource_id: None,
            timestamp,
            archived,
            archived_at: None,
            archive_location: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> AuditLogId {
        AuditLogId(self.id)
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

    /// Set the user_id field (chainable)
    pub fn with_user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the details field (chainable)
    pub fn with_details(mut self, value: serde_json::Value) -> Self {
        self.details = Some(value);
        self
    }

    /// Set the ip_address field (chainable)
    pub fn with_ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    /// Set the user_agent field (chainable)
    pub fn with_user_agent(mut self, value: String) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Set the session_id field (chainable)
    pub fn with_session_id(mut self, value: Uuid) -> Self {
        self.session_id = Some(value);
        self
    }

    /// Set the resource_type field (chainable)
    pub fn with_resource_type(mut self, value: String) -> Self {
        self.resource_type = Some(value);
        self
    }

    /// Set the resource_id field (chainable)
    pub fn with_resource_id(mut self, value: String) -> Self {
        self.resource_id = Some(value);
        self
    }

    /// Set the archived_at field (chainable)
    pub fn with_archived_at(mut self, value: DateTime<Utc>) -> Self {
        self.archived_at = Some(value);
        self
    }

    /// Set the archive_location field (chainable)
    pub fn with_archive_location(mut self, value: String) -> Self {
        self.archive_location = Some(value);
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
                "action" => {
                    if let Ok(v) = serde_json::from_value(value) { self.action = v; }
                }
                "details" => {
                    if let Ok(v) = serde_json::from_value(value) { self.details = v; }
                }
                "severity" => {
                    if let Ok(v) = serde_json::from_value(value) { self.severity = v; }
                }
                "ip_address" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ip_address = v; }
                }
                "user_agent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_agent = v; }
                }
                "session_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.session_id = v; }
                }
                "resource_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.resource_type = v; }
                }
                "resource_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.resource_id = v; }
                }
                "timestamp" => {
                    if let Ok(v) = serde_json::from_value(value) { self.timestamp = v; }
                }
                "archived" => {
                    if let Ok(v) = serde_json::from_value(value) { self.archived = v; }
                }
                "archived_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.archived_at = v; }
                }
                "archive_location" => {
                    if let Ok(v) = serde_json::from_value(value) { self.archive_location = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for AuditLog {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "AuditLog"
    }
}

impl backbone_core::PersistentEntity for AuditLog {
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

impl backbone_orm::EntityRepoMeta for AuditLog {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("session_id".to_string(), "uuid".to_string());
        m.insert("severity".to_string(), "audit_log_severity".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["action"]
    }
}

/// Builder for AuditLog entity
///
/// Provides a fluent API for constructing AuditLog instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct AuditLogBuilder {
    user_id: Option<Uuid>,
    action: Option<String>,
    details: Option<serde_json::Value>,
    severity: Option<AuditLogSeverity>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    session_id: Option<Uuid>,
    resource_type: Option<String>,
    resource_id: Option<String>,
    timestamp: Option<DateTime<Utc>>,
    archived: Option<bool>,
    archived_at: Option<DateTime<Utc>>,
    archive_location: Option<String>,
}

impl AuditLogBuilder {
    /// Set the user_id field (optional)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the action field (required)
    pub fn action(mut self, value: String) -> Self {
        self.action = Some(value);
        self
    }

    /// Set the details field (optional)
    pub fn details(mut self, value: serde_json::Value) -> Self {
        self.details = Some(value);
        self
    }

    /// Set the severity field (default: `AuditLogSeverity::default()`)
    pub fn severity(mut self, value: AuditLogSeverity) -> Self {
        self.severity = Some(value);
        self
    }

    /// Set the ip_address field (optional)
    pub fn ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    /// Set the user_agent field (optional)
    pub fn user_agent(mut self, value: String) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Set the session_id field (optional)
    pub fn session_id(mut self, value: Uuid) -> Self {
        self.session_id = Some(value);
        self
    }

    /// Set the resource_type field (optional)
    pub fn resource_type(mut self, value: String) -> Self {
        self.resource_type = Some(value);
        self
    }

    /// Set the resource_id field (optional)
    pub fn resource_id(mut self, value: String) -> Self {
        self.resource_id = Some(value);
        self
    }

    /// Set the timestamp field (default: `Utc::now()`)
    pub fn timestamp(mut self, value: DateTime<Utc>) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Set the archived field (default: `false`)
    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    /// Set the archived_at field (optional)
    pub fn archived_at(mut self, value: DateTime<Utc>) -> Self {
        self.archived_at = Some(value);
        self
    }

    /// Set the archive_location field (optional)
    pub fn archive_location(mut self, value: String) -> Self {
        self.archive_location = Some(value);
        self
    }

    /// Build the AuditLog entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<AuditLog, String> {
        let action = self.action.ok_or_else(|| "action is required".to_string())?;

        Ok(AuditLog {
            id: Uuid::new_v4(),
            user_id: self.user_id,
            action,
            details: self.details,
            severity: self.severity.unwrap_or(AuditLogSeverity::default()),
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            session_id: self.session_id,
            resource_type: self.resource_type,
            resource_id: self.resource_id,
            timestamp: self.timestamp.unwrap_or(Utc::now()),
            archived: self.archived.unwrap_or(false),
            archived_at: self.archived_at,
            archive_location: self.archive_location,
            metadata: AuditMetadata::default(),
        })
    }
}
