use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::SecurityEventType;
use super::SecurityEventSeverity;
use super::AuditMetadata;

use crate::domain::state_machine::{SecurityEventStateMachine, SecurityEventState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for SecurityEvent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SecurityEventId(pub Uuid);

impl SecurityEventId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for SecurityEventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for SecurityEventId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for SecurityEventId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<SecurityEventId> for Uuid {
    fn from(id: SecurityEventId) -> Self { id.0 }
}

impl AsRef<Uuid> for SecurityEventId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for SecurityEventId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SecurityEvent {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<Uuid>,
    pub event_type: SecurityEventType,
    pub severity: SecurityEventSeverity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    pub(crate) resolved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_by_user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_notes: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl SecurityEvent {
    /// Create a builder for SecurityEvent
    pub fn builder() -> SecurityEventBuilder {
        SecurityEventBuilder::default()
    }

    /// Create a new SecurityEvent with required fields
    pub fn new(event_type: SecurityEventType, severity: SecurityEventSeverity, resolved: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: None,
            session_id: None,
            event_type,
            severity,
            ip_address: None,
            user_agent: None,
            description: None,
            details: None,
            resolved,
            resolved_at: None,
            resolved_by_user_id: None,
            resolution_notes: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> SecurityEventId {
        SecurityEventId(self.id)
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

    /// Set the description field (chainable)
    pub fn with_description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the details field (chainable)
    pub fn with_details(mut self, value: serde_json::Value) -> Self {
        self.details = Some(value);
        self
    }

    /// Set the resolved_at field (chainable)
    pub fn with_resolved_at(mut self, value: DateTime<Utc>) -> Self {
        self.resolved_at = Some(value);
        self
    }

    /// Set the resolved_by_user_id field (chainable)
    pub fn with_resolved_by_user_id(mut self, value: Uuid) -> Self {
        self.resolved_by_user_id = Some(value);
        self
    }

    /// Set the resolution_notes field (chainable)
    pub fn with_resolution_notes(mut self, value: String) -> Self {
        self.resolution_notes = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the resolved state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.resolved` directly.
    pub fn transition_to(&mut self, new_state: SecurityEventState) -> Result<(), StateMachineError> {
        let current = self.resolved.to_string().parse::<SecurityEventState>()?;
        let mut sm = SecurityEventStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.resolved = new_state.to_string().parse::<bool>()
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
                "session_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.session_id = v; }
                }
                "event_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.event_type = v; }
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
                "description" => {
                    if let Ok(v) = serde_json::from_value(value) { self.description = v; }
                }
                "details" => {
                    if let Ok(v) = serde_json::from_value(value) { self.details = v; }
                }
                "resolved_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.resolved_at = v; }
                }
                "resolved_by_user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.resolved_by_user_id = v; }
                }
                "resolution_notes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.resolution_notes = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for SecurityEvent {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "SecurityEvent"
    }
}

impl backbone_core::PersistentEntity for SecurityEvent {
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

impl backbone_orm::EntityRepoMeta for SecurityEvent {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("session_id".to_string(), "uuid".to_string());
        m.insert("resolved_by_user_id".to_string(), "uuid".to_string());
        m.insert("event_type".to_string(), "security_event_type".to_string());
        m.insert("severity".to_string(), "security_event_severity".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for SecurityEvent entity
///
/// Provides a fluent API for constructing SecurityEvent instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct SecurityEventBuilder {
    user_id: Option<Uuid>,
    session_id: Option<Uuid>,
    event_type: Option<SecurityEventType>,
    severity: Option<SecurityEventSeverity>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    description: Option<String>,
    details: Option<serde_json::Value>,
    resolved: Option<bool>,
    resolved_at: Option<DateTime<Utc>>,
    resolved_by_user_id: Option<Uuid>,
    resolution_notes: Option<String>,
}

impl SecurityEventBuilder {
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

    /// Set the event_type field (required)
    pub fn event_type(mut self, value: SecurityEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    /// Set the severity field (required)
    pub fn severity(mut self, value: SecurityEventSeverity) -> Self {
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

    /// Set the description field (optional)
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the details field (optional)
    pub fn details(mut self, value: serde_json::Value) -> Self {
        self.details = Some(value);
        self
    }

    /// Set the resolved field (default: `false`)
    pub fn resolved(mut self, value: bool) -> Self {
        self.resolved = Some(value);
        self
    }

    /// Set the resolved_at field (optional)
    pub fn resolved_at(mut self, value: DateTime<Utc>) -> Self {
        self.resolved_at = Some(value);
        self
    }

    /// Set the resolved_by_user_id field (optional)
    pub fn resolved_by_user_id(mut self, value: Uuid) -> Self {
        self.resolved_by_user_id = Some(value);
        self
    }

    /// Set the resolution_notes field (optional)
    pub fn resolution_notes(mut self, value: String) -> Self {
        self.resolution_notes = Some(value);
        self
    }

    /// Build the SecurityEvent entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<SecurityEvent, String> {
        let event_type = self.event_type.ok_or_else(|| "event_type is required".to_string())?;
        let severity = self.severity.ok_or_else(|| "severity is required".to_string())?;

        Ok(SecurityEvent {
            id: Uuid::new_v4(),
            user_id: self.user_id,
            session_id: self.session_id,
            event_type,
            severity,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            description: self.description,
            details: self.details,
            resolved: self.resolved.unwrap_or(false),
            resolved_at: self.resolved_at,
            resolved_by_user_id: self.resolved_by_user_id,
            resolution_notes: self.resolution_notes,
            metadata: AuditMetadata::default(),
        })
    }
}
