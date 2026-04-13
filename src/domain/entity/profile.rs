use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::Gender;
use super::AuditMetadata;

/// Strongly-typed ID for Profile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProfileId(pub Uuid);

impl ProfileId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for ProfileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ProfileId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for ProfileId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<ProfileId> for Uuid {
    fn from(id: ProfileId) -> Self { id.0 }
}

impl AsRef<Uuid> for ProfileId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for ProfileId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Profile {
    pub user_id: Uuid,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pob: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture_url: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Profile {
    /// Create a builder for Profile
    pub fn builder() -> ProfileBuilder {
        ProfileBuilder::default()
    }

    /// Create a new Profile with required fields
    pub fn new(first_name: String) -> Self {
        Self {
            user_id: Uuid::new_v4(),
            first_name,
            middle_name: None,
            last_name: None,
            dob: None,
            pob: None,
            gender: None,
            phone_number: None,
            profile_picture_url: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.user_id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> ProfileId {
        ProfileId(self.user_id)
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

    /// Set the middle_name field (chainable)
    pub fn with_middle_name(mut self, value: String) -> Self {
        self.middle_name = Some(value);
        self
    }

    /// Set the last_name field (chainable)
    pub fn with_last_name(mut self, value: String) -> Self {
        self.last_name = Some(value);
        self
    }

    /// Set the dob field (chainable)
    pub fn with_dob(mut self, value: NaiveDate) -> Self {
        self.dob = Some(value);
        self
    }

    /// Set the pob field (chainable)
    pub fn with_pob(mut self, value: String) -> Self {
        self.pob = Some(value);
        self
    }

    /// Set the gender field (chainable)
    pub fn with_gender(mut self, value: Gender) -> Self {
        self.gender = Some(value);
        self
    }

    /// Set the phone_number field (chainable)
    pub fn with_phone_number(mut self, value: String) -> Self {
        self.phone_number = Some(value);
        self
    }

    /// Set the profile_picture_url field (chainable)
    pub fn with_profile_picture_url(mut self, value: String) -> Self {
        self.profile_picture_url = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "first_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.first_name = v; }
                }
                "middle_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.middle_name = v; }
                }
                "last_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_name = v; }
                }
                "dob" => {
                    if let Ok(v) = serde_json::from_value(value) { self.dob = v; }
                }
                "pob" => {
                    if let Ok(v) = serde_json::from_value(value) { self.pob = v; }
                }
                "gender" => {
                    if let Ok(v) = serde_json::from_value(value) { self.gender = v; }
                }
                "phone_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.phone_number = v; }
                }
                "profile_picture_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.profile_picture_url = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Profile {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.user_id
    }

    fn entity_type() -> &'static str {
        "Profile"
    }
}

impl backbone_core::PersistentEntity for Profile {
    fn entity_id(&self) -> String {
        self.user_id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.user_id = uuid;
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

impl backbone_orm::EntityRepoMeta for Profile {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("gender".to_string(), "gender".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["first_name"]
    }
}

/// Builder for Profile entity
///
/// Provides a fluent API for constructing Profile instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct ProfileBuilder {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    dob: Option<NaiveDate>,
    pob: Option<String>,
    gender: Option<Gender>,
    phone_number: Option<String>,
    profile_picture_url: Option<String>,
}

impl ProfileBuilder {
    /// Set the first_name field (required)
    pub fn first_name(mut self, value: String) -> Self {
        self.first_name = Some(value);
        self
    }

    /// Set the middle_name field (optional)
    pub fn middle_name(mut self, value: String) -> Self {
        self.middle_name = Some(value);
        self
    }

    /// Set the last_name field (optional)
    pub fn last_name(mut self, value: String) -> Self {
        self.last_name = Some(value);
        self
    }

    /// Set the dob field (optional)
    pub fn dob(mut self, value: NaiveDate) -> Self {
        self.dob = Some(value);
        self
    }

    /// Set the pob field (optional)
    pub fn pob(mut self, value: String) -> Self {
        self.pob = Some(value);
        self
    }

    /// Set the gender field (optional)
    pub fn gender(mut self, value: Gender) -> Self {
        self.gender = Some(value);
        self
    }

    /// Set the phone_number field (optional)
    pub fn phone_number(mut self, value: String) -> Self {
        self.phone_number = Some(value);
        self
    }

    /// Set the profile_picture_url field (optional)
    pub fn profile_picture_url(mut self, value: String) -> Self {
        self.profile_picture_url = Some(value);
        self
    }

    /// Build the Profile entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Profile, String> {
        let first_name = self.first_name.ok_or_else(|| "first_name is required".to_string())?;

        Ok(Profile {
            user_id: Uuid::new_v4(),
            first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            dob: self.dob,
            pob: self.pob,
            gender: self.gender,
            phone_number: self.phone_number,
            profile_picture_url: self.profile_picture_url,
            metadata: AuditMetadata::default(),
        })
    }
}
