//! Integration Events published by Sapiens bounded context
//!
//! These events are published for consumption by other bounded contexts:
//! - **Postman**: For sending welcome emails, notifications, etc.
//! - **Bucket**: For creating user directories, setting storage quotas
//! - **Other modules**: For user-related workflows
//!
//! # Event Naming Convention
//!
//! Events follow the pattern: `{context}.{aggregate}.{action}`
//! - `sapiens.user.created`
//! - `sapiens.user.email_verified`
//! - `sapiens.role.permissions_changed`
//!
//! # Example Consumer
//!
//! ```rust,ignore
//! // In Postman module:
//! #[async_trait]
//! impl IntegrationEventHandler for WelcomeEmailHandler {
//!     async fn handle(&self, envelope: IntegrationEventEnvelope) -> Result<(), EventError> {
//!         if envelope.event_type == "sapiens.user.created" {
//!             let event: UserCreatedIntegrationEvent = envelope.deserialize()?;
//!             self.email_service.send_welcome(&event.email).await?;
//!         }
//!         Ok(())
//!     }
//!
//!     fn event_patterns(&self) -> Vec<&'static str> {
//!         vec!["sapiens.user.created"]
//!     }
//! }
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use backbone_messaging::IntegrationEvent;

// ============================================================
// User Integration Events
// ============================================================

/// User was created in Sapiens
///
/// Published when a new user account is registered.
/// Consumers can use this to:
/// - Send welcome emails (Postman)
/// - Create user directory (Bucket)
/// - Initialize user settings in other modules
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserCreatedIntegrationEvent {
    /// Unique user identifier
    pub user_id: String,
    /// User's email address
    pub email: String,
    /// User's username
    pub username: String,
    /// User's first name
    pub first_name: String,
    /// User's last name
    pub last_name: String,
    /// Optional display name
    pub display_name: Option<String>,
    /// When the user was created
    pub occurred_at: DateTime<Utc>,
    /// Correlation ID for distributed tracing
    pub correlation_id: Option<String>,
}

impl IntegrationEvent for UserCreatedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "sapiens.user.created"
    }

    fn source_context(&self) -> &'static str {
        "sapiens"
    }

    fn aggregate_id(&self) -> &str {
        &self.user_id
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

/// User email was verified
///
/// Published when a user verifies their email address.
/// Consumers can use this to:
/// - Send confirmation email (Postman)
/// - Unlock email-gated features
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserEmailVerifiedIntegrationEvent {
    /// User ID
    pub user_id: String,
    /// Verified email address
    pub email: String,
    /// When the verification occurred
    pub occurred_at: DateTime<Utc>,
    /// Correlation ID
    pub correlation_id: Option<String>,
}

impl IntegrationEvent for UserEmailVerifiedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "sapiens.user.email_verified"
    }

    fn source_context(&self) -> &'static str {
        "sapiens"
    }

    fn aggregate_id(&self) -> &str {
        &self.user_id
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

/// User password was changed
///
/// Published when a user changes their password.
/// Consumers can use this to:
/// - Send security notification (Postman)
/// - Invalidate existing tokens
/// - Log security event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPasswordChangedIntegrationEvent {
    /// User ID
    pub user_id: String,
    /// When the password was changed
    pub occurred_at: DateTime<Utc>,
    /// Whether other sessions should be invalidated
    pub require_reauth: bool,
    /// Correlation ID
    pub correlation_id: Option<String>,
}

impl IntegrationEvent for UserPasswordChangedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "sapiens.user.password_changed"
    }

    fn source_context(&self) -> &'static str {
        "sapiens"
    }

    fn aggregate_id(&self) -> &str {
        &self.user_id
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

/// User was deactivated
///
/// Published when a user account is deactivated.
/// Consumers can use this to:
/// - Send notification (Postman)
/// - Revoke access in other modules
/// - Suspend user data processing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDeactivatedIntegrationEvent {
    /// User ID
    pub user_id: String,
    /// Reason for deactivation
    pub reason: String,
    /// When the deactivation occurred
    pub occurred_at: DateTime<Utc>,
    /// Correlation ID
    pub correlation_id: Option<String>,
}

impl IntegrationEvent for UserDeactivatedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "sapiens.user.deactivated"
    }

    fn source_context(&self) -> &'static str {
        "sapiens"
    }

    fn aggregate_id(&self) -> &str {
        &self.user_id
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

/// User was activated
///
/// Published when a user account is activated or reactivated.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserActivatedIntegrationEvent {
    /// User ID
    pub user_id: String,
    /// When the activation occurred
    pub occurred_at: DateTime<Utc>,
    /// Correlation ID
    pub correlation_id: Option<String>,
}

impl IntegrationEvent for UserActivatedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "sapiens.user.activated"
    }

    fn source_context(&self) -> &'static str {
        "sapiens"
    }

    fn aggregate_id(&self) -> &str {
        &self.user_id
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

/// User was suspended
///
/// Published when a user account is suspended (e.g., policy violation).
/// Consumers can use this to:
/// - Send notification (Postman)
/// - Block access in other modules
/// - Trigger compliance workflows
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSuspendedIntegrationEvent {
    /// User ID
    pub user_id: String,
    /// Reason for suspension
    pub reason: String,
    /// When the suspension occurred
    pub occurred_at: DateTime<Utc>,
    /// Correlation ID
    pub correlation_id: Option<String>,
}

impl IntegrationEvent for UserSuspendedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "sapiens.user.suspended"
    }

    fn source_context(&self) -> &'static str {
        "sapiens"
    }

    fn aggregate_id(&self) -> &str {
        &self.user_id
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

/// User was deleted (soft delete)
///
/// Published when a user account is deleted.
/// Consumers can use this to:
/// - Archive user data (Bucket)
/// - GDPR compliance workflows
/// - Clean up user resources
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDeletedIntegrationEvent {
    /// User ID
    pub user_id: String,
    /// When the deletion occurred
    pub occurred_at: DateTime<Utc>,
    /// Correlation ID
    pub correlation_id: Option<String>,
}

impl IntegrationEvent for UserDeletedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "sapiens.user.deleted"
    }

    fn source_context(&self) -> &'static str {
        "sapiens"
    }

    fn aggregate_id(&self) -> &str {
        &self.user_id
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

/// User account was locked
///
/// Published when a user account is locked (e.g., too many failed login attempts).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAccountLockedIntegrationEvent {
    /// User ID
    pub user_id: String,
    /// Reason for lock
    pub reason: String,
    /// When the lock expires
    pub locked_until: DateTime<Utc>,
    /// When the lock occurred
    pub occurred_at: DateTime<Utc>,
    /// Correlation ID
    pub correlation_id: Option<String>,
}

impl IntegrationEvent for UserAccountLockedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "sapiens.user.account_locked"
    }

    fn source_context(&self) -> &'static str {
        "sapiens"
    }

    fn aggregate_id(&self) -> &str {
        &self.user_id
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

// ============================================================
// Role Integration Events
// ============================================================

/// Role permissions were changed
///
/// Published when a role's permissions are modified.
/// Consumers can use this to:
/// - Invalidate cached permissions
/// - Update authorization in other modules
/// - Audit permission changes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolePermissionsChangedIntegrationEvent {
    /// Role ID
    pub role_id: String,
    /// Role name (for reference)
    pub role_name: String,
    /// Permissions that were added
    pub added_permissions: Vec<String>,
    /// Permissions that were removed
    pub removed_permissions: Vec<String>,
    /// When the change occurred
    pub occurred_at: DateTime<Utc>,
    /// Correlation ID
    pub correlation_id: Option<String>,
}

impl IntegrationEvent for RolePermissionsChangedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "sapiens.role.permissions_changed"
    }

    fn source_context(&self) -> &'static str {
        "sapiens"
    }

    fn aggregate_id(&self) -> &str {
        &self.role_id
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

/// User's roles were changed
///
/// Published when a user's role assignments change.
/// Consumers can use this to:
/// - Update access in other modules
/// - Trigger onboarding workflows
/// - Audit role changes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRolesChangedIntegrationEvent {
    /// User ID
    pub user_id: String,
    /// Roles that were assigned
    pub added_roles: Vec<String>,
    /// Roles that were removed
    pub removed_roles: Vec<String>,
    /// When the change occurred
    pub occurred_at: DateTime<Utc>,
    /// Correlation ID
    pub correlation_id: Option<String>,
}

impl IntegrationEvent for UserRolesChangedIntegrationEvent {
    fn event_type(&self) -> &'static str {
        "sapiens.user.roles_changed"
    }

    fn source_context(&self) -> &'static str {
        "sapiens"
    }

    fn aggregate_id(&self) -> &str {
        &self.user_id
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_created_event() {
        let event = UserCreatedIntegrationEvent {
            user_id: "user-123".to_string(),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            display_name: Some("John Doe".to_string()),
            occurred_at: Utc::now(),
            correlation_id: Some("trace-123".to_string()),
        };

        assert_eq!(event.event_type(), "sapiens.user.created");
        assert_eq!(event.source_context(), "sapiens");
        assert_eq!(event.aggregate_id(), "user-123");
    }

    #[test]
    fn test_event_serialization() {
        let event = UserCreatedIntegrationEvent {
            user_id: "user-123".to_string(),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            display_name: None,
            occurred_at: Utc::now(),
            correlation_id: None,
        };

        // Serialize to JSON
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("user-123"));
        assert!(json.contains("testuser"));

        // Deserialize back
        let restored: UserCreatedIntegrationEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.user_id, event.user_id);
        assert_eq!(restored.email, event.email);
    }
}
