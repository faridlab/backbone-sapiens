//! Unit Tests for SecurityEventDomainService
//!
//! Tests the SecurityEvent domain service business logic for:
//! - Security event severity analysis
//! - Automatic response triggering
//! - Risk score calculation
//! - IP blocking decisions

use chrono::Utc;
use uuid::Uuid;

use crate::domain::entity::{SecurityEvent, SecurityEventType, SecurityEventSeverity};
use crate::domain::services::SecurityEventDomainService;

// ============================================================
// Test Fixtures
// ============================================================

fn create_test_security_event(
    event_type: SecurityEventType,
    severity: SecurityEventSeverity,
) -> SecurityEvent {
    SecurityEvent::new(event_type, severity, "Test event".to_string())
}

// ============================================================
// Security Event Domain Service Tests
// ============================================================

#[cfg(test)]
mod security_event_service_tests {
    use super::*;

    // ========================================================================
    // Is High Priority Tests
    // ========================================================================

    #[test]
    fn test_is_high_priority_critical() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Critical,
        );

        assert!(service.is_high_priority(&event));
    }

    #[test]
    fn test_is_high_priority_high() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::High,
        );

        assert!(service.is_high_priority(&event));
    }

    #[test]
    fn test_is_not_high_priority_medium() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Medium,
        );

        assert!(!service.is_high_priority(&event));
    }

    #[test]
    fn test_is_not_high_priority_low() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Low,
        );

        assert!(!service.is_high_priority(&event));
    }

    // ========================================================================
    // Requires Immediate Attention Tests
    // ========================================================================

    #[test]
    fn test_critical_requires_immediate_attention() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Critical,
        );

        assert!(service.requires_immediate_attention(&event));
    }

    #[test]
    fn test_high_brute_force_requires_immediate_attention() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::BruteForceAttack,
            SecurityEventSeverity::High,
        );

        assert!(service.requires_immediate_attention(&event));
    }

    #[test]
    fn test_high_privilege_escalation_requires_immediate_attention() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::PrivilegeEscalation,
            SecurityEventSeverity::High,
        );

        assert!(service.requires_immediate_attention(&event));
    }

    #[test]
    fn test_high_malicious_content_requires_immediate_attention() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::MaliciousContent,
            SecurityEventSeverity::High,
        );

        assert!(service.requires_immediate_attention(&event));
    }

    #[test]
    fn test_high_other_event_does_not_require_immediate_attention() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::AccountLocked,
            SecurityEventSeverity::High,
        );

        assert!(!service.requires_immediate_attention(&event));
    }

    // ========================================================================
    // Should Trigger Auto Response Tests
    // ========================================================================

    #[test]
    fn test_brute_force_triggers_auto_response() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::BruteForceAttack,
            SecurityEventSeverity::High,
        );

        assert!(service.should_trigger_auto_response(&event));
    }

    #[test]
    fn test_privilege_escalation_triggers_auto_response() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::PrivilegeEscalation,
            SecurityEventSeverity::High,
        );

        assert!(service.should_trigger_auto_response(&event));
    }

    #[test]
    fn test_malicious_content_triggers_auto_response() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::MaliciousContent,
            SecurityEventSeverity::Critical,
        );

        assert!(service.should_trigger_auto_response(&event));
    }

    #[test]
    fn test_failed_login_does_not_trigger_auto_response() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Medium,
        );

        assert!(!service.should_trigger_auto_response(&event));
    }

    // ========================================================================
    // Calculate Login Failure Severity Tests
    // ========================================================================

    #[test]
    fn test_calculate_severity_critical_many_attempts_new_location() {
        let service = SecurityEventDomainService::new();

        let severity = service.calculate_login_failure_severity(10, 5, true);
        assert_eq!(severity, SecurityEventSeverity::Critical);
    }

    #[test]
    fn test_calculate_severity_high_many_attempts() {
        let service = SecurityEventDomainService::new();

        let severity = service.calculate_login_failure_severity(5, 10, false);
        assert_eq!(severity, SecurityEventSeverity::High);
    }

    #[test]
    fn test_calculate_severity_medium_some_attempts() {
        let service = SecurityEventDomainService::new();

        let severity = service.calculate_login_failure_severity(3, 15, false);
        assert_eq!(severity, SecurityEventSeverity::Medium);
    }

    #[test]
    fn test_calculate_severity_low_few_attempts() {
        let service = SecurityEventDomainService::new();

        let severity = service.calculate_login_failure_severity(1, 1, false);
        assert_eq!(severity, SecurityEventSeverity::Low);
    }

    // ========================================================================
    // Should Block IP Tests
    // ========================================================================

    #[test]
    fn test_should_block_ip_with_many_critical_events() {
        let service = SecurityEventDomainService::new();

        let event1 = create_test_security_event(
            SecurityEventType::BruteForceAttack,
            SecurityEventSeverity::Critical,
        );
        let event2 = create_test_security_event(
            SecurityEventType::PrivilegeEscalation,
            SecurityEventSeverity::Critical,
        );
        let event3 = create_test_security_event(
            SecurityEventType::MaliciousContent,
            SecurityEventSeverity::High,
        );

        assert!(service.should_block_ip(&[&event1, &event2, &event3]));
    }

    #[test]
    fn test_should_not_block_ip_with_few_events() {
        let service = SecurityEventDomainService::new();

        let event1 = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Medium,
        );
        let event2 = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Low,
        );

        assert!(!service.should_block_ip(&[&event1, &event2]));
    }

    #[test]
    fn test_should_not_block_ip_empty_list() {
        let service = SecurityEventDomainService::new();

        assert!(!service.should_block_ip(&[]));
    }

    // ========================================================================
    // Calculate Risk Score Tests
    // ========================================================================

    #[test]
    fn test_risk_score_critical() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Critical,
        );

        assert_eq!(service.calculate_risk_score(&event), 95);
    }

    #[test]
    fn test_risk_score_high_brute_force() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::BruteForceAttack,
            SecurityEventSeverity::High,
        );

        assert_eq!(service.calculate_risk_score(&event), 90); // 75 + 15
    }

    #[test]
    fn test_risk_score_high_privilege_escalation() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::PrivilegeEscalation,
            SecurityEventSeverity::High,
        );

        assert_eq!(service.calculate_risk_score(&event), 95); // 75 + 20
    }

    #[test]
    fn test_risk_score_medium() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Medium,
        );

        assert_eq!(service.calculate_risk_score(&event), 50);
    }

    #[test]
    fn test_risk_score_low() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Low,
        );

        assert_eq!(service.calculate_risk_score(&event), 20);
    }

    // ========================================================================
    // Indicates Attack Pattern Tests
    // ========================================================================

    #[test]
    fn test_indicates_attack_pattern_with_multiple_high_severity() {
        let service = SecurityEventDomainService::new();

        let event1 = create_test_security_event(
            SecurityEventType::BruteForceAttack,
            SecurityEventSeverity::High,
        );
        let event2 = create_test_security_event(
            SecurityEventType::PrivilegeEscalation,
            SecurityEventSeverity::Critical,
        );
        let event3 = create_test_security_event(
            SecurityEventType::AccountLocked,
            SecurityEventSeverity::High,
        );

        assert!(service.indicates_attack_pattern(&[&event1, &event2, &event3], 3));
    }

    #[test]
    fn test_does_not_indicate_attack_pattern_below_threshold() {
        let service = SecurityEventDomainService::new();

        let event1 = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Medium,
        );
        let event2 = create_test_security_event(
            SecurityEventType::SuspiciousLogin,
            SecurityEventSeverity::Low,
        );

        assert!(!service.indicates_attack_pattern(&[&event1, &event2], 3));
    }

    // ========================================================================
    // Should Notify Tests
    // ========================================================================

    #[test]
    fn test_should_notify_critical() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Critical,
        );

        assert!(service.should_notify(&event));
    }

    #[test]
    fn test_should_notify_account_locked() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::AccountLocked,
            SecurityEventSeverity::High,
        );

        assert!(service.should_notify(&event));
    }

    #[test]
    fn test_should_notify_suspicious_activity() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::SuspiciousActivity,
            SecurityEventSeverity::High,
        );

        assert!(service.should_notify(&event));
    }

    #[test]
    fn test_should_not_notify_low_severity() {
        let service = SecurityEventDomainService::new();
        let event = create_test_security_event(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Low,
        );

        assert!(!service.should_notify(&event));
    }
}
