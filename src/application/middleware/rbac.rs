//! RBAC (Role-Based Access Control) Middleware
//!
//! Provides permission-based access control for API endpoints.

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use std::collections::HashMap;

use super::auth::{AuthContext, AuthExtractor};
use crate::domain::permission::{Action, PermissionChecker, PermissionError};

/// RBAC middleware configuration
#[derive(Clone)]
pub struct RbacMiddleware<P: PermissionChecker> {
    permission_checker: Arc<P>,
}

impl<P: PermissionChecker + 'static> RbacMiddleware<P> {
    pub fn new(permission_checker: Arc<P>) -> Self {
        Self { permission_checker }
    }

    /// Create a permission guard for a specific action
    pub fn guard(&self, action: Action) -> PermissionGuard<P> {
        PermissionGuard {
            permission_checker: self.permission_checker.clone(),
            required_action: action,
            resource_owner_field: None,
        }
    }

    /// Create a permission guard with owner check
    pub fn guard_owner(&self, action: Action, owner_field: &'static str) -> PermissionGuard<P> {
        PermissionGuard {
            permission_checker: self.permission_checker.clone(),
            required_action: action,
            resource_owner_field: Some(owner_field),
        }
    }
}

/// Permission guard for individual route handlers
#[derive(Clone)]
pub struct PermissionGuard<P: PermissionChecker> {
    permission_checker: Arc<P>,
    required_action: Action,
    resource_owner_field: Option<&'static str>,
}

impl<P: PermissionChecker + 'static> PermissionGuard<P> {
    /// Check if the auth context has permission for the action
    pub fn check(&self, ctx: &AuthContext) -> Result<(), PermissionError> {
        // Super admin bypasses all permission checks
        if ctx.is_super_admin() {
            return Ok(());
        }

        // Check each role until one has permission
        for role in &ctx.roles {
            if self.permission_checker.can(role, self.required_action) {
                return Ok(());
            }
        }

        Err(PermissionError::ActionNotAllowed {
            action: self.required_action,
            role: ctx.roles.first().cloned().unwrap_or_default(),
        })
    }

    /// Check with owner override - allows action if user owns the resource
    pub fn check_with_owner(&self, ctx: &AuthContext, owner_id: &str) -> Result<(), PermissionError> {
        // Super admin bypasses
        if ctx.is_super_admin() {
            return Ok(());
        }

        // Owner can always perform owner-allowed actions
        if ctx.user_id.to_string() == owner_id {
            // Check if "owner" role would have permission
            if self.permission_checker.can("owner", self.required_action) {
                return Ok(());
            }
        }

        self.check(ctx)
    }

    /// Check field-level permission
    pub fn check_field(&self, ctx: &AuthContext, field: &str) -> Result<(), PermissionError> {
        if ctx.is_super_admin() {
            return Ok(());
        }

        for role in &ctx.roles {
            if self.permission_checker.can_access_field(role, self.required_action, field) {
                return Ok(());
            }
        }

        Err(PermissionError::FieldNotAccessible {
            field: field.to_string(),
            action: self.required_action,
        })
    }

    /// Get allowed fields for the first matching role
    pub fn allowed_fields(&self, ctx: &AuthContext) -> Vec<String> {
        if ctx.is_super_admin() {
            return vec![]; // Empty means all fields allowed
        }

        for role in &ctx.roles {
            let fields = self.permission_checker.allowed_fields(role, self.required_action);
            if !fields.is_empty() {
                return fields;
            }
        }

        vec![]
    }

    /// Filter a HashMap of fields to only include allowed fields
    pub fn filter_fields<V: Clone>(&self, ctx: &AuthContext, fields: &HashMap<String, V>) -> HashMap<String, V> {
        let allowed = self.allowed_fields(ctx);

        // If empty, allow all fields
        if allowed.is_empty() {
            return fields.clone();
        }

        fields
            .iter()
            .filter(|(k, _)| allowed.contains(k))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

/// Marker type for requiring a specific permission
pub struct RequirePermission {
    pub action: Action,
    pub resource: &'static str,
}

impl RequirePermission {
    pub const fn new(action: Action, resource: &'static str) -> Self {
        Self { action, resource }
    }

    /// Check if auth context satisfies this permission requirement
    pub fn check<P: PermissionChecker>(&self, ctx: &AuthContext, checker: &P) -> Result<(), PermissionError> {
        if ctx.is_super_admin() {
            return Ok(());
        }

        for role in &ctx.roles {
            if checker.can(role, self.action) {
                return Ok(());
            }
        }

        Err(PermissionError::ActionNotAllowed {
            action: self.action,
            role: ctx.roles.first().cloned().unwrap_or_default(),
        })
    }
}

/// Permission error response
impl IntoResponse for PermissionError {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "error": {
                "code": "PERMISSION_DENIED",
                "message": self.to_string(),
            }
        });

        (StatusCode::FORBIDDEN, axum::Json(body)).into_response()
    }
}

/// Macro for creating permission-protected handlers
#[macro_export]
macro_rules! require_permission {
    ($action:expr) => {
        move |auth: $crate::application::middleware::AuthExtractor| async move {
            // Permission check will be done in handler
            auth
        }
    };
}

/// Helper function to create Axum layer for RBAC
pub fn rbac_layer<P: PermissionChecker + Clone + Send + Sync + 'static>(
    checker: Arc<P>,
    action: Action,
) -> impl Fn(AuthExtractor, Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>> + Clone {
    move |auth: AuthExtractor, request: Request, next: Next| {
        let checker = checker.clone();
        Box::pin(async move {
            // Check permission
            for role in &auth.roles {
                if checker.can(role, action) {
                    return next.run(request).await;
                }
            }

            // Super admin bypass
            if auth.is_super_admin() {
                return next.run(request).await;
            }

            PermissionError::ActionNotAllowed {
                action,
                role: auth.roles.first().cloned().unwrap_or_default(),
            }
            .into_response()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::permission::{Action, PermissionChecker};
    use std::collections::HashSet;
    use uuid::Uuid;

    /// Mock permission checker for tests
    #[derive(Debug, Clone)]
    struct MockPermissions {
        admin_actions: HashSet<Action>,
        user_actions: HashSet<Action>,
    }

    impl MockPermissions {
        fn new() -> Self {
            let mut admin_actions = HashSet::new();
            admin_actions.insert(Action::Create);
            admin_actions.insert(Action::Read);
            admin_actions.insert(Action::Update);
            admin_actions.insert(Action::Delete);
            admin_actions.insert(Action::List);

            let mut user_actions = HashSet::new();
            user_actions.insert(Action::Read);
            user_actions.insert(Action::List);

            Self { admin_actions, user_actions }
        }
    }

    impl PermissionChecker for MockPermissions {
        fn can(&self, role: &str, action: Action) -> bool {
            match role {
                "admin" => self.admin_actions.contains(&action),
                "user" => self.user_actions.contains(&action),
                _ => false,
            }
        }

        fn can_access_field(&self, role: &str, action: Action, _field: &str) -> bool {
            self.can(role, action)
        }

        fn allowed_fields(&self, _role: &str, _action: Action) -> Vec<String> {
            vec!["id".to_string(), "username".to_string(), "email".to_string()]
        }

        fn roles(&self) -> Vec<&str> {
            vec!["admin", "user"]
        }
    }

    fn create_admin_context() -> AuthContext {
        AuthContext {
            user_id: Uuid::new_v4(),
            username: "admin".to_string(),
            email: "admin@test.com".to_string(),
            roles: vec!["admin".to_string()],
            permissions: vec![],
            session_id: None,
            email_verified: true,
        }
    }

    fn create_user_context() -> AuthContext {
        AuthContext {
            user_id: Uuid::new_v4(),
            username: "user".to_string(),
            email: "user@test.com".to_string(),
            roles: vec!["user".to_string()],
            permissions: vec![],
            session_id: None,
            email_verified: true,
        }
    }

    #[test]
    fn test_rbac_admin_create() {
        let permissions = MockPermissions::new();
        let rbac = RbacMiddleware::new(Arc::new(permissions));
        let guard = rbac.guard(Action::Create);

        let admin = create_admin_context();
        assert!(guard.check(&admin).is_ok());
    }

    #[test]
    fn test_rbac_user_read() {
        let permissions = MockPermissions::new();
        let rbac = RbacMiddleware::new(Arc::new(permissions));
        let guard = rbac.guard(Action::Read);

        let user = create_user_context();
        assert!(guard.check(&user).is_ok());
    }

    #[test]
    fn test_rbac_user_delete_denied() {
        let permissions = MockPermissions::new();
        let rbac = RbacMiddleware::new(Arc::new(permissions));
        let guard = rbac.guard(Action::Delete);

        let user = create_user_context();
        assert!(guard.check(&user).is_err());
    }

    #[test]
    fn test_rbac_owner_check() {
        let permissions = MockPermissions::new();
        let rbac = RbacMiddleware::new(Arc::new(permissions));
        let guard = rbac.guard(Action::Update);

        let admin = create_admin_context();
        let owner_id = admin.user_id.to_string();

        // Admin can update
        assert!(guard.check_with_owner(&admin, &owner_id).is_ok());
    }

    #[test]
    fn test_field_filtering() {
        let permissions = MockPermissions::new();
        let rbac = RbacMiddleware::new(Arc::new(permissions));
        let guard = rbac.guard(Action::Read);

        let user = create_user_context();

        let mut fields: HashMap<String, serde_json::Value> = HashMap::new();
        fields.insert("id".to_string(), serde_json::json!("123"));
        fields.insert("username".to_string(), serde_json::json!("test"));
        fields.insert("password_hash".to_string(), serde_json::json!("secret"));

        let filtered = guard.filter_fields(&user, &fields);

        // The allowed fields depend on the MockPermissions configuration
        assert!(filtered.contains_key("id"));
        assert!(filtered.contains_key("username"));
    }
}
