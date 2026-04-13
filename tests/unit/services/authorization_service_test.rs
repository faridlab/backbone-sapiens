//! Authorization Service Unit Tests
//!
//! Tests for authorization and permission checking.

use backbone_sapiens::domain::entity::{Role, Permission, RolePermission, User, UserStatus};
use backbone_sapiens::domain::repositories::UserRepository;
use crate::unit::mocks::{MockUserRepository, MockRoleRepository, MockPermissionRepository};
use uuid::Uuid;
use std::collections::HashMap;

/// Test authorization service
struct TestAuthorizationService {
    user_repo: MockUserRepository,
    role_repo: MockRoleRepository,
    permission_repo: MockPermissionRepository,
    role_permissions: std::sync::Arc<tokio::sync::RwLock<HashMap<(Uuid, Uuid), bool>>>,
}

impl TestAuthorizationService {
    fn new() -> Self {
        Self {
            user_repo: MockUserRepository::new(),
            role_repo: MockRoleRepository::new(),
            permission_repo: MockPermissionRepository::new(),
            role_permissions: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    async fn grant_permission(&self, role_id: Uuid, permission_id: Uuid) {
        self.role_permissions.write().await.insert((role_id, permission_id), true);
    }

    async fn check_permission(&self, user_id: Uuid, resource: &str, action: &str) -> bool {
        // Check if permission exists
        let permissions = self.permission_repo.permissions.read().await;
        let perm_key = format!("{}:{}", resource, action);

        let permission_id = if permissions.contains_key(&perm_key) {
            Some(Uuid::new_v4()) // Simplified
        } else {
            return false;
        };

        // Check user roles
        let user = match self.user_repo.find_by_id(&user_id.to_string()).await.unwrap() {
            Some(u) => u,
            None => return false,
        };

        // For simplicity, check if user has admin role
        if let Some(metadata) = user.metadata.as_object() {
            if let Some(roles) = metadata.get("roles").and_then(|v| v.as_array()) {
                for role in roles {
                    if let Some(name) = role.as_str() {
                        if name == "admin" {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    async fn assign_role(&self, user_id: Uuid, role_name: &str) {
        if let Ok(Some(mut user)) = self.user_repo.find_by_id(&user_id.to_string()).await {
            if let Some(ref mut map) = user.metadata.as_object_mut() {
                let roles = vec![role_name.to_string()];
                map.insert("roles".to_string(), serde_json::json!(roles));
                let _ = self.user_repo.update(&user_id.to_string(), &user).await;
            }
        }
    }
}

// ============================================================
// Permission Setup Tests
// ============================================================

#[cfg(test)]
mod permission_setup_tests {
    use super::*;

    #[tokio::test]
    async fn test_create_permission() {
        let permission = Permission {
            id: Uuid::new_v4(),
            resource: "users".to_string(),
            action: "read".to_string(),
            description: Some("Read users".to_string()),
            metadata: serde_json::json!({}),
        };

        assert_eq!(permission.resource, "users");
        assert_eq!(permission.action, "read");
    }

    #[tokio::test]
    async fn test_create_role() {
        let role = Role {
            id: Uuid::new_v4(),
            name: "admin".to_string(),
            description: Some("Administrator role".to_string()),
            is_system_role: true,
            metadata: serde_json::json!({}),
        };

        assert_eq!(role.name, "admin");
        assert!(role.is_system_role);
    }

    #[tokio::test]
    async fn test_role_permission_assignment() {
        let role_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();

        let role_permission = RolePermission {
            id: Uuid::new_v4(),
            role_id,
            permission_id,
            conditions: None,
            metadata: serde_json::json!({}),
        };

        assert_eq!(role_permission.role_id, role_id);
        assert_eq!(role_permission.permission_id, permission_id);
    }
}

// ============================================================
// Authorization Tests
// ============================================================

#[cfg(test)]
mod authorization_tests {
    use super::*;

    #[tokio::test]
    async fn test_admin_has_all_permissions() {
        let service = TestAuthorizationService::new();

        // Create admin user
        let mut user = User::new(
            "admin@example.com".to_string(),
            "password".to_string(),
            "Admin".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Active;
        user.metadata = serde_json::json!({
            "roles": ["admin"]
        });

        let user_id = user.id;
        service.user_repo.add_user(user).await;

        // Add permission
        let permission = Permission {
            id: Uuid::new_v4(),
            resource: "users".to_string(),
            action: "delete".to_string(),
            description: None,
            metadata: serde_json::json!({}),
        };
        service.permission_repo.add_permission(permission).await;

        // Check permission
        let has_permission = service.check_permission(user_id, "users", "delete").await;
        assert!(has_permission);
    }

    #[tokio::test]
    async fn test_regular_user_limited_permissions() {
        let service = TestAuthorizationService::new();

        // Create regular user
        let mut user = User::new(
            "user@example.com".to_string(),
            "password".to_string(),
            "Regular".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Active;
        user.metadata = serde_json::json!({
            "roles": ["user"]
        });

        let user_id = user.id;
        service.user_repo.add_user(user).await;

        // Add permission
        let permission = Permission {
            id: Uuid::new_v4(),
            resource: "users".to_string(),
            action: "delete".to_string(),
            description: None,
            metadata: serde_json::json!({}),
        };
        service.permission_repo.add_permission(permission).await;

        // Check permission - regular user should not have delete permission
        let has_permission = service.check_permission(user_id, "users", "delete").await;
        assert!(!has_permission);
    }

    #[tokio::test]
    async fn test_nonexistent_user_no_permissions() {
        let service = TestAuthorizationService::new();

        let has_permission = service.check_permission(Uuid::new_v4(), "users", "read").await;
        assert!(!has_permission);
    }

    #[tokio::test]
    async fn test_assign_role_to_user() {
        let service = TestAuthorizationService::new();

        let mut user = User::new(
            "user@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Active;

        let user_id = user.id;
        service.user_repo.add_user(user).await;

        // Assign admin role
        service.assign_role(user_id, "admin").await;

        // Verify role was assigned
        let updated_user = service.user_repo.find_by_id(&user_id.to_string()).await.unwrap().unwrap();
        assert!(updated_user.metadata.get("roles").is_some());
    }
}

// ============================================================
// Role Hierarchy Tests
// ============================================================

#[cfg(test)]
mod role_hierarchy_tests {
    use super::*;

    #[test]
    fn test_role_comparison() {
        let admin = Role {
            id: Uuid::new_v4(),
            name: "admin".to_string(),
            description: Some("Full access".to_string()),
            is_system_role: true,
            metadata: serde_json::json!({ "level": 100 }),
        };

        let user_role = Role {
            id: Uuid::new_v4(),
            name: "user".to_string(),
            description: Some("Limited access".to_string()),
            is_system_role: false,
            metadata: serde_json::json!({ "level": 10 }),
        };

        assert!(admin.is_system_role);
        assert!(!user_role.is_system_role);
    }

    #[test]
    fn test_permission_combination() {
        let permissions = vec![
            Permission {
                id: Uuid::new_v4(),
                resource: "users".to_string(),
                action: "read".to_string(),
                description: None,
                metadata: serde_json::json!({}),
            },
            Permission {
                id: Uuid::new_v4(),
                resource: "users".to_string(),
                action: "write".to_string(),
                description: None,
                metadata: serde_json::json!({}),
            },
        ];

        assert_eq!(permissions.len(), 2);
        assert_eq!(permissions[0].action, "read");
        assert_eq!(permissions[1].action, "write");
    }
}
