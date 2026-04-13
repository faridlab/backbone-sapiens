use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::ResourcePermissionStatus;
use super::AuditMetadata;

use crate::domain::state_machine::{ResourcePermissionStateMachine, ResourcePermissionState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for ResourcePermission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourcePermissionId(pub Uuid);

impl ResourcePermissionId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for ResourcePermissionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ResourcePermissionId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for ResourcePermissionId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<ResourcePermissionId> for Uuid {
    fn from(id: ResourcePermissionId) -> Self { id.0 }
}

impl AsRef<Uuid> for ResourcePermissionId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for ResourcePermissionId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResourcePermission {
    pub id: Uuid,
    pub permission_id: Uuid,
    pub resource_type: String,
    pub resource_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granted_to_user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granted_to_role_id: Option<Uuid>,
    pub granted_by: Uuid,
    pub granted_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub(crate) status: ResourcePermissionStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl ResourcePermission {
    /// Create a builder for ResourcePermission
    pub fn builder() -> ResourcePermissionBuilder {
        ResourcePermissionBuilder::default()
    }

    /// Create a new ResourcePermission with required fields
    pub fn new(permission_id: Uuid, resource_type: String, resource_id: String, granted_by: Uuid, granted_at: DateTime<Utc>, status: ResourcePermissionStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            permission_id,
            resource_type,
            resource_id,
            granted_to_user_id: None,
            granted_to_role_id: None,
            granted_by,
            granted_at,
            expires_at: None,
            reason: None,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> ResourcePermissionId {
        ResourcePermissionId(self.id)
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
    pub fn status(&self) -> &ResourcePermissionStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the granted_to_user_id field (chainable)
    pub fn with_granted_to_user_id(mut self, value: Uuid) -> Self {
        self.granted_to_user_id = Some(value);
        self
    }

    /// Set the granted_to_role_id field (chainable)
    pub fn with_granted_to_role_id(mut self, value: Uuid) -> Self {
        self.granted_to_role_id = Some(value);
        self
    }

    /// Set the expires_at field (chainable)
    pub fn with_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
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
    pub fn transition_to(&mut self, new_state: ResourcePermissionState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<ResourcePermissionState>()?;
        let mut sm = ResourcePermissionStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<ResourcePermissionStatus>()
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
                "permission_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.permission_id = v; }
                }
                "resource_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.resource_type = v; }
                }
                "resource_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.resource_id = v; }
                }
                "granted_to_user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.granted_to_user_id = v; }
                }
                "granted_to_role_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.granted_to_role_id = v; }
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
                "reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.reason = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for ResourcePermission {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "ResourcePermission"
    }
}

impl backbone_core::PersistentEntity for ResourcePermission {
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

impl backbone_orm::EntityRepoMeta for ResourcePermission {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("permission_id".to_string(), "uuid".to_string());
        m.insert("granted_to_user_id".to_string(), "uuid".to_string());
        m.insert("granted_to_role_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "resource_permission_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["resource_type", "resource_id"]
    }
}

/// Builder for ResourcePermission entity
///
/// Provides a fluent API for constructing ResourcePermission instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct ResourcePermissionBuilder {
    permission_id: Option<Uuid>,
    resource_type: Option<String>,
    resource_id: Option<String>,
    granted_to_user_id: Option<Uuid>,
    granted_to_role_id: Option<Uuid>,
    granted_by: Option<Uuid>,
    granted_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    reason: Option<String>,
    status: Option<ResourcePermissionStatus>,
}

impl ResourcePermissionBuilder {
    /// Set the permission_id field (required)
    pub fn permission_id(mut self, value: Uuid) -> Self {
        self.permission_id = Some(value);
        self
    }

    /// Set the resource_type field (required)
    pub fn resource_type(mut self, value: String) -> Self {
        self.resource_type = Some(value);
        self
    }

    /// Set the resource_id field (required)
    pub fn resource_id(mut self, value: String) -> Self {
        self.resource_id = Some(value);
        self
    }

    /// Set the granted_to_user_id field (optional)
    pub fn granted_to_user_id(mut self, value: Uuid) -> Self {
        self.granted_to_user_id = Some(value);
        self
    }

    /// Set the granted_to_role_id field (optional)
    pub fn granted_to_role_id(mut self, value: Uuid) -> Self {
        self.granted_to_role_id = Some(value);
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

    /// Set the expires_at field (optional)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the reason field (optional)
    pub fn reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }

    /// Set the status field (default: `ResourcePermissionStatus::default()`)
    pub fn status(mut self, value: ResourcePermissionStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the ResourcePermission entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<ResourcePermission, String> {
        let permission_id = self.permission_id.ok_or_else(|| "permission_id is required".to_string())?;
        let resource_type = self.resource_type.ok_or_else(|| "resource_type is required".to_string())?;
        let resource_id = self.resource_id.ok_or_else(|| "resource_id is required".to_string())?;
        let granted_by = self.granted_by.ok_or_else(|| "granted_by is required".to_string())?;

        Ok(ResourcePermission {
            id: Uuid::new_v4(),
            permission_id,
            resource_type,
            resource_id,
            granted_to_user_id: self.granted_to_user_id,
            granted_to_role_id: self.granted_to_role_id,
            granted_by,
            granted_at: self.granted_at.unwrap_or(Utc::now()),
            expires_at: self.expires_at,
            reason: self.reason,
            status: self.status.unwrap_or(ResourcePermissionStatus::default()),
            metadata: AuditMetadata::default(),
        })
    }
}
