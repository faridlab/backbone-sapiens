use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::OAuthProviderType;
use super::AuditMetadata;

use crate::domain::state_machine::{OAuthProviderStateMachine, OAuthProviderState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for OAuthProvider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OAuthProviderId(pub Uuid);

impl OAuthProviderId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for OAuthProviderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for OAuthProviderId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for OAuthProviderId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<OAuthProviderId> for Uuid {
    fn from(id: OAuthProviderId) -> Self { id.0 }
}

impl AsRef<Uuid> for OAuthProviderId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for OAuthProviderId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OAuthProvider {
    pub id: Uuid,
    pub provider_name: OAuthProviderType,
    pub display_name: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub authorization_url: String,
    pub token_url: String,
    pub user_info_url: String,
    pub(crate) is_active: bool,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl OAuthProvider {
    /// Create a builder for OAuthProvider
    pub fn builder() -> OAuthProviderBuilder {
        OAuthProviderBuilder::default()
    }

    /// Create a new OAuthProvider with required fields
    pub fn new(provider_name: OAuthProviderType, display_name: String, client_id: String, client_secret: String, redirect_uri: String, scopes: Vec<String>, authorization_url: String, token_url: String, user_info_url: String, is_active: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            provider_name,
            display_name,
            client_id,
            client_secret,
            redirect_uri,
            scopes,
            authorization_url,
            token_url,
            user_info_url,
            is_active,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> OAuthProviderId {
        OAuthProviderId(self.id)
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
    // State Machine
    // ==========================================================

    /// Transition to a new state via the is_active state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.is_active` directly.
    pub fn transition_to(&mut self, new_state: OAuthProviderState) -> Result<(), StateMachineError> {
        let current = self.is_active.to_string().parse::<OAuthProviderState>()?;
        let mut sm = OAuthProviderStateMachine::from_state(current);
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
                "provider_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.provider_name = v; }
                }
                "display_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.display_name = v; }
                }
                "client_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.client_id = v; }
                }
                "client_secret" => {
                    if let Ok(v) = serde_json::from_value(value) { self.client_secret = v; }
                }
                "redirect_uri" => {
                    if let Ok(v) = serde_json::from_value(value) { self.redirect_uri = v; }
                }
                "scopes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.scopes = v; }
                }
                "authorization_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.authorization_url = v; }
                }
                "token_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.token_url = v; }
                }
                "user_info_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_info_url = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for OAuthProvider {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "OAuthProvider"
    }
}

impl backbone_core::PersistentEntity for OAuthProvider {
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

impl backbone_orm::EntityRepoMeta for OAuthProvider {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("provider_name".to_string(), "o_auth_provider_type".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["display_name", "client_id", "client_secret"]
    }
}

/// Builder for OAuthProvider entity
///
/// Provides a fluent API for constructing OAuthProvider instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct OAuthProviderBuilder {
    provider_name: Option<OAuthProviderType>,
    display_name: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    redirect_uri: Option<String>,
    scopes: Option<Vec<String>>,
    authorization_url: Option<String>,
    token_url: Option<String>,
    user_info_url: Option<String>,
    is_active: Option<bool>,
}

impl OAuthProviderBuilder {
    /// Set the provider_name field (required)
    pub fn provider_name(mut self, value: OAuthProviderType) -> Self {
        self.provider_name = Some(value);
        self
    }

    /// Set the display_name field (required)
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    /// Set the client_id field (required)
    pub fn client_id(mut self, value: String) -> Self {
        self.client_id = Some(value);
        self
    }

    /// Set the client_secret field (required)
    pub fn client_secret(mut self, value: String) -> Self {
        self.client_secret = Some(value);
        self
    }

    /// Set the redirect_uri field (required)
    pub fn redirect_uri(mut self, value: String) -> Self {
        self.redirect_uri = Some(value);
        self
    }

    /// Set the scopes field (required)
    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    /// Set the authorization_url field (required)
    pub fn authorization_url(mut self, value: String) -> Self {
        self.authorization_url = Some(value);
        self
    }

    /// Set the token_url field (required)
    pub fn token_url(mut self, value: String) -> Self {
        self.token_url = Some(value);
        self
    }

    /// Set the user_info_url field (required)
    pub fn user_info_url(mut self, value: String) -> Self {
        self.user_info_url = Some(value);
        self
    }

    /// Set the is_active field (default: `true`)
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Build the OAuthProvider entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<OAuthProvider, String> {
        let provider_name = self.provider_name.ok_or_else(|| "provider_name is required".to_string())?;
        let display_name = self.display_name.ok_or_else(|| "display_name is required".to_string())?;
        let client_id = self.client_id.ok_or_else(|| "client_id is required".to_string())?;
        let client_secret = self.client_secret.ok_or_else(|| "client_secret is required".to_string())?;
        let redirect_uri = self.redirect_uri.ok_or_else(|| "redirect_uri is required".to_string())?;
        let scopes = self.scopes.ok_or_else(|| "scopes is required".to_string())?;
        let authorization_url = self.authorization_url.ok_or_else(|| "authorization_url is required".to_string())?;
        let token_url = self.token_url.ok_or_else(|| "token_url is required".to_string())?;
        let user_info_url = self.user_info_url.ok_or_else(|| "user_info_url is required".to_string())?;

        Ok(OAuthProvider {
            id: Uuid::new_v4(),
            provider_name,
            display_name,
            client_id,
            client_secret,
            redirect_uri,
            scopes,
            authorization_url,
            token_url,
            user_info_url,
            is_active: self.is_active.unwrap_or(true),
            metadata: AuditMetadata::default(),
        })
    }
}
