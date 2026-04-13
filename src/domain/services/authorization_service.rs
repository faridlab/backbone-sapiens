use crate::domain::entity::{User, Role, Permission, DirectPermissionGrant, RoleAssignment, EffectivePermissionCache, PermissionConflict};
// Re-export PermissionContext for use in authorization
pub use crate::domain::value_objects::PermissionContext;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration, Timelike, Datelike};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

/// Advanced authorization service with role hierarchy, conflict detection, and permission calculation
#[async_trait::async_trait]
pub trait AuthorizationService: Send + Sync {
    /// Check if user has specific permission with context
    async fn check_permission(&self, user_id: Uuid, permission_key: &str, context: &PermissionContext) -> Result<AuthorizationResult>;

    /// Calculate all effective permissions for a user
    async fn calculate_effective_permissions(&self, user_id: Uuid, scope: Option<&PermissionScope>) -> Result<EffectivePermissions>;

    /// Assign role to user with conflict detection
    async fn assign_role(&self, request: RoleAssignmentRequest) -> Result<RoleAssignmentResult>;

    /// Remove role assignment with conflict resolution
    async fn remove_role(&self, user_id: Uuid, role_id: Uuid, removal_reason: &str) -> Result<RoleRemovalResult>;

    /// Grant direct permission to user
    async fn grant_direct_permission(&self, request: DirectPermissionGrantRequest) -> Result<DirectPermissionGrantResult>;

    /// Revoke direct permission from user
    async fn revoke_direct_permission(&self, user_id: Uuid, permission_id: Uuid, revocation_reason: &str) -> Result<PermissionRevocationResult>;

    /// Detect and resolve role conflicts
    async fn detect_role_conflicts(&self, user_id: Uuid) -> Result<Vec<RoleConflict>>;

    /// Get role hierarchy tree
    async fn get_role_hierarchy(&self) -> Result<RoleHierarchy>;

    /// Invalidate permission cache for user
    async fn invalidate_user_cache(&self, user_id: Uuid, reason: &str) -> Result<()>;
}

pub struct AuthorizationServiceImpl {
    user_repository: Arc<dyn UserRepository>,
    role_repository: Arc<dyn RoleRepository>,
    permission_repository: Arc<dyn PermissionRepository>,
    direct_permission_grant_repository: Arc<dyn DirectPermissionGrantRepository>,
    role_assignment_repository: Arc<dyn RoleAssignmentRepository>,
    effective_permission_cache_repository: Arc<dyn EffectivePermissionCacheRepository>,
    conflict_detector: Arc<dyn ConflictDetector>,
    permission_calculator: Arc<dyn PermissionCalculator>,
    cache_manager: Arc<dyn PermissionCacheManager>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResult {
    pub granted: bool,
    pub permission_key: String,
    pub sources: Vec<PermissionSource>,
    pub conditions_met: bool,
    pub cache_hit: bool,
    pub response_time_ms: u64,
    pub denial_reasons: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSource {
    pub source_type: PermissionSourceType,
    pub source_id: Uuid,
    pub source_name: String,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub priority: i32,
    pub conditions: Option<PermissionConditions>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionSourceType {
    Role,
    DirectGrant,
    InheritedRole,
    OrganizationRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConditions {
    pub time_restrictions: Option<TimeRestrictions>,
    pub ip_restrictions: Option<Vec<String>>,
    pub location_restrictions: Option<LocationRestrictions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    pub business_hours_only: bool,
    pub allowed_days: Option<Vec<String>>,
    pub time_window: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationRestrictions {
    pub allowed_countries: Option<Vec<String>>,
    pub allowed_regions: Option<Vec<String>>,
}

// PermissionContext is imported from value_objects

#[derive(Debug, Clone)]
pub struct PermissionScope {
    pub organization_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub resource_type: Option<String>,
    pub resource_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone)]
pub struct EffectivePermissions {
    pub user_id: Uuid,
    pub permissions: HashMap<String, PermissionInfo>,
    pub computed_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub sources_summary: HashMap<PermissionSourceType, usize>,
    pub conflicts: Vec<RoleConflict>,
}

#[derive(Debug, Clone)]
pub struct PermissionInfo {
    pub granted: bool,
    pub sources: Vec<PermissionSource>,
    pub effective_priority: i32,
    pub conditions: Option<PermissionConditions>,
    pub last_used: Option<DateTime<Utc>>,
    pub usage_count: u32,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct RoleAssignmentResult {
    pub success: bool,
    pub assignment_id: Uuid,
    pub conflicts_detected: Vec<RoleConflict>,
    pub conflicts_resolved: Vec<RoleConflict>,
    pub warnings: Vec<String>,
    pub requires_approval: bool,
}

#[derive(Debug, Clone)]
pub struct RoleConflict {
    pub conflict_id: Uuid,
    pub conflict_type: RoleConflictType,
    pub severity: ConflictSeverity,
    pub description: String,
    pub conflicting_assignments: Vec<Uuid>,
    pub resolution_status: ConflictResolutionStatus,
    pub suggested_resolution: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RoleConflictType {
    MutuallyExclusiveRoles,
    PermissionOverlap,
    ScopeConflict,
    TimeConflict,
    PolicyViolation,
    HierarchicalConflict,
}

#[derive(Debug, Clone)]
pub enum ConflictSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub enum ConflictResolutionStatus {
    Pending,
    Resolved,
    Escalated,
    Ignored,
    RequiresManualIntervention,
}

#[derive(Debug, Clone)]
pub struct RoleRemovalResult {
    pub success: bool,
    pub removed_assignment_id: Option<Uuid>,
    pub cascading_removals: Vec<Uuid>,
    pub affected_permissions: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DirectPermissionGrantRequest {
    pub user_id: Uuid,
    pub permission_id: Uuid,
    pub granted_by: Uuid,
    pub reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub scope: Option<PermissionScope>,
    pub conditions: Option<PermissionConditions>,
}

#[derive(Debug, Clone)]
pub struct DirectPermissionGrantResult {
    pub success: bool,
    pub grant_id: Uuid,
    pub conflicts_detected: Vec<PermissionConflict>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PermissionRevocationResult {
    pub success: bool,
    pub revoked_grant_id: Option<Uuid>,
    pub affected_permissions: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RoleHierarchy {
    pub roles: Vec<RoleHierarchyNode>,
    pub inheritance_map: HashMap<Uuid, Vec<Uuid>>,
    pub conflict_rules: Vec<ConflictRule>,
}

#[derive(Debug, Clone)]
pub struct RoleHierarchyNode {
    pub role_id: Uuid,
    pub role_name: String,
    pub description: Option<String>,
    pub parent_ids: Vec<Uuid>,
    pub child_ids: Vec<Uuid>,
    pub depth: i32,
    pub path: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct ConflictRule {
    pub rule_id: Uuid,
    pub rule_type: RoleConflictType,
    pub roles_involved: Vec<Uuid>,
    pub conditions: Value,
    pub severity: ConflictSeverity,
    pub auto_resolve: bool,
    pub resolution_strategy: Option<String>,
}

#[async_trait::async_trait]
impl AuthorizationService for AuthorizationServiceImpl {
    async fn check_permission(&self, user_id: Uuid, permission_key: &str, context: &PermissionContext) -> Result<AuthorizationResult> {
        let start_time = std::time::Instant::now();

        // Try cache first
        if let Ok(cached_result) = self.check_permission_cache(user_id, permission_key, context).await {
            let response_time = start_time.elapsed().as_millis() as u64;
            return Ok(AuthorizationResult {
                granted: cached_result.granted,
                permission_key: permission_key.to_string(),
                sources: cached_result.sources,
                conditions_met: cached_result.conditions_met,
                cache_hit: true,
                response_time_ms: response_time,
                denial_reasons: cached_result.denial_reasons,
                warnings: cached_result.warnings,
            });
        }

        // Calculate effective permissions if not in cache
        let effective_permissions = self.calculate_effective_permissions(user_id, None).await?;

        let permission_info = effective_permissions.permissions.get(permission_key);

        let (granted, sources, conditions_met, denial_reasons) = if let Some(info) = permission_info {
            // Check if conditions are met
            let conditions_met = self.check_conditions(&info.conditions, context).await?;

            if info.granted && conditions_met {
                (true, info.sources.clone(), true, vec![])
            } else {
                let mut reasons = vec![];
                if !info.granted {
                    reasons.push("Permission not granted".to_string());
                }
                if !conditions_met {
                    reasons.push("Permission conditions not met".to_string());
                }
                (false, info.sources.clone(), false, reasons)
            }
        } else {
            (false, vec![], true, vec!["Permission not found".to_string()])
        };

        // Update cache
        self.cache_permission_result(user_id, permission_key, &AuthorizationResult {
            granted,
            permission_key: permission_key.to_string(),
            sources: sources.clone(),
            conditions_met,
            cache_hit: false,
            response_time_ms: 0,
            denial_reasons: denial_reasons.clone(),
            warnings: vec![],
        }).await?;

        let response_time = start_time.elapsed().as_millis() as u64;

        Ok(AuthorizationResult {
            granted,
            permission_key: permission_key.to_string(),
            sources,
            conditions_met,
            cache_hit: false,
            response_time_ms: response_time,
            denial_reasons,
            warnings: vec![],
        })
    }

    async fn calculate_effective_permissions(&self, user_id: Uuid, _scope: Option<&PermissionScope>) -> Result<EffectivePermissions> {
        let role_assignments = self.role_assignment_repository
            .find_active_by_user(&user_id).await?;
        let direct_grants = self.direct_permission_grant_repository
            .find_active_by_user(&user_id).await?;

        let mut permissions = HashMap::new();
        self.permission_calculator.collect_role_permissions(&role_assignments, &mut permissions).await?;
        self.permission_calculator.collect_direct_grant_permissions(&direct_grants, &mut permissions).await?;

        let conflicts = self.conflict_detector.detect_permission_conflicts(user_id, &permissions).await?;
        let resolved_permissions = self.conflict_detector.resolve_conflicts(&permissions, &conflicts).await?;

        let mut sources_summary = HashMap::new();
        sources_summary.insert(PermissionSourceType::Role, role_assignments.len());
        sources_summary.insert(PermissionSourceType::DirectGrant, direct_grants.len());

        let computed_at = Utc::now();

        Ok(EffectivePermissions {
            user_id,
            permissions: resolved_permissions,
            computed_at,
            expires_at: computed_at + Duration::minutes(30),
            sources_summary,
            conflicts,
        })
    }

    async fn assign_role(&self, request: RoleAssignmentRequest) -> Result<RoleAssignmentResult> {
        // Validate user and role exist
        self.user_repository.find_by_id(&request.user_id).await?
            .ok_or_else(|| anyhow!("User not found: {}", request.user_id))?;

        let role = self.role_repository.find_by_id(&request.role_id).await?
            .ok_or_else(|| anyhow!("Role not found: {}", request.role_id))?;

        // Check if assigner has permission
        if !self.can_assign_role(request.assigned_by, &role).await? {
            return Ok(RoleAssignmentResult {
                success: false,
                assignment_id: Uuid::new_v4(),
                conflicts_detected: vec![],
                conflicts_resolved: vec![],
                warnings: vec!["Insufficient permissions to assign this role".to_string()],
                requires_approval: false,
            });
        }

        // Detect conflicts before assignment
        let existing_assignments = self.role_assignment_repository
            .find_active_by_user(&request.user_id).await?;

        let conflicts = self.conflict_detector.detect_assignment_conflicts(
            &request.role_id,
            &existing_assignments,
            &request.scope,
        ).await?;

        let requires_approval = conflicts.iter().any(|c| matches!(c.severity, ConflictSeverity::High | ConflictSeverity::Critical));

        // Create role assignment using the proper entity constructor
        let mut role_assignment = crate::domain::entity::RoleAssignment::new(
            request.user_id,
            request.role_id,
            request.assigned_by,
        );

        // Set context if provided
        if let Some(reason) = request.reason {
            role_assignment = role_assignment.with_context(reason);
        }

        // Set expiration if provided
        if let Some(expires_at) = request.expires_at {
            role_assignment = role_assignment.with_expiration(expires_at);
        }

        // Convert conditions to JSON for storage in metadata
        let conditions_json = request.conditions.as_ref().map(Self::conditions_to_json);

        // Update role assignment with conditions
        role_assignment.conditions = conditions_json;

        // Save assignment
        let saved_assignment = self.role_assignment_repository.save(&role_assignment).await?;

        // Invalidate user's permission cache
        self.invalidate_user_cache(request.user_id, "Role assignment changed").await?;

        // Auto-resolve conflicts if possible
        let resolved_conflicts = if !conflicts.is_empty() {
            self.conflict_detector.auto_resolve_conflicts(&saved_assignment, &conflicts).await?
        } else {
            vec![]
        };

        Ok(RoleAssignmentResult {
            success: true,
            assignment_id: saved_assignment.id,
            conflicts_detected: conflicts,
            conflicts_resolved: resolved_conflicts,
            warnings: vec![],
            requires_approval,
        })
    }

    async fn remove_role(&self, user_id: Uuid, role_id: Uuid, removal_reason: &str) -> Result<RoleRemovalResult> {
        // Find active assignment
        let assignment = self.role_assignment_repository
            .find_active_by_user_and_role(&user_id, &role_id).await?;

        if let Some(mut assignment) = assignment {
            // Check if we can remove this assignment
            if !self.can_remove_role_assignment(&assignment).await? {
                return Ok(RoleRemovalResult {
                    success: false,
                    removed_assignment_id: None,
                    cascading_removals: vec![],
                    affected_permissions: vec![],
                    warnings: vec!["Cannot remove this role assignment - protected assignment".to_string()],
                });
            }

            // Find cascading child assignments inherited from this assignment
            let cascading_removals = self.role_assignment_repository
                .find_child_assignments(&assignment.id).await?;

            // Get affected permissions before removal
            let effective_permissions = self.calculate_effective_permissions(user_id, None).await?;
            let affected_permissions = effective_permissions.permissions
                .iter()
                .filter(|(_key, info)| info.sources.iter().any(|s|
                    s.source_id == assignment.role_id &&
                    matches!(s.source_type, PermissionSourceType::Role | PermissionSourceType::InheritedRole)
                ))
                .map(|(key, _)| key.clone())
                .collect();

            // Soft delete the assignment via repository for proper audit trail
            self.role_assignment_repository.delete(&assignment.id).await?;

            // Remove cascading child assignments
            for child_id in &cascading_removals {
                self.role_assignment_repository.delete(child_id).await?;
            }

            // Invalidate user's permission cache
            self.invalidate_user_cache(user_id, "Role assignment removed").await?;

            Ok(RoleRemovalResult {
                success: true,
                removed_assignment_id: Some(assignment.id),
                cascading_removals,
                affected_permissions,
                warnings: vec![],
            })
        } else {
            Ok(RoleRemovalResult {
                success: false,
                removed_assignment_id: None,
                cascading_removals: vec![],
                affected_permissions: vec![],
                warnings: vec!["Role assignment not found".to_string()],
            })
        }
    }

    async fn grant_direct_permission(&self, request: DirectPermissionGrantRequest) -> Result<DirectPermissionGrantResult> {
        // Validate user and permission exist
        self.user_repository.find_by_id(&request.user_id).await?
            .ok_or_else(|| anyhow!("User not found: {}", request.user_id))?;

        let permission = self.permission_repository.find_by_id(&request.permission_id).await?
            .ok_or_else(|| anyhow!("Permission not found: {}", request.permission_id))?;

        // Check if granter has permission
        if !self.can_grant_permission(request.granted_by, &permission).await? {
            return Ok(DirectPermissionGrantResult {
                success: false,
                grant_id: Uuid::new_v4(),
                conflicts_detected: vec![],
                warnings: vec!["Insufficient permissions to grant this permission".to_string()],
            });
        }

        // Check for existing grant
        let existing_grant = self.direct_permission_grant_repository
            .find_active_by_user_and_permission(&request.user_id, &request.permission_id).await?;

        if existing_grant.is_some() {
            return Ok(DirectPermissionGrantResult {
                success: false,
                grant_id: Uuid::new_v4(),
                conflicts_detected: vec![],
                warnings: vec!["Permission already granted to user".to_string()],
            });
        }

        // Detect conflicts
        let conflicts = self.conflict_detector.detect_permission_grant_conflicts(
            &request.permission_id,
            &request.user_id,
            &request.scope,
        ).await?;

        // Create direct permission grant using the proper entity constructor
        let mut permission_grant = crate::domain::entity::DirectPermissionGrant::new(
            request.user_id,
            request.permission_id,
            request.granted_by,
        );

        // Set reason if provided
        if let Some(reason) = request.reason {
            permission_grant = permission_grant.with_reason(reason);
        }

        // Set expiration if provided
        if let Some(expires_at) = request.expires_at {
            permission_grant = permission_grant.with_expiration(expires_at);
        }

        // Convert conditions to JSON for storage
        let conditions_json = request.conditions.as_ref().map(Self::conditions_to_json);

        // Store conditions in the entity
        permission_grant.conditions = conditions_json;
        // Metadata is already set by the entity constructor

        // Save grant
        let saved_grant = self.direct_permission_grant_repository.save(&permission_grant).await?;

        // Invalidate user's permission cache
        self.invalidate_user_cache(request.user_id, "Direct permission granted").await?;

        Ok(DirectPermissionGrantResult {
            success: true,
            grant_id: saved_grant.id,
            conflicts_detected: conflicts.iter().map(Self::to_entity_conflict).collect(),
            warnings: vec![],
        })
    }

    async fn revoke_direct_permission(&self, user_id: Uuid, permission_id: Uuid, revocation_reason: &str) -> Result<PermissionRevocationResult> {
        // Find active grant
        let grant = self.direct_permission_grant_repository
            .find_active_by_user_and_permission(&user_id, &permission_id).await?;

        if let Some(mut grant) = grant {
            // Mark as revoked using the entity method
            grant.revoke();

            // Save revoked grant
            self.direct_permission_grant_repository.save(&grant).await?;

            // Get affected permissions
            let permission_key = match self.permission_repository.find_by_id(&permission_id).await? {
                Some(p) => format!("{}:{}", p.resource, p.action),
                None => String::new(),
            };

            // Invalidate user's permission cache
            self.invalidate_user_cache(user_id, "Direct permission revoked").await?;

            Ok(PermissionRevocationResult {
                success: true,
                revoked_grant_id: Some(grant.id),
                affected_permissions: vec![permission_key],
                warnings: vec![],
            })
        } else {
            Ok(PermissionRevocationResult {
                success: false,
                revoked_grant_id: None,
                affected_permissions: vec![],
                warnings: vec!["Permission grant not found".to_string()],
            })
        }
    }

    async fn detect_role_conflicts(&self, user_id: Uuid) -> Result<Vec<RoleConflict>> {
        let assignments = self.role_assignment_repository
            .find_active_by_user(&user_id).await?;

        let conflicts = self.conflict_detector.detect_assignment_conflicts_between(
            &assignments,
        ).await?;

        Ok(conflicts)
    }

    async fn get_role_hierarchy(&self) -> Result<RoleHierarchy> {
        let roles = self.role_repository.find_all().await?;
        let assignments = self.role_assignment_repository.find_all().await?;

        // Index assignments by ID for parent lookups
        let assignment_by_id: HashMap<Uuid, &RoleAssignment> = assignments.iter()
            .map(|a| (a.id, a))
            .collect();

        // Derive role-to-role parent/child relationships from assignment hierarchy.
        // When an assignment has a parent_assignment_id, the parent assignment's role
        // is a parent of this assignment's role in the hierarchy.
        let mut role_parents: HashMap<Uuid, HashSet<Uuid>> = HashMap::new();
        let mut role_children: HashMap<Uuid, HashSet<Uuid>> = HashMap::new();

        for assignment in &assignments {
            if let Some(parent_id) = assignment.parent_assignment_id {
                if let Some(parent_assignment) = assignment_by_id.get(&parent_id) {
                    let parent_role_id = parent_assignment.role_id;
                    let child_role_id = assignment.role_id;
                    if parent_role_id != child_role_id {
                        role_parents.entry(child_role_id).or_default().insert(parent_role_id);
                        role_children.entry(parent_role_id).or_default().insert(child_role_id);
                    }
                }
            }
        }

        // Build hierarchy nodes
        let mut inheritance_map = HashMap::new();
        let mut role_nodes = Vec::new();

        for role in &roles {
            let parent_ids: Vec<Uuid> = role_parents.get(&role.id)
                .map(|s| s.iter().copied().collect())
                .unwrap_or_default();
            let child_ids: Vec<Uuid> = role_children.get(&role.id)
                .map(|s| s.iter().copied().collect())
                .unwrap_or_default();

            inheritance_map.insert(role.id, child_ids.clone());

            let depth = Self::calculate_role_depth_from_parents(role.id, &role_parents);
            let path = Self::calculate_role_path_from_parents(role.id, &role_parents);

            role_nodes.push(RoleHierarchyNode {
                role_id: role.id,
                role_name: role.name.to_string(),
                description: role.description.clone(),
                parent_ids,
                child_ids,
                depth,
                path,
            });
        }

        let conflict_rules = self.conflict_detector.get_all_conflict_rules().await?;

        Ok(RoleHierarchy {
            roles: role_nodes,
            inheritance_map,
            conflict_rules,
        })
    }

    async fn invalidate_user_cache(&self, user_id: Uuid, reason: &str) -> Result<()> {
        self.cache_manager.invalidate_user_cache(user_id, reason).await
    }
}

/// Parse permission conditions from a JSON Value stored on assignments/grants
pub fn parse_conditions_from_json(json: &Value) -> Option<PermissionConditions> {
    let time_restrictions = json.get("time_restrictions")
        .and_then(|tr| {
            if tr.is_null() { return None; }
            Some(TimeRestrictions {
                business_hours_only: tr.get("business_hours_only")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                allowed_days: tr.get("allowed_days")
                    .or_else(|| tr.get("allowed_hours"))
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect()),
                time_window: tr.get("time_window")
                    .and_then(|v| v.as_str())
                    .map(String::from),
            })
        });

    let ip_restrictions = json.get("ip_whitelist")
        .or_else(|| json.get("ip_restrictions"))
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
        .filter(|v| !v.is_empty());

    let location_restrictions = json.get("location_restrictions")
        .and_then(|lr| {
            if lr.is_null() { return None; }
            let allowed_countries = lr.get("allowed_countries")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>());
            let allowed_regions = lr.get("allowed_regions")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>());
            Some(LocationRestrictions {
                allowed_countries,
                allowed_regions,
            })
        });

    if time_restrictions.is_some() || ip_restrictions.is_some() || location_restrictions.is_some() {
        Some(PermissionConditions {
            time_restrictions,
            ip_restrictions,
            location_restrictions,
        })
    } else {
        None
    }
}

// Implementation methods
impl AuthorizationServiceImpl {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        role_repository: Arc<dyn RoleRepository>,
        permission_repository: Arc<dyn PermissionRepository>,
        direct_permission_grant_repository: Arc<dyn DirectPermissionGrantRepository>,
        role_assignment_repository: Arc<dyn RoleAssignmentRepository>,
        effective_permission_cache_repository: Arc<dyn EffectivePermissionCacheRepository>,
        conflict_detector: Arc<dyn ConflictDetector>,
        permission_calculator: Arc<dyn PermissionCalculator>,
        cache_manager: Arc<dyn PermissionCacheManager>,
    ) -> Self {
        Self {
            user_repository,
            role_repository,
            permission_repository,
            direct_permission_grant_repository,
            role_assignment_repository,
            effective_permission_cache_repository,
            conflict_detector,
            permission_calculator,
            cache_manager,
        }
    }

    async fn check_permission_cache(&self, user_id: Uuid, permission_key: &str, _context: &PermissionContext) -> Result<AuthorizationResult> {
        let cache_entry = self.effective_permission_cache_repository
            .find_by_user_and_permission(user_id, permission_key)
            .await?
            .ok_or_else(|| anyhow!("Cache miss"))?;

        // Reject entries older than 5 minutes (use updated_at, fallback to created_at)
        let cache_time = cache_entry.updated_at()
            .or(cache_entry.created_at());
        if let Some(ts) = cache_time {
            if Utc::now() - ts > Duration::minutes(5) {
                return Err(anyhow!("Cache expired"));
            }
        }

        serde_json::from_value(cache_entry.computed_permissions)
            .map_err(|e| anyhow!("Cache deserialization failed: {}", e))
    }

    async fn cache_permission_result(&self, user_id: Uuid, permission_key: &str, result: &AuthorizationResult) -> Result<()> {
        let computed_permissions = serde_json::to_value(result)
            .map_err(|e| anyhow!("Cache serialization failed: {}", e))?;

        let now = Utc::now();

        // Check if entry already exists for this (user_id, permission_key)
        let existing = self.effective_permission_cache_repository
            .find_by_user_and_permission(user_id, permission_key)
            .await
            .ok()
            .flatten();

        if let Some(mut entry) = existing {
            entry.computed_permissions = computed_permissions;
            entry.metadata.updated_at = Some(now);
            let _ = self.effective_permission_cache_repository
                .update(&entry.id.to_string(), &entry)
                .await;
        } else {
            let entry = crate::domain::entity::EffectivePermissionCache {
                id: Uuid::new_v4(),
                user_id,
                permission_key: permission_key.to_string(),
                computed_permissions,
                computation_details: serde_json::json!({}),
                scope: serde_json::json!({}),
                conditions: None,
                usage: serde_json::json!({}),
                performance: serde_json::json!({}),
                cache_stats: serde_json::json!({}),
                invalidation: None,
                metadata: crate::domain::entity::AuditMetadata {
                    created_at: Some(now),
                    updated_at: Some(now),
                    ..Default::default()
                },
            };
            let _ = self.effective_permission_cache_repository.save(&entry).await;
        }

        Ok(())
    }

    async fn check_conditions(&self, conditions: &Option<PermissionConditions>, context: &PermissionContext) -> Result<bool> {
        if let Some(conditions) = conditions {
            // Check time restrictions
            if let Some(time_restrictions) = &conditions.time_restrictions {
                if !self.check_time_restrictions(time_restrictions, &context.timestamp).await? {
                    return Ok(false);
                }
            }

            // Check IP restrictions
            if let Some(ip_restrictions) = &conditions.ip_restrictions {
                if !ip_restrictions.is_empty() {
                    let ip_address = context.ip_address.as_deref()
                        .ok_or_else(|| anyhow!("IP restrictions configured but no IP address in context"))?;
                    if !self.is_ip_allowed(ip_address, ip_restrictions).await? {
                        return Ok(false);
                    }
                }
            }

            // Check location restrictions
            if let Some(location_restrictions) = &conditions.location_restrictions {
                if !self.is_location_allowed(context, location_restrictions).await? {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /// Serialize PermissionConditions to JSON for storage on assignments/grants
    fn conditions_to_json(conditions: &PermissionConditions) -> Value {
        serde_json::json!({
            "time_restrictions": conditions.time_restrictions.as_ref().map(|tr| {
                serde_json::json!({
                    "allowed_days": tr.allowed_days.clone(),
                    "business_hours_only": tr.business_hours_only,
                    "time_window": tr.time_window.clone(),
                    "timezone": "UTC"
                })
            }),
            "location_restrictions": conditions.location_restrictions.as_ref().map(|lr| {
                serde_json::json!({
                    "allowed_countries": lr.allowed_countries.clone().unwrap_or_default(),
                    "allowed_regions": lr.allowed_regions.clone().unwrap_or_default()
                })
            }),
            "ip_whitelist": conditions.ip_restrictions.clone().unwrap_or_default()
        })
    }

    /// Convert service-local RoleConflict to entity PermissionConflict
    fn to_entity_conflict(conflict: &RoleConflict) -> crate::domain::entity::PermissionConflict {
        crate::domain::entity::PermissionConflict {
            id: conflict.conflict_id,
            conflict_type: format!("{:?}", conflict.conflict_type),
            severity: match conflict.severity {
                ConflictSeverity::Low => crate::domain::entity::ConflictSeverity::Low,
                ConflictSeverity::Medium => crate::domain::entity::ConflictSeverity::Medium,
                ConflictSeverity::High => crate::domain::entity::ConflictSeverity::High,
                ConflictSeverity::Critical => crate::domain::entity::ConflictSeverity::Critical,
            },
            description: Some(conflict.description.clone()),
            conflicting_entity_id: conflict.conflicting_assignments.first().copied().unwrap_or(Uuid::nil()),
            resolution_status: match conflict.resolution_status {
                ConflictResolutionStatus::Pending => crate::domain::entity::ConflictResolutionStatus::Pending,
                ConflictResolutionStatus::Resolved => crate::domain::entity::ConflictResolutionStatus::Resolved,
                ConflictResolutionStatus::Escalated => crate::domain::entity::ConflictResolutionStatus::Escalated,
                ConflictResolutionStatus::Ignored => crate::domain::entity::ConflictResolutionStatus::Ignored,
                ConflictResolutionStatus::RequiresManualIntervention => crate::domain::entity::ConflictResolutionStatus::Escalated,
            },
            resolved_at: None,
            metadata: crate::domain::entity::AuditMetadata::new(),
        }
    }

    async fn check_time_restrictions(&self, restrictions: &TimeRestrictions, timestamp: &DateTime<Utc>) -> Result<bool> {
        // Check business hours (9:00 - 17:00 UTC)
        if restrictions.business_hours_only {
            let hour = timestamp.hour();
            if hour < 9 || hour >= 17 {
                return Ok(false);
            }
            // Also restrict to weekdays (Mon-Fri) for business hours
            let weekday = timestamp.weekday();
            if weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun {
                return Ok(false);
            }
        }

        // Check allowed days of the week
        if let Some(allowed_days) = &restrictions.allowed_days {
            if !allowed_days.is_empty() {
                let current_day = match timestamp.weekday() {
                    chrono::Weekday::Mon => "monday",
                    chrono::Weekday::Tue => "tuesday",
                    chrono::Weekday::Wed => "wednesday",
                    chrono::Weekday::Thu => "thursday",
                    chrono::Weekday::Fri => "friday",
                    chrono::Weekday::Sat => "saturday",
                    chrono::Weekday::Sun => "sunday",
                };
                let is_allowed = allowed_days.iter().any(|d| d.to_lowercase() == current_day);
                if !is_allowed {
                    return Ok(false);
                }
            }
        }

        // Check time window (format: "HH:MM-HH:MM")
        if let Some(time_window) = &restrictions.time_window {
            if let Some((start, end)) = time_window.split_once('-') {
                let current_time = format!("{:02}:{:02}", timestamp.hour(), timestamp.minute());
                let start = start.trim();
                let end = end.trim();
                if start <= end {
                    // Same-day window (e.g., "09:00-17:00")
                    if current_time < *start || current_time >= *end {
                        return Ok(false);
                    }
                } else {
                    // Overnight window (e.g., "22:00-06:00") — allowed if before end OR after start
                    if current_time < *start && current_time >= *end {
                        return Ok(false);
                    }
                }
            }
        }

        Ok(true)
    }

    async fn is_ip_allowed(&self, ip: &str, allowed_ranges: &[String]) -> Result<bool> {
        if allowed_ranges.is_empty() {
            return Ok(true); // No restrictions configured
        }

        for range in allowed_ranges {
            // Exact match
            if ip == range {
                return Ok(true);
            }

            // Prefix match (e.g., "10.0.0." matches "10.0.0.1")
            if range.ends_with('.') && ip.starts_with(range.as_str()) {
                return Ok(true);
            }

            // CIDR notation match (e.g., "10.0.0.0/24")
            if let Some((network, prefix_len_str)) = range.split_once('/') {
                if let Ok(prefix_len) = prefix_len_str.parse::<u32>() {
                    if Self::ip_matches_cidr(ip, network, prefix_len) {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false) // IP not in any allowed range
    }

    /// Check if an IPv4 address matches a CIDR range.
    /// Note: IPv6 addresses are not supported and will return false.
    fn ip_matches_cidr(ip: &str, network: &str, prefix_len: u32) -> bool {
        if prefix_len > 32 {
            return false;
        }
        let ip_parts: Vec<u32> = ip.split('.').filter_map(|p| p.parse().ok()).collect();
        let net_parts: Vec<u32> = network.split('.').filter_map(|p| p.parse().ok()).collect();
        if ip_parts.len() != 4 || net_parts.len() != 4 {
            return false;
        }
        let ip_u32 = (ip_parts[0] << 24) | (ip_parts[1] << 16) | (ip_parts[2] << 8) | ip_parts[3];
        let net_u32 = (net_parts[0] << 24) | (net_parts[1] << 16) | (net_parts[2] << 8) | net_parts[3];
        let mask = if prefix_len == 0 { 0u32 } else { !0u32 << (32 - prefix_len) };
        (ip_u32 & mask) == (net_u32 & mask)
    }

    async fn is_location_allowed(&self, context: &PermissionContext, restrictions: &LocationRestrictions) -> Result<bool> {
        let location = context.additional_context.get("location");

        if let Some(location_value) = location {
            // Check allowed countries
            if let Some(allowed_countries) = &restrictions.allowed_countries {
                if !allowed_countries.is_empty() {
                    let country = location_value.get("country")
                        .and_then(|c| c.as_str())
                        .unwrap_or("");
                    if !allowed_countries.iter().any(|c| c.eq_ignore_ascii_case(country)) {
                        return Ok(false);
                    }
                }
            }

            // Check allowed regions
            if let Some(allowed_regions) = &restrictions.allowed_regions {
                if !allowed_regions.is_empty() {
                    let region = location_value.get("region")
                        .and_then(|r| r.as_str())
                        .unwrap_or("");
                    if !allowed_regions.iter().any(|r| r.eq_ignore_ascii_case(region)) {
                        return Ok(false);
                    }
                }
            }

            Ok(true)
        } else {
            // No location info in context — deny if restrictions are configured
            let has_restrictions = restrictions.allowed_countries.as_ref().is_some_and(|c| !c.is_empty())
                || restrictions.allowed_regions.as_ref().is_some_and(|r| !r.is_empty());
            Ok(!has_restrictions)
        }
    }

    async fn get_role_permissions(&self, role_id: &Uuid) -> Result<HashMap<String, crate::domain::entity::Permission>> {
        let permissions = self.role_repository.find_permissions_for_role(role_id).await?;
        let mut permission_map = HashMap::new();
        for permission in permissions {
            let key = format!("{}:{}", permission.resource, permission.action);
            permission_map.insert(key, permission);
        }
        Ok(permission_map)
    }

    /// Check if a user has a specific permission key via roles or direct grants
    async fn has_permission_key(&self, user_id: Uuid, required_key: &str) -> Result<bool> {
        let assignments = self.role_assignment_repository
            .find_active_by_user(&user_id).await?;
        let role_ids: Vec<Uuid> = assignments.iter().map(|a| a.role_id).collect();
        let all_role_perms = self.role_repository.find_permissions_for_roles(&role_ids).await?;

        for perms in all_role_perms.values() {
            for perm in perms {
                let key = format!("{}:{}", perm.resource, perm.action);
                if key == required_key || key == "*:*" {
                    return Ok(true);
                }
            }
        }

        let direct_grants = self.direct_permission_grant_repository
            .find_active_by_user(&user_id).await?;
        let perm_ids: Vec<Uuid> = direct_grants.iter().map(|g| g.permission_id).collect();
        let direct_perms = self.permission_repository.find_by_ids(&perm_ids).await?;

        for perm in &direct_perms {
            let key = format!("{}:{}", perm.resource, perm.action);
            if key == required_key || key == "*:*" {
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn can_assign_role(&self, assigner_id: Uuid, _role: &crate::domain::entity::Role) -> Result<bool> {
        self.has_permission_key(assigner_id, "role:assign").await
    }

    async fn can_remove_role_assignment(&self, assignment: &crate::domain::entity::RoleAssignment) -> Result<bool> {
        // System-protected assignments cannot be removed:
        // - Default roles assigned by the system (assigned_by is nil UUID)
        if let Some(role) = self.role_repository.find_by_id(&assignment.role_id).await? {
            if role.is_default && assignment.assigned_by == Uuid::nil() {
                return Ok(false);
            }
        }

        // Inherited assignments should be removed by removing the parent assignment
        if assignment.parent_assignment_id.is_some() {
            return Ok(false);
        }

        Ok(true)
    }

    async fn can_grant_permission(&self, granter_id: Uuid, _permission: &crate::domain::entity::Permission) -> Result<bool> {
        self.has_permission_key(granter_id, "permission:grant").await
    }

    /// Calculate depth of a role in the hierarchy by walking the parent chain.
    /// Roles with no parents (roots) have depth 0.
    fn calculate_role_depth_from_parents(role_id: Uuid, parents: &HashMap<Uuid, HashSet<Uuid>>) -> i32 {
        let mut depth = 0;
        let mut current = role_id;
        let mut visited = HashSet::new();
        visited.insert(current);

        while let Some(parent_set) = parents.get(&current) {
            if let Some(&first_parent) = parent_set.iter().next() {
                if !visited.insert(first_parent) {
                    break; // Cycle detected
                }
                depth += 1;
                current = first_parent;
            } else {
                break;
            }
        }
        depth
    }

    /// Calculate the path from root to a role by walking parents, then reversing.
    fn calculate_role_path_from_parents(role_id: Uuid, parents: &HashMap<Uuid, HashSet<Uuid>>) -> Vec<Uuid> {
        let mut path = vec![role_id];
        let mut current = role_id;
        let mut visited = HashSet::new();
        visited.insert(current);

        while let Some(parent_set) = parents.get(&current) {
            if let Some(&first_parent) = parent_set.iter().next() {
                if !visited.insert(first_parent) {
                    break; // Cycle detected
                }
                path.push(first_parent);
                current = first_parent;
            } else {
                break;
            }
        }
        path.reverse(); // Root first
        path
    }
}

// Repository traits (these would be auto-generated)
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<crate::domain::entity::User>>;
}

#[async_trait::async_trait]
pub trait RoleRepository: Send + Sync {
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<crate::domain::entity::Role>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<crate::domain::entity::Role>>;
    async fn find_all(&self) -> Result<Vec<crate::domain::entity::Role>>;
    /// Get all permissions associated with a role (via role_permissions join table)
    async fn find_permissions_for_role(&self, role_id: &Uuid) -> Result<Vec<crate::domain::entity::Permission>>;
    /// Batch: get permissions for multiple roles in a single query
    async fn find_permissions_for_roles(&self, role_ids: &[Uuid]) -> Result<HashMap<Uuid, Vec<crate::domain::entity::Permission>>>;
}

#[async_trait::async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<crate::domain::entity::Permission>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<crate::domain::entity::Permission>>;
}

#[async_trait::async_trait]
pub trait DirectPermissionGrantRepository: Send + Sync {
    async fn find_active_by_user(&self, user_id: &Uuid) -> Result<Vec<crate::domain::entity::DirectPermissionGrant>>;
    async fn find_active_by_user_and_permission(&self, user_id: &Uuid, permission_id: &Uuid) -> Result<Option<crate::domain::entity::DirectPermissionGrant>>;
    async fn save(&self, grant: &crate::domain::entity::DirectPermissionGrant) -> Result<crate::domain::entity::DirectPermissionGrant>;
}

#[async_trait::async_trait]
pub trait RoleAssignmentRepository: Send + Sync {
    async fn find_active_by_user(&self, user_id: &Uuid) -> Result<Vec<crate::domain::entity::RoleAssignment>>;
    async fn find_active_by_user_and_role(&self, user_id: &Uuid, role_id: &Uuid) -> Result<Option<crate::domain::entity::RoleAssignment>>;
    async fn find_child_assignments(&self, parent_id: &Uuid) -> Result<Vec<Uuid>>;
    async fn save(&self, assignment: &crate::domain::entity::RoleAssignment) -> Result<crate::domain::entity::RoleAssignment>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
    async fn find_all(&self) -> Result<Vec<crate::domain::entity::RoleAssignment>>;
}

#[async_trait::async_trait]
pub trait EffectivePermissionCacheRepository: Send + Sync {
    async fn save(&self, cache: &crate::domain::entity::EffectivePermissionCache) -> Result<crate::domain::entity::EffectivePermissionCache>;
    async fn find_by_user_and_permission(&self, user_id: Uuid, permission_key: &str) -> Result<Option<crate::domain::entity::EffectivePermissionCache>>;
    async fn update(&self, id: &str, cache: &crate::domain::entity::EffectivePermissionCache) -> Result<Option<crate::domain::entity::EffectivePermissionCache>>;
}

#[async_trait::async_trait]
pub trait ConflictDetector: Send + Sync {
    async fn detect_permission_conflicts(&self, user_id: Uuid, permissions: &HashMap<String, PermissionInfo>) -> Result<Vec<RoleConflict>>;
    async fn resolve_conflicts(&self, permissions: &HashMap<String, PermissionInfo>, conflicts: &[RoleConflict]) -> Result<HashMap<String, PermissionInfo>>;
    async fn detect_assignment_conflicts(&self, role_id: &Uuid, existing_assignments: &[crate::domain::entity::RoleAssignment], scope: &Option<PermissionScope>) -> Result<Vec<RoleConflict>>;
    async fn detect_assignment_conflicts_between(&self, assignments: &[crate::domain::entity::RoleAssignment]) -> Result<Vec<RoleConflict>>;
    async fn auto_resolve_conflicts(&self, assignment: &crate::domain::entity::RoleAssignment, conflicts: &[RoleConflict]) -> Result<Vec<RoleConflict>>;
    async fn detect_permission_grant_conflicts(&self, permission_id: &Uuid, user_id: &Uuid, scope: &Option<PermissionScope>) -> Result<Vec<RoleConflict>>;
    async fn get_all_conflict_rules(&self) -> Result<Vec<ConflictRule>>;
}

#[async_trait::async_trait]
pub trait PermissionCalculator: Send + Sync {
    /// Collect permissions from role assignments into the permissions map
    async fn collect_role_permissions(
        &self,
        assignments: &[crate::domain::entity::RoleAssignment],
        permissions: &mut HashMap<String, PermissionInfo>,
    ) -> Result<()>;

    /// Collect permissions from direct grants into the permissions map
    async fn collect_direct_grant_permissions(
        &self,
        grants: &[crate::domain::entity::DirectPermissionGrant],
        permissions: &mut HashMap<String, PermissionInfo>,
    ) -> Result<()>;
}

#[async_trait::async_trait]
pub trait PermissionCacheManager: Send + Sync {
    async fn invalidate_user_cache(&self, user_id: Uuid, reason: &str) -> Result<()>;
}

