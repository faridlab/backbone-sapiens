//! Authorization Domain Types
//!
//! Custom types for authorization and RBAC/ABAC functionality.
//! These types are used by the authorization service.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// AUTHORIZATION SERVICE TRAIT
// ============================================================================

/// Authorization service trait
#[async_trait]
pub trait AuthorizationService: Send + Sync {
    /// Check if a user has a specific permission
    async fn check_permission(
        &self,
        user_id: Uuid,
        resource: String,
        action: String,
    ) -> Result<bool, AuthorizationError>;

    /// Check if a user has a specific permission with context
    async fn check_permission_with_context(
        &self,
        user_id: Uuid,
        permission: String,
        context: &PermissionContext,
    ) -> Result<AuthorizationResult, AuthorizationError> {
        // Default implementation - convert context to resource and call check_permission
        let resource = context.resource.clone();
        let action = permission;
        self.check_permission(user_id, resource, action).await.map(|authorized| {
            AuthorizationResult {
                authorized,
                reason: None,
            }
        })
    }

    /// Get all permissions for a user
    async fn get_user_permissions(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<PermissionInfo>, AuthorizationError>;

    /// Assign a role to a user
    async fn assign_role(
        &self,
        request: RoleAssignmentRequest,
    ) -> Result<RoleAssignmentResult, AuthorizationError>;

    /// Remove a role from a user
    async fn remove_role(
        &self,
        user_id: Uuid,
        role_id: Uuid,
        reason: String,
    ) -> Result<RoleRemovalResult, AuthorizationError>;

    /// Grant a direct permission to a user
    async fn grant_permission(
        &self,
        request: DirectPermissionGrantRequest,
    ) -> Result<DirectPermissionGrantResult, AuthorizationError>;

    /// Grant a direct permission to a user (alias for grant_permission)
    async fn grant_direct_permission(
        &self,
        request: DirectPermissionGrantRequest,
    ) -> Result<DirectPermissionGrantResult, AuthorizationError> {
        self.grant_permission(request).await
    }

    /// Revoke a direct permission from a user
    async fn revoke_permission(
        &self,
        user_id: Uuid,
        permission_id: Uuid,
        reason: String,
    ) -> Result<PermissionRevocationResult, AuthorizationError>;

    /// Revoke a direct permission from a user (alias for revoke_permission)
    async fn revoke_direct_permission(
        &self,
        user_id: Uuid,
        permission_id: Uuid,
        reason: String,
    ) -> Result<PermissionRevocationResult, AuthorizationError> {
        self.revoke_permission(user_id, permission_id, reason).await
    }

    /// Get effective permissions for a user (including role-based)
    async fn get_effective_permissions(
        &self,
        user_id: Uuid,
    ) -> Result<EffectivePermissions, AuthorizationError>;

    /// Calculate effective permissions for a user (alias for get_effective_permissions)
    async fn calculate_effective_permissions(
        &self,
        user_id: Uuid,
    ) -> Result<EffectivePermissions, AuthorizationError> {
        self.get_effective_permissions(user_id).await
    }

    /// Get role hierarchy
    async fn get_role_hierarchy(&self) -> Result<RoleHierarchy, AuthorizationError>;
}

// ============================================================================
// DOMAIN TYPES
// ============================================================================

/// Authorization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResult {
    pub authorized: bool,
    pub reason: Option<String>,
}

/// Permission source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionSource {
    Role { role_id: Uuid, role_name: String },
    Direct { granted_at: DateTime<Utc>, granted_by: Uuid },
    Temporary { expires_at: DateTime<Utc> },
}

/// Permission source type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionSourceType {
    Role,
    Direct,
    DirectGrant,
    Temporary,
    InheritedRole,
    OrganizationRole,
}

/// Permission conditions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionConditions {
    pub time_restrictions: Option<TimeRestrictions>,
    pub location_restrictions: Option<LocationRestrictions>,
    pub ip_restrictions: Option<Vec<String>>,
    pub custom_attributes: HashMap<String, String>,
}

impl Default for TimeRestrictions {
    fn default() -> Self {
        Self {
            business_hours_only: None,
            allowed_days: None,
            time_window: None,
        }
    }
}

/// Time restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    pub business_hours_only: Option<bool>,
    pub allowed_days: Option<Vec<u8>>,  // 0-6 (Sunday-Saturday)
    pub time_window: Option<TimeWindow>,
}

/// Time window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    pub start_hour: Option<u8>,
    pub end_hour: Option<u8>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

/// Location restrictions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocationRestrictions {
    pub allowed_countries: Option<Vec<String>>,
    pub allowed_regions: Option<Vec<String>>,
    pub ip_ranges: Option<Vec<String>>,
}

/// Permission context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionContext {
    pub user_id: Uuid,
    pub resource: String,
    pub action: String,
    pub resource_id: Option<String>,
    pub resource_type: Option<String>,
    pub organization_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    pub attributes: HashMap<String, String>,
}

/// Effective permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectivePermissions {
    pub user_id: Uuid,
    pub permissions: Vec<PermissionInfo>,
    pub roles: Vec<RoleInfo>,
    pub last_updated: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub computed_at: DateTime<Utc>,
    pub sources_summary: std::collections::HashMap<String, usize>,
    pub conflicts: Vec<RoleConflict>,
}

/// Permission info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionInfo {
    pub id: Uuid,
    pub name: String,
    pub resource: String,
    pub action: String,
    pub source: PermissionSource,
    pub conditions: Option<PermissionConditions>,
}

/// Role info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleInfo {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<PermissionInfo>,
    // Additional fields for hierarchy
    pub role_id: Option<Uuid>,
    pub role_name: Option<String>,
    pub parent_ids: Vec<Uuid>,
    pub child_ids: Vec<Uuid>,
    pub path: Option<String>,
    pub depth: Option<i32>,
}

/// Role assignment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssignmentRequest {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_by: Uuid,
    pub reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub priority: Option<i32>,
    pub scope: Option<PermissionScope>,
    pub conditions: Option<PermissionConditions>,
}

/// Role assignment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssignmentResult {
    pub success: bool,
    pub assignment_id: Uuid,
    pub conflicts_detected: Vec<RoleConflict>,
    pub conflicts_resolved: Vec<RoleConflict>,
    pub warnings: Vec<String>,
    pub requires_approval: bool,
    pub message: String,
}

/// Role conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleConflict {
    pub conflict_id: Uuid,
    pub conflict_type: RoleConflictType,
    pub severity: ConflictSeverity,
    pub description: String,
    pub resolution_status: ConflictResolutionStatus,
    pub conflicting_assignments: Vec<RoleAssignmentRequest>,
    pub suggested_resolution: Option<String>,
}

/// Role conflict type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoleConflictType {
    OverlappingPermissions,
    MutuallyExclusiveRoles,
    SeparationOfDutiesViolation,
    RoleLimitExceeded,
    HierarchyViolation,
}

/// Conflict severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Conflict resolution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolutionStatus {
    Pending,
    AutoResolved,
    ManualReviewRequired,
    Resolved,
    Ignored,
}

/// Role removal result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleRemovalResult {
    pub success: bool,
    pub message: String,
    pub removed_assignment_id: Uuid,
    pub cascading_removals: Vec<Uuid>,
    pub affected_permissions: Vec<Uuid>,
    pub warnings: Vec<String>,
}

/// Direct permission grant request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectPermissionGrantRequest {
    pub user_id: Uuid,
    pub permission_id: Uuid,
    pub granted_by: Uuid,
    pub reason: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub scope: Option<PermissionScope>,
    pub conditions: Option<PermissionConditions>,
}

/// Direct permission grant result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectPermissionGrantResult {
    pub success: bool,
    pub grant_id: Uuid,
    pub message: String,
    pub conflicts_detected: Vec<RoleConflict>,
    pub warnings: Vec<String>,
}

/// Permission revocation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRevocationResult {
    pub success: bool,
    pub message: String,
    pub revoked_grant_id: Option<Uuid>,
    pub affected_permissions: Vec<Uuid>,
    pub warnings: Vec<String>,
}

/// Permission scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionScope {
    pub organization_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub resource_type: Option<String>,
    pub resource_ids: Option<Vec<String>>,
}

/// Role hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleHierarchy {
    pub nodes: Vec<RoleHierarchyNode>,
    pub relationships: Vec<RoleRelationship>,
    pub roles: Vec<RoleInfo>,
    pub inheritance_map: HashMap<Uuid, Vec<Uuid>>,
    pub conflict_rules: Vec<String>,
}

/// Role hierarchy node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleHierarchyNode {
    pub role_id: Uuid,
    pub role_name: String,
    pub level: i32,
    pub parent_role_ids: Vec<Uuid>,
    pub child_role_ids: Vec<Uuid>,
}

/// Role relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleRelationship {
    pub parent_role_id: Uuid,
    pub child_role_id: Uuid,
    pub relationship_type: String,
}

/// Authorization error
#[derive(Debug, thiserror::Error)]
pub enum AuthorizationError {
    #[error("User not found: {0}")]
    UserNotFound(Uuid),

    #[error("Role not found: {0}")]
    RoleNotFound(Uuid),

    #[error("Permission not found: {0}")]
    PermissionNotFound(Uuid),

    #[error("Access denied: {0}")]
    AccessDenied(String),

    #[error("Invalid assignment: {0}")]
    InvalidAssignment(String),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
