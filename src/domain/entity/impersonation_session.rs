use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::ImpersonationSessionStatus;
use super::AuditMetadata;

use crate::domain::state_machine::{ImpersonationSessionStateMachine, ImpersonationSessionState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for ImpersonationSession
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ImpersonationSessionId(pub Uuid);

impl ImpersonationSessionId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for ImpersonationSessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ImpersonationSessionId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for ImpersonationSessionId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<ImpersonationSessionId> for Uuid {
    fn from(id: ImpersonationSessionId) -> Self { id.0 }
}

impl AsRef<Uuid> for ImpersonationSessionId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for ImpersonationSessionId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ImpersonationSession {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub target_user_id: Uuid,
    pub session_id: Uuid,
    pub started_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<DateTime<Utc>>,
    pub max_duration_minutes: i32,
    pub reason: String,
    pub actions_performed: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated_by: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_reason: Option<String>,
    pub(crate) status: ImpersonationSessionStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl ImpersonationSession {
    /// Create a builder for ImpersonationSession
    pub fn builder() -> ImpersonationSessionBuilder {
        ImpersonationSessionBuilder::default()
    }

    /// Create a new ImpersonationSession with required fields
    pub fn new(admin_id: Uuid, target_user_id: Uuid, session_id: Uuid, started_at: DateTime<Utc>, max_duration_minutes: i32, reason: String, actions_performed: i32, status: ImpersonationSessionStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            admin_id,
            target_user_id,
            session_id,
            started_at,
            ended_at: None,
            max_duration_minutes,
            reason,
            actions_performed,
            terminated_by: None,
            termination_reason: None,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> ImpersonationSessionId {
        ImpersonationSessionId(self.id)
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
    pub fn status(&self) -> &ImpersonationSessionStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the ended_at field (chainable)
    pub fn with_ended_at(mut self, value: DateTime<Utc>) -> Self {
        self.ended_at = Some(value);
        self
    }

    /// Set the terminated_by field (chainable)
    pub fn with_terminated_by(mut self, value: Uuid) -> Self {
        self.terminated_by = Some(value);
        self
    }

    /// Set the termination_reason field (chainable)
    pub fn with_termination_reason(mut self, value: String) -> Self {
        self.termination_reason = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: ImpersonationSessionState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<ImpersonationSessionState>()?;
        let mut sm = ImpersonationSessionStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<ImpersonationSessionStatus>()
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
                "admin_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.admin_id = v; }
                }
                "target_user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.target_user_id = v; }
                }
                "session_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.session_id = v; }
                }
                "started_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.started_at = v; }
                }
                "ended_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ended_at = v; }
                }
                "max_duration_minutes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_duration_minutes = v; }
                }
                "reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.reason = v; }
                }
                "actions_performed" => {
                    if let Ok(v) = serde_json::from_value(value) { self.actions_performed = v; }
                }
                "terminated_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.terminated_by = v; }
                }
                "termination_reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.termination_reason = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for ImpersonationSession {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "ImpersonationSession"
    }
}

impl backbone_core::PersistentEntity for ImpersonationSession {
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

impl backbone_orm::EntityRepoMeta for ImpersonationSession {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("admin_id".to_string(), "uuid".to_string());
        m.insert("target_user_id".to_string(), "uuid".to_string());
        m.insert("session_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "impersonation_session_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["reason"]
    }
}

/// Builder for ImpersonationSession entity
///
/// Provides a fluent API for constructing ImpersonationSession instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct ImpersonationSessionBuilder {
    admin_id: Option<Uuid>,
    target_user_id: Option<Uuid>,
    session_id: Option<Uuid>,
    started_at: Option<DateTime<Utc>>,
    ended_at: Option<DateTime<Utc>>,
    max_duration_minutes: Option<i32>,
    reason: Option<String>,
    actions_performed: Option<i32>,
    terminated_by: Option<Uuid>,
    termination_reason: Option<String>,
    status: Option<ImpersonationSessionStatus>,
}

impl ImpersonationSessionBuilder {
    /// Set the admin_id field (required)
    pub fn admin_id(mut self, value: Uuid) -> Self {
        self.admin_id = Some(value);
        self
    }

    /// Set the target_user_id field (required)
    pub fn target_user_id(mut self, value: Uuid) -> Self {
        self.target_user_id = Some(value);
        self
    }

    /// Set the session_id field (required)
    pub fn session_id(mut self, value: Uuid) -> Self {
        self.session_id = Some(value);
        self
    }

    /// Set the started_at field (default: `Utc::now()`)
    pub fn started_at(mut self, value: DateTime<Utc>) -> Self {
        self.started_at = Some(value);
        self
    }

    /// Set the ended_at field (optional)
    pub fn ended_at(mut self, value: DateTime<Utc>) -> Self {
        self.ended_at = Some(value);
        self
    }

    /// Set the max_duration_minutes field (default: `60`)
    pub fn max_duration_minutes(mut self, value: i32) -> Self {
        self.max_duration_minutes = Some(value);
        self
    }

    /// Set the reason field (required)
    pub fn reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }

    /// Set the actions_performed field (default: `0`)
    pub fn actions_performed(mut self, value: i32) -> Self {
        self.actions_performed = Some(value);
        self
    }

    /// Set the terminated_by field (optional)
    pub fn terminated_by(mut self, value: Uuid) -> Self {
        self.terminated_by = Some(value);
        self
    }

    /// Set the termination_reason field (optional)
    pub fn termination_reason(mut self, value: String) -> Self {
        self.termination_reason = Some(value);
        self
    }

    /// Set the status field (default: `ImpersonationSessionStatus::default()`)
    pub fn status(mut self, value: ImpersonationSessionStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the ImpersonationSession entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<ImpersonationSession, String> {
        let admin_id = self.admin_id.ok_or_else(|| "admin_id is required".to_string())?;
        let target_user_id = self.target_user_id.ok_or_else(|| "target_user_id is required".to_string())?;
        let session_id = self.session_id.ok_or_else(|| "session_id is required".to_string())?;
        let reason = self.reason.ok_or_else(|| "reason is required".to_string())?;

        Ok(ImpersonationSession {
            id: Uuid::new_v4(),
            admin_id,
            target_user_id,
            session_id,
            started_at: self.started_at.unwrap_or(Utc::now()),
            ended_at: self.ended_at,
            max_duration_minutes: self.max_duration_minutes.unwrap_or(60),
            reason,
            actions_performed: self.actions_performed.unwrap_or(0),
            terminated_by: self.terminated_by,
            termination_reason: self.termination_reason,
            status: self.status.unwrap_or(ImpersonationSessionStatus::default()),
            metadata: AuditMetadata::default(),
        })
    }
}
