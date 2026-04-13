//! Unit Tests for ImpersonationSessionDomainService
//!
//! Tests the ImpersonationSession domain service business logic for:
//! - Admin impersonation session management
//! - Session expiration calculation
//! - Concurrent session limits
//! - Self-impersonation prevention

use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::domain::entity::{ImpersonationSession, ImpersonationSessionStatus};
use crate::domain::services::ImpersonationSessionDomainService;

// ============================================================
// Test Fixtures
// ============================================================

fn create_test_impersonation_session(
    admin_id: Uuid,
    target_user_id: Uuid,
    max_duration_minutes: i32,
) -> ImpersonationSession {
    let now = Utc::now();
    ImpersonationSession {
        id: Uuid::new_v4(),
        admin_id,
        target_user_id,
        started_at: now,
        max_duration_minutes,
        ended_at: None,
        status: ImpersonationSessionStatus::Active,
        reason: "Testing impersonation".to_string(),
        actions_performed: 0,
        metadata: serde_json::json!({}),
    }
}

// ============================================================
// Impersonation Session Domain Service Tests
// ============================================================

#[cfg(test)]
mod impersonation_session_service_tests {
    use super::*;

    // ========================================================================
    // Can Start Impersonation Tests
    // ========================================================================

    #[test]
    fn test_can_start_impersonation_valid() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();

        let result = service
            .can_start_impersonation(admin_id, target_user_id, "Support needed", 60);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_cannot_impersonate_self() {
        let service = ImpersonationSessionDomainService::new();
        let user_id = Uuid::new_v4();

        let result = service
            .can_start_impersonation(user_id, user_id, "Self test", 60);
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_start_with_empty_reason() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();

        let result = service.can_start_impersonation(admin_id, target_user_id, "", 60);
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_start_with_short_reason() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();

        let result = service.can_start_impersonation(admin_id, target_user_id, "ok", 60);
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_start_with_negative_duration() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();

        let result = service
            .can_start_impersonation(admin_id, target_user_id, "Valid reason", -10);
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_start_with_excessive_duration() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();

        let result = service
            .can_start_impersonation(admin_id, target_user_id, "Valid reason", 500);
        assert!(result.is_err());
    }

    // ========================================================================
    // Is Active Tests
    // ========================================================================

    #[test]
    fn test_is_active_with_active_status() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let session = create_test_impersonation_session(admin_id, target_user_id, 60);

        assert!(service.is_active(&session, Utc::now()));
    }

    #[test]
    fn test_is_not_active_with_ended_status() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let mut session = create_test_impersonation_session(admin_id, target_user_id, 60);
        session.status = ImpersonationSessionStatus::Ended;

        assert!(!service.is_active(&session, Utc::now()));
    }

    #[test]
    fn test_is_not_active_with_expired_status() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let mut session = create_test_impersonation_session(admin_id, target_user_id, 60);
        session.status = ImpersonationSessionStatus::Expired;

        assert!(!service.is_active(&session, Utc::now()));
    }

    #[test]
    fn test_is_not_active_when_time_expired() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let mut session = create_test_impersonation_session(admin_id, target_user_id, 60);
        session.started_at = Utc::now() - Duration::hours(2); // Started 2 hours ago
        session.max_duration_minutes = 60; // But max is 60 minutes

        assert!(!service.is_active(&session, Utc::now()));
    }

    // ========================================================================
    // Calculate Expiration Tests
    // ========================================================================

    #[test]
    fn test_calculate_expiration_60_minutes() {
        let service = ImpersonationSessionDomainService::new();
        let started_at = Utc::now();
        let expiration = service.calculate_expiration(started_at, 60);

        let expected = started_at + Duration::minutes(60);
        assert!((expiration - expected).num_seconds().abs() < 1);
    }

    #[test]
    fn test_calculate_expiration_8_hours_max() {
        let service = ImpersonationSessionDomainService::new();
        let started_at = Utc::now();
        let expiration = service.calculate_expiration(started_at, 480);

        let expected = started_at + Duration::hours(8);
        assert!((expiration - expected).num_seconds().abs() < 1);
    }

    // ========================================================================
    // Expires Soon Tests
    // ========================================================================

    #[test]
    fn test_expires_soon_with_10_minutes_left() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let mut session = create_test_impersonation_session(admin_id, target_user_id, 60);
        session.started_at = Utc::now() - Duration::minutes(50); // 50 minutes elapsed

        assert!(service.expires_soon(&session, Utc::now()));
    }

    #[test]
    fn test_expires_soon_with_exactly_15_minutes_left() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let mut session = create_test_impersonation_session(admin_id, target_user_id, 60);
        session.started_at = Utc::now() - Duration::minutes(45); // 45 minutes elapsed

        assert!(service.expires_soon(&session, Utc::now()));
    }

    #[test]
    fn test_does_not_expire_soon_with_time_remaining() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let session = create_test_impersonation_session(admin_id, target_user_id, 60);

        assert!(!service.expires_soon(&session, Utc::now()));
    }

    // ========================================================================
    // Remaining Duration Tests
    // ========================================================================

    #[test]
    fn test_remaining_duration_full_time() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let session = create_test_impersonation_session(admin_id, target_user_id, 60);

        let remaining = service.remaining_duration(&session, Utc::now());
        assert_eq!(remaining.num_minutes(), 60);
    }

    #[test]
    fn test_remaining_duration_partial_elapsed() {
        let service = ImpersonationSessionDomainService::new();
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let mut session = create_test_impersonation_session(admin_id, target_user_id, 60);
        session.started_at = Utc::now() - Duration::minutes(20);

        let remaining = service.remaining_duration(&session, Utc::now());
        assert_eq!(remaining.num_minutes(), 40);
    }

    // ========================================================================
    // Exceeds Max Active Sessions Tests
    // ========================================================================

    #[test]
    fn test_under_max_active_sessions() {
        let service = ImpersonationSessionDomainService::new();

        assert!(!service.exceeds_max_active_sessions(2, 3));
    }

    #[test]
    fn test_at_max_active_sessions() {
        let service = ImpersonationSessionDomainService::new();

        assert!(service.exceeds_max_active_sessions(3, 3));
    }

    #[test]
    fn test_over_max_active_sessions() {
        let service = ImpersonationSessionDomainService::new();

        assert!(service.exceeds_max_active_sessions(5, 3));
    }
}
