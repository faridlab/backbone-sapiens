use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for BackupCode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BackupCodeId(pub Uuid);

impl BackupCodeId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for BackupCodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for BackupCodeId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for BackupCodeId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<BackupCodeId> for Uuid {
    fn from(id: BackupCodeId) -> Self { id.0 }
}

impl AsRef<Uuid> for BackupCodeId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for BackupCodeId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BackupCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub batch_id: String,
    pub code_hash: String,
    pub code_index: i32,
    pub is_used: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_user_agent: Option<String>,
    pub expires_at: DateTime<Utc>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl BackupCode {
    /// Create a builder for BackupCode
    pub fn builder() -> BackupCodeBuilder {
        BackupCodeBuilder::default()
    }

    /// Create a new BackupCode with required fields
    pub fn new(user_id: Uuid, batch_id: String, code_hash: String, code_index: i32, is_used: bool, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            batch_id,
            code_hash,
            code_index,
            is_used,
            used_at: None,
            used_ip: None,
            used_user_agent: None,
            expires_at,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> BackupCodeId {
        BackupCodeId(self.id)
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

    /// Set the used_at field (chainable)
    pub fn with_used_at(mut self, value: DateTime<Utc>) -> Self {
        self.used_at = Some(value);
        self
    }

    /// Set the used_ip field (chainable)
    pub fn with_used_ip(mut self, value: String) -> Self {
        self.used_ip = Some(value);
        self
    }

    /// Set the used_user_agent field (chainable)
    pub fn with_used_user_agent(mut self, value: String) -> Self {
        self.used_user_agent = Some(value);
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
                "batch_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.batch_id = v; }
                }
                "code_hash" => {
                    if let Ok(v) = serde_json::from_value(value) { self.code_hash = v; }
                }
                "code_index" => {
                    if let Ok(v) = serde_json::from_value(value) { self.code_index = v; }
                }
                "is_used" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_used = v; }
                }
                "used_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.used_at = v; }
                }
                "used_ip" => {
                    if let Ok(v) = serde_json::from_value(value) { self.used_ip = v; }
                }
                "used_user_agent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.used_user_agent = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for BackupCode {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "BackupCode"
    }
}

impl backbone_core::PersistentEntity for BackupCode {
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

impl backbone_orm::EntityRepoMeta for BackupCode {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["batch_id", "code_hash"]
    }
}

/// Builder for BackupCode entity
///
/// Provides a fluent API for constructing BackupCode instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct BackupCodeBuilder {
    user_id: Option<Uuid>,
    batch_id: Option<String>,
    code_hash: Option<String>,
    code_index: Option<i32>,
    is_used: Option<bool>,
    used_at: Option<DateTime<Utc>>,
    used_ip: Option<String>,
    used_user_agent: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

impl BackupCodeBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the batch_id field (required)
    pub fn batch_id(mut self, value: String) -> Self {
        self.batch_id = Some(value);
        self
    }

    /// Set the code_hash field (required)
    pub fn code_hash(mut self, value: String) -> Self {
        self.code_hash = Some(value);
        self
    }

    /// Set the code_index field (required)
    pub fn code_index(mut self, value: i32) -> Self {
        self.code_index = Some(value);
        self
    }

    /// Set the is_used field (default: `false`)
    pub fn is_used(mut self, value: bool) -> Self {
        self.is_used = Some(value);
        self
    }

    /// Set the used_at field (optional)
    pub fn used_at(mut self, value: DateTime<Utc>) -> Self {
        self.used_at = Some(value);
        self
    }

    /// Set the used_ip field (optional)
    pub fn used_ip(mut self, value: String) -> Self {
        self.used_ip = Some(value);
        self
    }

    /// Set the used_user_agent field (optional)
    pub fn used_user_agent(mut self, value: String) -> Self {
        self.used_user_agent = Some(value);
        self
    }

    /// Set the expires_at field (required)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Build the BackupCode entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<BackupCode, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let batch_id = self.batch_id.ok_or_else(|| "batch_id is required".to_string())?;
        let code_hash = self.code_hash.ok_or_else(|| "code_hash is required".to_string())?;
        let code_index = self.code_index.ok_or_else(|| "code_index is required".to_string())?;
        let expires_at = self.expires_at.ok_or_else(|| "expires_at is required".to_string())?;

        Ok(BackupCode {
            id: Uuid::new_v4(),
            user_id,
            batch_id,
            code_hash,
            code_index,
            is_used: self.is_used.unwrap_or(false),
            used_at: self.used_at,
            used_ip: self.used_ip,
            used_user_agent: self.used_user_agent,
            expires_at,
            metadata: AuditMetadata::default(),
        })
    }
}
