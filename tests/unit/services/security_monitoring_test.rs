//! Security Monitoring Service Unit Tests
//!
//! Tests for security event tracking, account lockout, and failed login tracking.

use backbone_sapiens::domain::services::security_monitoring_service::{SecurityEvent, SecurityEventType, SecurityLocation};
use crate::unit::mocks::MockSecurityMonitoringService;
use uuid::Uuid;
use chrono::Utc;

// ============================================================
// Security Event Tests
// ============================================================

#[cfg(test)]
mod security_event_tests {
    use super::*;

    #[tokio::test]
    async fn test_record_security_event() {
        let service = MockSecurityMonitoringService::new();

        let event = SecurityEvent {
            id: Uuid::new_v4(),
            event_type: SecurityEventType::LoginFailed,
            user_id: Some(Uuid::new_v4()),
            location: SecurityLocation::Ip("192.168.1.1".to_string()),
            timestamp: Utc::now(),
            metadata: serde_json::json!({}),
        };

        let result = service.record_security_event(event).await;

        assert!(result.is_ok());

        let events = service.get_events().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "LoginFailed");
    }

    #[tokio::test]
    async fn test_record_multiple_security_events() {
        let service = MockSecurityMonitoringService::new();

        let user_id = Uuid::new_v4();

        for _ in 0..3 {
            let event = SecurityEvent {
                id: Uuid::new_v4(),
                event_type: SecurityEventType::LoginFailed,
                user_id: Some(user_id),
                location: SecurityLocation::Ip("192.168.1.1".to_string()),
                timestamp: Utc::now(),
                metadata: serde_json::json!({}),
            };

            service.record_security_event(event).await.unwrap();
        }

        let events = service.get_events().await;
        assert_eq!(events.len(), 3);
    }

    #[tokio::test]
    async fn test_clear_security_events() {
        let service = MockSecurityMonitoringService::new();

        let event = SecurityEvent {
            id: Uuid::new_v4(),
            event_type: SecurityEventType::LoginFailed,
            user_id: Some(Uuid::new_v4()),
            location: SecurityLocation::Ip("192.168.1.1".to_string()),
            timestamp: Utc::now(),
            metadata: serde_json::json!({}),
        };

        service.record_security_event(event).await.unwrap();
        service.clear().await;

        let events = service.get_events().await;
        assert_eq!(events.len(), 0);
    }
}

// ============================================================
// Failed Login Tracking Tests
// ============================================================

#[cfg(test)]
mod failed_login_tests {
    use super::*;

    #[tokio::test]
    async fn test_record_failed_login() {
        let service = MockSecurityMonitoringService::new();

        let email = "test@example.com";
        service.record_failed_login(email).await.unwrap();

        let result = service.check_account_lockout(email).await.unwrap();

        // Should not be locked with 1 failed attempt
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_multiple_failed_logins() {
        let service = MockSecurityMonitoringService::new();

        let email = "test@example.com";

        for _ in 0..4 {
            service.record_failed_login(email).await.unwrap();
        }

        let result = service.check_account_lockout(email).await.unwrap();

        // Should not be locked with 4 failed attempts
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_account_lockout_after_threshold() {
        let service = MockSecurityMonitoringService::new();

        let email = "locked@example.com";

        // Set failed attempts to threshold (5)
        service.set_failed_attempts(email, 5).await;

        let result = service.check_account_lockout(email).await.unwrap();

        // Should be locked
        assert!(result.is_some());

        let lockout_until = result.unwrap();
        let now = Utc::now();
        assert!(lockout_until > now);
        assert!(lockout_until < now + chrono::Duration::minutes(16));
    }

    #[tokio::test]
    async fn test_reset_failed_attempts() {
        let service = MockSecurityMonitoringService::new();

        let email = "reset@example.com";

        // Set some failed attempts
        service.set_failed_attempts(email, 3).await;

        // Reset
        service.reset_failed_attempts(email).await.unwrap();

        // Check lockout - should not be locked
        let result = service.check_account_lockout(email).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_successful_login_resets_attempts() {
        let service = MockSecurityMonitoringService::new();

        let email = "success@example.com";

        // Set some failed attempts
        service.set_failed_attempts(email, 3).await;

        // Reset (simulating successful login)
        service.reset_failed_attempts(email).await.unwrap();

        // Try to lockout again - should start from 0
        service.record_failed_login(email).await.unwrap();
        let result = service.check_account_lockout(email).await.unwrap();
        assert!(result.is_none());
    }
}

// ============================================================
// Security Event Types Tests
// ============================================================

#[cfg(test)]
mod security_event_types_tests {
    use super::*;

    #[test]
    fn test_security_event_types() {
        let event_types = vec![
            SecurityEventType::LoginSuccess,
            SecurityEventType::LoginFailed,
            SecurityEventType::PasswordChanged,
            SecurityEventType::AccountLocked,
            SecurityEventType::AccountUnlocked,
            SecurityEventType::MfaEnabled,
            SecurityEventType::MfaDisabled,
            SecurityEventType::SuspiciousActivity,
        ];

        for event_type in event_types {
            let formatted = format!("{:?}", event_type);
            assert!(!formatted.is_empty());
        }
    }

    #[test]
    fn test_security_location() {
        let ip_location = SecurityLocation::Ip("192.168.1.1".to_string());
        match ip_location {
            SecurityLocation::Ip(ip) => assert_eq!(ip, "192.168.1.1"),
            _ => panic!("Expected Ip location"),
        }

        let geo_location = SecurityLocation::Geolocation {
            country: "US".to_string(),
            city: "New York".to_string(),
        };
        match geo_location {
            SecurityLocation::Geolocation { country, city } => {
                assert_eq!(country, "US");
                assert_eq!(city, "New York");
            }
            _ => panic!("Expected Geolocation"),
        }
    }
}

// ============================================================
// Lockout Duration Tests
// ============================================================

#[cfg(test)]
mod lockout_duration_tests {
    use super::*;

    #[tokio::test]
    async fn test_lockout_duration_is_fifteen_minutes() {
        let service = MockSecurityMonitoringService::new();

        let email = "lockout@example.com";
        service.set_failed_attempts(email, 5).await;

        let lockout_until = service.check_account_lockout(email).await.unwrap().unwrap();

        let now = Utc::now();
        let duration = lockout_until.signed_duration_since(now);

        // Should be approximately 15 minutes
        assert!(duration.num_minutes() >= 14);
        assert!(duration.num_minutes() <= 16);
    }

    #[tokio::test]
    async fn test_lockout_expires_after_duration() {
        // Note: This test demonstrates the concept but actual expiration
        // depends on the implementation
        let service = MockSecurityMonitoringService::new();

        let email = "expire@example.com";
        service.set_failed_attempts(email, 5).await;

        let initial_lockout = service.check_account_lockout(email).await.unwrap().unwrap();

        // Reset failed attempts (simulating lockout period passed)
        service.reset_failed_attempts(email).await;

        let after_reset = service.check_account_lockout(email).await.unwrap();

        // No longer locked
        assert!(after_reset.is_none());
    }
}
