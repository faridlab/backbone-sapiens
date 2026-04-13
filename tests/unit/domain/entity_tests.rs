//! Domain Entity Unit Tests
//!
//! Comprehensive tests for core domain entities including:
//! - User entity creation, validation, and status transitions
//! - Session entity lifecycle and expiration
//! - Password reset entity token management
//! - Role and Permission entity validation

use backbone_sapiens::domain::entity::{
    User, Session, UserStatus, DeviceType, PasswordReset, PasswordResetStatus,
    Role, Permission, RolePermission, MFADevice, MFADeviceType, MFADeviceStatus,
};
use backbone_sapiens::domain::value_objects::{Email, DeviceFingerprint};
use chrono::{Utc, Duration};
use uuid::Uuid;

// ============================================================
// User Entity Tests
// ============================================================

#[cfg(test)]
mod user_tests {
    use super::*;

    /// Test user creation with basic fields
    #[test]
    fn test_user_creation() {
        let email = "test@example.com";
        let password = "SecureP@ssw0rd123!";
        let first_name = "John";
        let last_name = "Doe";

        let user = User::new(
            email.to_string(),
            password.to_string(),
            first_name.to_string(),
            last_name.to_string(),
        ).unwrap();

        assert_eq!(user.email, email);
        assert_eq!(user.username, format!("{}_{}", first_name.to_lowercase(), last_name.to_lowercase()));
        assert_eq!(user.status, UserStatus::PendingVerification);
        assert!(!user.email_verified);
        assert_eq!(user.failed_login_attempts, 0);
        assert!(user.locked_until.is_none());
        assert!(user.last_login.is_none());
    }

    /// Test user with password hash
    #[test]
    fn test_user_with_password_hash() {
        let email = "test@example.com";
        let password_hash = "hashed_password_12345";
        let first_name = "Jane";
        let last_name = "Smith";

        let user = User::with_password_hash(
            email.to_string(),
            password_hash.to_string(),
            first_name.to_string(),
            last_name.to_string(),
        );

        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, password_hash);
        assert_eq!(user.status, UserStatus::PendingVerification);
    }

    /// Test user locked status check
    #[test]
    fn test_user_is_locked() {
        let mut user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        ).unwrap();

        // User should not be locked initially
        assert!(!user.is_locked());

        // Set lock_until to future
        user.locked_until = Some(Utc::now() + Duration::hours(1));
        assert!(user.is_locked());

        // Set lock_until to past
        user.locked_until = Some(Utc::now() - Duration::hours(1));
        assert!(!user.is_locked());
    }

    /// Test user can authenticate check
    #[test]
    fn test_user_can_authenticate() {
        let mut user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        ).unwrap();

        // Pending verification cannot authenticate
        assert!(!user.can_authenticate());

        // Active user can authenticate
        user.status = UserStatus::Active;
        assert!(user.can_authenticate());

        // Active but locked user cannot authenticate
        user.locked_until = Some(Utc::now() + Duration::hours(1));
        assert!(!user.can_authenticate());

        // Suspended user cannot authenticate
        user.locked_until = None;
        user.status = UserStatus::Suspended;
        assert!(!user.can_authenticate());
    }

    /// Test user status transitions
    #[test]
    fn test_user_status_transitions() {
        let mut user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        ).unwrap();

        // PendingVerification -> Active
        assert_eq!(user.status, UserStatus::PendingVerification);
        user.status = UserStatus::Active;
        assert_eq!(user.status, UserStatus::Active);

        // Active -> Suspended
        user.status = UserStatus::Suspended;
        assert_eq!(user.status, UserStatus::Suspended);

        // Suspended -> Inactive
        user.status = UserStatus::Inactive;
        assert_eq!(user.status, UserStatus::Inactive);
    }

    /// Test user metadata accessors
    #[test]
    fn test_user_metadata_accessors() {
        let user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "John".to_string(),
            "Doe".to_string(),
        ).unwrap();

        assert_eq!(user.first_name(), Some("John".to_string()));
        assert_eq!(user.last_name(), Some("Doe".to_string()));
        assert!(user.organization_id().is_none());
    }

    /// Test user password change timestamp
    #[test]
    fn test_user_password_changed_at() {
        let mut user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        ).unwrap();

        let timestamp = Utc::now();
        user.set_password_changed_at(timestamp);

        if let Some(timestamp_str) = user.metadata.get("password_changed_at") {
            assert!(timestamp_str.is_string());
        } else {
            panic!("password_changed_at not set in metadata");
        }
    }

    /// Test user locked state
    #[test]
    fn test_user_locked_state() {
        let mut user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        ).unwrap();

        // Increment failed login attempts
        user.failed_login_attempts = 3;
        assert_eq!(user.failed_login_attempts, 3);

        // Lock account
        user.locked_until = Some(Utc::now() + Duration::minutes(30));
        assert!(user.is_locked());

        // Reset after cooldown
        user.failed_login_attempts = 0;
        user.locked_until = None;
        assert!(!user.is_locked());
        assert_eq!(user.failed_login_attempts, 0);
    }
}

// ============================================================
// Session Entity Tests
// ============================================================

#[cfg(test)]
mod session_tests {
    use super::*;

    /// Test session creation
    #[test]
    fn test_session_creation() {
        let user_id = Uuid::new_v4();
        let device_fingerprint = "fp123456".to_string();
        let ip_address = Some("192.168.1.1".to_string());
        let user_agent = Some("Mozilla/5.0".to_string());

        let session = Session::new(
            user_id,
            device_fingerprint.clone(),
            false, // remember_me = false
            ip_address.clone(),
            user_agent.clone(),
        );

        assert_eq!(session.user_id, user_id);
        assert_eq!(session.device_type, DeviceType::Web);
        assert!(session.is_active);
        assert!(session.revoked_at.is_none());
        assert_eq!(session.ip_address, ip_address);
        assert_eq!(session.user_agent, user_agent);
        assert!(!session.remember_me()); // Regular session expires in 24 hours

        // Regular session should expire in ~24 hours
        let duration = session.expires_at.signed_duration_since(session.created_at());
        assert_eq!(duration.num_hours(), 24);
    }

    /// Test remember me session
    #[test]
    fn test_remember_me_session() {
        let user_id = Uuid::new_v4();
        let session = Session::new(
            user_id,
            "fp123456".to_string(),
            true, // remember_me = true
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        // Remember me session should be remembered
        assert!(session.remember_me());

        // Should expire in ~30 days
        let duration = session.expires_at.signed_duration_since(session.created_at());
        assert_eq!(duration.num_days(), 30);
    }

    /// Test session expiration check
    #[test]
    fn test_session_expiration() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(
            user_id,
            "fp123456".to_string(),
            false,
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        // New session should not be expired
        assert!(!session.is_expired());

        // Expired session
        session.expires_at = Utc::now() - Duration::hours(1);
        assert!(session.is_expired());
    }

    /// Test session validity check
    #[test]
    fn test_session_validity() {
        let user_id = Uuid::new_v4();
        let session = Session::new(
            user_id,
            "fp123456".to_string(),
            false,
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        // Active and not expired = valid
        assert!(session.is_valid());

        // Revoked = not valid
        let mut revoked_session = session.clone();
        revoked_session.is_active = false;
        assert!(!revoked_session.is_valid());

        // Expired = not valid
        let mut expired_session = session.clone();
        expired_session.expires_at = Utc::now() - Duration::hours(1);
        assert!(!expired_session.is_valid());
    }

    /// Test session extension
    #[test]
    fn test_session_extension() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(
            user_id,
            "fp123456".to_string(),
            false,
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        let original_expires_at = session.expires_at;
        let extension = Duration::hours(1);

        session.extend(extension);

        assert!(session.expires_at > original_expires_at);
        assert!(session.last_activity.is_some());

        // Check metadata was updated
        assert!(session.extended_at().is_some());
    }

    /// Test session with expiration
    #[test]
    fn test_session_with_expiration() {
        let user_id = Uuid::new_v4();
        let session = Session::new(
            user_id,
            "fp123456".to_string(),
            false,
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        let new_expires_at = Utc::now() + Duration::days(7);
        let extended_session = session.clone().with_expiration(new_expires_at);

        assert_eq!(extended_session.expires_at, new_expires_at);
        assert!(extended_session.last_activity.is_some());
    }

    /// Test session activity update
    #[test]
    fn test_session_activity_update() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(
            user_id,
            "fp123456".to_string(),
            false,
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        let timestamp = Utc::now();
        session.update_activity(timestamp, None);

        assert_eq!(session.last_activity, Some(timestamp));
    }

    /// Test session requires reauth
    #[test]
    fn test_session_requires_reauth() {
        let user_id = Uuid::new_v4();
        let session = Session::new(
            user_id,
            "fp123456".to_string(),
            false,
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        // New session should not require reauth
        assert!(!session.requires_reauth());
    }

    /// Test session device fingerprint
    #[test]
    fn test_session_device_fingerprint() {
        let user_id = Uuid::new_v4();
        let fp = "test_fingerprint_12345".to_string();
        let session = Session::new(
            user_id,
            fp.clone(),
            false,
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        assert_eq!(session.device_fingerprint(), fp);
    }

    /// Test session getters
    #[test]
    fn test_session_getters() {
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        let ip = Some("192.168.1.1".to_string());

        let mut session = Session::new(
            user_id,
            "fp123456".to_string(),
            false,
            ip.clone(),
            Some("Mozilla/5.0".to_string()),
        );

        session.id = session_id;

        assert_eq!(session.id(), session_id);
        assert_eq!(session.user_id(), user_id);
        assert_eq!(session.ip_address(), ip.as_deref());
        assert_eq!(session.device_type(), &DeviceType::Web);
    }
}

// ============================================================
// Password Reset Entity Tests
// ============================================================

#[cfg(test)]
mod password_reset_tests {
    use super::*;

    /// Test password reset creation
    #[test]
    fn test_password_reset_creation() {
        let user_id = Uuid::new_v4();
        let email = "test@example.com";
        let token = "reset_token_123";
        let token_hash = "hash_123";
        let expires_hours = 24;

        let reset = PasswordReset::new(
            user_id,
            email.to_string(),
            token.to_string(),
            token_hash.to_string(),
            expires_hours,
        );

        assert_eq!(reset.user_id, user_id);
        assert_eq!(reset.email, email);
        assert_eq!(reset.token, token);
        assert!(!reset.requested_by_admin);
        assert!(!reset.is_expired());
        assert!(!reset.is_used());
        assert!(!reset.is_revoked());
        assert_eq!(reset.status(), PasswordResetStatus::Pending);
    }

    /// Test admin-initiated password reset
    #[test]
    fn test_admin_initiated_password_reset() {
        let user_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();
        let reset = PasswordReset::new_admin_initiated(
            user_id,
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            24,
            admin_id,
        );

        assert!(reset.requested_by_admin);
        assert_eq!(reset.requested_by_user_id, Some(admin_id));
    }

    /// Test password reset expiration
    #[test]
    fn test_password_reset_expiration() {
        let mut reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            1, // 1 hour
        );

        assert!(!reset.is_expired());

        // Set expiration to past
        reset.expires_at = Utc::now() - Duration::minutes(1);
        assert!(reset.is_expired());
        assert_eq!(reset.status(), PasswordResetStatus::Expired);
    }

    /// Test password reset used status
    #[test]
    fn test_password_reset_used() {
        let mut reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            24,
        );

        assert!(!reset.is_used());
        assert_eq!(reset.status(), PasswordResetStatus::Pending);

        reset.mark_as_used();
        assert!(reset.is_used());
        assert_eq!(reset.status(), PasswordResetStatus::Used);
    }

    /// Test password reset revocation
    #[test]
    fn test_password_reset_revocation() {
        let mut reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            24,
        );

        let admin_id = Uuid::new_v4();
        reset.revoke(admin_id, "User requested cancellation".to_string());

        assert!(reset.is_revoked());
        assert_eq!(reset.status(), PasswordResetStatus::Revoked);
        assert_eq!(reset.revoked_by_user_id, Some(admin_id));
        assert_eq!(reset.failure_reason, Some("User requested cancellation".to_string()));
    }

    /// Test password reset validity
    #[test]
    fn test_password_reset_validity() {
        let reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            24,
        );

        // New reset should be valid
        assert!(reset.is_valid());
        assert_eq!(reset.status(), PasswordResetStatus::Pending);
    }

    /// Test password reset with context
    #[test]
    fn test_password_reset_with_context() {
        let reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            24,
        )
        .with_context(
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        assert_eq!(reset.ip_address, Some("192.168.1.1".to_string()));
        assert_eq!(reset.user_agent, Some("Mozilla/5.0".to_string()));
    }

    /// Test password reset time until expiration
    #[test]
    fn test_password_reset_time_until_expiration() {
        let reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            24,
        );

        let time_until = reset.time_until_expiration();
        assert!(time_until.num_hours() > 23);
        assert!(time_until.num_hours() <= 24);
    }

    /// Test password reset expires within
    #[test]
    fn test_password_reset_expires_within() {
        let reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            1,
        );

        // Should expire within 2 hours
        assert!(reset.expires_within(Duration::hours(2)));

        // Should not expire within 30 minutes
        assert!(!reset.expires_within(Duration::minutes(30)));
    }

    /// Test password reset remaining time human
    #[test]
    fn test_password_reset_remaining_time_human() {
        let reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            2,
        );

        let remaining = reset.remaining_time_human();
        // Should show hours (2 hours = 120 minutes, less than a day)
        assert!(remaining.contains("hours") || remaining.contains("hour"));
    }

    /// Test password reset security attempts
    #[test]
    fn test_password_reset_security_attempts() {
        let mut reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            24,
        );

        if let Some(security) = &mut reset.security {
            assert_eq!(security.attempts_remaining, 5);

            security.record_attempt();
            assert_eq!(security.attempts_remaining, 4);

            // Use remaining attempts
            security.record_attempt();
            security.record_attempt();
            security.record_attempt();
            security.record_attempt();
            assert_eq!(security.attempts_remaining, 0);
            assert!(security.suspicious_activity);
        }
    }
}

// ============================================================
// Role and Permission Entity Tests
// ============================================================

#[cfg(test)]
mod role_permission_tests {
    use super::*;

    /// Test role creation
    #[test]
    fn test_role_creation() {
        let role_name = "admin";
        let description = "Administrator role";

        let role = Role {
            id: Uuid::new_v4(),
            name: role_name.to_string(),
            description: Some(description.to_string()),
            is_system_role: true,
            metadata: serde_json::json!({}),
        };

        assert_eq!(role.name, role_name);
        assert_eq!(role.description, Some(description.to_string()));
        assert!(role.is_system_role);
    }

    /// Test permission creation
    #[test]
    fn test_permission_creation() {
        let resource = "users";
        let action = "read";

        let permission = Permission {
            id: Uuid::new_v4(),
            resource: resource.to_string(),
            action: action.to_string(),
            description: Some("Read users".to_string()),
            metadata: serde_json::json!({}),
        };

        assert_eq!(permission.resource, resource);
        assert_eq!(permission.action, action);
    }

    /// Test role permission assignment
    #[test]
    fn test_role_permission_assignment() {
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
// MFA Device Entity Tests
// ============================================================

#[cfg(test)]
mod mfa_device_tests {
    use super::*;

    /// Test MFA device type enum
    #[test]
    fn test_mfa_device_type() {
        // Test that MFADeviceType exists and has expected variants
        let device_type = MFADeviceType::Totp;
        assert_eq!(format!("{:?}", device_type), "Totp");
    }

    /// Test MFA device status enum
    #[test]
    fn test_mfa_device_status() {
        // Test that MFADeviceStatus exists
        let status = MFADeviceStatus::Active;
        assert_eq!(format!("{:?}", status), "Active");
    }

    /// Test MFA device creation
    #[test]
    fn test_mfa_device_creation() {
        let user_id = Uuid::new_v4();

        let mfa_device = MFADevice {
            id: Uuid::new_v4(),
            user_id,
            device_type: MFADeviceType::Totp,
            status: MFADeviceStatus::Active,
            name: Some("My Authenticator".to_string()),
            secret: None, // Encrypted in storage
            verified_at: Some(Utc::now()),
            last_used_at: None,
            metadata: serde_json::json!({}),
        };

        assert_eq!(mfa_device.user_id, user_id);
        assert_eq!(mfa_device.device_type, MFADeviceType::Totp);
        assert_eq!(mfa_device.status, MFADeviceStatus::Active);
        assert_eq!(mfa_device.name, Some("My Authenticator".to_string()));
    }
}
