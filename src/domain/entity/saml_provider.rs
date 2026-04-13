use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::SAMLProviderStatus;
use super::AuditMetadata;

use crate::domain::state_machine::{SAMLProviderStateMachine, SAMLProviderState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for SAMLProvider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SAMLProviderId(pub Uuid);

impl SAMLProviderId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for SAMLProviderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for SAMLProviderId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for SAMLProviderId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<SAMLProviderId> for Uuid {
    fn from(id: SAMLProviderId) -> Self { id.0 }
}

impl AsRef<Uuid> for SAMLProviderId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for SAMLProviderId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SAMLProvider {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub entity_id: String,
    pub sso_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo_url: Option<String>,
    pub certificate: String,
    pub acs_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_url: Option<String>,
    pub name_id_format: String,
    pub attribute_mapping: serde_json::Value,
    pub is_active: bool,
    pub(crate) status: SAMLProviderStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl SAMLProvider {
    /// Create a builder for SAMLProvider
    pub fn builder() -> SAMLProviderBuilder {
        SAMLProviderBuilder::default()
    }

    /// Create a new SAMLProvider with required fields
    pub fn new(name: String, display_name: String, entity_id: String, sso_url: String, certificate: String, acs_url: String, name_id_format: String, attribute_mapping: serde_json::Value, is_active: bool, status: SAMLProviderStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            display_name,
            entity_id,
            sso_url,
            slo_url: None,
            certificate,
            acs_url,
            sls_url: None,
            name_id_format,
            attribute_mapping,
            is_active,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> SAMLProviderId {
        SAMLProviderId(self.id)
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
    pub fn status(&self) -> &SAMLProviderStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the slo_url field (chainable)
    pub fn with_slo_url(mut self, value: String) -> Self {
        self.slo_url = Some(value);
        self
    }

    /// Set the sls_url field (chainable)
    pub fn with_sls_url(mut self, value: String) -> Self {
        self.sls_url = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: SAMLProviderState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<SAMLProviderState>()?;
        let mut sm = SAMLProviderStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<SAMLProviderStatus>()
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
                "name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.name = v; }
                }
                "display_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.display_name = v; }
                }
                "entity_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.entity_id = v; }
                }
                "sso_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.sso_url = v; }
                }
                "slo_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.slo_url = v; }
                }
                "certificate" => {
                    if let Ok(v) = serde_json::from_value(value) { self.certificate = v; }
                }
                "acs_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.acs_url = v; }
                }
                "sls_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.sls_url = v; }
                }
                "name_id_format" => {
                    if let Ok(v) = serde_json::from_value(value) { self.name_id_format = v; }
                }
                "attribute_mapping" => {
                    if let Ok(v) = serde_json::from_value(value) { self.attribute_mapping = v; }
                }
                "is_active" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_active = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for SAMLProvider {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "SAMLProvider"
    }
}

impl backbone_core::PersistentEntity for SAMLProvider {
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

impl backbone_orm::EntityRepoMeta for SAMLProvider {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "saml_provider_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["name", "display_name", "entity_id", "sso_url", "certificate", "acs_url", "name_id_format"]
    }
}

/// Builder for SAMLProvider entity
///
/// Provides a fluent API for constructing SAMLProvider instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct SAMLProviderBuilder {
    name: Option<String>,
    display_name: Option<String>,
    entity_id: Option<String>,
    sso_url: Option<String>,
    slo_url: Option<String>,
    certificate: Option<String>,
    acs_url: Option<String>,
    sls_url: Option<String>,
    name_id_format: Option<String>,
    attribute_mapping: Option<serde_json::Value>,
    is_active: Option<bool>,
    status: Option<SAMLProviderStatus>,
}

impl SAMLProviderBuilder {
    /// Set the name field (required)
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the display_name field (required)
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    /// Set the entity_id field (required)
    pub fn entity_id(mut self, value: String) -> Self {
        self.entity_id = Some(value);
        self
    }

    /// Set the sso_url field (required)
    pub fn sso_url(mut self, value: String) -> Self {
        self.sso_url = Some(value);
        self
    }

    /// Set the slo_url field (optional)
    pub fn slo_url(mut self, value: String) -> Self {
        self.slo_url = Some(value);
        self
    }

    /// Set the certificate field (required)
    pub fn certificate(mut self, value: String) -> Self {
        self.certificate = Some(value);
        self
    }

    /// Set the acs_url field (required)
    pub fn acs_url(mut self, value: String) -> Self {
        self.acs_url = Some(value);
        self
    }

    /// Set the sls_url field (optional)
    pub fn sls_url(mut self, value: String) -> Self {
        self.sls_url = Some(value);
        self
    }

    /// Set the name_id_format field (default: `Default::default()`)
    pub fn name_id_format(mut self, value: String) -> Self {
        self.name_id_format = Some(value);
        self
    }

    /// Set the attribute_mapping field (default: `Default::default()`)
    pub fn attribute_mapping(mut self, value: serde_json::Value) -> Self {
        self.attribute_mapping = Some(value);
        self
    }

    /// Set the is_active field (default: `true`)
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Set the status field (default: `SAMLProviderStatus::default()`)
    pub fn status(mut self, value: SAMLProviderStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the SAMLProvider entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<SAMLProvider, String> {
        let name = self.name.ok_or_else(|| "name is required".to_string())?;
        let display_name = self.display_name.ok_or_else(|| "display_name is required".to_string())?;
        let entity_id = self.entity_id.ok_or_else(|| "entity_id is required".to_string())?;
        let sso_url = self.sso_url.ok_or_else(|| "sso_url is required".to_string())?;
        let certificate = self.certificate.ok_or_else(|| "certificate is required".to_string())?;
        let acs_url = self.acs_url.ok_or_else(|| "acs_url is required".to_string())?;

        Ok(SAMLProvider {
            id: Uuid::new_v4(),
            name,
            display_name,
            entity_id,
            sso_url,
            slo_url: self.slo_url,
            certificate,
            acs_url,
            sls_url: self.sls_url,
            name_id_format: self.name_id_format.unwrap_or(Default::default()),
            attribute_mapping: self.attribute_mapping.unwrap_or(Default::default()),
            is_active: self.is_active.unwrap_or(true),
            status: self.status.unwrap_or(SAMLProviderStatus::default()),
            metadata: AuditMetadata::default(),
        })
    }
}
