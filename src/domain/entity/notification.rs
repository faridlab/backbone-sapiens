use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::NotificationType;
use super::NotificationChannel;
use super::NotificationPriority;
use super::AuditMetadata;

/// Strongly-typed ID for Notification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NotificationId(pub Uuid);

impl NotificationId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for NotificationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for NotificationId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for NotificationId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<NotificationId> for Uuid {
    fn from(id: NotificationId) -> Self { id.0 }
}

impl AsRef<Uuid> for NotificationId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for NotificationId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub title: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    pub is_read: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    pub priority: NotificationPriority,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Notification {
    /// Create a builder for Notification
    pub fn builder() -> NotificationBuilder {
        NotificationBuilder::default()
    }

    /// Create a new Notification with required fields
    pub fn new(user_id: Uuid, notification_type: NotificationType, channel: NotificationChannel, title: String, message: String, is_read: bool, priority: NotificationPriority) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            notification_type,
            channel,
            title,
            message,
            data: None,
            is_read,
            read_at: None,
            sent_at: None,
            delivered_at: None,
            expires_at: None,
            priority,
            action_url: None,
            action_text: None,
            category: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> NotificationId {
        NotificationId(self.id)
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

    /// Set the data field (chainable)
    pub fn with_data(mut self, value: serde_json::Value) -> Self {
        self.data = Some(value);
        self
    }

    /// Set the read_at field (chainable)
    pub fn with_read_at(mut self, value: DateTime<Utc>) -> Self {
        self.read_at = Some(value);
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

    /// Set the expires_at field (chainable)
    pub fn with_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the action_url field (chainable)
    pub fn with_action_url(mut self, value: String) -> Self {
        self.action_url = Some(value);
        self
    }

    /// Set the action_text field (chainable)
    pub fn with_action_text(mut self, value: String) -> Self {
        self.action_text = Some(value);
        self
    }

    /// Set the category field (chainable)
    pub fn with_category(mut self, value: String) -> Self {
        self.category = Some(value);
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
                "notification_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.notification_type = v; }
                }
                "channel" => {
                    if let Ok(v) = serde_json::from_value(value) { self.channel = v; }
                }
                "title" => {
                    if let Ok(v) = serde_json::from_value(value) { self.title = v; }
                }
                "message" => {
                    if let Ok(v) = serde_json::from_value(value) { self.message = v; }
                }
                "data" => {
                    if let Ok(v) = serde_json::from_value(value) { self.data = v; }
                }
                "is_read" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_read = v; }
                }
                "read_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.read_at = v; }
                }
                "sent_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.sent_at = v; }
                }
                "delivered_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.delivered_at = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "priority" => {
                    if let Ok(v) = serde_json::from_value(value) { self.priority = v; }
                }
                "action_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.action_url = v; }
                }
                "action_text" => {
                    if let Ok(v) = serde_json::from_value(value) { self.action_text = v; }
                }
                "category" => {
                    if let Ok(v) = serde_json::from_value(value) { self.category = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Notification {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Notification"
    }
}

impl backbone_core::PersistentEntity for Notification {
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

impl backbone_orm::EntityRepoMeta for Notification {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("notification_type".to_string(), "notification_type".to_string());
        m.insert("channel".to_string(), "notification_channel".to_string());
        m.insert("priority".to_string(), "notification_priority".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["title", "message"]
    }
}

/// Builder for Notification entity
///
/// Provides a fluent API for constructing Notification instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct NotificationBuilder {
    user_id: Option<Uuid>,
    notification_type: Option<NotificationType>,
    channel: Option<NotificationChannel>,
    title: Option<String>,
    message: Option<String>,
    data: Option<serde_json::Value>,
    is_read: Option<bool>,
    read_at: Option<DateTime<Utc>>,
    sent_at: Option<DateTime<Utc>>,
    delivered_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    priority: Option<NotificationPriority>,
    action_url: Option<String>,
    action_text: Option<String>,
    category: Option<String>,
}

impl NotificationBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the notification_type field (required)
    pub fn notification_type(mut self, value: NotificationType) -> Self {
        self.notification_type = Some(value);
        self
    }

    /// Set the channel field (required)
    pub fn channel(mut self, value: NotificationChannel) -> Self {
        self.channel = Some(value);
        self
    }

    /// Set the title field (required)
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    /// Set the message field (required)
    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    /// Set the data field (optional)
    pub fn data(mut self, value: serde_json::Value) -> Self {
        self.data = Some(value);
        self
    }

    /// Set the is_read field (default: `false`)
    pub fn is_read(mut self, value: bool) -> Self {
        self.is_read = Some(value);
        self
    }

    /// Set the read_at field (optional)
    pub fn read_at(mut self, value: DateTime<Utc>) -> Self {
        self.read_at = Some(value);
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

    /// Set the expires_at field (optional)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the priority field (default: `NotificationPriority::default()`)
    pub fn priority(mut self, value: NotificationPriority) -> Self {
        self.priority = Some(value);
        self
    }

    /// Set the action_url field (optional)
    pub fn action_url(mut self, value: String) -> Self {
        self.action_url = Some(value);
        self
    }

    /// Set the action_text field (optional)
    pub fn action_text(mut self, value: String) -> Self {
        self.action_text = Some(value);
        self
    }

    /// Set the category field (optional)
    pub fn category(mut self, value: String) -> Self {
        self.category = Some(value);
        self
    }

    /// Build the Notification entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Notification, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let notification_type = self.notification_type.ok_or_else(|| "notification_type is required".to_string())?;
        let channel = self.channel.ok_or_else(|| "channel is required".to_string())?;
        let title = self.title.ok_or_else(|| "title is required".to_string())?;
        let message = self.message.ok_or_else(|| "message is required".to_string())?;

        Ok(Notification {
            id: Uuid::new_v4(),
            user_id,
            notification_type,
            channel,
            title,
            message,
            data: self.data,
            is_read: self.is_read.unwrap_or(false),
            read_at: self.read_at,
            sent_at: self.sent_at,
            delivered_at: self.delivered_at,
            expires_at: self.expires_at,
            priority: self.priority.unwrap_or(NotificationPriority::default()),
            action_url: self.action_url,
            action_text: self.action_text,
            category: self.category,
            metadata: AuditMetadata::default(),
        })
    }
}
