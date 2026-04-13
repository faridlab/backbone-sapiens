//! Session Management Unit Tests
//!
//! Tests for session management including:
//! - Create session
//! - Validate session
//! - Terminate session
//! - Session expiration

use backbone_sapiens::domain::entity::{Session, User};
use backbone_sapiens::domain::value_objects::DeviceFingerprint;
use backbone_sapiens::domain::services::{
    SessionManagementService, SessionConfig, SessionValidationResult,
};
use uuid::Uuid;
use chrono::{Utc, Duration};

/// Mock session data store
#[derive(Clone)]
struct MockSessionStore {
    sessions: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<Uuid, SessionData>>>,
}

#[derive(Clone)]
struct SessionData {
    session: Session,
    user_id: Uuid,
    device_fingerprint: Option<String>,
}

impl MockSessionStore {
    fn new() -> Self {
        Self {
            sessions: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    async fn add_session(&self, session: Session, user_id: Uuid) {
        let data = SessionData {
            session: session.clone(),
            user_id,
            device_fingerprint: Some(session.device_fingerprint().to_string()),
        };
        self.sessions.write().await.insert(session.id, data);
    }
}

/// Test session management service
struct TestSessionManagementService {
    store: MockSessionStore,
}

impl TestSessionManagementService {
    fn new() -> Self {
        Self {
            store: MockSessionStore::new(),
        }
    }

    async fn create_session_internal(
        &self,
        user_id: Uuid,
        remember_me: bool,
        device_fingerprint: Option<String>,
        ip_address: Option<String>,
    ) -> Result<Session, String> {
        let fp = device_fingerprint.unwrap_or_else(|| DeviceFingerprint::generate().to_string());
        let session = Session::new(user_id, fp, remember_me, ip_address, None);

        self.store.add_session(session.clone(), user_id).await;

        Ok(session)
    }

    async fn validate_session_internal(
        &self,
        session_id: Uuid,
    ) -> Result<SessionValidationResult, String> {
        let sessions = self.store.sessions.read().await;

        if let Some(data) = sessions.get(&session_id) {
            let session = &data.session;

            if !session.is_active {
                return Ok(SessionValidationResult {
                    valid: false,
                    user_id: data.user_id,
                    requires_reauth: false,
                    security_alerts: vec!["Session is inactive".to_string()],
                });
            }

            if session.is_expired() {
                return Ok(SessionValidationResult {
                    valid: false,
                    user_id: data.user_id,
                    requires_reauth: false,
                    security_alerts: vec!["Session has expired".to_string()],
                });
            }

            Ok(SessionValidationResult {
                valid: true,
                user_id: data.user_id,
                requires_reauth: session.requires_reauth(),
                security_alerts: vec![],
            })
        } else {
            Ok(SessionValidationResult {
                valid: false,
                user_id: Uuid::nil(),
                requires_reauth: false,
                security_alerts: vec!["Session not found".to_string()],
            })
        }
    }

    async fn terminate_session_internal(&self, session_id: Uuid) -> Result<bool, String> {
        let mut sessions = self.store.sessions.write().await;

        if let Some(mut data) = sessions.get_mut(&session_id) {
            data.session.is_active = false;
            data.session.revoked_at = Some(Utc::now());
            return Ok(true);
        }

        Ok(false)
    }

    async fn get_session_internal(&self, session_id: Uuid) -> Result<Option<Session>, String> {
        let sessions = self.store.sessions.read().await;
        Ok(sessions.get(&session_id).map(|data| data.session.clone()))
    }
}

// ============================================================
// Create Session Tests
// ============================================================

#[cfg(test)]
mod create_session_tests {
    use super::*;

    #[tokio::test]
    async fn test_create_session_success() {
        let service = TestSessionManagementService::new();
        let user_id = Uuid::new_v4();

        let session = service
            .create_session_internal(user_id, false, Some("fp123".to_string()), Some("127.0.0.1".to_string()))
            .await
            .unwrap();

        assert_eq!(session.user_id, user_id);
        assert!(session.is_active);
        assert!(!session.is_expired());
        assert_eq!(session.device_fingerprint(), "fp123");
        assert_eq!(session.ip_address, Some("127.0.0.1".to_string()));
    }

    #[tokio::test]
    async fn test_create_session_with_remember_me() {
        let service = TestSessionManagementService::new();
        let user_id = Uuid::new_v4();

        let session = service
            .create_session_internal(user_id, true, Some("fp123".to_string()), None)
            .await
            .unwrap();

        assert!(session.remember_me());

        // Remember me session should last ~30 days
        let duration = session.expires_at.signed_duration_since(session.created_at());
        assert_eq!(duration.num_days(), 30);
    }

    #[tokio::test]
    async fn test_create_regular_session_duration() {
        let service = TestSessionManagementService::new();
        let user_id = Uuid::new_v4();

        let session = service
            .create_session_internal(user_id, false, Some("fp123".to_string()), None)
            .await
            .unwrap();

        assert!(!session.remember_me());

        // Regular session should last ~24 hours
        let duration = session.expires_at.signed_duration_since(session.created_at());
        assert_eq!(duration.num_hours(), 24);
    }

    #[tokio::test]
    async fn test_create_session_generates_unique_ids() {
        let service = TestSessionManagementService::new();
        let user_id = Uuid::new_v4();

        let session1 = service
            .create_session_internal(user_id, false, Some("fp1".to_string()), None)
            .await
            .unwrap();

        let session2 = service
            .create_session_internal(user_id, false, Some("fp2".to_string()), None)
            .await
            .unwrap();

        assert_ne!(session1.id, session2.id);
    }
}

// ============================================================
// Validate Session Tests
// ============================================================

#[cfg(test)]
mod validate_session_tests {
    use super::*;

    #[tokio::test]
    async fn test_validate_valid_session() {
        let service = TestSessionManagementService::new();
        let user_id = Uuid::new_v4();

        let session = service
            .create_session_internal(user_id, false, Some("fp123".to_string()), None)
            .await
            .unwrap();

        let result = service.validate_session_internal(session.id).await.unwrap();

        assert!(result.valid);
        assert_eq!(result.user_id, user_id);
        assert!(!result.requires_reauth);
        assert!(result.security_alerts.is_empty());
    }

    #[tokio::test]
    async fn test_validate_expired_session() {
        let service = TestSessionManagementService::new();
        let user_id = Uuid::new_v4();

        let mut session = service
            .create_session_internal(user_id, false, Some("fp123".to_string()), None)
            .await
            .unwrap();

        // Expire the session
        session.expires_at = Utc::now() - Duration::minutes(1);
        service.store.add_session(session, user_id).await;

        let result = service.validate_session_internal(session.id).await.unwrap();

        assert!(!result.valid);
        assert!(result.security_alerts.iter().any(|s| s.contains("expired")));
    }

    #[tokio::test]
    async fn test_validate_nonexistent_session() {
        let service = TestSessionManagementService::new();

        let result = service.validate_session_internal(Uuid::new_v4()).await.unwrap();

        assert!(!result.valid);
        assert!(result.security_alerts.iter().any(|s| s.contains("not found")));
    }

    #[tokio::test]
    async fn test_validate_inactive_session() {
        let service = TestSessionManagementService::new();
        let user_id = Uuid::new_v4();

        let mut session = service
            .create_session_internal(user_id, false, Some("fp123".to_string()), None)
            .await
            .unwrap();

        // Deactivate session
        session.is_active = false;
        service.store.add_session(session, user_id).await;

        let result = service.validate_session_internal(session.id).await.unwrap();

        assert!(!result.valid);
        assert!(result.security_alerts.iter().any(|s| s.contains("inactive")));
    }
}

// ============================================================
// Terminate Session Tests
// ============================================================

#[cfg(test)]
mod terminate_session_tests {
    use super::*;

    #[tokio::test]
    async fn test_terminate_session_success() {
        let service = TestSessionManagementService::new();
        let user_id = Uuid::new_v4();

        let session = service
            .create_session_internal(user_id, false, Some("fp123".to_string()), None)
            .await
            .unwrap();

        let result = service.terminate_session_internal(session.id).await.unwrap();

        assert!(result);

        // Verify session is now inactive
        let retrieved = service.get_session_internal(session.id).await.unwrap().unwrap();
        assert!(!retrieved.is_active);
        assert!(retrieved.revoked_at.is_some());
    }

    #[tokio::test]
    async fn test_terminate_nonexistent_session() {
        let service = TestSessionManagementService::new();

        let result = service.terminate_session_internal(Uuid::new_v4()).await.unwrap();

        assert!(!result);
    }

    #[tokio::test]
    async fn test_terminated_session_not_valid() {
        let service = TestSessionManagementService::new();
        let user_id = Uuid::new_v4();

        let session = service
            .create_session_internal(user_id, false, Some("fp123".to_string()), None)
            .await
            .unwrap();

        // Terminate the session
        service.terminate_session_internal(session.id).await.unwrap();

        // Should no longer be valid
        let result = service.validate_session_internal(session.id).await.unwrap();
        assert!(!result.valid);
    }
}

// ============================================================
// Session Entity Tests
// ============================================================

#[cfg(test)]
mod session_entity_tests {
    use super::*;

    #[test]
    fn test_session_is_expired() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(user_id, "fp123".to_string(), false, None, None);

        assert!(!session.is_expired());

        session.expires_at = Utc::now() - Duration::seconds(1);
        assert!(session.is_expired());
    }

    #[test]
    fn test_session_is_valid() {
        let user_id = Uuid::new_v4();

        // Active and not expired = valid
        let session = Session::new(user_id, "fp123".to_string(), false, None, None);
        assert!(session.is_valid());

        // Inactive = not valid
        let mut inactive_session = Session::new(user_id, "fp123".to_string(), false, None, None);
        inactive_session.is_active = false;
        assert!(!inactive_session.is_valid());
    }

    #[test]
    fn test_session_extension() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(user_id, "fp123".to_string(), false, None, None);

        let original_expires = session.expires_at;

        session.extend(Duration::hours(1));

        assert!(session.expires_at > original_expires);
        assert!(session.extended_at().is_some());
    }

    #[test]
    fn test_session_activity_update() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(user_id, "fp123".to_string(), false, None, None);

        let timestamp = Utc::now();
        session.update_activity(timestamp, None);

        assert_eq!(session.last_activity, Some(timestamp));
    }

    #[test]
    fn test_session_getters() {
        let user_id = Uuid::new_v4();
        let fp = "test_fingerprint";
        let ip = Some("192.168.1.1".to_string());

        let session = Session::new(user_id, fp.to_string(), false, ip.clone(), None);

        assert_eq!(session.user_id(), user_id);
        assert_eq!(session.device_fingerprint(), fp);
        assert_eq!(session.ip_address(), ip.as_deref());
    }
}
