//! Application middleware
//!
//! Provides authentication and authorization middleware for the Sapiens module.

pub mod auth;
pub mod authentication_context;
pub mod rbac;

pub use auth::{AuthContext, AuthMiddleware, AuthExtractor, CurrentUser, AuthError, OptionalAuth};
pub use authentication_context::{AuthContext as AuthenticationContext, auth_middleware, optional_auth_middleware};
pub use rbac::{RbacMiddleware, RequirePermission, PermissionGuard, rbac_layer};
