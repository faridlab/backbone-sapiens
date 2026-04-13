use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::ConflictResolutionType;
use super::RoleAssignmentStatus;
use super::AuditMetadata;

/// Strongly-typed ID for RoleAssignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RoleAssignmentId(pub Uuid);

impl RoleAssignmentId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for RoleAssignmentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for RoleAssignmentId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for RoleAssignmentId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<RoleAssignmentId> for Uuid {
    fn from(id: RoleAssignmentId) -> Self { id.0 }
}

impl AsRef<Uuid> for RoleAssignmentId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for RoleAssignmentId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RoleAssignment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_by: Uuid,
    pub assigned_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_assignment_id: Option<Uuid>,
    pub inheritance_level: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<Uuid>,
    pub conflicts_with: Vec<Uuid>,
    pub conflict_resolution: ConflictResolutionType,
    pub has_conflicts: bool,
    pub conflict_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_conflicts: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<DateTime<Utc>>,
    pub usage_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_history: Option<serde_json::Value>,
    pub status: RoleAssignmentStatus,
    pub warning_sent: bool,
    pub auto_renew: bool,
    pub renewal_period_days: i32,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl RoleAssignment {
    /// Create a builder for RoleAssignment
    pub fn builder() -> RoleAssignmentBuilder {
        RoleAssignmentBuilder::default()
    }

    /// Create a new RoleAssignment with required fields
    pub fn new(user_id: Uuid, role_id: Uuid, assigned_by: Uuid, assigned_at: DateTime<Utc>, priority: i32, inheritance_level: i32, conflicts_with: Vec<Uuid>, conflict_resolution: ConflictResolutionType, has_conflicts: bool, conflict_types: Vec<String>, usage_count: i32, status: RoleAssignmentStatus, warning_sent: bool, auto_renew: bool, renewal_period_days: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            role_id,
            assigned_by,
            assigned_at,
            expires_at: None,
            reason: None,
            priority,
            scope: None,
            conditions: None,
            parent_assignment_id: None,
            inheritance_level,
            inherited_from: None,
            conflicts_with,
            conflict_resolution,
            has_conflicts,
            conflict_types,
            detected_conflicts: None,
            last_used_at: None,
            usage_count,
            modification_history: None,
            status,
            warning_sent,
            auto_renew,
            renewal_period_days,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> RoleAssignmentId {
        RoleAssignmentId(self.id)
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
    pub fn status(&self) -> &RoleAssignmentStatus {
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

    /// Set the parent_assignment_id field (chainable)
    pub fn with_parent_assignment_id(mut self, value: Uuid) -> Self {
        self.parent_assignment_id = Some(value);
        self
    }

    /// Set the inherited_from field (chainable)
    pub fn with_inherited_from(mut self, value: Uuid) -> Self {
        self.inherited_from = Some(value);
        self
    }

    /// Set the detected_conflicts field (chainable)
    pub fn with_detected_conflicts(mut self, value: serde_json::Value) -> Self {
        self.detected_conflicts = Some(value);
        self
    }

    /// Set the last_used_at field (chainable)
    pub fn with_last_used_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_used_at = Some(value);
        self
    }

    /// Set the modification_history field (chainable)
    pub fn with_modification_history(mut self, value: serde_json::Value) -> Self {
        self.modification_history = Some(value);
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
                "assigned_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.assigned_by = v; }
                }
                "assigned_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.assigned_at = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.reason = v; }
                }
                "priority" => {
                    if let Ok(v) = serde_json::from_value(value) { self.priority = v; }
                }
                "scope" => {
                    if let Ok(v) = serde_json::from_value(value) { self.scope = v; }
                }
                "conditions" => {
                    if let Ok(v) = serde_json::from_value(value) { self.conditions = v; }
                }
                "parent_assignment_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.parent_assignment_id = v; }
                }
                "inheritance_level" => {
                    if let Ok(v) = serde_json::from_value(value) { self.inheritance_level = v; }
                }
                "inherited_from" => {
                    if let Ok(v) = serde_json::from_value(value) { self.inherited_from = v; }
                }
                "conflicts_with" => {
                    if let Ok(v) = serde_json::from_value(value) { self.conflicts_with = v; }
                }
                "conflict_resolution" => {
                    if let Ok(v) = serde_json::from_value(value) { self.conflict_resolution = v; }
                }
                "has_conflicts" => {
                    if let Ok(v) = serde_json::from_value(value) { self.has_conflicts = v; }
                }
                "conflict_types" => {
                    if let Ok(v) = serde_json::from_value(value) { self.conflict_types = v; }
                }
                "detected_conflicts" => {
                    if let Ok(v) = serde_json::from_value(value) { self.detected_conflicts = v; }
                }
                "last_used_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_used_at = v; }
                }
                "usage_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.usage_count = v; }
                }
                "modification_history" => {
                    if let Ok(v) = serde_json::from_value(value) { self.modification_history = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "warning_sent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.warning_sent = v; }
                }
                "auto_renew" => {
                    if let Ok(v) = serde_json::from_value(value) { self.auto_renew = v; }
                }
                "renewal_period_days" => {
                    if let Ok(v) = serde_json::from_value(value) { self.renewal_period_days = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for RoleAssignment {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "RoleAssignment"
    }
}

impl backbone_core::PersistentEntity for RoleAssignment {
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

impl backbone_orm::EntityRepoMeta for RoleAssignment {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("role_id".to_string(), "uuid".to_string());
        m.insert("parent_assignment_id".to_string(), "uuid".to_string());
        m.insert("conflict_resolution".to_string(), "conflict_resolution_type".to_string());
        m.insert("status".to_string(), "role_assignment_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for RoleAssignment entity
///
/// Provides a fluent API for constructing RoleAssignment instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct RoleAssignmentBuilder {
    user_id: Option<Uuid>,
    role_id: Option<Uuid>,
    assigned_by: Option<Uuid>,
    assigned_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    reason: Option<String>,
    priority: Option<i32>,
    scope: Option<serde_json::Value>,
    conditions: Option<serde_json::Value>,
    parent_assignment_id: Option<Uuid>,
    inheritance_level: Option<i32>,
    inherited_from: Option<Uuid>,
    conflicts_with: Option<Vec<Uuid>>,
    conflict_resolution: Option<ConflictResolutionType>,
    has_conflicts: Option<bool>,
    conflict_types: Option<Vec<String>>,
    detected_conflicts: Option<serde_json::Value>,
    last_used_at: Option<DateTime<Utc>>,
    usage_count: Option<i32>,
    modification_history: Option<serde_json::Value>,
    status: Option<RoleAssignmentStatus>,
    warning_sent: Option<bool>,
    auto_renew: Option<bool>,
    renewal_period_days: Option<i32>,
}

impl RoleAssignmentBuilder {
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

    /// Set the assigned_by field (required)
    pub fn assigned_by(mut self, value: Uuid) -> Self {
        self.assigned_by = Some(value);
        self
    }

    /// Set the assigned_at field (default: `Utc::now()`)
    pub fn assigned_at(mut self, value: DateTime<Utc>) -> Self {
        self.assigned_at = Some(value);
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

    /// Set the priority field (default: `50`)
    pub fn priority(mut self, value: i32) -> Self {
        self.priority = Some(value);
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

    /// Set the parent_assignment_id field (optional)
    pub fn parent_assignment_id(mut self, value: Uuid) -> Self {
        self.parent_assignment_id = Some(value);
        self
    }

    /// Set the inheritance_level field (default: `0`)
    pub fn inheritance_level(mut self, value: i32) -> Self {
        self.inheritance_level = Some(value);
        self
    }

    /// Set the inherited_from field (optional)
    pub fn inherited_from(mut self, value: Uuid) -> Self {
        self.inherited_from = Some(value);
        self
    }

    /// Set the conflicts_with field (required)
    pub fn conflicts_with(mut self, value: Vec<Uuid>) -> Self {
        self.conflicts_with = Some(value);
        self
    }

    /// Set the conflict_resolution field (default: `ConflictResolutionType::default()`)
    pub fn conflict_resolution(mut self, value: ConflictResolutionType) -> Self {
        self.conflict_resolution = Some(value);
        self
    }

    /// Set the has_conflicts field (default: `false`)
    pub fn has_conflicts(mut self, value: bool) -> Self {
        self.has_conflicts = Some(value);
        self
    }

    /// Set the conflict_types field (required)
    pub fn conflict_types(mut self, value: Vec<String>) -> Self {
        self.conflict_types = Some(value);
        self
    }

    /// Set the detected_conflicts field (optional)
    pub fn detected_conflicts(mut self, value: serde_json::Value) -> Self {
        self.detected_conflicts = Some(value);
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

    /// Set the modification_history field (optional)
    pub fn modification_history(mut self, value: serde_json::Value) -> Self {
        self.modification_history = Some(value);
        self
    }

    /// Set the status field (default: `RoleAssignmentStatus::default()`)
    pub fn status(mut self, value: RoleAssignmentStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the warning_sent field (default: `false`)
    pub fn warning_sent(mut self, value: bool) -> Self {
        self.warning_sent = Some(value);
        self
    }

    /// Set the auto_renew field (default: `false`)
    pub fn auto_renew(mut self, value: bool) -> Self {
        self.auto_renew = Some(value);
        self
    }

    /// Set the renewal_period_days field (default: `30`)
    pub fn renewal_period_days(mut self, value: i32) -> Self {
        self.renewal_period_days = Some(value);
        self
    }

    /// Build the RoleAssignment entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<RoleAssignment, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let role_id = self.role_id.ok_or_else(|| "role_id is required".to_string())?;
        let assigned_by = self.assigned_by.ok_or_else(|| "assigned_by is required".to_string())?;
        let conflicts_with = self.conflicts_with.ok_or_else(|| "conflicts_with is required".to_string())?;
        let conflict_types = self.conflict_types.ok_or_else(|| "conflict_types is required".to_string())?;

        Ok(RoleAssignment {
            id: Uuid::new_v4(),
            user_id,
            role_id,
            assigned_by,
            assigned_at: self.assigned_at.unwrap_or(Utc::now()),
            expires_at: self.expires_at,
            reason: self.reason,
            priority: self.priority.unwrap_or(50),
            scope: self.scope,
            conditions: self.conditions,
            parent_assignment_id: self.parent_assignment_id,
            inheritance_level: self.inheritance_level.unwrap_or(0),
            inherited_from: self.inherited_from,
            conflicts_with,
            conflict_resolution: self.conflict_resolution.unwrap_or(ConflictResolutionType::default()),
            has_conflicts: self.has_conflicts.unwrap_or(false),
            conflict_types,
            detected_conflicts: self.detected_conflicts,
            last_used_at: self.last_used_at,
            usage_count: self.usage_count.unwrap_or(0),
            modification_history: self.modification_history,
            status: self.status.unwrap_or(RoleAssignmentStatus::default()),
            warning_sent: self.warning_sent.unwrap_or(false),
            auto_renew: self.auto_renew.unwrap_or(false),
            renewal_period_days: self.renewal_period_days.unwrap_or(30),
            metadata: AuditMetadata::default(),
        })
    }
}
