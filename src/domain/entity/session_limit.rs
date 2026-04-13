use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

use crate::domain::state_machine::{SessionLimitStateMachine, SessionLimitState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for SessionLimit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionLimitId(pub Uuid);

impl SessionLimitId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for SessionLimitId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for SessionLimitId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for SessionLimitId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<SessionLimitId> for Uuid {
    fn from(id: SessionLimitId) -> Self { id.0 }
}

impl AsRef<Uuid> for SessionLimitId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for SessionLimitId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SessionLimit {
    pub id: Uuid,
    pub user_id: Uuid,
    pub max_sessions: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sessions_per_device: Option<i32>,
    pub(crate) enforce_limit: bool,
    pub current_session_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_session_revoke_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl SessionLimit {
    /// Create a builder for SessionLimit
    pub fn builder() -> SessionLimitBuilder {
        SessionLimitBuilder::default()
    }

    /// Create a new SessionLimit with required fields
    pub fn new(user_id: Uuid, max_sessions: i32, enforce_limit: bool, current_session_count: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            max_sessions,
            max_sessions_per_device: None,
            enforce_limit,
            current_session_count,
            last_session_revoke_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> SessionLimitId {
        SessionLimitId(self.id)
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

    /// Set the max_sessions_per_device field (chainable)
    pub fn with_max_sessions_per_device(mut self, value: i32) -> Self {
        self.max_sessions_per_device = Some(value);
        self
    }

    /// Set the last_session_revoke_at field (chainable)
    pub fn with_last_session_revoke_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_session_revoke_at = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the enforce_limit state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.enforce_limit` directly.
    pub fn transition_to(&mut self, new_state: SessionLimitState) -> Result<(), StateMachineError> {
        let current = self.enforce_limit.to_string().parse::<SessionLimitState>()?;
        let mut sm = SessionLimitStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.enforce_limit = new_state.to_string().parse::<bool>()
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
                "max_sessions" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_sessions = v; }
                }
                "max_sessions_per_device" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_sessions_per_device = v; }
                }
                "current_session_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.current_session_count = v; }
                }
                "last_session_revoke_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_session_revoke_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for SessionLimit {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "SessionLimit"
    }
}

impl backbone_core::PersistentEntity for SessionLimit {
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

impl backbone_orm::EntityRepoMeta for SessionLimit {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for SessionLimit entity
///
/// Provides a fluent API for constructing SessionLimit instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct SessionLimitBuilder {
    user_id: Option<Uuid>,
    max_sessions: Option<i32>,
    max_sessions_per_device: Option<i32>,
    enforce_limit: Option<bool>,
    current_session_count: Option<i32>,
    last_session_revoke_at: Option<DateTime<Utc>>,
}

impl SessionLimitBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the max_sessions field (default: `5`)
    pub fn max_sessions(mut self, value: i32) -> Self {
        self.max_sessions = Some(value);
        self
    }

    /// Set the max_sessions_per_device field (optional)
    pub fn max_sessions_per_device(mut self, value: i32) -> Self {
        self.max_sessions_per_device = Some(value);
        self
    }

    /// Set the enforce_limit field (default: `true`)
    pub fn enforce_limit(mut self, value: bool) -> Self {
        self.enforce_limit = Some(value);
        self
    }

    /// Set the current_session_count field (default: `0`)
    pub fn current_session_count(mut self, value: i32) -> Self {
        self.current_session_count = Some(value);
        self
    }

    /// Set the last_session_revoke_at field (optional)
    pub fn last_session_revoke_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_session_revoke_at = Some(value);
        self
    }

    /// Build the SessionLimit entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<SessionLimit, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;

        Ok(SessionLimit {
            id: Uuid::new_v4(),
            user_id,
            max_sessions: self.max_sessions.unwrap_or(5),
            max_sessions_per_device: self.max_sessions_per_device,
            enforce_limit: self.enforce_limit.unwrap_or(true),
            current_session_count: self.current_session_count.unwrap_or(0),
            last_session_revoke_at: self.last_session_revoke_at,
            metadata: AuditMetadata::default(),
        })
    }
}
