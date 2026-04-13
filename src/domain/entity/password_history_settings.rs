use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for PasswordHistorySettings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PasswordHistorySettingsId(pub Uuid);

impl PasswordHistorySettingsId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for PasswordHistorySettingsId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PasswordHistorySettingsId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for PasswordHistorySettingsId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<PasswordHistorySettingsId> for Uuid {
    fn from(id: PasswordHistorySettingsId) -> Self { id.0 }
}

impl AsRef<Uuid> for PasswordHistorySettingsId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for PasswordHistorySettingsId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordHistorySettings {
    pub id: Uuid,
    pub track_history: bool,
    pub history_count: i32,
    pub prevent_reuse: bool,
    pub prevent_similar: bool,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl PasswordHistorySettings {
    /// Create a builder for PasswordHistorySettings
    pub fn builder() -> PasswordHistorySettingsBuilder {
        PasswordHistorySettingsBuilder::default()
    }

    /// Create a new PasswordHistorySettings with required fields
    pub fn new(track_history: bool, history_count: i32, prevent_reuse: bool, prevent_similar: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            track_history,
            history_count,
            prevent_reuse,
            prevent_similar,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> PasswordHistorySettingsId {
        PasswordHistorySettingsId(self.id)
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
                "track_history" => {
                    if let Ok(v) = serde_json::from_value(value) { self.track_history = v; }
                }
                "history_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.history_count = v; }
                }
                "prevent_reuse" => {
                    if let Ok(v) = serde_json::from_value(value) { self.prevent_reuse = v; }
                }
                "prevent_similar" => {
                    if let Ok(v) = serde_json::from_value(value) { self.prevent_similar = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for PasswordHistorySettings {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "PasswordHistorySettings"
    }
}

impl backbone_core::PersistentEntity for PasswordHistorySettings {
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

impl backbone_orm::EntityRepoMeta for PasswordHistorySettings {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for PasswordHistorySettings entity
///
/// Provides a fluent API for constructing PasswordHistorySettings instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct PasswordHistorySettingsBuilder {
    track_history: Option<bool>,
    history_count: Option<i32>,
    prevent_reuse: Option<bool>,
    prevent_similar: Option<bool>,
}

impl PasswordHistorySettingsBuilder {
    /// Set the track_history field (default: `true`)
    pub fn track_history(mut self, value: bool) -> Self {
        self.track_history = Some(value);
        self
    }

    /// Set the history_count field (default: `5`)
    pub fn history_count(mut self, value: i32) -> Self {
        self.history_count = Some(value);
        self
    }

    /// Set the prevent_reuse field (default: `true`)
    pub fn prevent_reuse(mut self, value: bool) -> Self {
        self.prevent_reuse = Some(value);
        self
    }

    /// Set the prevent_similar field (default: `true`)
    pub fn prevent_similar(mut self, value: bool) -> Self {
        self.prevent_similar = Some(value);
        self
    }

    /// Build the PasswordHistorySettings entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<PasswordHistorySettings, String> {

        Ok(PasswordHistorySettings {
            id: Uuid::new_v4(),
            track_history: self.track_history.unwrap_or(true),
            history_count: self.history_count.unwrap_or(5),
            prevent_reuse: self.prevent_reuse.unwrap_or(true),
            prevent_similar: self.prevent_similar.unwrap_or(true),
            metadata: AuditMetadata::default(),
        })
    }
}
