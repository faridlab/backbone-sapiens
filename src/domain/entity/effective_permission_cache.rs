use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for EffectivePermissionCache
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EffectivePermissionCacheId(pub Uuid);

impl EffectivePermissionCacheId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for EffectivePermissionCacheId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EffectivePermissionCacheId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for EffectivePermissionCacheId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<EffectivePermissionCacheId> for Uuid {
    fn from(id: EffectivePermissionCacheId) -> Self { id.0 }
}

impl AsRef<Uuid> for EffectivePermissionCacheId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for EffectivePermissionCacheId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EffectivePermissionCache {
    pub id: Uuid,
    pub user_id: Uuid,
    pub permission_key: String,
    pub computed_permissions: serde_json::Value,
    pub computation_details: serde_json::Value,
    pub scope: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<serde_json::Value>,
    pub usage: serde_json::Value,
    pub performance: serde_json::Value,
    pub cache_stats: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidation: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl EffectivePermissionCache {
    /// Create a builder for EffectivePermissionCache
    pub fn builder() -> EffectivePermissionCacheBuilder {
        EffectivePermissionCacheBuilder::default()
    }

    /// Create a new EffectivePermissionCache with required fields
    pub fn new(user_id: Uuid, permission_key: String, computed_permissions: serde_json::Value, computation_details: serde_json::Value, scope: serde_json::Value, usage: serde_json::Value, performance: serde_json::Value, cache_stats: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            permission_key,
            computed_permissions,
            computation_details,
            scope,
            conditions: None,
            usage,
            performance,
            cache_stats,
            invalidation: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> EffectivePermissionCacheId {
        EffectivePermissionCacheId(self.id)
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

    /// Set the conditions field (chainable)
    pub fn with_conditions(mut self, value: serde_json::Value) -> Self {
        self.conditions = Some(value);
        self
    }

    /// Set the invalidation field (chainable)
    pub fn with_invalidation(mut self, value: serde_json::Value) -> Self {
        self.invalidation = Some(value);
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
                "permission_key" => {
                    if let Ok(v) = serde_json::from_value(value) { self.permission_key = v; }
                }
                "computed_permissions" => {
                    if let Ok(v) = serde_json::from_value(value) { self.computed_permissions = v; }
                }
                "computation_details" => {
                    if let Ok(v) = serde_json::from_value(value) { self.computation_details = v; }
                }
                "scope" => {
                    if let Ok(v) = serde_json::from_value(value) { self.scope = v; }
                }
                "conditions" => {
                    if let Ok(v) = serde_json::from_value(value) { self.conditions = v; }
                }
                "usage" => {
                    if let Ok(v) = serde_json::from_value(value) { self.usage = v; }
                }
                "performance" => {
                    if let Ok(v) = serde_json::from_value(value) { self.performance = v; }
                }
                "cache_stats" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cache_stats = v; }
                }
                "invalidation" => {
                    if let Ok(v) = serde_json::from_value(value) { self.invalidation = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for EffectivePermissionCache {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "EffectivePermissionCache"
    }
}

impl backbone_core::PersistentEntity for EffectivePermissionCache {
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

impl backbone_orm::EntityRepoMeta for EffectivePermissionCache {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["permission_key"]
    }
}

/// Builder for EffectivePermissionCache entity
///
/// Provides a fluent API for constructing EffectivePermissionCache instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct EffectivePermissionCacheBuilder {
    user_id: Option<Uuid>,
    permission_key: Option<String>,
    computed_permissions: Option<serde_json::Value>,
    computation_details: Option<serde_json::Value>,
    scope: Option<serde_json::Value>,
    conditions: Option<serde_json::Value>,
    usage: Option<serde_json::Value>,
    performance: Option<serde_json::Value>,
    cache_stats: Option<serde_json::Value>,
    invalidation: Option<serde_json::Value>,
}

impl EffectivePermissionCacheBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the permission_key field (required)
    pub fn permission_key(mut self, value: String) -> Self {
        self.permission_key = Some(value);
        self
    }

    /// Set the computed_permissions field (required)
    pub fn computed_permissions(mut self, value: serde_json::Value) -> Self {
        self.computed_permissions = Some(value);
        self
    }

    /// Set the computation_details field (required)
    pub fn computation_details(mut self, value: serde_json::Value) -> Self {
        self.computation_details = Some(value);
        self
    }

    /// Set the scope field (required)
    pub fn scope(mut self, value: serde_json::Value) -> Self {
        self.scope = Some(value);
        self
    }

    /// Set the conditions field (optional)
    pub fn conditions(mut self, value: serde_json::Value) -> Self {
        self.conditions = Some(value);
        self
    }

    /// Set the usage field (required)
    pub fn usage(mut self, value: serde_json::Value) -> Self {
        self.usage = Some(value);
        self
    }

    /// Set the performance field (required)
    pub fn performance(mut self, value: serde_json::Value) -> Self {
        self.performance = Some(value);
        self
    }

    /// Set the cache_stats field (required)
    pub fn cache_stats(mut self, value: serde_json::Value) -> Self {
        self.cache_stats = Some(value);
        self
    }

    /// Set the invalidation field (optional)
    pub fn invalidation(mut self, value: serde_json::Value) -> Self {
        self.invalidation = Some(value);
        self
    }

    /// Build the EffectivePermissionCache entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<EffectivePermissionCache, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let permission_key = self.permission_key.ok_or_else(|| "permission_key is required".to_string())?;
        let computed_permissions = self.computed_permissions.ok_or_else(|| "computed_permissions is required".to_string())?;
        let computation_details = self.computation_details.ok_or_else(|| "computation_details is required".to_string())?;
        let scope = self.scope.ok_or_else(|| "scope is required".to_string())?;
        let usage = self.usage.ok_or_else(|| "usage is required".to_string())?;
        let performance = self.performance.ok_or_else(|| "performance is required".to_string())?;
        let cache_stats = self.cache_stats.ok_or_else(|| "cache_stats is required".to_string())?;

        Ok(EffectivePermissionCache {
            id: Uuid::new_v4(),
            user_id,
            permission_key,
            computed_permissions,
            computation_details,
            scope,
            conditions: self.conditions,
            usage,
            performance,
            cache_stats,
            invalidation: self.invalidation,
            metadata: AuditMetadata::default(),
        })
    }
}
