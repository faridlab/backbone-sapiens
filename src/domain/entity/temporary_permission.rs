use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::TemporaryPermissionStatus;
use super::AuditMetadata;

use crate::domain::state_machine::{TemporaryPermissionStateMachine, TemporaryPermissionState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for TemporaryPermission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TemporaryPermissionId(pub Uuid);

impl TemporaryPermissionId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for TemporaryPermissionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for TemporaryPermissionId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for TemporaryPermissionId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<TemporaryPermissionId> for Uuid {
    fn from(id: TemporaryPermissionId) -> Self { id.0 }
}

impl AsRef<Uuid> for TemporaryPermissionId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for TemporaryPermissionId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TemporaryPermission {
    pub id: Uuid,
    pub user_id: Uuid,
    pub permission_id: Uuid,
    pub granted_by: Uuid,
    pub granted_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_by: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub notified_before_expiry: bool,
    pub(crate) status: TemporaryPermissionStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl TemporaryPermission {
    /// Create a builder for TemporaryPermission
    pub fn builder() -> TemporaryPermissionBuilder {
        TemporaryPermissionBuilder::default()
    }

    /// Create a new TemporaryPermission with required fields
    pub fn new(user_id: Uuid, permission_id: Uuid, granted_by: Uuid, granted_at: DateTime<Utc>, expires_at: DateTime<Utc>, notified_before_expiry: bool, status: TemporaryPermissionStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            permission_id,
            granted_by,
            granted_at,
            expires_at,
            revoked_at: None,
            revoked_by: None,
            reason: None,
            notified_before_expiry,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> TemporaryPermissionId {
        TemporaryPermissionId(self.id)
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
    pub fn status(&self) -> &TemporaryPermissionStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the revoked_at field (chainable)
    pub fn with_revoked_at(mut self, value: DateTime<Utc>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    /// Set the revoked_by field (chainable)
    pub fn with_revoked_by(mut self, value: Uuid) -> Self {
        self.revoked_by = Some(value);
        self
    }

    /// Set the reason field (chainable)
    pub fn with_reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: TemporaryPermissionState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<TemporaryPermissionState>()?;
        let mut sm = TemporaryPermissionStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<TemporaryPermissionStatus>()
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
                "permission_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.permission_id = v; }
                }
                "granted_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.granted_by = v; }
                }
                "granted_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.granted_at = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "revoked_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.revoked_at = v; }
                }
                "revoked_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.revoked_by = v; }
                }
                "reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.reason = v; }
                }
                "notified_before_expiry" => {
                    if let Ok(v) = serde_json::from_value(value) { self.notified_before_expiry = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for TemporaryPermission {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "TemporaryPermission"
    }
}

impl backbone_core::PersistentEntity for TemporaryPermission {
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

impl backbone_orm::EntityRepoMeta for TemporaryPermission {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("permission_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "temporary_permission_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for TemporaryPermission entity
///
/// Provides a fluent API for constructing TemporaryPermission instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct TemporaryPermissionBuilder {
    user_id: Option<Uuid>,
    permission_id: Option<Uuid>,
    granted_by: Option<Uuid>,
    granted_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    revoked_at: Option<DateTime<Utc>>,
    revoked_by: Option<Uuid>,
    reason: Option<String>,
    notified_before_expiry: Option<bool>,
    status: Option<TemporaryPermissionStatus>,
}

impl TemporaryPermissionBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the permission_id field (required)
    pub fn permission_id(mut self, value: Uuid) -> Self {
        self.permission_id = Some(value);
        self
    }

    /// Set the granted_by field (required)
    pub fn granted_by(mut self, value: Uuid) -> Self {
        self.granted_by = Some(value);
        self
    }

    /// Set the granted_at field (default: `Utc::now()`)
    pub fn granted_at(mut self, value: DateTime<Utc>) -> Self {
        self.granted_at = Some(value);
        self
    }

    /// Set the expires_at field (required)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the revoked_at field (optional)
    pub fn revoked_at(mut self, value: DateTime<Utc>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    /// Set the revoked_by field (optional)
    pub fn revoked_by(mut self, value: Uuid) -> Self {
        self.revoked_by = Some(value);
        self
    }

    /// Set the reason field (optional)
    pub fn reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }

    /// Set the notified_before_expiry field (default: `false`)
    pub fn notified_before_expiry(mut self, value: bool) -> Self {
        self.notified_before_expiry = Some(value);
        self
    }

    /// Set the status field (default: `TemporaryPermissionStatus::default()`)
    pub fn status(mut self, value: TemporaryPermissionStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the TemporaryPermission entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<TemporaryPermission, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let permission_id = self.permission_id.ok_or_else(|| "permission_id is required".to_string())?;
        let granted_by = self.granted_by.ok_or_else(|| "granted_by is required".to_string())?;
        let expires_at = self.expires_at.ok_or_else(|| "expires_at is required".to_string())?;

        Ok(TemporaryPermission {
            id: Uuid::new_v4(),
            user_id,
            permission_id,
            granted_by,
            granted_at: self.granted_at.unwrap_or(Utc::now()),
            expires_at,
            revoked_at: self.revoked_at,
            revoked_by: self.revoked_by,
            reason: self.reason,
            notified_before_expiry: self.notified_before_expiry.unwrap_or(false),
            status: self.status.unwrap_or(TemporaryPermissionStatus::default()),
            metadata: AuditMetadata::default(),
        })
    }
}
