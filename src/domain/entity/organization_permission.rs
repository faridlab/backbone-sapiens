use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for OrganizationPermission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrganizationPermissionId(pub Uuid);

impl OrganizationPermissionId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for OrganizationPermissionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for OrganizationPermissionId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for OrganizationPermissionId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<OrganizationPermissionId> for Uuid {
    fn from(id: OrganizationPermissionId) -> Self { id.0 }
}

impl AsRef<Uuid> for OrganizationPermissionId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for OrganizationPermissionId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationPermission {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub permission_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<Uuid>,
    pub granted_by: Uuid,
    pub granted_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub is_active: bool,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl OrganizationPermission {
    /// Create a builder for OrganizationPermission
    pub fn builder() -> OrganizationPermissionBuilder {
        OrganizationPermissionBuilder::default()
    }

    /// Create a new OrganizationPermission with required fields
    pub fn new(organization_id: Uuid, permission_id: Uuid, granted_by: Uuid, granted_at: DateTime<Utc>, is_active: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            organization_id,
            permission_id,
            permission_name: None,
            resource_id: None,
            resource_type: None,
            role_id: None,
            granted_by,
            granted_at,
            expires_at: None,
            reason: None,
            is_active,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> OrganizationPermissionId {
        OrganizationPermissionId(self.id)
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

    /// Set the permission_name field (chainable)
    pub fn with_permission_name(mut self, value: String) -> Self {
        self.permission_name = Some(value);
        self
    }

    /// Set the resource_id field (chainable)
    pub fn with_resource_id(mut self, value: Uuid) -> Self {
        self.resource_id = Some(value);
        self
    }

    /// Set the resource_type field (chainable)
    pub fn with_resource_type(mut self, value: String) -> Self {
        self.resource_type = Some(value);
        self
    }

    /// Set the role_id field (chainable)
    pub fn with_role_id(mut self, value: Uuid) -> Self {
        self.role_id = Some(value);
        self
    }

    /// Set the expires_at field (chainable)
    pub fn with_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the reason field (chainable)
    pub fn with_reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "organization_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.organization_id = v; }
                }
                "permission_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.permission_id = v; }
                }
                "permission_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.permission_name = v; }
                }
                "resource_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.resource_id = v; }
                }
                "resource_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.resource_type = v; }
                }
                "role_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.role_id = v; }
                }
                "granted_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.granted_by = v; }
                }
                "granted_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.granted_at = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.reason = v; }
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

impl super::Entity for OrganizationPermission {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "OrganizationPermission"
    }
}

impl backbone_core::PersistentEntity for OrganizationPermission {
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

impl backbone_orm::EntityRepoMeta for OrganizationPermission {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("organization_id".to_string(), "uuid".to_string());
        m.insert("permission_id".to_string(), "uuid".to_string());
        m.insert("resource_id".to_string(), "uuid".to_string());
        m.insert("role_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for OrganizationPermission entity
///
/// Provides a fluent API for constructing OrganizationPermission instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct OrganizationPermissionBuilder {
    organization_id: Option<Uuid>,
    permission_id: Option<Uuid>,
    permission_name: Option<String>,
    resource_id: Option<Uuid>,
    resource_type: Option<String>,
    role_id: Option<Uuid>,
    granted_by: Option<Uuid>,
    granted_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    reason: Option<String>,
    is_active: Option<bool>,
}

impl OrganizationPermissionBuilder {
    /// Set the organization_id field (required)
    pub fn organization_id(mut self, value: Uuid) -> Self {
        self.organization_id = Some(value);
        self
    }

    /// Set the permission_id field (required)
    pub fn permission_id(mut self, value: Uuid) -> Self {
        self.permission_id = Some(value);
        self
    }

    /// Set the permission_name field (optional)
    pub fn permission_name(mut self, value: String) -> Self {
        self.permission_name = Some(value);
        self
    }

    /// Set the resource_id field (optional)
    pub fn resource_id(mut self, value: Uuid) -> Self {
        self.resource_id = Some(value);
        self
    }

    /// Set the resource_type field (optional)
    pub fn resource_type(mut self, value: String) -> Self {
        self.resource_type = Some(value);
        self
    }

    /// Set the role_id field (optional)
    pub fn role_id(mut self, value: Uuid) -> Self {
        self.role_id = Some(value);
        self
    }

    /// Set the granted_by field (required)
    pub fn granted_by(mut self, value: Uuid) -> Self {
        self.granted_by = Some(value);
        self
    }

    /// Set the granted_at field (default: `Utc::now()`)
    pub fn granted_at(mut self, value: DateTime<Utc>) -> Self {
        self.granted_at = Some(value);
        self
    }

    /// Set the expires_at field (optional)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the reason field (optional)
    pub fn reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }

    /// Set the is_active field (default: `true`)
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Build the OrganizationPermission entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<OrganizationPermission, String> {
        let organization_id = self.organization_id.ok_or_else(|| "organization_id is required".to_string())?;
        let permission_id = self.permission_id.ok_or_else(|| "permission_id is required".to_string())?;
        let granted_by = self.granted_by.ok_or_else(|| "granted_by is required".to_string())?;

        Ok(OrganizationPermission {
            id: Uuid::new_v4(),
            organization_id,
            permission_id,
            permission_name: self.permission_name,
            resource_id: self.resource_id,
            resource_type: self.resource_type,
            role_id: self.role_id,
            granted_by,
            granted_at: self.granted_at.unwrap_or(Utc::now()),
            expires_at: self.expires_at,
            reason: self.reason,
            is_active: self.is_active.unwrap_or(true),
            metadata: AuditMetadata::default(),
        })
    }
}
