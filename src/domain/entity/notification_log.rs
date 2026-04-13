use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::NotificationChannel;
use super::NotificationLogStatus;
use super::AuditMetadata;

/// Strongly-typed ID for NotificationLog
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NotificationLogId(pub Uuid);

impl NotificationLogId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for NotificationLogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for NotificationLogId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for NotificationLogId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<NotificationLogId> for Uuid {
    fn from(id: NotificationLogId) -> Self { id.0 }
}

impl AsRef<Uuid> for NotificationLogId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for NotificationLogId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NotificationLog {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<Uuid>,
    pub recipient_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<Uuid>,
    pub channel: NotificationChannel,
    pub status: NotificationLogStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl NotificationLog {
    /// Create a builder for NotificationLog
    pub fn builder() -> NotificationLogBuilder {
        NotificationLogBuilder::default()
    }

    /// Create a new NotificationLog with required fields
    pub fn new(recipient_type: String, channel: NotificationChannel, status: NotificationLogStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            notification_id: None,
            template_id: None,
            recipient_type,
            recipient_id: None,
            channel,
            status,
            sent_at: None,
            delivered_at: None,
            failed_at: None,
            error_message: None,
            external_id: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> NotificationLogId {
        NotificationLogId(self.id)
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
    pub fn status(&self) -> &NotificationLogStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the notification_id field (chainable)
    pub fn with_notification_id(mut self, value: Uuid) -> Self {
        self.notification_id = Some(value);
        self
    }

    /// Set the template_id field (chainable)
    pub fn with_template_id(mut self, value: Uuid) -> Self {
        self.template_id = Some(value);
        self
    }

    /// Set the recipient_id field (chainable)
    pub fn with_recipient_id(mut self, value: Uuid) -> Self {
        self.recipient_id = Some(value);
        self
    }

    /// Set the sent_at field (chainable)
    pub fn with_sent_at(mut self, value: DateTime<Utc>) -> Self {
        self.sent_at = Some(value);
        self
    }

    /// Set the delivered_at field (chainable)
    pub fn with_delivered_at(mut self, value: DateTime<Utc>) -> Self {
        self.delivered_at = Some(value);
        self
    }

    /// Set the failed_at field (chainable)
    pub fn with_failed_at(mut self, value: DateTime<Utc>) -> Self {
        self.failed_at = Some(value);
        self
    }

    /// Set the error_message field (chainable)
    pub fn with_error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    /// Set the external_id field (chainable)
    pub fn with_external_id(mut self, value: String) -> Self {
        self.external_id = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "notification_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.notification_id = v; }
                }
                "template_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.template_id = v; }
                }
                "recipient_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.recipient_type = v; }
                }
                "recipient_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.recipient_id = v; }
                }
                "channel" => {
                    if let Ok(v) = serde_json::from_value(value) { self.channel = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "sent_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.sent_at = v; }
                }
                "delivered_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.delivered_at = v; }
                }
                "failed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.failed_at = v; }
                }
                "error_message" => {
                    if let Ok(v) = serde_json::from_value(value) { self.error_message = v; }
                }
                "external_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.external_id = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for NotificationLog {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "NotificationLog"
    }
}

impl backbone_core::PersistentEntity for NotificationLog {
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

impl backbone_orm::EntityRepoMeta for NotificationLog {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("notification_id".to_string(), "uuid".to_string());
        m.insert("template_id".to_string(), "uuid".to_string());
        m.insert("recipient_id".to_string(), "uuid".to_string());
        m.insert("channel".to_string(), "notification_channel".to_string());
        m.insert("status".to_string(), "notification_log_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["recipient_type"]
    }
}

/// Builder for NotificationLog entity
///
/// Provides a fluent API for constructing NotificationLog instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct NotificationLogBuilder {
    notification_id: Option<Uuid>,
    template_id: Option<Uuid>,
    recipient_type: Option<String>,
    recipient_id: Option<Uuid>,
    channel: Option<NotificationChannel>,
    status: Option<NotificationLogStatus>,
    sent_at: Option<DateTime<Utc>>,
    delivered_at: Option<DateTime<Utc>>,
    failed_at: Option<DateTime<Utc>>,
    error_message: Option<String>,
    external_id: Option<String>,
}

impl NotificationLogBuilder {
    /// Set the notification_id field (optional)
    pub fn notification_id(mut self, value: Uuid) -> Self {
        self.notification_id = Some(value);
        self
    }

    /// Set the template_id field (optional)
    pub fn template_id(mut self, value: Uuid) -> Self {
        self.template_id = Some(value);
        self
    }

    /// Set the recipient_type field (required)
    pub fn recipient_type(mut self, value: String) -> Self {
        self.recipient_type = Some(value);
        self
    }

    /// Set the recipient_id field (optional)
    pub fn recipient_id(mut self, value: Uuid) -> Self {
        self.recipient_id = Some(value);
        self
    }

    /// Set the channel field (required)
    pub fn channel(mut self, value: NotificationChannel) -> Self {
        self.channel = Some(value);
        self
    }

    /// Set the status field (required)
    pub fn status(mut self, value: NotificationLogStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the sent_at field (optional)
    pub fn sent_at(mut self, value: DateTime<Utc>) -> Self {
        self.sent_at = Some(value);
        self
    }

    /// Set the delivered_at field (optional)
    pub fn delivered_at(mut self, value: DateTime<Utc>) -> Self {
        self.delivered_at = Some(value);
        self
    }

    /// Set the failed_at field (optional)
    pub fn failed_at(mut self, value: DateTime<Utc>) -> Self {
        self.failed_at = Some(value);
        self
    }

    /// Set the error_message field (optional)
    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    /// Set the external_id field (optional)
    pub fn external_id(mut self, value: String) -> Self {
        self.external_id = Some(value);
        self
    }

    /// Build the NotificationLog entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<NotificationLog, String> {
        let recipient_type = self.recipient_type.ok_or_else(|| "recipient_type is required".to_string())?;
        let channel = self.channel.ok_or_else(|| "channel is required".to_string())?;
        let status = self.status.ok_or_else(|| "status is required".to_string())?;

        Ok(NotificationLog {
            id: Uuid::new_v4(),
            notification_id: self.notification_id,
            template_id: self.template_id,
            recipient_type,
            recipient_id: self.recipient_id,
            channel,
            status,
            sent_at: self.sent_at,
            delivered_at: self.delivered_at,
            failed_at: self.failed_at,
            error_message: self.error_message,
            external_id: self.external_id,
            metadata: AuditMetadata::default(),
        })
    }
}
