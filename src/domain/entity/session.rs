use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::DeviceType;
use super::AuditMetadata;

use crate::domain::state_machine::{SessionStateMachine, SessionState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for Session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for SessionId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for SessionId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<SessionId> for Uuid {
    fn from(id: SessionId) -> Self { id.0 }
}

impl AsRef<Uuid> for SessionId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for SessionId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_at: Option<DateTime<Utc>>,
    pub remember_me: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    pub device_type: DeviceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,
    pub(crate) is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Session {
    /// Create a builder for Session
    pub fn builder() -> SessionBuilder {
        SessionBuilder::default()
    }

    /// Create a new Session with required fields
    pub fn new(user_id: Uuid, token_hash: String, expires_at: DateTime<Utc>, remember_me: bool, device_type: DeviceType, is_active: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            token_hash,
            expires_at,
            extended_at: None,
            remember_me,
            last_activity: None,
            ip_address: None,
            user_agent: None,
            device_type,
            device_fingerprint: None,
            is_active,
            revoked_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> SessionId {
        SessionId(self.id)
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

    /// Set the extended_at field (chainable)
    pub fn with_extended_at(mut self, value: DateTime<Utc>) -> Self {
        self.extended_at = Some(value);
        self
    }

    /// Set the last_activity field (chainable)
    pub fn with_last_activity(mut self, value: DateTime<Utc>) -> Self {
        self.last_activity = Some(value);
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

    /// Set the device_fingerprint field (chainable)
    pub fn with_device_fingerprint(mut self, value: String) -> Self {
        self.device_fingerprint = Some(value);
        self
    }

    /// Set the revoked_at field (chainable)
    pub fn with_revoked_at(mut self, value: DateTime<Utc>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the is_active state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.is_active` directly.
    pub fn transition_to(&mut self, new_state: SessionState) -> Result<(), StateMachineError> {
        let current = self.is_active.to_string().parse::<SessionState>()?;
        let mut sm = SessionStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.is_active = new_state.to_string().parse::<bool>()
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
                "token_hash" => {
                    if let Ok(v) = serde_json::from_value(value) { self.token_hash = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "extended_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.extended_at = v; }
                }
                "remember_me" => {
                    if let Ok(v) = serde_json::from_value(value) { self.remember_me = v; }
                }
                "last_activity" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_activity = v; }
                }
                "ip_address" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ip_address = v; }
                }
                "user_agent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_agent = v; }
                }
                "device_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_type = v; }
                }
                "device_fingerprint" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_fingerprint = v; }
                }
                "revoked_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.revoked_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Session {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Session"
    }
}

impl backbone_core::PersistentEntity for Session {
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

impl backbone_orm::EntityRepoMeta for Session {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("device_type".to_string(), "device_type".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["token_hash"]
    }
}

/// Builder for Session entity
///
/// Provides a fluent API for constructing Session instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct SessionBuilder {
    user_id: Option<Uuid>,
    token_hash: Option<String>,
    expires_at: Option<DateTime<Utc>>,
    extended_at: Option<DateTime<Utc>>,
    remember_me: Option<bool>,
    last_activity: Option<DateTime<Utc>>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    device_type: Option<DeviceType>,
    device_fingerprint: Option<String>,
    is_active: Option<bool>,
    revoked_at: Option<DateTime<Utc>>,
}

impl SessionBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the token_hash field (required)
    pub fn token_hash(mut self, value: String) -> Self {
        self.token_hash = Some(value);
        self
    }

    /// Set the expires_at field (required)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the extended_at field (optional)
    pub fn extended_at(mut self, value: DateTime<Utc>) -> Self {
        self.extended_at = Some(value);
        self
    }

    /// Set the remember_me field (default: `false`)
    pub fn remember_me(mut self, value: bool) -> Self {
        self.remember_me = Some(value);
        self
    }

    /// Set the last_activity field (optional)
    pub fn last_activity(mut self, value: DateTime<Utc>) -> Self {
        self.last_activity = Some(value);
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

    /// Set the device_type field (default: `DeviceType::default()`)
    pub fn device_type(mut self, value: DeviceType) -> Self {
        self.device_type = Some(value);
        self
    }

    /// Set the device_fingerprint field (optional)
    pub fn device_fingerprint(mut self, value: String) -> Self {
        self.device_fingerprint = Some(value);
        self
    }

    /// Set the is_active field (default: `true`)
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Set the revoked_at field (optional)
    pub fn revoked_at(mut self, value: DateTime<Utc>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    /// Build the Session entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Session, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let token_hash = self.token_hash.ok_or_else(|| "token_hash is required".to_string())?;
        let expires_at = self.expires_at.ok_or_else(|| "expires_at is required".to_string())?;

        Ok(Session {
            id: Uuid::new_v4(),
            user_id,
            token_hash,
            expires_at,
            extended_at: self.extended_at,
            remember_me: self.remember_me.unwrap_or(false),
            last_activity: self.last_activity,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            device_type: self.device_type.unwrap_or(DeviceType::default()),
            device_fingerprint: self.device_fingerprint,
            is_active: self.is_active.unwrap_or(true),
            revoked_at: self.revoked_at,
            metadata: AuditMetadata::default(),
        })
    }
}
