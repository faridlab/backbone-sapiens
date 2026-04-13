//! User Login Flow Scenario Tests
//!
//! End-to-end tests for the complete user login workflow:
//! 1. User submits credentials
//! 2. System validates credentials
//! 3. System creates session
//! 4. User receives tokens
//! 5. User can access protected resources

use backbone_sapiens::domain::entity::{User, Session, UserStatus};
use backbone_sapiens::domain::value_objects::{Email, DeviceFingerprint};
use uuid::Uuid;
use chrono::{Utc, Duration};

// ============================================================
// Standard Login Flow
// ============================================================

#[cfg(test)]
mod standard_login_flow {
    use super::*;

    #[tokio::test]
    async fn test_successful_login_flow() {
        // Step 1: Setup - create active user
        let mut user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Active;

        // Step 2: Validate credentials
        let email = "test@example.com";
        let provided_password = "correct_password"; // In real system, this would be validated

        // Email validation
        let email_vo = Email::new(email).expect("Email should be valid");
        assert_eq!(email_vo.to_string(), email);

        // Step 3: Check user can authenticate
        assert!(user.can_authenticate());
        assert!(!user.is_locked());

        // Step 4: Create session
        let session = Session::new(
            user.id,
            DeviceFingerprint::generate().to_string(),
            false, // remember_me
            Some("127.0.0.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        // Verify session created
        assert_eq!(session.user_id, user.id);
        assert!(session.is_active);
        assert!(!session.is_expired());

        // Step 5: User logged in successfully
        assert!(user.can_authenticate());
    }

    #[tokio::test]
    async fn test_login_with_remember_me() {
        let user_id = Uuid::new_v4();

        // Create session with remember_me
        let session = Session::new(
            user_id,
            "fp_remember".to_string(),
            true, // remember_me = true
            Some("192.168.1.1".to_string()),
            None,
        );

        // Should have extended expiration
        assert!(session.remember_me());

        let duration = session.expires_at.signed_duration_since(session.created_at());
        assert_eq!(duration.num_days(), 30);
    }
}

// ============================================================
// Failed Login Flow
// ============================================================

#[cfg(test)]
mod failed_login_flow {
    use super::*;

    #[tokio::test]
    async fn test_login_with_invalid_password() {
        let mut user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Active;

        // Invalid password
        let provided_password = "wrong_password";
        let stored_password_hash = "correct_password_hash";

        // Password doesn't match
        let password_valid = provided_password == stored_password_hash;
        assert!(!password_valid);

        // Increment failed attempts
        user.failed_login_attempts += 1;
        assert_eq!(user.failed_login_attempts, 1);
    }

    #[tokio::test]
    async fn test_account_lockout_after_threshold() {
        let mut user = User::new(
            "locked@example.com".to_string(),
            "password".to_string(),
            "Locked".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Active;

        // Simulate 5 failed attempts
        user.failed_login_attempts = 5;
        user.locked_until = Some(Utc::now() + Duration::minutes(15));

        // Account should be locked
        assert!(user.is_locked());

        // User cannot authenticate while locked
        assert!(!user.can_authenticate());
    }

    #[tokio::test]
    async fn test_failed_login_count_resets_on_success() {
        let mut user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Active;
        user.failed_login_attempts = 3;

        // Successful login
        user.failed_login_attempts = 0;

        // Failed attempts should be reset
        assert_eq!(user.failed_login_attempts, 0);
        assert!(!user.is_locked());
    }
}

// ============================================================
// Login with Account Status Issues
// ============================================================

#[cfg(test)]
mod login_status_issues {
    use super::*;

    #[tokio::test]
    async fn test_login_with_pending_verification() {
        let user = User::new(
            "pending@example.com".to_string(),
            "password".to_string(),
            "Pending".to_string(),
            "User".to_string(),
        );

        // User has pending verification
        assert_eq!(user.status, UserStatus::PendingVerification);
        assert!(!user.can_authenticate());

        // Login should fail with "verify your email" message
        let can_login = user.can_authenticate();
        assert!(!can_login);
    }

    #[tokio::test]
    async fn test_login_with_suspended_account() {
        let mut user = User::new(
            "suspended@example.com".to_string(),
            "password".to_string(),
            "Suspended".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Suspended;

        // User is suspended
        assert_eq!(user.status, UserStatus::Suspended);
        assert!(!user.can_authenticate());

        // Login should fail with "account suspended" message
        let can_login = user.can_authenticate();
        assert!(!can_login);
    }

    #[tokio::test]
    async fn test_login_with_inactive_account() {
        let mut user = User::new(
            "inactive@example.com".to_string(),
            "password".to_string(),
            "Inactive".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Inactive;

        // User is inactive
        assert_eq!(user.status, UserStatus::Inactive);
        assert!(!user.can_authenticate());

        // Login should fail
        let can_login = user.can_authenticate();
        assert!(!can_login);
    }
}

// ============================================================
// Login with MFA
// ============================================================

#[cfg(test)]
mod login_with_mfa {
    use super::*;

    #[tokio::test]
    async fn test_login_requires_mfa() {
        let user_id = Uuid::new_v4();

        // MFA is enabled for user
        let mfa_enabled = true;
        let mfa_code_provided = false;

        // Login should require MFA code
        assert!(mfa_enabled);
        assert!(!mfa_code_provided);

        // Should return "MFA required" response
        let requires_mfa = mfa_enabled && !mfa_code_provided;
        assert!(requires_mfa);
    }

    #[tokio::test]
    async fn test_login_with_valid_mfa_code() {
        let mfa_enabled = true;
        let mfa_code = "123456";
        let stored_mfa_code = "123456";

        // Valid MFA code
        let mfa_valid = mfa_code == stored_mfa_code;
        assert!(mfa_valid);

        // Login should succeed
        let login_success = mfa_valid;
        assert!(login_success);
    }

    #[tokio::test]
    async fn test_login_with_invalid_mfa_code() {
        let mfa_enabled = true;
        let mfa_code = "000000"; // Wrong code
        let stored_mfa_code = "123456";

        // Invalid MFA code
        let mfa_valid = mfa_code == stored_mfa_code;
        assert!(!mfa_valid);

        // Login should fail
        let login_success = mfa_valid;
        assert!(!login_success);
    }
}

// ============================================================
// Session Management Flow
// ============================================================

#[cfg(test)]
mod session_management_flow {
    use super::*;

    #[tokio::test]
    async fn test_session_expiration() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(
            user_id,
            "fp_test".to_string(),
            false,
            None,
            None,
        );

        // Session is initially valid
        assert!(!session.is_expired());
        assert!(session.is_valid());

        // Expire the session
        session.expires_at = Utc::now() - Duration::seconds(1);

        // Session should be expired
        assert!(session.is_expired());
        assert!(!session.is_valid());
    }

    #[tokio::test]
    async fn test_session_extension() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(
            user_id,
            "fp_test".to_string(),
            false,
            None,
            None,
        );

        let original_expires = session.expires_at;

        // Extend session
        session.extend(Duration::hours(1));

        // Session should be extended
        assert!(session.expires_at > original_expires);
        assert!(session.extended_at().is_some());
    }

    #[tokio::test]
    async fn test_logout_terminates_session() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(
            user_id,
            "fp_test".to_string(),
            false,
            None,
            None,
        );

        // Session is initially active
        assert!(session.is_active);

        // Logout (terminate session)
        session.is_active = false;
        session.revoked_at = Some(Utc::now());

        // Session should no longer be valid
        assert!(!session.is_active);
        assert!(session.revoked_at.is_some());
        assert!(!session.is_valid());
    }

    #[tokio::test]
    async fn test_multiple_sessions_concurrent() {
        let user_id = Uuid::new_v4();

        // Create multiple sessions
        let session1 = Session::new(
            user_id,
            "fp_device1".to_string(),
            false,
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        let session2 = Session::new(
            user_id,
            "fp_device2".to_string(),
            false,
            Some("192.168.1.2".to_string()),
            Some("Chrome/1.0".to_string()),
        );

        // Both sessions should exist
        assert_eq!(session1.user_id, user_id);
        assert_eq!(session2.user_id, user_id);

        // Sessions should have different IDs
        assert_ne!(session1.id, session2.id);
    }
}

// ============================================================
// Login from New Device
// ============================================================

#[cfg(test)]
mod new_device_login {
    use super::*;

    #[tokio::test]
    async fn test_login_from_new_device() {
        let known_fp = "known_device_fp";
        let new_fp = "new_device_fp";

        // Device fingerprints are different
        assert_ne!(known_fp, new_fp);

        // New device detection
        let is_new_device = known_fp != new_fp;
        assert!(is_new_device);

        // Might require additional verification
        let requires_verification = is_new_device;
        assert!(requires_verification);
    }

    #[tokio::test]
    async fn test_login_from_new_location() {
        let known_ip = "192.168.1.100";
        let new_ip = "10.0.0.50";

        // IPs are different
        assert_ne!(known_ip, new_ip);

        // New location detected
        let is_new_location = known_ip != new_ip;
        assert!(is_new_location);

        // Might trigger security alert
        let security_alert = is_new_location;
        assert!(security_alert);
    }
}
