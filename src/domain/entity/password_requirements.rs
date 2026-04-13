use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for PasswordRequirements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PasswordRequirementsId(pub Uuid);

impl PasswordRequirementsId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for PasswordRequirementsId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PasswordRequirementsId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for PasswordRequirementsId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<PasswordRequirementsId> for Uuid {
    fn from(id: PasswordRequirementsId) -> Self { id.0 }
}

impl AsRef<Uuid> for PasswordRequirementsId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for PasswordRequirementsId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordRequirements {
    pub id: Uuid,
    pub min_length: i32,
    pub max_length: i32,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_digits: bool,
    pub require_special_characters: bool,
    pub require_no_common_passwords: bool,
    pub require_no_user_info: bool,
    pub min_entropy_bits: i32,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl PasswordRequirements {
    /// Create a builder for PasswordRequirements
    pub fn builder() -> PasswordRequirementsBuilder {
        PasswordRequirementsBuilder::default()
    }

    /// Create a new PasswordRequirements with required fields
    pub fn new(min_length: i32, max_length: i32, require_uppercase: bool, require_lowercase: bool, require_digits: bool, require_special_characters: bool, require_no_common_passwords: bool, require_no_user_info: bool, min_entropy_bits: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            min_length,
            max_length,
            require_uppercase,
            require_lowercase,
            require_digits,
            require_special_characters,
            require_no_common_passwords,
            require_no_user_info,
            min_entropy_bits,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> PasswordRequirementsId {
        PasswordRequirementsId(self.id)
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
                "min_length" => {
                    if let Ok(v) = serde_json::from_value(value) { self.min_length = v; }
                }
                "max_length" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_length = v; }
                }
                "require_uppercase" => {
                    if let Ok(v) = serde_json::from_value(value) { self.require_uppercase = v; }
                }
                "require_lowercase" => {
                    if let Ok(v) = serde_json::from_value(value) { self.require_lowercase = v; }
                }
                "require_digits" => {
                    if let Ok(v) = serde_json::from_value(value) { self.require_digits = v; }
                }
                "require_special_characters" => {
                    if let Ok(v) = serde_json::from_value(value) { self.require_special_characters = v; }
                }
                "require_no_common_passwords" => {
                    if let Ok(v) = serde_json::from_value(value) { self.require_no_common_passwords = v; }
                }
                "require_no_user_info" => {
                    if let Ok(v) = serde_json::from_value(value) { self.require_no_user_info = v; }
                }
                "min_entropy_bits" => {
                    if let Ok(v) = serde_json::from_value(value) { self.min_entropy_bits = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for PasswordRequirements {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "PasswordRequirements"
    }
}

impl backbone_core::PersistentEntity for PasswordRequirements {
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

impl backbone_orm::EntityRepoMeta for PasswordRequirements {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for PasswordRequirements entity
///
/// Provides a fluent API for constructing PasswordRequirements instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct PasswordRequirementsBuilder {
    min_length: Option<i32>,
    max_length: Option<i32>,
    require_uppercase: Option<bool>,
    require_lowercase: Option<bool>,
    require_digits: Option<bool>,
    require_special_characters: Option<bool>,
    require_no_common_passwords: Option<bool>,
    require_no_user_info: Option<bool>,
    min_entropy_bits: Option<i32>,
}

impl PasswordRequirementsBuilder {
    /// Set the min_length field (default: `12`)
    pub fn min_length(mut self, value: i32) -> Self {
        self.min_length = Some(value);
        self
    }

    /// Set the max_length field (default: `128`)
    pub fn max_length(mut self, value: i32) -> Self {
        self.max_length = Some(value);
        self
    }

    /// Set the require_uppercase field (default: `true`)
    pub fn require_uppercase(mut self, value: bool) -> Self {
        self.require_uppercase = Some(value);
        self
    }

    /// Set the require_lowercase field (default: `true`)
    pub fn require_lowercase(mut self, value: bool) -> Self {
        self.require_lowercase = Some(value);
        self
    }

    /// Set the require_digits field (default: `true`)
    pub fn require_digits(mut self, value: bool) -> Self {
        self.require_digits = Some(value);
        self
    }

    /// Set the require_special_characters field (default: `true`)
    pub fn require_special_characters(mut self, value: bool) -> Self {
        self.require_special_characters = Some(value);
        self
    }

    /// Set the require_no_common_passwords field (default: `true`)
    pub fn require_no_common_passwords(mut self, value: bool) -> Self {
        self.require_no_common_passwords = Some(value);
        self
    }

    /// Set the require_no_user_info field (default: `true`)
    pub fn require_no_user_info(mut self, value: bool) -> Self {
        self.require_no_user_info = Some(value);
        self
    }

    /// Set the min_entropy_bits field (default: `50`)
    pub fn min_entropy_bits(mut self, value: i32) -> Self {
        self.min_entropy_bits = Some(value);
        self
    }

    /// Build the PasswordRequirements entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<PasswordRequirements, String> {

        Ok(PasswordRequirements {
            id: Uuid::new_v4(),
            min_length: self.min_length.unwrap_or(12),
            max_length: self.max_length.unwrap_or(128),
            require_uppercase: self.require_uppercase.unwrap_or(true),
            require_lowercase: self.require_lowercase.unwrap_or(true),
            require_digits: self.require_digits.unwrap_or(true),
            require_special_characters: self.require_special_characters.unwrap_or(true),
            require_no_common_passwords: self.require_no_common_passwords.unwrap_or(true),
            require_no_user_info: self.require_no_user_info.unwrap_or(true),
            min_entropy_bits: self.min_entropy_bits.unwrap_or(50),
            metadata: AuditMetadata::default(),
        })
    }
}
