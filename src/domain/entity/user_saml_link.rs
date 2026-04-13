use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::UserSAMLLinkStatus;
use super::AuditMetadata;

use crate::domain::state_machine::{UserSAMLLinkStateMachine, UserSAMLLinkState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for UserSAMLLink
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserSAMLLinkId(pub Uuid);

impl UserSAMLLinkId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for UserSAMLLinkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserSAMLLinkId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for UserSAMLLinkId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<UserSAMLLinkId> for Uuid {
    fn from(id: UserSAMLLinkId) -> Self { id.0 }
}

impl AsRef<Uuid> for UserSAMLLinkId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for UserSAMLLinkId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSAMLLink {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider_id: Uuid,
    pub name_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_index: Option<String>,
    pub first_login_at: DateTime<Utc>,
    pub last_login_at: DateTime<Utc>,
    pub attributes: serde_json::Value,
    pub is_primary: bool,
    pub(crate) status: UserSAMLLinkStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl UserSAMLLink {
    /// Create a builder for UserSAMLLink
    pub fn builder() -> UserSAMLLinkBuilder {
        UserSAMLLinkBuilder::default()
    }

    /// Create a new UserSAMLLink with required fields
    pub fn new(user_id: Uuid, provider_id: Uuid, name_id: String, first_login_at: DateTime<Utc>, last_login_at: DateTime<Utc>, attributes: serde_json::Value, is_primary: bool, status: UserSAMLLinkStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            provider_id,
            name_id,
            session_index: None,
            first_login_at,
            last_login_at,
            attributes,
            is_primary,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> UserSAMLLinkId {
        UserSAMLLinkId(self.id)
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
    pub fn status(&self) -> &UserSAMLLinkStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the session_index field (chainable)
    pub fn with_session_index(mut self, value: String) -> Self {
        self.session_index = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: UserSAMLLinkState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<UserSAMLLinkState>()?;
        let mut sm = UserSAMLLinkStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<UserSAMLLinkStatus>()
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
                "provider_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.provider_id = v; }
                }
                "name_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.name_id = v; }
                }
                "session_index" => {
                    if let Ok(v) = serde_json::from_value(value) { self.session_index = v; }
                }
                "first_login_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.first_login_at = v; }
                }
                "last_login_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_login_at = v; }
                }
                "attributes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.attributes = v; }
                }
                "is_primary" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_primary = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for UserSAMLLink {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "UserSAMLLink"
    }
}

impl backbone_core::PersistentEntity for UserSAMLLink {
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

impl backbone_orm::EntityRepoMeta for UserSAMLLink {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("provider_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "user_saml_link_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["name_id"]
    }
}

/// Builder for UserSAMLLink entity
///
/// Provides a fluent API for constructing UserSAMLLink instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct UserSAMLLinkBuilder {
    user_id: Option<Uuid>,
    provider_id: Option<Uuid>,
    name_id: Option<String>,
    session_index: Option<String>,
    first_login_at: Option<DateTime<Utc>>,
    last_login_at: Option<DateTime<Utc>>,
    attributes: Option<serde_json::Value>,
    is_primary: Option<bool>,
    status: Option<UserSAMLLinkStatus>,
}

impl UserSAMLLinkBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the provider_id field (required)
    pub fn provider_id(mut self, value: Uuid) -> Self {
        self.provider_id = Some(value);
        self
    }

    /// Set the name_id field (required)
    pub fn name_id(mut self, value: String) -> Self {
        self.name_id = Some(value);
        self
    }

    /// Set the session_index field (optional)
    pub fn session_index(mut self, value: String) -> Self {
        self.session_index = Some(value);
        self
    }

    /// Set the first_login_at field (default: `Utc::now()`)
    pub fn first_login_at(mut self, value: DateTime<Utc>) -> Self {
        self.first_login_at = Some(value);
        self
    }

    /// Set the last_login_at field (default: `Utc::now()`)
    pub fn last_login_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_login_at = Some(value);
        self
    }

    /// Set the attributes field (default: `Default::default()`)
    pub fn attributes(mut self, value: serde_json::Value) -> Self {
        self.attributes = Some(value);
        self
    }

    /// Set the is_primary field (default: `false`)
    pub fn is_primary(mut self, value: bool) -> Self {
        self.is_primary = Some(value);
        self
    }

    /// Set the status field (default: `UserSAMLLinkStatus::default()`)
    pub fn status(mut self, value: UserSAMLLinkStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the UserSAMLLink entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<UserSAMLLink, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let provider_id = self.provider_id.ok_or_else(|| "provider_id is required".to_string())?;
        let name_id = self.name_id.ok_or_else(|| "name_id is required".to_string())?;

        Ok(UserSAMLLink {
            id: Uuid::new_v4(),
            user_id,
            provider_id,
            name_id,
            session_index: self.session_index,
            first_login_at: self.first_login_at.unwrap_or(Utc::now()),
            last_login_at: self.last_login_at.unwrap_or(Utc::now()),
            attributes: self.attributes.unwrap_or(Default::default()),
            is_primary: self.is_primary.unwrap_or(false),
            status: self.status.unwrap_or(UserSAMLLinkStatus::default()),
            metadata: AuditMetadata::default(),
        })
    }
}
