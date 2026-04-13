use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::PasswordHistoryStatus;
use super::AuditMetadata;

/// Strongly-typed ID for PasswordHistory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PasswordHistoryId(pub Uuid);

impl PasswordHistoryId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for PasswordHistoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PasswordHistoryId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for PasswordHistoryId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<PasswordHistoryId> for Uuid {
    fn from(id: PasswordHistoryId) -> Self { id.0 }
}

impl AsRef<Uuid> for PasswordHistoryId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for PasswordHistoryId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordHistory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub password_hash: String,
    pub password_strength: serde_json::Value,
    pub password_metadata: serde_json::Value,
    pub creation_context: serde_json::Value,
    pub status: PasswordHistoryStatus,
    pub usage: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl PasswordHistory {
    /// Create a builder for PasswordHistory
    pub fn builder() -> PasswordHistoryBuilder {
        PasswordHistoryBuilder::default()
    }

    /// Create a new PasswordHistory with required fields
    pub fn new(user_id: Uuid, password_hash: String, password_strength: serde_json::Value, password_metadata: serde_json::Value, creation_context: serde_json::Value, status: PasswordHistoryStatus, usage: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            password_hash,
            password_strength,
            password_metadata,
            creation_context,
            status,
            usage,
            expiry: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> PasswordHistoryId {
        PasswordHistoryId(self.id)
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
    pub fn status(&self) -> &PasswordHistoryStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the expiry field (chainable)
    pub fn with_expiry(mut self, value: serde_json::Value) -> Self {
        self.expiry = Some(value);
        self
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
                "password_hash" => {
                    if let Ok(v) = serde_json::from_value(value) { self.password_hash = v; }
                }
                "password_strength" => {
                    if let Ok(v) = serde_json::from_value(value) { self.password_strength = v; }
                }
                "password_metadata" => {
                    if let Ok(v) = serde_json::from_value(value) { self.password_metadata = v; }
                }
                "creation_context" => {
                    if let Ok(v) = serde_json::from_value(value) { self.creation_context = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "usage" => {
                    if let Ok(v) = serde_json::from_value(value) { self.usage = v; }
                }
                "expiry" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expiry = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for PasswordHistory {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "PasswordHistory"
    }
}

impl backbone_core::PersistentEntity for PasswordHistory {
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

impl backbone_orm::EntityRepoMeta for PasswordHistory {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "password_history_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["password_hash"]
    }
}

/// Builder for PasswordHistory entity
///
/// Provides a fluent API for constructing PasswordHistory instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct PasswordHistoryBuilder {
    user_id: Option<Uuid>,
    password_hash: Option<String>,
    password_strength: Option<serde_json::Value>,
    password_metadata: Option<serde_json::Value>,
    creation_context: Option<serde_json::Value>,
    status: Option<PasswordHistoryStatus>,
    usage: Option<serde_json::Value>,
    expiry: Option<serde_json::Value>,
}

impl PasswordHistoryBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the password_hash field (required)
    pub fn password_hash(mut self, value: String) -> Self {
        self.password_hash = Some(value);
        self
    }

    /// Set the password_strength field (required)
    pub fn password_strength(mut self, value: serde_json::Value) -> Self {
        self.password_strength = Some(value);
        self
    }

    /// Set the password_metadata field (required)
    pub fn password_metadata(mut self, value: serde_json::Value) -> Self {
        self.password_metadata = Some(value);
        self
    }

    /// Set the creation_context field (required)
    pub fn creation_context(mut self, value: serde_json::Value) -> Self {
        self.creation_context = Some(value);
        self
    }

    /// Set the status field (default: `PasswordHistoryStatus::default()`)
    pub fn status(mut self, value: PasswordHistoryStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the usage field (required)
    pub fn usage(mut self, value: serde_json::Value) -> Self {
        self.usage = Some(value);
        self
    }

    /// Set the expiry field (optional)
    pub fn expiry(mut self, value: serde_json::Value) -> Self {
        self.expiry = Some(value);
        self
    }

    /// Build the PasswordHistory entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<PasswordHistory, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let password_hash = self.password_hash.ok_or_else(|| "password_hash is required".to_string())?;
        let password_strength = self.password_strength.ok_or_else(|| "password_strength is required".to_string())?;
        let password_metadata = self.password_metadata.ok_or_else(|| "password_metadata is required".to_string())?;
        let creation_context = self.creation_context.ok_or_else(|| "creation_context is required".to_string())?;
        let usage = self.usage.ok_or_else(|| "usage is required".to_string())?;

        Ok(PasswordHistory {
            id: Uuid::new_v4(),
            user_id,
            password_hash,
            password_strength,
            password_metadata,
            creation_context,
            status: self.status.unwrap_or(PasswordHistoryStatus::default()),
            usage,
            expiry: self.expiry,
            metadata: AuditMetadata::default(),
        })
    }
}
