use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::DirectPermissionGrantStatus;
use super::AuditMetadata;

/// Strongly-typed ID for DirectPermissionGrant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DirectPermissionGrantId(pub Uuid);

impl DirectPermissionGrantId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for DirectPermissionGrantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for DirectPermissionGrantId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for DirectPermissionGrantId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<DirectPermissionGrantId> for Uuid {
    fn from(id: DirectPermissionGrantId) -> Self { id.0 }
}

impl AsRef<Uuid> for DirectPermissionGrantId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for DirectPermissionGrantId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DirectPermissionGrant {
    pub id: Uuid,
    pub user_id: Uuid,
    pub permission_id: Uuid,
    pub granted_by: Uuid,
    pub granted_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<DateTime<Utc>>,
    pub usage_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log: Option<serde_json::Value>,
    pub status: DirectPermissionGrantStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_by: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl DirectPermissionGrant {
    /// Create a builder for DirectPermissionGrant
    pub fn builder() -> DirectPermissionGrantBuilder {
        DirectPermissionGrantBuilder::default()
    }

    /// Create a new DirectPermissionGrant with required fields
    pub fn new(user_id: Uuid, permission_id: Uuid, granted_by: Uuid, granted_at: DateTime<Utc>, usage_count: i32, status: DirectPermissionGrantStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            permission_id,
            granted_by,
            granted_at,
            expires_at: None,
            reason: None,
            scope: None,
            conditions: None,
            last_used_at: None,
            usage_count,
            access_log: None,
            status,
            revoked_at: None,
            revoked_by: None,
            revocation_reason: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> DirectPermissionGrantId {
        DirectPermissionGrantId(self.id)
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
    pub fn status(&self) -> &DirectPermissionGrantStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

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

    /// Set the scope field (chainable)
    pub fn with_scope(mut self, value: serde_json::Value) -> Self {
        self.scope = Some(value);
        self
    }

    /// Set the conditions field (chainable)
    pub fn with_conditions(mut self, value: serde_json::Value) -> Self {
        self.conditions = Some(value);
        self
    }

    /// Set the last_used_at field (chainable)
    pub fn with_last_used_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_used_at = Some(value);
        self
    }

    /// Set the access_log field (chainable)
    pub fn with_access_log(mut self, value: serde_json::Value) -> Self {
        self.access_log = Some(value);
        self
    }

    /// Set the revoked_at field (chainable)
    pub fn with_revoked_at(mut self, value: DateTime<Utc>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    /// Set the revoked_by field (chainable)
    pub fn with_revoked_by(mut self, value: Uuid) -> Self {
        self.revoked_by = Some(value);
        self
    }

    /// Set the revocation_reason field (chainable)
    pub fn with_revocation_reason(mut self, value: String) -> Self {
        self.revocation_reason = Some(value);
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
                "permission_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.permission_id = v; }
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
                "scope" => {
                    if let Ok(v) = serde_json::from_value(value) { self.scope = v; }
                }
                "conditions" => {
                    if let Ok(v) = serde_json::from_value(value) { self.conditions = v; }
                }
                "last_used_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_used_at = v; }
                }
                "usage_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.usage_count = v; }
                }
                "access_log" => {
                    if let Ok(v) = serde_json::from_value(value) { self.access_log = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "revoked_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.revoked_at = v; }
                }
                "revoked_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.revoked_by = v; }
                }
                "revocation_reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.revocation_reason = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for DirectPermissionGrant {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "DirectPermissionGrant"
    }
}

impl backbone_core::PersistentEntity for DirectPermissionGrant {
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

impl backbone_orm::EntityRepoMeta for DirectPermissionGrant {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("permission_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "direct_permission_grant_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for DirectPermissionGrant entity
///
/// Provides a fluent API for constructing DirectPermissionGrant instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct DirectPermissionGrantBuilder {
    user_id: Option<Uuid>,
    permission_id: Option<Uuid>,
    granted_by: Option<Uuid>,
    granted_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    reason: Option<String>,
    scope: Option<serde_json::Value>,
    conditions: Option<serde_json::Value>,
    last_used_at: Option<DateTime<Utc>>,
    usage_count: Option<i32>,
    access_log: Option<serde_json::Value>,
    status: Option<DirectPermissionGrantStatus>,
    revoked_at: Option<DateTime<Utc>>,
    revoked_by: Option<Uuid>,
    revocation_reason: Option<String>,
}

impl DirectPermissionGrantBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the permission_id field (required)
    pub fn permission_id(mut self, value: Uuid) -> Self {
        self.permission_id = Some(value);
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

    /// Set the scope field (optional)
    pub fn scope(mut self, value: serde_json::Value) -> Self {
        self.scope = Some(value);
        self
    }

    /// Set the conditions field (optional)
    pub fn conditions(mut self, value: serde_json::Value) -> Self {
        self.conditions = Some(value);
        self
    }

    /// Set the last_used_at field (optional)
    pub fn last_used_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_used_at = Some(value);
        self
    }

    /// Set the usage_count field (default: `0`)
    pub fn usage_count(mut self, value: i32) -> Self {
        self.usage_count = Some(value);
        self
    }

    /// Set the access_log field (optional)
    pub fn access_log(mut self, value: serde_json::Value) -> Self {
        self.access_log = Some(value);
        self
    }

    /// Set the status field (default: `DirectPermissionGrantStatus::default()`)
    pub fn status(mut self, value: DirectPermissionGrantStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the revoked_at field (optional)
    pub fn revoked_at(mut self, value: DateTime<Utc>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    /// Set the revoked_by field (optional)
    pub fn revoked_by(mut self, value: Uuid) -> Self {
        self.revoked_by = Some(value);
        self
    }

    /// Set the revocation_reason field (optional)
    pub fn revocation_reason(mut self, value: String) -> Self {
        self.revocation_reason = Some(value);
        self
    }

    /// Build the DirectPermissionGrant entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<DirectPermissionGrant, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let permission_id = self.permission_id.ok_or_else(|| "permission_id is required".to_string())?;
        let granted_by = self.granted_by.ok_or_else(|| "granted_by is required".to_string())?;

        Ok(DirectPermissionGrant {
            id: Uuid::new_v4(),
            user_id,
            permission_id,
            granted_by,
            granted_at: self.granted_at.unwrap_or(Utc::now()),
            expires_at: self.expires_at,
            reason: self.reason,
            scope: self.scope,
            conditions: self.conditions,
            last_used_at: self.last_used_at,
            usage_count: self.usage_count.unwrap_or(0),
            access_log: self.access_log,
            status: self.status.unwrap_or(DirectPermissionGrantStatus::default()),
            revoked_at: self.revoked_at,
            revoked_by: self.revoked_by,
            revocation_reason: self.revocation_reason,
            metadata: AuditMetadata::default(),
        })
    }
}
