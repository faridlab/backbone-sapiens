use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for UserRole
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserRoleId(pub Uuid);

impl UserRoleId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for UserRoleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserRoleId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for UserRoleId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<UserRoleId> for Uuid {
    fn from(id: UserRoleId) -> Self { id.0 }
}

impl AsRef<Uuid> for UserRoleId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for UserRoleId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserRole {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_by: Option<Uuid>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl UserRole {
    /// Create a builder for UserRole
    pub fn builder() -> UserRoleBuilder {
        UserRoleBuilder::default()
    }

    /// Create a new UserRole with required fields
    pub fn new(user_id: Uuid, role_id: Uuid, assigned_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            role_id,
            assigned_at,
            assigned_by: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> UserRoleId {
        UserRoleId(self.id)
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

    /// Set the assigned_by field (chainable)
    pub fn with_assigned_by(mut self, value: Uuid) -> Self {
        self.assigned_by = Some(value);
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
                "role_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.role_id = v; }
                }
                "assigned_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.assigned_at = v; }
                }
                "assigned_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.assigned_by = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for UserRole {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "UserRole"
    }
}

impl backbone_core::PersistentEntity for UserRole {
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

impl backbone_orm::EntityRepoMeta for UserRole {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("role_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for UserRole entity
///
/// Provides a fluent API for constructing UserRole instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct UserRoleBuilder {
    user_id: Option<Uuid>,
    role_id: Option<Uuid>,
    assigned_at: Option<DateTime<Utc>>,
    assigned_by: Option<Uuid>,
}

impl UserRoleBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the role_id field (required)
    pub fn role_id(mut self, value: Uuid) -> Self {
        self.role_id = Some(value);
        self
    }

    /// Set the assigned_at field (default: `Utc::now()`)
    pub fn assigned_at(mut self, value: DateTime<Utc>) -> Self {
        self.assigned_at = Some(value);
        self
    }

    /// Set the assigned_by field (optional)
    pub fn assigned_by(mut self, value: Uuid) -> Self {
        self.assigned_by = Some(value);
        self
    }

    /// Build the UserRole entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<UserRole, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let role_id = self.role_id.ok_or_else(|| "role_id is required".to_string())?;

        Ok(UserRole {
            id: Uuid::new_v4(),
            user_id,
            role_id,
            assigned_at: self.assigned_at.unwrap_or(Utc::now()),
            assigned_by: self.assigned_by,
            metadata: AuditMetadata::default(),
        })
    }
}
