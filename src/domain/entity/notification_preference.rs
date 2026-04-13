use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::DigestFrequency;
use super::AuditMetadata;

use crate::domain::state_machine::{NotificationPreferenceStateMachine, NotificationPreferenceState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for NotificationPreference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NotificationPreferenceId(pub Uuid);

impl NotificationPreferenceId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for NotificationPreferenceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for NotificationPreferenceId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for NotificationPreferenceId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<NotificationPreferenceId> for Uuid {
    fn from(id: NotificationPreferenceId) -> Self { id.0 }
}

impl AsRef<Uuid> for NotificationPreferenceId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for NotificationPreferenceId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NotificationPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: String,
    pub channel_enabled: bool,
    pub channels: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_hours_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_hours_end: Option<String>,
    pub quiet_timezone: String,
    pub digest_enabled: bool,
    pub digest_frequency: DigestFrequency,
    pub(crate) enabled: bool,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl NotificationPreference {
    /// Create a builder for NotificationPreference
    pub fn builder() -> NotificationPreferenceBuilder {
        NotificationPreferenceBuilder::default()
    }

    /// Create a new NotificationPreference with required fields
    pub fn new(user_id: Uuid, notification_type: String, channel_enabled: bool, channels: serde_json::Value, quiet_timezone: String, digest_enabled: bool, digest_frequency: DigestFrequency, enabled: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            notification_type,
            channel_enabled,
            channels,
            quiet_hours_start: None,
            quiet_hours_end: None,
            quiet_timezone,
            digest_enabled,
            digest_frequency,
            enabled,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> NotificationPreferenceId {
        NotificationPreferenceId(self.id)
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

    /// Set the quiet_hours_start field (chainable)
    pub fn with_quiet_hours_start(mut self, value: String) -> Self {
        self.quiet_hours_start = Some(value);
        self
    }

    /// Set the quiet_hours_end field (chainable)
    pub fn with_quiet_hours_end(mut self, value: String) -> Self {
        self.quiet_hours_end = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the enabled state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.enabled` directly.
    pub fn transition_to(&mut self, new_state: NotificationPreferenceState) -> Result<(), StateMachineError> {
        let current = self.enabled.to_string().parse::<NotificationPreferenceState>()?;
        let mut sm = NotificationPreferenceStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.enabled = new_state.to_string().parse::<bool>()
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
                "notification_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.notification_type = v; }
                }
                "channel_enabled" => {
                    if let Ok(v) = serde_json::from_value(value) { self.channel_enabled = v; }
                }
                "channels" => {
                    if let Ok(v) = serde_json::from_value(value) { self.channels = v; }
                }
                "quiet_hours_start" => {
                    if let Ok(v) = serde_json::from_value(value) { self.quiet_hours_start = v; }
                }
                "quiet_hours_end" => {
                    if let Ok(v) = serde_json::from_value(value) { self.quiet_hours_end = v; }
                }
                "quiet_timezone" => {
                    if let Ok(v) = serde_json::from_value(value) { self.quiet_timezone = v; }
                }
                "digest_enabled" => {
                    if let Ok(v) = serde_json::from_value(value) { self.digest_enabled = v; }
                }
                "digest_frequency" => {
                    if let Ok(v) = serde_json::from_value(value) { self.digest_frequency = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for NotificationPreference {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "NotificationPreference"
    }
}

impl backbone_core::PersistentEntity for NotificationPreference {
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

impl backbone_orm::EntityRepoMeta for NotificationPreference {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("digest_frequency".to_string(), "digest_frequency".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["notification_type", "quiet_timezone"]
    }
}

/// Builder for NotificationPreference entity
///
/// Provides a fluent API for constructing NotificationPreference instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct NotificationPreferenceBuilder {
    user_id: Option<Uuid>,
    notification_type: Option<String>,
    channel_enabled: Option<bool>,
    channels: Option<serde_json::Value>,
    quiet_hours_start: Option<String>,
    quiet_hours_end: Option<String>,
    quiet_timezone: Option<String>,
    digest_enabled: Option<bool>,
    digest_frequency: Option<DigestFrequency>,
    enabled: Option<bool>,
}

impl NotificationPreferenceBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the notification_type field (required)
    pub fn notification_type(mut self, value: String) -> Self {
        self.notification_type = Some(value);
        self
    }

    /// Set the channel_enabled field (default: `true`)
    pub fn channel_enabled(mut self, value: bool) -> Self {
        self.channel_enabled = Some(value);
        self
    }

    /// Set the channels field (default: `Default::default()`)
    pub fn channels(mut self, value: serde_json::Value) -> Self {
        self.channels = Some(value);
        self
    }

    /// Set the quiet_hours_start field (optional)
    pub fn quiet_hours_start(mut self, value: String) -> Self {
        self.quiet_hours_start = Some(value);
        self
    }

    /// Set the quiet_hours_end field (optional)
    pub fn quiet_hours_end(mut self, value: String) -> Self {
        self.quiet_hours_end = Some(value);
        self
    }

    /// Set the quiet_timezone field (default: `Default::default()`)
    pub fn quiet_timezone(mut self, value: String) -> Self {
        self.quiet_timezone = Some(value);
        self
    }

    /// Set the digest_enabled field (default: `false`)
    pub fn digest_enabled(mut self, value: bool) -> Self {
        self.digest_enabled = Some(value);
        self
    }

    /// Set the digest_frequency field (default: `DigestFrequency::default()`)
    pub fn digest_frequency(mut self, value: DigestFrequency) -> Self {
        self.digest_frequency = Some(value);
        self
    }

    /// Set the enabled field (default: `true`)
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Build the NotificationPreference entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<NotificationPreference, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let notification_type = self.notification_type.ok_or_else(|| "notification_type is required".to_string())?;

        Ok(NotificationPreference {
            id: Uuid::new_v4(),
            user_id,
            notification_type,
            channel_enabled: self.channel_enabled.unwrap_or(true),
            channels: self.channels.unwrap_or(Default::default()),
            quiet_hours_start: self.quiet_hours_start,
            quiet_hours_end: self.quiet_hours_end,
            quiet_timezone: self.quiet_timezone.unwrap_or(Default::default()),
            digest_enabled: self.digest_enabled.unwrap_or(false),
            digest_frequency: self.digest_frequency.unwrap_or(DigestFrequency::default()),
            enabled: self.enabled.unwrap_or(true),
            metadata: AuditMetadata::default(),
        })
    }
}
