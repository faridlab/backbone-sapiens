use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::UserOAuthLinkStatus;
use super::AuditMetadata;

use crate::domain::state_machine::{UserOAuthLinkStateMachine, UserOAuthLinkState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for UserOAuthLink
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserOAuthLinkId(pub Uuid);

impl UserOAuthLinkId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for UserOAuthLinkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserOAuthLinkId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for UserOAuthLinkId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<UserOAuthLinkId> for Uuid {
    fn from(id: UserOAuthLinkId) -> Self { id.0 }
}

impl AsRef<Uuid> for UserOAuthLinkId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for UserOAuthLinkId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserOAuthLink {
    pub id: Uuid,
    pub user_id: Uuid,
    pub oauth_provider_id: Uuid,
    pub provider_user_id: String,
    pub provider_email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_username: Option<String>,
    pub(crate) is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_expires_at: Option<DateTime<Utc>>,
    pub is_primary: bool,
    pub link_status: UserOAuthLinkStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synced: Option<DateTime<Utc>>,
    pub sync_enabled: bool,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl UserOAuthLink {
    /// Create a builder for UserOAuthLink
    pub fn builder() -> UserOAuthLinkBuilder {
        UserOAuthLinkBuilder::default()
    }

    /// Create a new UserOAuthLink with required fields
    pub fn new(user_id: Uuid, oauth_provider_id: Uuid, provider_user_id: String, provider_email: String, is_active: bool, is_primary: bool, link_status: UserOAuthLinkStatus, sync_enabled: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            oauth_provider_id,
            provider_user_id,
            provider_email,
            provider_username: None,
            is_active,
            access_token: None,
            refresh_token: None,
            token_expires_at: None,
            is_primary,
            link_status,
            last_synced: None,
            sync_enabled,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> UserOAuthLinkId {
        UserOAuthLinkId(self.id)
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

    /// Set the provider_username field (chainable)
    pub fn with_provider_username(mut self, value: String) -> Self {
        self.provider_username = Some(value);
        self
    }

    /// Set the access_token field (chainable)
    pub fn with_access_token(mut self, value: String) -> Self {
        self.access_token = Some(value);
        self
    }

    /// Set the refresh_token field (chainable)
    pub fn with_refresh_token(mut self, value: String) -> Self {
        self.refresh_token = Some(value);
        self
    }

    /// Set the token_expires_at field (chainable)
    pub fn with_token_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.token_expires_at = Some(value);
        self
    }

    /// Set the last_synced field (chainable)
    pub fn with_last_synced(mut self, value: DateTime<Utc>) -> Self {
        self.last_synced = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the is_active state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.is_active` directly.
    pub fn transition_to(&mut self, new_state: UserOAuthLinkState) -> Result<(), StateMachineError> {
        let current = self.is_active.to_string().parse::<UserOAuthLinkState>()?;
        let mut sm = UserOAuthLinkStateMachine::from_state(current);
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
                "oauth_provider_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.oauth_provider_id = v; }
                }
                "provider_user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.provider_user_id = v; }
                }
                "provider_email" => {
                    if let Ok(v) = serde_json::from_value(value) { self.provider_email = v; }
                }
                "provider_username" => {
                    if let Ok(v) = serde_json::from_value(value) { self.provider_username = v; }
                }
                "access_token" => {
                    if let Ok(v) = serde_json::from_value(value) { self.access_token = v; }
                }
                "refresh_token" => {
                    if let Ok(v) = serde_json::from_value(value) { self.refresh_token = v; }
                }
                "token_expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.token_expires_at = v; }
                }
                "is_primary" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_primary = v; }
                }
                "link_status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.link_status = v; }
                }
                "last_synced" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_synced = v; }
                }
                "sync_enabled" => {
                    if let Ok(v) = serde_json::from_value(value) { self.sync_enabled = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for UserOAuthLink {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "UserOAuthLink"
    }
}

impl backbone_core::PersistentEntity for UserOAuthLink {
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

impl backbone_orm::EntityRepoMeta for UserOAuthLink {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("oauth_provider_id".to_string(), "uuid".to_string());
        m.insert("link_status".to_string(), "user_o_auth_link_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["provider_user_id", "provider_email"]
    }
}

/// Builder for UserOAuthLink entity
///
/// Provides a fluent API for constructing UserOAuthLink instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct UserOAuthLinkBuilder {
    user_id: Option<Uuid>,
    oauth_provider_id: Option<Uuid>,
    provider_user_id: Option<String>,
    provider_email: Option<String>,
    provider_username: Option<String>,
    is_active: Option<bool>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    token_expires_at: Option<DateTime<Utc>>,
    is_primary: Option<bool>,
    link_status: Option<UserOAuthLinkStatus>,
    last_synced: Option<DateTime<Utc>>,
    sync_enabled: Option<bool>,
}

impl UserOAuthLinkBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the oauth_provider_id field (required)
    pub fn oauth_provider_id(mut self, value: Uuid) -> Self {
        self.oauth_provider_id = Some(value);
        self
    }

    /// Set the provider_user_id field (required)
    pub fn provider_user_id(mut self, value: String) -> Self {
        self.provider_user_id = Some(value);
        self
    }

    /// Set the provider_email field (required)
    pub fn provider_email(mut self, value: String) -> Self {
        self.provider_email = Some(value);
        self
    }

    /// Set the provider_username field (optional)
    pub fn provider_username(mut self, value: String) -> Self {
        self.provider_username = Some(value);
        self
    }

    /// Set the is_active field (default: `true`)
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Set the access_token field (optional)
    pub fn access_token(mut self, value: String) -> Self {
        self.access_token = Some(value);
        self
    }

    /// Set the refresh_token field (optional)
    pub fn refresh_token(mut self, value: String) -> Self {
        self.refresh_token = Some(value);
        self
    }

    /// Set the token_expires_at field (optional)
    pub fn token_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.token_expires_at = Some(value);
        self
    }

    /// Set the is_primary field (default: `false`)
    pub fn is_primary(mut self, value: bool) -> Self {
        self.is_primary = Some(value);
        self
    }

    /// Set the link_status field (default: `UserOAuthLinkStatus::default()`)
    pub fn link_status(mut self, value: UserOAuthLinkStatus) -> Self {
        self.link_status = Some(value);
        self
    }

    /// Set the last_synced field (optional)
    pub fn last_synced(mut self, value: DateTime<Utc>) -> Self {
        self.last_synced = Some(value);
        self
    }

    /// Set the sync_enabled field (default: `true`)
    pub fn sync_enabled(mut self, value: bool) -> Self {
        self.sync_enabled = Some(value);
        self
    }

    /// Build the UserOAuthLink entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<UserOAuthLink, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let oauth_provider_id = self.oauth_provider_id.ok_or_else(|| "oauth_provider_id is required".to_string())?;
        let provider_user_id = self.provider_user_id.ok_or_else(|| "provider_user_id is required".to_string())?;
        let provider_email = self.provider_email.ok_or_else(|| "provider_email is required".to_string())?;

        Ok(UserOAuthLink {
            id: Uuid::new_v4(),
            user_id,
            oauth_provider_id,
            provider_user_id,
            provider_email,
            provider_username: self.provider_username,
            is_active: self.is_active.unwrap_or(true),
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            token_expires_at: self.token_expires_at,
            is_primary: self.is_primary.unwrap_or(false),
            link_status: self.link_status.unwrap_or(UserOAuthLinkStatus::default()),
            last_synced: self.last_synced,
            sync_enabled: self.sync_enabled.unwrap_or(true),
            metadata: AuditMetadata::default(),
        })
    }
}
