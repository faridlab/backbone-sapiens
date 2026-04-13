//! Domain Tests for v2.0 Entities
//!
//! Tests the domain logic for:
//! - TemporaryPermission entity
//! - ImpersonationSession entity
//! - SecurityEvent entity
//! - AnonymizationRecord entity
//! - ResourcePermission entity

use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::domain::entity::{
    AnonymizationMethod, AnonymizationRecord, ImpersonationSession,
    ImpersonationSessionStatus, ResourcePermission, ResourcePermissionStatus,
    SecurityEvent, SecurityEventSeverity, SecurityEventType, TemporaryPermission,
    TemporaryPermissionStatus,
};

// ============================================================
// TemporaryPermission Entity Tests
// ============================================================

#[cfg(test)]
mod temporary_permission_entity_tests {
    use super::*;

    #[test]
    fn test_temporary_permission_creation() {
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let granted_by = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::days(1);

        let perm = TemporaryPermission {
            id: Uuid::new_v4(),
            user_id,
            permission_id,
            granted_by,
            granted_at: Utc::now(),
            expires_at,
            revoked_at: None,
            notified_before_expiry: false,
            status: TemporaryPermissionStatus::Active,
            metadata: serde_json::json!({}),
        };

        assert_eq!(perm.status, TemporaryPermissionStatus::Active);
        assert!(perm.revoked_at.is_none());
        assert!(!perm.notified_before_expiry);
    }

    #[test]
    fn test_temporary_permission_pending_to_active() {
        let mut perm = TemporaryPermission {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            permission_id: Uuid::new_v4(),
            granted_by: Uuid::new_v4(),
            granted_at: Utc::now(),
            expires_at: Utc::now() + Duration::days(1),
            revoked_at: None,
            notified_before_expiry: false,
            status: TemporaryPermissionStatus::Pending,
            metadata: serde_json::json!({}),
        };

        perm.status = TemporaryPermissionStatus::Active;
        assert_eq!(perm.status, TemporaryPermissionStatus::Active);
    }

    #[test]
    fn test_temporary_permission_expires() {
        let mut perm = TemporaryPermission {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            permission_id: Uuid::new_v4(),
            granted_by: Uuid::new_v4(),
            granted_at: Utc::now(),
            expires_at: Utc::now() - Duration::hours(1),
            revoked_at: None,
            notified_before_expiry: false,
            status: TemporaryPermissionStatus::Active,
            metadata: serde_json::json!({}),
        };

        perm.status = TemporaryPermissionStatus::Expired;
        assert_eq!(perm.status, TemporaryPermissionStatus::Expired);
    }
}

// ============================================================
// ImpersonationSession Entity Tests
// ============================================================

#[cfg(test)]
mod impersonation_session_entity_tests {
    use super::*;

    #[test]
    fn test_impersonation_session_creation() {
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let now = Utc::now();

        let session = ImpersonationSession {
            id: Uuid::new_v4(),
            admin_id,
            target_user_id,
            started_at: now,
            max_duration_minutes: 60,
            ended_at: None,
            status: ImpersonationSessionStatus::Active,
            reason: "Support needed".to_string(),
            actions_performed: 0,
            metadata: serde_json::json!({}),
        };

        assert_eq!(session.status, ImpersonationSessionStatus::Active);
        assert!(session.ended_at.is_none());
        assert_eq!(session.actions_performed, 0);
    }

    #[test]
    fn test_impersonation_session_termination() {
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let now = Utc::now();

        let mut session = ImpersonationSession {
            id: Uuid::new_v4(),
            admin_id,
            target_user_id,
            started_at: now,
            max_duration_minutes: 60,
            ended_at: None,
            status: ImpersonationSessionStatus::Active,
            reason: "Support needed".to_string(),
            actions_performed: 5,
            metadata: serde_json::json!({}),
        };

        session.ended_at = Some(now + Duration::minutes(30));
        session.status = ImpersonationSessionStatus::Ended;

        assert_eq!(session.status, ImpersonationSessionStatus::Ended);
        assert!(session.ended_at.is_some());
        assert_eq!(session.actions_performed, 5);
    }

    #[test]
    fn test_impersonation_session_expiration() {
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let now = Utc::now();

        let mut session = ImpersonationSession {
            id: Uuid::new_v4(),
            admin_id,
            target_user_id,
            started_at: now,
            max_duration_minutes: 60,
            ended_at: None,
            status: ImpersonationSessionStatus::Active,
            reason: "Support needed".to_string(),
            actions_performed: 0,
            metadata: serde_json::json!({}),
        };

        session.status = ImpersonationSessionStatus::Expired;
        assert_eq!(session.status, ImpersonationSessionStatus::Expired);
    }
}

// ============================================================
// SecurityEvent Entity Tests
// ============================================================

#[cfg(test)]
mod security_event_entity_tests {
    use super::*;

    #[test]
    fn test_security_event_creation() {
        let event = SecurityEvent::new(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Medium,
            "Failed login attempt".to_string(),
        );

        assert_eq!(event.event_type, SecurityEventType::FailedLogin);
        assert_eq!(event.severity, SecurityEventSeverity::Medium);
        assert!(!event.resolved);
        assert!(event.resolved_at.is_none());
        assert!(event.resolved_by_user_id.is_none());
    }

    #[test]
    fn test_security_event_resolution() {
        let mut event = SecurityEvent::new(
            SecurityEventType::BruteForceAttack,
            SecurityEventSeverity::High,
            "Brute force detected".to_string(),
        );

        let resolver_id = Uuid::new_v4();
        let notes = "False positive - user verified".to_string();

        event.resolve(resolver_id, notes.clone());

        assert!(event.resolved);
        assert_eq!(event.resolved_by_user_id, Some(resolver_id));
        assert_eq!(event.resolution_notes, Some(notes));
        assert!(event.resolved_at.is_some());
    }

    #[test]
    fn test_security_event_is_high_priority() {
        let high_event = SecurityEvent::new(
            SecurityEventType::PrivilegeEscalation,
            SecurityEventSeverity::High,
            "Privilege escalation".to_string(),
        );

        assert!(high_event.is_high_priority());
    }

    #[test]
    fn test_security_event_not_high_priority() {
        let low_event = SecurityEvent::new(
            SecurityEventType::FailedLogin,
            SecurityEventSeverity::Low,
            "Failed login".to_string(),
        );

        assert!(!low_event.is_high_priority());
    }

    #[test]
    fn test_security_event_requires_immediate_attention_critical() {
        let critical_event = SecurityEvent::new(
            SecurityEventType::BruteForceAttack,
            SecurityEventSeverity::Critical,
            "Critical attack".to_string(),
        );

        assert!(critical_event.requires_immediate_attention());
    }

    #[test]
    fn test_security_event_requires_immediate_attention_high_brute_force() {
        let high_event = SecurityEvent::new(
            SecurityEventType::BruteForceAttack,
            SecurityEventSeverity::High,
            "Brute force".to_string(),
        );

        assert!(high_event.requires_immediate_attention());
    }
}

// ============================================================
// ResourcePermission Entity Tests
// ============================================================

#[cfg(test)]
mod resource_permission_entity_tests {
    use super::*;

    fn create_test_resource_permission(
        granted_to_user_id: Option<Uuid>,
        granted_to_role_id: Option<Uuid>,
        resource_type: &str,
        resource_id: &str,
    ) -> ResourcePermission {
        ResourcePermission {
            id: Uuid::new_v4(),
            permission_id: Uuid::new_v4(),
            granted_to_user_id,
            granted_to_role_id,
            resource_type: resource_type.to_string(),
            resource_id: resource_id.to_string(),
            granted_by: Uuid::new_v4(),
            granted_at: Utc::now(),
            expires_at: None,
            revoked_at: None,
            reason: Some("Access needed".to_string()),
            status: ResourcePermissionStatus::Active,
            metadata: serde_json::json!({}),
        }
    }

    #[test]
    fn test_resource_permission_for_user() {
        let user_id = Uuid::new_v4();
        let perm = create_test_resource_permission(
            Some(user_id),
            None,
            "document",
            "doc-123",
        );

        assert_eq!(perm.granted_to_user_id, Some(user_id));
        assert!(perm.granted_to_role_id.is_none());
        assert_eq!(perm.status, ResourcePermissionStatus::Active);
    }

    #[test]
    fn test_resource_permission_for_role() {
        let role_id = Uuid::new_v4();
        let perm = create_test_resource_permission(None, Some(role_id), "document", "doc-123");

        assert!(perm.granted_to_user_id.is_none());
        assert_eq!(perm.granted_to_role_id, Some(role_id));
    }

    #[test]
    fn test_resource_permission_expiration() {
        let mut perm = create_test_resource_permission(
            Some(Uuid::new_v4()),
            None,
            "document",
            "doc-123",
        );
        perm.expires_at = Some(Utc::now() - Duration::hours(1));
        perm.status = ResourcePermissionStatus::Expired;

        assert_eq!(perm.status, ResourcePermissionStatus::Expired);
    }

    #[test]
    fn test_resource_permission_revocation() {
        let mut perm = create_test_resource_permission(
            Some(Uuid::new_v4()),
            None,
            "document",
            "doc-123",
        );
        perm.revoked_at = Some(Utc::now());
        perm.status = ResourcePermissionStatus::Revoked;

        assert_eq!(perm.status, ResourcePermissionStatus::Revoked);
        assert!(perm.revoked_at.is_some());
    }
}

// ============================================================
// AnonymizationRecord Entity Tests
// ============================================================

#[cfg(test)]
mod anonymization_record_entity_tests {
    use super::*;

    #[test]
    fn test_anonymization_record_full_method() {
        let user_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();
        let now = Utc::now();

        let record = AnonymizationRecord {
            id: Uuid::new_v4(),
            user_id,
            original_email: "user@example.com".to_string(),
            original_username: "testuser".to_string(),
            anonymized_by: admin_id,
            anonymized_at: now,
            reason: "GDPR request".to_string(),
            method: AnonymizationMethod::Full,
            retention_period_days: Some(30),
            records_affected: 150,
            status: crate::domain::entity::AnonymizationStatus::Completed,
            metadata: serde_json::json!({}),
        };

        assert_eq!(record.method, AnonymizationMethod::Full);
        assert_eq!(record.records_affected, 150);
        assert_eq!(
            record.status,
            crate::domain::entity::AnonymizationStatus::Completed
        );
    }

    #[test]
    fn test_anonymization_record_partial_method() {
        let record = AnonymizationRecord {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            original_email: "user@example.com".to_string(),
            original_username: "testuser".to_string(),
            anonymized_by: Uuid::new_v4(),
            anonymized_at: Utc::now(),
            reason: "Partial anonymization".to_string(),
            method: AnonymizationMethod::Partial,
            retention_period_days: None,
            records_affected: 50,
            status: crate::domain::entity::AnonymizationStatus::Completed,
            metadata: serde_json::json!({}),
        };

        assert_eq!(record.method, AnonymizationMethod::Partial);
        assert_eq!(record.records_affected, 50);
    }

    #[test]
    fn test_anonymization_record_pseudonymization_method() {
        let record = AnonymizationRecord {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            original_email: "user@example.com".to_string(),
            original_username: "testuser".to_string(),
            anonymized_by: Uuid::new_v4(),
            anonymized_at: Utc::now(),
            reason: "Pseudonymization".to_string(),
            method: AnonymizationMethod::Pseudonymization,
            retention_period_days: Some(365),
            records_affected: 10,
            status: crate::domain::entity::AnonymizationStatus::Completed,
            metadata: serde_json::json!({}),
        };

        assert_eq!(record.method, AnonymizationMethod::Pseudonymization);
        assert_eq!(record.retention_period_days, Some(365));
    }
}
