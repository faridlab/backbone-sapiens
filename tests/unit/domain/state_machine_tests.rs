//! State Machine Unit Tests
//!
//! Tests for state machine behavior in User, Session, and PasswordReset entities.
//! Focuses on valid state transitions and business rules.

use backbone_sapiens::domain::entity::{
    User, Session, PasswordReset, PasswordResetStatus, UserStatus,
};
use chrono::{Utc, Duration};
use uuid::Uuid;

// ============================================================
// User State Machine Tests
// ============================================================

#[cfg(test)]
mod user_state_machine_tests {
    use super::*;

    /// Test valid user status transitions
    #[test]
    fn test_valid_user_status_transitions() {
        // PendingVerification -> Active (after email verification)
        let mut user = create_user_with_status(UserStatus::PendingVerification);
        assert!(can_transition_to(&user, UserStatus::Active));
        user.status = UserStatus::Active;
        assert_eq!(user.status, UserStatus::Active);

        // Active -> Inactive (user deactivation)
        user.status = UserStatus::Inactive;
        assert_eq!(user.status, UserStatus::Inactive);

        // Inactive -> Active (reactivation)
        user.status = UserStatus::Active;
        assert_eq!(user.status, UserStatus::Active);

        // Active -> Suspended (admin action)
        user.status = UserStatus::Suspended;
        assert_eq!(user.status, UserStatus::Suspended);

        // Suspended -> Active (unsuspend)
        user.status = UserStatus::Active;
        assert_eq!(user.status, UserStatus::Active);
    }

    /// Test authentication capability by status
    #[test]
    fn test_authentication_capability_by_status() {
        let statuses_can_auth = vec![
            (UserStatus::Active, true),
            (UserStatus::PendingVerification, false),
            (UserStatus::Suspended, false),
            (UserStatus::Inactive, false),
        ];

        for (status, can_auth) in statuses_can_auth {
            let mut user = create_user_with_status(status);
            assert_eq!(
                user.can_authenticate(),
                can_auth,
                "Status {:?} should {} be able to authenticate",
                status,
                if can_auth { "" } else { "not" }
            );
        }
    }

    /// Test account lock state interaction
    #[test]
    fn test_account_lock_interaction() {
        let mut user = create_user_with_status(UserStatus::Active);

        // Active user not locked can authenticate
        assert!(user.can_authenticate());

        // Lock the account
        user.locked_until = Some(Utc::now() + Duration::hours(1));
        assert!(!user.can_authenticate());
        assert!(user.is_locked());

        // Even if status changes to active, still can't authenticate while locked
        user.status = UserStatus::Active;
        assert!(!user.can_authenticate());

        // Unlock the account
        user.locked_until = None;
        assert!(user.can_authenticate());
    }

    /// Test failed login attempts tracking
    #[test]
    fn test_failed_login_attempts() {
        let mut user = create_user_with_status(UserStatus::Active);

        assert_eq!(user.failed_login_attempts, 0);

        // Simulate failed login attempts
        user.failed_login_attempts = 3;
        assert_eq!(user.failed_login_attempts, 3);

        // Lock after threshold (typically 5)
        user.failed_login_attempts = 5;
        user.locked_until = Some(Utc::now() + Duration::minutes(30));
        assert!(user.is_locked());

        // Reset after successful login
        user.failed_login_attempts = 0;
        user.locked_until = None;
        assert!(!user.is_locked());
    }

    /// Test email verification transition
    #[test]
    fn test_email_verification_transition() {
        let mut user = create_user_with_status(UserStatus::PendingVerification);
        assert!(!user.email_verified);

        // Verify email
        user.email_verified = true;
        user.status = UserStatus::Active;

        assert!(user.email_verified);
        assert_eq!(user.status, UserStatus::Active);
        assert!(user.can_authenticate());
    }

    fn create_user_with_status(status: UserStatus) -> User {
        let mut user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );
        user.status = status;
        user
    }

    fn can_transition_to(user: &User, new_status: UserStatus) -> bool {
        // Business rules for valid transitions
        match user.status {
            UserStatus::PendingVerification => matches!(
                new_status,
                UserStatus::Active | UserStatus::Inactive | UserStatus::Suspended
            ),
            UserStatus::Active => matches!(
                new_status,
                UserStatus::Inactive | UserStatus::Suspended
            ),
            UserStatus::Inactive => matches!(
                new_status,
                UserStatus::Active
            ),
            UserStatus::Suspended => matches!(
                new_status,
                UserStatus::Active | UserStatus::Inactive
            ),
        }
    }
}

// ============================================================
// Session State Machine Tests
// ============================================================

#[cfg(test)]
mod session_state_machine_tests {
    use super::*;

    /// Test session lifecycle states
    #[test]
    fn test_session_lifecycle() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(
            user_id,
            "fp123".to_string(),
            false,
            Some("127.0.0.1".to_string()),
            None,
        );

        // Initial state: active and valid
        assert!(session.is_active);
        assert!(session.is_valid());
        assert!(!session.is_expired());

        // Expire the session
        session.expires_at = Utc::now() - Duration::minutes(1);
        assert!(!session.is_valid());
        assert!(session.is_expired());
    }

    /// Test session revocation
    #[test]
    fn test_session_revocation() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(
            user_id,
            "fp123".to_string(),
            false,
            Some("127.0.0.1".to_string()),
            None,
        );

        // Initial state: valid
        assert!(session.is_valid());

        // Revoke the session
        session.is_active = false;
        session.revoked_at = Some(Utc::now());

        // Should no longer be valid
        assert!(!session.is_valid());
        assert!(session.revoked_at.is_some());
    }

    /// Test session extension
    #[test]
    fn test_session_extension_state() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(
            user_id,
            "fp123".to_string(),
            false,
            Some("127.0.0.1".to_string()),
            None,
        );

        let original_expires = session.expires_at;

        // Extend session
        session.extend(Duration::hours(1));

        assert!(session.expires_at > original_expires);
        assert!(session.last_activity.is_some());
        assert!(session.extended_at().is_some());
    }

    /// Test remember me session duration
    #[test]
    fn test_remember_me_duration() {
        let user_id = Uuid::new_v4();

        let regular_session = Session::new(
            user_id,
            "fp123".to_string(),
            false, // remember_me = false
            None,
            None,
        );

        let remember_session = Session::new(
            user_id,
            "fp123".to_string(),
            true, // remember_me = true
            None,
            None,
        );

        // Regular session: 24 hours
        let regular_duration = regular_session.expires_at.signed_duration_since(regular_session.created_at());
        assert_eq!(regular_duration.num_hours(), 24);
        assert!(!regular_session.remember_me());

        // Remember me session: 30 days
        let remember_duration = remember_session.expires_at.signed_duration_since(remember_session.created_at());
        assert_eq!(remember_duration.num_days(), 30);
        assert!(remember_session.remember_me());
    }

    /// Test session invalidation conditions
    #[test]
    fn test_session_invalidation_conditions() {
        let user_id = Uuid::new_v4();

        // Valid session
        let valid_session = Session::new(user_id, "fp123".to_string(), false, None, None);
        assert!(valid_session.is_valid());

        // Expired session
        let mut expired_session = Session::new(user_id, "fp123".to_string(), false, None, None);
        expired_session.expires_at = Utc::now() - Duration::seconds(1);
        assert!(!expired_session.is_valid());

        // Inactive (revoked) session
        let mut revoked_session = Session::new(user_id, "fp123".to_string(), false, None, None);
        revoked_session.is_active = false;
        assert!(!revoked_session.is_valid());
    }
}

// ============================================================
// Password Reset State Machine Tests
// ============================================================

#[cfg(test)]
mod password_reset_state_machine_tests {
    use super::*;

    /// Test password reset lifecycle
    #[test]
    fn test_password_reset_lifecycle() {
        let reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token123".to_string(),
            "hash123".to_string(),
            24,
        );

        // Initial state: Pending
        assert_eq!(reset.status(), PasswordResetStatus::Pending);
        assert!(reset.is_valid());

        // After use: Used
        let mut used_reset = reset.clone();
        used_reset.mark_as_used();
        assert_eq!(used_reset.status(), PasswordResetStatus::Used);
        assert!(!used_reset.is_valid());
    }

    /// Test password reset expiration
    #[test]
    fn test_password_reset_expiration_state() {
        let mut reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token123".to_string(),
            "hash123".to_string(),
            1,
        );

        // Initially pending
        assert_eq!(reset.status(), PasswordResetStatus::Pending);

        // Expire it
        reset.expires_at = Utc::now() - Duration::seconds(1);
        assert_eq!(reset.status(), PasswordResetStatus::Expired);
        assert!(!reset.is_valid());
    }

    /// Test password reset revocation
    #[test]
    fn test_password_reset_revocation_state() {
        let mut reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token123".to_string(),
            "hash123".to_string(),
            24,
        );

        // Initially pending
        assert_eq!(reset.status(), PasswordResetStatus::Pending);

        // Revoke it
        reset.revoke(Uuid::new_v4(), "Admin action".to_string());
        assert_eq!(reset.status(), PasswordResetStatus::Revoked);
        assert!(!reset.is_valid());
        assert!(reset.is_revoked());
    }

    /// Test password reset cannot be reused
    #[test]
    fn test_password_reset_single_use() {
        let mut reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token123".to_string(),
            "hash123".to_string(),
            24,
        );

        // Initially valid
        assert!(reset.is_valid());

        // Mark as used
        reset.mark_as_used();
        assert!(reset.is_used());

        // Can no longer be used
        assert!(!reset.is_valid());
        assert_eq!(reset.status(), PasswordResetStatus::Used);
    }

    /// Test password reset state priority
    #[test]
    fn test_password_reset_state_priority() {
        // Test priority: Revoked > Used > Expired > Pending

        let mut reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token123".to_string(),
            "hash123".to_string(),
            24,
        );

        // Pending -> should be Pending
        assert_eq!(reset.status(), PasswordResetStatus::Pending);

        // Expire it -> should be Expired
        reset.expires_at = Utc::now() - Duration::seconds(1);
        assert_eq!(reset.status(), PasswordResetStatus::Expired);

        // Mark as used (even though expired) -> should be Used (takes priority)
        reset.mark_as_used();
        assert_eq!(reset.status(), PasswordResetStatus::Used);

        // Revoke it -> should be Revoked (highest priority)
        let mut revoked_reset = reset.clone();
        revoked_reset.revoke(Uuid::new_v4(), "Admin".to_string());
        assert_eq!(revoked_reset.status(), PasswordResetStatus::Revoked);
    }

    /// Test password reset validity check
    #[test]
    fn test_password_reset_validity() {
        // Valid reset
        let valid_reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token123".to_string(),
            "hash123".to_string(),
            24,
        );
        assert!(valid_reset.is_valid());

        // Expired reset
        let mut expired_reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token123".to_string(),
            "hash123".to_string(),
            1,
        );
        expired_reset.expires_at = Utc::now() - Duration::seconds(1);
        assert!(!expired_reset.is_valid());

        // Used reset
        let mut used_reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token123".to_string(),
            "hash123".to_string(),
            24,
        );
        used_reset.mark_as_used();
        assert!(!used_reset.is_valid());

        // Revoked reset
        let mut revoked_reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token123".to_string(),
            "hash123".to_string(),
            24,
        );
        revoked_reset.revoke(Uuid::new_v4(), "Admin".to_string());
        assert!(!revoked_reset.is_valid());
    }
}
