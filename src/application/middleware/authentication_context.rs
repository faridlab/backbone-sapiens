//! Authentication Context
//!
//! Provides authentication context for Axum handlers using backbone-auth integration.

use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use backbone_auth::jwt::JwtService;
use uuid::Uuid;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, OnceLock};

use crate::domain::entity::RoleAssignmentStatus;
use crate::domain::repositories::{
    UserRepository,
    SessionRepository, SessionPaginationParams, SessionFilter,
    RoleAssignmentRepository, RoleAssignmentPaginationParams, RoleAssignmentFilter,
    RoleRepository,
    RolePermissionRepository, RolePermissionPaginationParams, RolePermissionFilter,
    PermissionRepository,
};

/// Global JWT service, lazily initialized from JWT_SECRET environment variable.
/// Shared by auth_middleware and optional_auth_middleware.
fn jwt_service() -> &'static JwtService {
    static JWT: OnceLock<JwtService> = OnceLock::new();
    JWT.get_or_init(|| {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string());
        JwtService::new(&secret)
    })
}

/// Repositories needed for enriching AuthContext with real user data.
/// Initialize via `init_auth_context_repositories()` during app startup.
pub struct AuthContextRepositories {
    pub user_repository: Arc<dyn UserRepository>,
    pub session_repository: Arc<dyn SessionRepository>,
    pub role_assignment_repository: Arc<dyn RoleAssignmentRepository>,
    pub role_repository: Arc<dyn RoleRepository>,
    pub role_permission_repository: Arc<dyn RolePermissionRepository>,
    pub permission_repository: Arc<dyn PermissionRepository>,
}

static AUTH_REPOS: OnceLock<AuthContextRepositories> = OnceLock::new();

/// Initialize the repository dependencies for auth context enrichment.
/// Call once during app startup after repositories are created.
pub fn init_auth_context_repositories(repos: AuthContextRepositories) {
    let _ = AUTH_REPOS.set(repos);
}

/// Authentication context for extracting user information from JWT tokens
/// This integrates with backbone-auth for JWT validation and token extraction
#[derive(Debug, Clone)]
pub struct AuthContext {
    /// User ID extracted from JWT token
    pub user_id: Uuid,
    /// Session ID extracted from JWT token or session store
    pub session_id: Uuid,
    /// Email address of the authenticated user
    pub email: String,
    /// Roles associated with the user
    pub roles: Vec<String>,
    /// Permissions granted to the user
    pub permissions: Vec<String>,
    /// Device fingerprint for session tracking
    pub device_fingerprint: Option<String>,
    /// IP address of the current request
    pub ip_address: Option<String>,
    /// Whether the session is remembered (long-lived)
    pub is_remembered: bool,
}

/// Authentication middleware for Axum
/// Validates JWT tokens and attaches AuthContext to request extensions
pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_bearer_token(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_context = validate_jwt_token(jwt_service(), token).await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(auth_context);
    Ok(next.run(request).await)
}

/// Extract Bearer token from Authorization header
fn extract_bearer_token(request: &Request) -> Option<&str> {
    request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
}

/// Optional authentication middleware for Axum
/// Attaches auth context if token is present, but doesn't require it
pub async fn optional_auth_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(token) = extract_bearer_token(&request) {
        if let Ok(auth_context) = validate_jwt_token(jwt_service(), token).await {
            request.extensions_mut().insert(auth_context);
        }
    }

    next.run(request).await
}

/// Extension trait for Request to easily get auth context
pub trait AuthExt {
    fn get_auth_context(&self) -> Option<&AuthContext>;
}

impl AuthExt for Request {
    fn get_auth_context(&self) -> Option<&AuthContext> {
        self.extensions().get::<AuthContext>()
    }
}

// =========================================================================
// Auth Context Enrichment — DB lookups for real user data
// =========================================================================

/// Enrich auth context with real user data from the database.
/// Falls back to placeholder defaults if repositories are not initialized.
async fn enrich_auth_context(user_id: Uuid) -> (String, Uuid, Vec<String>, Vec<String>) {
    let repos = match AUTH_REPOS.get() {
        Some(repos) => repos,
        None => return default_enrichment(),
    };

    let user_id_str = user_id.to_string();

    // Run independent lookups concurrently: email, session, and role assignments
    let (email_result, session_id, (roles, role_ids)) = tokio::join!(
        repos.user_repository.find_by_id(&user_id_str),
        lookup_active_session(&*repos.session_repository, user_id),
        lookup_user_roles(
            &*repos.role_assignment_repository,
            &*repos.role_repository,
            user_id,
        ),
    );

    let email = email_result.ok().flatten().map(|u| u.email).unwrap_or_default();

    // Permission lookup depends on role_ids from above
    let permissions = lookup_user_permissions(
        &role_ids,
        &*repos.role_permission_repository,
        &*repos.permission_repository,
    ).await;

    (email, session_id, roles, permissions)
}

/// Default enrichment values when repositories are not initialized
fn default_enrichment() -> (String, Uuid, Vec<String>, Vec<String>) {
    (
        String::new(),
        Uuid::new_v4(),
        vec!["user".to_string()],
        vec!["read:profile".to_string()],
    )
}

/// Look up the most recent active session for a user.
async fn lookup_active_session(
    session_repo: &dyn SessionRepository,
    user_id: Uuid,
) -> Uuid {
    let filter = SessionFilter {
        user_id: Some(user_id),
        is_active: Some(true),
        ..Default::default()
    };
    let params = SessionPaginationParams::new(1, 1);

    session_repo
        .list_with_filters(params, filter)
        .await
        .ok()
        .and_then(|result| result.data.into_iter().next())
        .map(|session| session.id)
        .unwrap_or_else(Uuid::new_v4)
}

/// Look up active role names for a user.
/// Returns (role_names, role_ids) — role_ids are reused for permission lookup.
async fn lookup_user_roles(
    role_assignment_repo: &dyn RoleAssignmentRepository,
    role_repo: &dyn RoleRepository,
    user_id: Uuid,
) -> (Vec<String>, Vec<Uuid>) {
    let filter = RoleAssignmentFilter {
        user_id: Some(user_id),
        status: Some(RoleAssignmentStatus::Active),
        ..Default::default()
    };
    let params = RoleAssignmentPaginationParams::new(1, 100);

    let assignments = match role_assignment_repo.list_with_filters(params, filter).await {
        Ok(result) => result.data,
        Err(_) => return (vec!["user".to_string()], vec![]),
    };

    if assignments.is_empty() {
        return (vec!["user".to_string()], vec![]);
    }

    let role_ids: Vec<Uuid> = assignments.iter().map(|a| a.role_id).collect();

    // Resolve role names concurrently
    // Convert Uuids to Strings first to avoid temporary lifetime issues
    let role_id_strings: Vec<String> = role_ids.iter().map(|id| id.to_string()).collect();
    let role_futures: Vec<_> = role_id_strings.iter()
        .map(|id_str| role_repo.find_by_id(id_str))
        .collect();
    let role_results = futures::future::join_all(role_futures).await;

    let mut role_names: Vec<String> = role_results.into_iter()
        .filter_map(|r| r.ok().flatten().map(|role| role.name))
        .collect();

    if role_names.is_empty() {
        role_names.push("user".to_string());
    }

    (role_names, role_ids)
}

/// Look up permissions for a set of role_ids via the role_permission join table.
/// Returns deduplicated permission keys in "resource:action" format.
async fn lookup_user_permissions(
    role_ids: &[Uuid],
    role_permission_repo: &dyn RolePermissionRepository,
    permission_repo: &dyn PermissionRepository,
) -> Vec<String> {
    if role_ids.is_empty() {
        return vec!["read:profile".to_string()];
    }

    // Collect permission_ids from role_permissions concurrently for all roles
    let rp_futures: Vec<_> = role_ids.iter()
        .map(|role_id| {
            let filter = RolePermissionFilter {
                role_id: Some(*role_id),
                permission_id: None,
            };
            let params = RolePermissionPaginationParams::new(1, 500);
            role_permission_repo.list_with_filters(params, filter)
        })
        .collect();
    let rp_results = futures::future::join_all(rp_futures).await;

    let permission_ids: HashSet<Uuid> = rp_results.into_iter()
        .filter_map(|r| r.ok())
        .flat_map(|result| result.data.into_iter().map(|rp| rp.permission_id))
        .collect();

    if permission_ids.is_empty() {
        return vec!["read:profile".to_string()];
    }

    // Resolve permission_ids → "resource:action" keys concurrently
    // Convert Uuids to Strings first to avoid temporary lifetime issues
    let perm_id_strings: Vec<String> = permission_ids.iter().map(|pid| pid.to_string()).collect();
    let perm_futures: Vec<_> = perm_id_strings.iter()
        .map(|pid_str| permission_repo.find_by_id(pid_str))
        .collect();
    let perm_results = futures::future::join_all(perm_futures).await;

    let mut permissions: Vec<String> = perm_results.into_iter()
        .filter_map(|r| r.ok().flatten())
        .map(|perm| format!("{}:{}", perm.resource, perm.action))
        .collect();

    if permissions.is_empty() {
        permissions.push("read:profile".to_string());
    }

    permissions
}

// =========================================================================
// JWT Validation
// =========================================================================

/// Validate JWT token and enrich AuthContext with real user data from the database.
///
/// Extracts `user_id` from JWT `sub` claim, then queries repositories for:
/// - Email address from User table
/// - Active session ID from Session table
/// - Role names from RoleAssignment → Role tables
/// - Permission keys from RolePermission → Permission tables
///
/// Falls back to placeholder defaults when repositories are not initialized
/// (e.g., in tests or before `init_auth_context_repositories()` is called).
async fn validate_jwt_token(jwt_service: &JwtService, token: &str) -> Result<AuthContext, String> {
    let claims = jwt_service.validate_token(token)
        .map_err(|e| format!("JWT validation failed: {}", e))?;

    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|e| format!("Invalid user ID in token: {}", e))?;

    let (email, session_id, roles, permissions) = enrich_auth_context(user_id).await;

    Ok(AuthContext {
        user_id,
        session_id,
        email,
        roles,
        permissions,
        device_fingerprint: None,
        ip_address: None,
        is_remembered: false,
    })
}

/// Permission-based authentication middleware
/// Checks if the authenticated user has the required permissions
pub fn require_permission(
    permission: &'static str,
) -> impl Fn(Request, Next) -> Pin<Box<dyn Future<Output = Result<Response, StatusCode>> + Send>> + Clone {
    move |request: Request, next: Next| {
        let permission = permission.to_string();
        Box::pin(async move {
            if let Some(auth_context) = request.extensions().get::<AuthContext>() {
                if auth_context.permissions.contains(&permission) {
                    Ok(next.run(request).await)
                } else {
                    Err(StatusCode::FORBIDDEN)
                }
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        })
    }
}

/// Role-based authentication middleware
/// Checks if the authenticated user has the required role
pub fn require_role(
    role: &'static str,
) -> impl Fn(Request, Next) -> Pin<Box<dyn Future<Output = Result<Response, StatusCode>> + Send>> + Clone {
    move |request: Request, next: Next| {
        let role = role.to_string();
        Box::pin(async move {
            if let Some(auth_context) = request.extensions().get::<AuthContext>() {
                if auth_context.roles.contains(&role) {
                    Ok(next.run(request).await)
                } else {
                    Err(StatusCode::FORBIDDEN)
                }
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        })
    }
}

// Implement FromRequest for AuthContext to allow direct extraction in handlers
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for AuthContext
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Try to get auth context from extensions (set by middleware)
        parts.extensions.get::<AuthContext>().cloned()
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        middleware,
        routing::get,
        Router,
    };
    use backbone_auth::jwt::{JwtService as TestJwtService, Claims as TestClaims};
    use tower::ServiceExt;

    fn create_test_token() -> String {
        // Use the same secret as the default in jwt_service()
        let jwt = TestJwtService::new("your-super-secret-jwt-key-change-in-production");
        let now = chrono::Utc::now();
        let claims = TestClaims {
            sub: uuid::Uuid::new_v4().to_string(),
            exp: (now + chrono::Duration::hours(1)).timestamp() as usize,
            iat: now.timestamp() as usize,
            iss: "sapiens".to_string(),
        };
        jwt.create_token(&claims).unwrap()
    }

    #[tokio::test]
    async fn test_auth_middleware_success() {
        let token = create_test_token();

        let app = Router::new()
            .route("/protected", get(|| async { "Protected content" }))
            .layer(middleware::from_fn(auth_middleware));

        let request = Request::builder()
            .uri("/protected")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_auth_middleware_missing_token() {
        let app = Router::new()
            .route("/protected", get(|| async { "Protected content" }))
            .layer(middleware::from_fn(auth_middleware));

        let request = Request::builder()
            .uri("/protected")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_auth_middleware_invalid_token() {
        let app = Router::new()
            .route("/protected", get(|| async { "Protected content" }))
            .layer(middleware::from_fn(auth_middleware));

        let request = Request::builder()
            .uri("/protected")
            .header("Authorization", "Bearer invalid_token")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_optional_auth_without_token() {
        let app = Router::new()
            .route("/optional", get(|req: Request<Body>| async move {
                if req.extensions().get::<AuthContext>().is_some() {
                    "Authenticated"
                } else {
                    "Not authenticated"
                }
            }))
            .layer(middleware::from_fn(optional_auth_middleware));

        let request = Request::builder()
            .uri("/optional")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}