use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::LDAPDirectoryStatus;
use super::AuditMetadata;

use crate::domain::state_machine::{LDAPDirectoryStateMachine, LDAPDirectoryState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for LDAPDirectory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LDAPDirectoryId(pub Uuid);

impl LDAPDirectoryId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for LDAPDirectoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for LDAPDirectoryId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for LDAPDirectoryId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<LDAPDirectoryId> for Uuid {
    fn from(id: LDAPDirectoryId) -> Self { id.0 }
}

impl AsRef<Uuid> for LDAPDirectoryId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for LDAPDirectoryId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LDAPDirectory {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub host: String,
    pub port: i32,
    pub use_ssl: bool,
    pub use_tls: bool,
    pub bind_dn: String,
    pub bind_password: String,
    pub search_base: String,
    pub search_filter: String,
    pub attribute_mapping: serde_json::Value,
    pub sync_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_interval_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_result: Option<serde_json::Value>,
    pub is_active: bool,
    pub(crate) status: LDAPDirectoryStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl LDAPDirectory {
    /// Create a builder for LDAPDirectory
    pub fn builder() -> LDAPDirectoryBuilder {
        LDAPDirectoryBuilder::default()
    }

    /// Create a new LDAPDirectory with required fields
    pub fn new(name: String, display_name: String, host: String, port: i32, use_ssl: bool, use_tls: bool, bind_dn: String, bind_password: String, search_base: String, search_filter: String, attribute_mapping: serde_json::Value, sync_enabled: bool, is_active: bool, status: LDAPDirectoryStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            display_name,
            host,
            port,
            use_ssl,
            use_tls,
            bind_dn,
            bind_password,
            search_base,
            search_filter,
            attribute_mapping,
            sync_enabled,
            sync_interval_minutes: None,
            last_sync_at: None,
            last_sync_result: None,
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
    pub fn typed_id(&self) -> LDAPDirectoryId {
        LDAPDirectoryId(self.id)
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
    pub fn status(&self) -> &LDAPDirectoryStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the sync_interval_minutes field (chainable)
    pub fn with_sync_interval_minutes(mut self, value: i32) -> Self {
        self.sync_interval_minutes = Some(value);
        self
    }

    /// Set the last_sync_at field (chainable)
    pub fn with_last_sync_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_sync_at = Some(value);
        self
    }

    /// Set the last_sync_result field (chainable)
    pub fn with_last_sync_result(mut self, value: serde_json::Value) -> Self {
        self.last_sync_result = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: LDAPDirectoryState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<LDAPDirectoryState>()?;
        let mut sm = LDAPDirectoryStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<LDAPDirectoryStatus>()
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
                "host" => {
                    if let Ok(v) = serde_json::from_value(value) { self.host = v; }
                }
                "port" => {
                    if let Ok(v) = serde_json::from_value(value) { self.port = v; }
                }
                "use_ssl" => {
                    if let Ok(v) = serde_json::from_value(value) { self.use_ssl = v; }
                }
                "use_tls" => {
                    if let Ok(v) = serde_json::from_value(value) { self.use_tls = v; }
                }
                "bind_dn" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bind_dn = v; }
                }
                "bind_password" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bind_password = v; }
                }
                "search_base" => {
                    if let Ok(v) = serde_json::from_value(value) { self.search_base = v; }
                }
                "search_filter" => {
                    if let Ok(v) = serde_json::from_value(value) { self.search_filter = v; }
                }
                "attribute_mapping" => {
                    if let Ok(v) = serde_json::from_value(value) { self.attribute_mapping = v; }
                }
                "sync_enabled" => {
                    if let Ok(v) = serde_json::from_value(value) { self.sync_enabled = v; }
                }
                "sync_interval_minutes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.sync_interval_minutes = v; }
                }
                "last_sync_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_sync_at = v; }
                }
                "last_sync_result" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_sync_result = v; }
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

impl super::Entity for LDAPDirectory {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "LDAPDirectory"
    }
}

impl backbone_core::PersistentEntity for LDAPDirectory {
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

impl backbone_orm::EntityRepoMeta for LDAPDirectory {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "ldap_directory_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["name", "display_name", "host", "bind_dn", "bind_password", "search_base", "search_filter"]
    }
}

/// Builder for LDAPDirectory entity
///
/// Provides a fluent API for constructing LDAPDirectory instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct LDAPDirectoryBuilder {
    name: Option<String>,
    display_name: Option<String>,
    host: Option<String>,
    port: Option<i32>,
    use_ssl: Option<bool>,
    use_tls: Option<bool>,
    bind_dn: Option<String>,
    bind_password: Option<String>,
    search_base: Option<String>,
    search_filter: Option<String>,
    attribute_mapping: Option<serde_json::Value>,
    sync_enabled: Option<bool>,
    sync_interval_minutes: Option<i32>,
    last_sync_at: Option<DateTime<Utc>>,
    last_sync_result: Option<serde_json::Value>,
    is_active: Option<bool>,
    status: Option<LDAPDirectoryStatus>,
}

impl LDAPDirectoryBuilder {
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

    /// Set the host field (required)
    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }

    /// Set the port field (default: `389`)
    pub fn port(mut self, value: i32) -> Self {
        self.port = Some(value);
        self
    }

    /// Set the use_ssl field (default: `true`)
    pub fn use_ssl(mut self, value: bool) -> Self {
        self.use_ssl = Some(value);
        self
    }

    /// Set the use_tls field (default: `false`)
    pub fn use_tls(mut self, value: bool) -> Self {
        self.use_tls = Some(value);
        self
    }

    /// Set the bind_dn field (required)
    pub fn bind_dn(mut self, value: String) -> Self {
        self.bind_dn = Some(value);
        self
    }

    /// Set the bind_password field (required)
    pub fn bind_password(mut self, value: String) -> Self {
        self.bind_password = Some(value);
        self
    }

    /// Set the search_base field (required)
    pub fn search_base(mut self, value: String) -> Self {
        self.search_base = Some(value);
        self
    }

    /// Set the search_filter field (default: `Default::default()`)
    pub fn search_filter(mut self, value: String) -> Self {
        self.search_filter = Some(value);
        self
    }

    /// Set the attribute_mapping field (default: `Default::default()`)
    pub fn attribute_mapping(mut self, value: serde_json::Value) -> Self {
        self.attribute_mapping = Some(value);
        self
    }

    /// Set the sync_enabled field (default: `false`)
    pub fn sync_enabled(mut self, value: bool) -> Self {
        self.sync_enabled = Some(value);
        self
    }

    /// Set the sync_interval_minutes field (optional)
    pub fn sync_interval_minutes(mut self, value: i32) -> Self {
        self.sync_interval_minutes = Some(value);
        self
    }

    /// Set the last_sync_at field (optional)
    pub fn last_sync_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_sync_at = Some(value);
        self
    }

    /// Set the last_sync_result field (optional)
    pub fn last_sync_result(mut self, value: serde_json::Value) -> Self {
        self.last_sync_result = Some(value);
        self
    }

    /// Set the is_active field (default: `true`)
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Set the status field (default: `LDAPDirectoryStatus::default()`)
    pub fn status(mut self, value: LDAPDirectoryStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the LDAPDirectory entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<LDAPDirectory, String> {
        let name = self.name.ok_or_else(|| "name is required".to_string())?;
        let display_name = self.display_name.ok_or_else(|| "display_name is required".to_string())?;
        let host = self.host.ok_or_else(|| "host is required".to_string())?;
        let bind_dn = self.bind_dn.ok_or_else(|| "bind_dn is required".to_string())?;
        let bind_password = self.bind_password.ok_or_else(|| "bind_password is required".to_string())?;
        let search_base = self.search_base.ok_or_else(|| "search_base is required".to_string())?;

        Ok(LDAPDirectory {
            id: Uuid::new_v4(),
            name,
            display_name,
            host,
            port: self.port.unwrap_or(389),
            use_ssl: self.use_ssl.unwrap_or(true),
            use_tls: self.use_tls.unwrap_or(false),
            bind_dn,
            bind_password,
            search_base,
            search_filter: self.search_filter.unwrap_or(Default::default()),
            attribute_mapping: self.attribute_mapping.unwrap_or(Default::default()),
            sync_enabled: self.sync_enabled.unwrap_or(false),
            sync_interval_minutes: self.sync_interval_minutes,
            last_sync_at: self.last_sync_at,
            last_sync_result: self.last_sync_result,
            is_active: self.is_active.unwrap_or(true),
            status: self.status.unwrap_or(LDAPDirectoryStatus::default()),
            metadata: AuditMetadata::default(),
        })
    }
}
