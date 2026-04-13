//! Unit Tests for SessionLimitDomainService
//!
//! Tests the SessionLimit domain service business logic for:
//! - Concurrent session management
//! - Session limit enforcement
//! - Per-device session limits

use chrono::Utc;
use uuid::Uuid;

use crate::domain::entity::SessionLimit;
use crate::domain::services::SessionLimitDomainService;

// ============================================================
// Test Fixtures
// ============================================================

fn create_test_session_limit(user_id: Uuid, max_sessions: i32, enforce: bool) -> SessionLimit {
    SessionLimit {
        id: Uuid::new_v4(),
        user_id,
        max_sessions,
        max_sessions_per_device: None,
        enforce_limit: enforce,
        current_session_count: 0,
        last_session_revoke_at: None,
        metadata: serde_json::json!({}),
    }
}

// ============================================================
// Session Limit Domain Service Tests
// ============================================================

#[cfg(test)]
mod session_limit_service_tests {
    use super::*;

    // ========================================================================
    // Can Create Session Tests
    // ========================================================================

    #[test]
    fn test_can_create_session_within_limit() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let limit = create_test_session_limit(user_id, 5, true);

        // With 0 current sessions, should be able to create
        assert!(service.can_create_session(&limit, 0).unwrap());
    }

    #[test]
    fn test_can_create_session_at_limit() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let limit = create_test_session_limit(user_id, 5, true);

        // At limit (5 sessions), should NOT be able to create more
        assert!(!service.can_create_session(&limit, 5).unwrap());
    }

    #[test]
    fn test_can_create_session_below_limit() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let limit = create_test_session_limit(user_id, 5, true);

        // Below limit (3 sessions), should be able to create
        assert!(service.can_create_session(&limit, 3).unwrap());
    }

    #[test]
    fn test_can_create_session_when_not_enforced() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let limit = create_test_session_limit(user_id, 5, false);

        // Even at limit, when not enforcing, should allow
        assert!(service.can_create_session(&limit, 10).unwrap());
        assert!(service.can_create_session(&limit, 100).unwrap());
    }

    // ========================================================================
    // Get Effective Limit Tests
    // ========================================================================

    #[test]
    fn test_get_effective_limit_default() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let limit = create_test_session_limit(user_id, 10, true);

        // Without per-device limit, should return max_sessions
        assert_eq!(service.get_effective_limit(&limit, None), 10);
    }

    #[test]
    fn test_get_effective_limit_with_per_device() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let mut limit = create_test_session_limit(user_id, 10, true);
        limit.max_sessions_per_device = Some(2);

        // Per-device limit should be used when device type is specified
        assert_eq!(service.get_effective_limit(&limit, Some("mobile".to_string())), 2);
    }

    // ========================================================================
    // Sessions to Revoke Tests
    // ========================================================================

    #[test]
    fn test_sessions_to_revoke_at_limit() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let limit = create_test_session_limit(user_id, 5, true);

        // At limit with 5 sessions, should revoke 4 to keep 1 slot for current
        assert_eq!(service.sessions_to_revoke(&limit, 5), 4);
    }

    #[test]
    fn test_sessions_to_revoke_over_limit() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let limit = create_test_session_limit(user_id, 5, true);

        // Over limit with 7 sessions, should revoke 6
        assert_eq!(service.sessions_to_revoke(&limit, 7), 6);
    }

    #[test]
    fn test_sessions_to_revoke_below_limit() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let limit = create_test_session_limit(user_id, 5, true);

        // Below limit, nothing to revoke
        assert_eq!(service.sessions_to_revoke(&limit, 2), 0);
    }

    #[test]
    fn test_sessions_to_revoke_when_not_enforced() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let limit = create_test_session_limit(user_id, 5, false);

        // When not enforcing, nothing to revoke even at high counts
        assert_eq!(service.sessions_to_revoke(&limit, 100), 0);
    }

    // ========================================================================
    // Create Default Tests
    // ========================================================================

    #[test]
    fn test_create_default_session_limit() {
        let user_id = Uuid::new_v4();
        let limit = SessionLimitDomainService::create_default(user_id);

        assert_eq!(limit.user_id, user_id);
        assert_eq!(limit.max_sessions, 5);
        assert!(limit.enforce_limit);
        assert_eq!(limit.current_session_count, 0);
        assert!(limit.last_session_revoke_at.is_none());
    }

    // ========================================================================
    // Validation Tests
    // ========================================================================

    #[test]
    fn test_validate_valid_limit() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let limit = create_test_session_limit(user_id, 10, true);

        assert!(service.validate(&limit).is_ok());
    }

    #[test]
    fn test_can_create_valid_limit() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let limit = create_test_session_limit(user_id, 10, true);

        assert!(service.can_create(&limit).unwrap());
    }

    #[test]
    fn test_can_update_limit() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let current = create_test_session_limit(user_id, 5, true);
        let updated = create_test_session_limit(user_id, 10, true);

        assert!(service.can_update(&current, &updated).unwrap());
    }

    #[test]
    fn test_can_delete_limit() {
        let service = SessionLimitDomainService::new();
        let user_id = Uuid::new_v4();
        let limit = create_test_session_limit(user_id, 5, true);

        assert!(service.can_delete(&limit).unwrap());
    }
}
