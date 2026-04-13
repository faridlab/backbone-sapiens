use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for PasswordExpirationSettings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PasswordExpirationSettingsId(pub Uuid);

impl PasswordExpirationSettingsId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for PasswordExpirationSettingsId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PasswordExpirationSettingsId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for PasswordExpirationSettingsId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<PasswordExpirationSettingsId> for Uuid {
    fn from(id: PasswordExpirationSettingsId) -> Self { id.0 }
}

impl AsRef<Uuid> for PasswordExpirationSettingsId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for PasswordExpirationSettingsId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordExpirationSettings {
    pub id: Uuid,
    pub max_age_days: i32,
    pub warning_days: i32,
    pub grace_logins: i32,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl PasswordExpirationSettings {
    /// Create a builder for PasswordExpirationSettings
    pub fn builder() -> PasswordExpirationSettingsBuilder {
        PasswordExpirationSettingsBuilder::default()
    }

    /// Create a new PasswordExpirationSettings with required fields
    pub fn new(max_age_days: i32, warning_days: i32, grace_logins: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            max_age_days,
            warning_days,
            grace_logins,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> PasswordExpirationSettingsId {
        PasswordExpirationSettingsId(self.id)
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
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "max_age_days" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_age_days = v; }
                }
                "warning_days" => {
                    if let Ok(v) = serde_json::from_value(value) { self.warning_days = v; }
                }
                "grace_logins" => {
                    if let Ok(v) = serde_json::from_value(value) { self.grace_logins = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for PasswordExpirationSettings {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "PasswordExpirationSettings"
    }
}

impl backbone_core::PersistentEntity for PasswordExpirationSettings {
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

impl backbone_orm::EntityRepoMeta for PasswordExpirationSettings {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for PasswordExpirationSettings entity
///
/// Provides a fluent API for constructing PasswordExpirationSettings instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct PasswordExpirationSettingsBuilder {
    max_age_days: Option<i32>,
    warning_days: Option<i32>,
    grace_logins: Option<i32>,
}

impl PasswordExpirationSettingsBuilder {
    /// Set the max_age_days field (default: `90`)
    pub fn max_age_days(mut self, value: i32) -> Self {
        self.max_age_days = Some(value);
        self
    }

    /// Set the warning_days field (default: `7`)
    pub fn warning_days(mut self, value: i32) -> Self {
        self.warning_days = Some(value);
        self
    }

    /// Set the grace_logins field (default: `3`)
    pub fn grace_logins(mut self, value: i32) -> Self {
        self.grace_logins = Some(value);
        self
    }

    /// Build the PasswordExpirationSettings entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<PasswordExpirationSettings, String> {

        Ok(PasswordExpirationSettings {
            id: Uuid::new_v4(),
            max_age_days: self.max_age_days.unwrap_or(90),
            warning_days: self.warning_days.unwrap_or(7),
            grace_logins: self.grace_logins.unwrap_or(3),
            metadata: AuditMetadata::default(),
        })
    }
}
