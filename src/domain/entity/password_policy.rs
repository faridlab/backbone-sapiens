use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for PasswordPolicy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PasswordPolicyId(pub Uuid);

impl PasswordPolicyId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for PasswordPolicyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PasswordPolicyId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for PasswordPolicyId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<PasswordPolicyId> for Uuid {
    fn from(id: PasswordPolicyId) -> Self { id.0 }
}

impl AsRef<Uuid> for PasswordPolicyId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for PasswordPolicyId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordPolicy {
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<Uuid>,
    pub is_active: bool,
    pub password_requirements: serde_json::Value,
    pub password_history: serde_json::Value,
    pub reset_settings: serde_json::Value,
    pub expiry_settings: serde_json::Value,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl PasswordPolicy {
    /// Create a builder for PasswordPolicy
    pub fn builder() -> PasswordPolicyBuilder {
        PasswordPolicyBuilder::default()
    }

    /// Create a new PasswordPolicy with required fields
    pub fn new(name: String, is_active: bool, password_requirements: serde_json::Value, password_history: serde_json::Value, reset_settings: serde_json::Value, expiry_settings: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            organization_id: None,
            is_active,
            password_requirements,
            password_history,
            reset_settings,
            expiry_settings,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> PasswordPolicyId {
        PasswordPolicyId(self.id)
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

    /// Set the organization_id field (chainable)
    pub fn with_organization_id(mut self, value: Uuid) -> Self {
        self.organization_id = Some(value);
        self
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
                "organization_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.organization_id = v; }
                }
                "is_active" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_active = v; }
                }
                "password_requirements" => {
                    if let Ok(v) = serde_json::from_value(value) { self.password_requirements = v; }
                }
                "password_history" => {
                    if let Ok(v) = serde_json::from_value(value) { self.password_history = v; }
                }
                "reset_settings" => {
                    if let Ok(v) = serde_json::from_value(value) { self.reset_settings = v; }
                }
                "expiry_settings" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expiry_settings = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for PasswordPolicy {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "PasswordPolicy"
    }
}

impl backbone_core::PersistentEntity for PasswordPolicy {
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

impl backbone_orm::EntityRepoMeta for PasswordPolicy {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("organization_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["name"]
    }
}

/// Builder for PasswordPolicy entity
///
/// Provides a fluent API for constructing PasswordPolicy instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct PasswordPolicyBuilder {
    name: Option<String>,
    organization_id: Option<Uuid>,
    is_active: Option<bool>,
    password_requirements: Option<serde_json::Value>,
    password_history: Option<serde_json::Value>,
    reset_settings: Option<serde_json::Value>,
    expiry_settings: Option<serde_json::Value>,
}

impl PasswordPolicyBuilder {
    /// Set the name field (required)
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the organization_id field (optional)
    pub fn organization_id(mut self, value: Uuid) -> Self {
        self.organization_id = Some(value);
        self
    }

    /// Set the is_active field (default: `true`)
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Set the password_requirements field (required)
    pub fn password_requirements(mut self, value: serde_json::Value) -> Self {
        self.password_requirements = Some(value);
        self
    }

    /// Set the password_history field (required)
    pub fn password_history(mut self, value: serde_json::Value) -> Self {
        self.password_history = Some(value);
        self
    }

    /// Set the reset_settings field (required)
    pub fn reset_settings(mut self, value: serde_json::Value) -> Self {
        self.reset_settings = Some(value);
        self
    }

    /// Set the expiry_settings field (required)
    pub fn expiry_settings(mut self, value: serde_json::Value) -> Self {
        self.expiry_settings = Some(value);
        self
    }

    /// Build the PasswordPolicy entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<PasswordPolicy, String> {
        let name = self.name.ok_or_else(|| "name is required".to_string())?;
        let password_requirements = self.password_requirements.ok_or_else(|| "password_requirements is required".to_string())?;
        let password_history = self.password_history.ok_or_else(|| "password_history is required".to_string())?;
        let reset_settings = self.reset_settings.ok_or_else(|| "reset_settings is required".to_string())?;
        let expiry_settings = self.expiry_settings.ok_or_else(|| "expiry_settings is required".to_string())?;

        Ok(PasswordPolicy {
            id: Uuid::new_v4(),
            name,
            organization_id: self.organization_id,
            is_active: self.is_active.unwrap_or(true),
            password_requirements,
            password_history,
            reset_settings,
            expiry_settings,
            metadata: AuditMetadata::default(),
        })
    }
}
