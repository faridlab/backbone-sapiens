use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AnalyticsEventType;
use super::AuditMetadata;

/// Strongly-typed ID for AnalyticsEvent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnalyticsEventId(pub Uuid);

impl AnalyticsEventId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for AnalyticsEventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for AnalyticsEventId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for AnalyticsEventId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<AnalyticsEventId> for Uuid {
    fn from(id: AnalyticsEventId) -> Self { id.0 }
}

impl AsRef<Uuid> for AnalyticsEventId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for AnalyticsEventId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AnalyticsEvent {
    pub id: Uuid,
    pub event_type: AnalyticsEventType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geographic_data: Option<serde_json::Value>,
    pub processed: bool,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl AnalyticsEvent {
    /// Create a builder for AnalyticsEvent
    pub fn builder() -> AnalyticsEventBuilder {
        AnalyticsEventBuilder::default()
    }

    /// Create a new AnalyticsEvent with required fields
    pub fn new(event_type: AnalyticsEventType, timestamp: DateTime<Utc>, processed: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            user_id: None,
            session_id: None,
            resource_type: None,
            resource_id: None,
            properties: None,
            timestamp,
            ip_address: None,
            user_agent: None,
            geographic_data: None,
            processed,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> AnalyticsEventId {
        AnalyticsEventId(self.id)
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

    /// Set the properties field (chainable)
    pub fn with_properties(mut self, value: serde_json::Value) -> Self {
        self.properties = Some(value);
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

    /// Set the geographic_data field (chainable)
    pub fn with_geographic_data(mut self, value: serde_json::Value) -> Self {
        self.geographic_data = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "event_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.event_type = v; }
                }
                "user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_id = v; }
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
                "properties" => {
                    if let Ok(v) = serde_json::from_value(value) { self.properties = v; }
                }
                "timestamp" => {
                    if let Ok(v) = serde_json::from_value(value) { self.timestamp = v; }
                }
                "ip_address" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ip_address = v; }
                }
                "user_agent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_agent = v; }
                }
                "geographic_data" => {
                    if let Ok(v) = serde_json::from_value(value) { self.geographic_data = v; }
                }
                "processed" => {
                    if let Ok(v) = serde_json::from_value(value) { self.processed = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for AnalyticsEvent {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "AnalyticsEvent"
    }
}

impl backbone_core::PersistentEntity for AnalyticsEvent {
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

impl backbone_orm::EntityRepoMeta for AnalyticsEvent {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("session_id".to_string(), "uuid".to_string());
        m.insert("event_type".to_string(), "analytics_event_type".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for AnalyticsEvent entity
///
/// Provides a fluent API for constructing AnalyticsEvent instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct AnalyticsEventBuilder {
    event_type: Option<AnalyticsEventType>,
    user_id: Option<Uuid>,
    session_id: Option<Uuid>,
    resource_type: Option<String>,
    resource_id: Option<String>,
    properties: Option<serde_json::Value>,
    timestamp: Option<DateTime<Utc>>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    geographic_data: Option<serde_json::Value>,
    processed: Option<bool>,
}

impl AnalyticsEventBuilder {
    /// Set the event_type field (required)
    pub fn event_type(mut self, value: AnalyticsEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    /// Set the user_id field (optional)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
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

    /// Set the properties field (optional)
    pub fn properties(mut self, value: serde_json::Value) -> Self {
        self.properties = Some(value);
        self
    }

    /// Set the timestamp field (default: `Utc::now()`)
    pub fn timestamp(mut self, value: DateTime<Utc>) -> Self {
        self.timestamp = Some(value);
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

    /// Set the geographic_data field (optional)
    pub fn geographic_data(mut self, value: serde_json::Value) -> Self {
        self.geographic_data = Some(value);
        self
    }

    /// Set the processed field (default: `false`)
    pub fn processed(mut self, value: bool) -> Self {
        self.processed = Some(value);
        self
    }

    /// Build the AnalyticsEvent entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<AnalyticsEvent, String> {
        let event_type = self.event_type.ok_or_else(|| "event_type is required".to_string())?;

        Ok(AnalyticsEvent {
            id: Uuid::new_v4(),
            event_type,
            user_id: self.user_id,
            session_id: self.session_id,
            resource_type: self.resource_type,
            resource_id: self.resource_id,
            properties: self.properties,
            timestamp: self.timestamp.unwrap_or(Utc::now()),
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            geographic_data: self.geographic_data,
            processed: self.processed.unwrap_or(false),
            metadata: AuditMetadata::default(),
        })
    }
}
