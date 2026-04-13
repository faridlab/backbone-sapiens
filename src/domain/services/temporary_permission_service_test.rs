//! Unit Tests for TemporaryPermissionDomainService
//!
//! Tests the TemporaryPermission domain service business logic for:
//! - Time-bound permission grants
//! - Duration validation (max 30 days)
//! - Duplicate permission checking
//! - Expiry notification timing

use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::domain::entity::{TemporaryPermission, TemporaryPermissionStatus};
use crate::domain::services::TemporaryPermissionDomainService;

// ============================================================
// Test Fixtures
// ============================================================

fn create_test_temporary_permission(
    user_id: Uuid,
    permission_id: Uuid,
    expires_at: chrono::DateTime<Utc>,
) -> TemporaryPermission {
    let now = Utc::now();
    TemporaryPermission {
        id: Uuid::new_v4(),
        user_id,
        permission_id,
        granted_by: Uuid::new_v4(),
        granted_at: now,
        expires_at,
        revoked_at: None,
        notified_before_expiry: false,
        status: TemporaryPermissionStatus::Active,
        metadata: serde_json::json!({}),
    }
}

// ============================================================
// Temporary Permission Domain Service Tests
// ============================================================

#[cfg(test)]
mod temporary_permission_service_tests {
    use super::*;

    // ========================================================================
    // Is Valid Tests
    // ========================================================================

    #[test]
    fn test_is_valid_active_and_not_expired() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::hours(24);

        let perm = create_test_temporary_permission(user_id, permission_id, expires_at);

        assert!(service.is_valid(&perm, Utc::now()));
    }

    #[test]
    fn test_is_valid_revoked() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let mut perm = create_test_temporary_permission(
            user_id,
            permission_id,
            Utc::now() + Duration::hours(24),
        );
        perm.status = TemporaryPermissionStatus::Revoked;

        assert!(!service.is_valid(&perm, Utc::now()));
    }

    #[test]
    fn test_is_valid_expired() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let expires_at = Utc::now() - Duration::hours(1); // Expired 1 hour ago

        let perm = create_test_temporary_permission(user_id, permission_id, expires_at);

        assert!(!service.is_valid(&perm, Utc::now()));
    }

    // ========================================================================
    // Expires Soon Tests
    // ========================================================================

    #[test]
    fn test_expires_soon_within_24_hours() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::hours(12); // 12 hours left

        let perm = create_test_temporary_permission(user_id, permission_id, expires_at);

        assert!(service.expires_soon(&perm, Utc::now()));
    }

    #[test]
    fn test_expires_soon_at_exactly_24_hours() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::hours(24);

        let perm = create_test_temporary_permission(user_id, permission_id, expires_at);

        assert!(service.expires_soon(&perm, Utc::now()));
    }

    #[test]
    fn test_expires_soon_more_than_24_hours() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::hours(48); // 48 hours left

        let perm = create_test_temporary_permission(user_id, permission_id, expires_at);

        assert!(!service.expires_soon(&perm, Utc::now()));
    }

    // ========================================================================
    // Validate Duration Tests
    // ========================================================================

    #[test]
    fn test_validate_duration_within_max() {
        let service = TemporaryPermissionDomainService::new();
        let granted_at = Utc::now();
        let expires_at = granted_at + Duration::days(7); // 7 days

        assert!(service
            .validate_duration(granted_at, expires_at, 30)
            .is_ok());
    }

    #[test]
    fn test_validate_duration_exceeds_max() {
        let service = TemporaryPermissionDomainService::new();
        let granted_at = Utc::now();
        let expires_at = granted_at + Duration::days(31); // 31 days

        assert!(service
            .validate_duration(granted_at, expires_at, 30)
            .is_err());
    }

    #[test]
    fn test_validate_duration_expiration_before_grant() {
        let service = TemporaryPermissionDomainService::new();
        let granted_at = Utc::now();
        let expires_at = granted_at - Duration::hours(1); // Before grant

        assert!(service
            .validate_duration(granted_at, expires_at, 30)
            .is_err());
    }

    #[test]
    fn test_validate_duration_exactly_max_days() {
        let service = TemporaryPermissionDomainService::new();
        let granted_at = Utc::now();
        let expires_at = granted_at + Duration::days(30); // Exactly 30 days

        assert!(service
            .validate_duration(granted_at, expires_at, 30)
            .is_ok());
    }

    // ========================================================================
    // Should Notify Expiry Tests
    // ========================================================================

    #[test]
    fn test_should_notify_expiry_not_yet_notified() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::hours(12);

        let mut perm = create_test_temporary_permission(user_id, permission_id, expires_at);
        perm.notified_before_expiry = false;

        assert!(service.should_notify_expiry(&perm, Utc::now()));
    }

    #[test]
    fn test_should_not_notify_already_notified() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::hours(12);

        let mut perm = create_test_temporary_permission(user_id, permission_id, expires_at);
        perm.notified_before_expiry = true;

        assert!(!service.should_notify_expiry(&perm, Utc::now()));
    }

    #[test]
    fn test_should_not_notify_not_expiring_soon() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::hours(48);

        let mut perm = create_test_temporary_permission(user_id, permission_id, expires_at);
        perm.notified_before_expiry = false;

        assert!(!service.should_notify_expiry(&perm, Utc::now()));
    }

    // ========================================================================
    // Has Duplicate Tests
    // ========================================================================

    #[test]
    fn test_has_duplicate_exact_match() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::days(7);

        let existing = vec![create_test_temporary_permission(
            user_id,
            permission_id,
            expires_at,
        )];

        let new_perm = create_test_temporary_permission(user_id, permission_id, expires_at);

        assert!(service.has_duplicate(&new_perm, &existing, true));
    }

    #[test]
    fn test_has_duplicate_different_permission() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let other_permission_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::days(7);

        let existing = vec![create_test_temporary_permission(
            user_id,
            other_permission_id,
            expires_at,
        )];

        let new_perm = create_test_temporary_permission(user_id, permission_id, expires_at);

        assert!(!service.has_duplicate(&new_perm, &existing, true));
    }

    #[test]
    fn test_has_duplicate_expired_ignored() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let now = Utc::now();

        let mut existing_perm = create_test_temporary_permission(
            user_id,
            permission_id,
            now - Duration::hours(1),
        );
        existing_perm.status = TemporaryPermissionStatus::Expired;

        let existing = vec![existing_perm];
        let new_perm = create_test_temporary_permission(
            user_id,
            permission_id,
            now + Duration::days(7),
        );

        // active_only=true should ignore expired
        assert!(!service.has_duplicate(&new_perm, &existing, true));
    }

    // ========================================================================
    // Remaining Duration Tests
    // ========================================================================

    #[test]
    fn test_remaining_duration_future_expiration() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let now = Utc::now();
        let expires_at = now + Duration::hours(24);

        let perm = create_test_temporary_permission(user_id, permission_id, expires_at);

        let remaining = service.remaining_duration(&perm, now);
        assert_eq!(remaining.num_hours(), 24);
    }

    #[test]
    fn test_remaining_duration_expired() {
        let service = TemporaryPermissionDomainService::new();
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let now = Utc::now();
        let expires_at = now - Duration::hours(12);

        let perm = create_test_temporary_permission(user_id, permission_id, expires_at);

        let remaining = service.remaining_duration(&perm, now);
        assert!(remaining.num_hours() < 0);
    }
}
