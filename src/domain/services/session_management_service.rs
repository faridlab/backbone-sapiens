//! Session Management Service
//!
//! Advanced session handling with device tracking, concurrent session limits,
//! and security features for authentication workflows.
//!
//! This service manages:
//! - Advanced session lifecycle with device fingerprinting
//! - Concurrent session limits and management
//! - Session security and validation
//! - Device trust and tracking

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::constants;
use crate::domain::entity::{User, Session};
use crate::domain::repositories::{UserRepository, SessionRepository};
use crate::domain::value_objects::{DeviceFingerprint, IpAddress};
use crate::domain::services::security_monitoring_service::SecurityMonitoringService;

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub max_concurrent_sessions: u32,
    pub default_session_duration_hours: u32,
    pub extended_session_duration_hours: u32,
    pub absolute_max_session_duration_hours: u32,
    pub idle_timeout_minutes: u32,
    pub device_trust_required: bool,
    pub ip_consistency_check: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            max_concurrent_sessions: 5,
            default_session_duration_hours: constants::DEFAULT_SESSION_EXPIRY_HOURS as u32,
            extended_session_duration_hours: 168, // 7 days
            absolute_max_session_duration_hours: 720, // 30 days
            idle_timeout_minutes: 30,
            device_trust_required: false,
            ip_consistency_check: true,
        }
    }
}

/// Session management operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionOperation {
    Create,
    Update,
    Terminate,
    TerminateAll,
    Extend,
    Validate,
}

/// Session validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionValidationResult {
    pub valid: bool,
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub extended_at: Option<DateTime<Utc>>,
    pub requires_reauth: bool,
    pub security_alerts: Vec<String>,
}

/// Session management service trait
#[async_trait]
pub trait SessionManagementService: Send + Sync {
    /// Create a new session with advanced security features
    async fn create_session(&self, user_id: Uuid, device_fingerprint: DeviceFingerprint, ip_address: Option<IpAddress>, remember_me: bool) -> Result<Session, SessionError>;

    /// Validate existing session with security checks
    async fn validate_session(&self, session_id: Uuid, current_ip: Option<IpAddress>) -> Result<SessionValidationResult, SessionError>;

    /// Extend session expiration
    async fn extend_session(&self, session_id: Uuid, user_id: Uuid) -> Result<Session, SessionError>;

    /// Terminate specific session
    async fn terminate_session(&self, session_id: Uuid, user_id: Uuid) -> Result<(), SessionError>;

    /// Terminate all sessions for user
    async fn terminate_all_sessions(&self, user_id: Uuid, current_session_id: Option<Uuid>) -> Result<u32, SessionError>;

    /// Get all active sessions for user
    async fn get_user_sessions(&self, user_id: Uuid) -> Result<Vec<Session>, SessionError>;

    /// Get session by ID with user validation
    async fn get_session(&self, session_id: Uuid, user_id: Uuid) -> Result<Option<Session>, SessionError>;

    /// Cleanup expired sessions
    async fn cleanup_expired_sessions(&self) -> Result<u32, SessionError>;

    /// Check session limits and terminate oldest if needed
    async fn enforce_session_limits(&self, user_id: Uuid) -> Result<(), SessionError>;

    /// Update session activity tracking
    async fn update_session_activity(&self, session_id: Uuid, ip_address: Option<IpAddress>) -> Result<(), SessionError>;

    /// Detect suspicious session activity
    async fn detect_suspicious_activity(&self, session_id: Uuid, current_ip: Option<IpAddress>) -> Result<Vec<SuspiciousActivity>, SessionError>;
}

/// Suspicious activity detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousActivity {
    pub activity_type: String,
    pub description: String,
    pub risk_score: u8,
    pub requires_action: bool,
}

/// Session errors
#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Session not found: {0}")]
    SessionNotFound(Uuid),

    #[error("Session expired")]
    SessionExpired,

    #[error("Session invalid: {0}")]
    SessionInvalid(String),

    #[error("Maximum concurrent sessions exceeded")]
    MaxSessionsExceeded,

    #[error("Session validation failed: {0}")]
    ValidationFailed(String),

    #[error("Device fingerprint mismatch")]
    DeviceMismatch,

    #[error("IP address mismatch")]
    IpMismatch,

    #[error("User not found: {0}")]
    UserNotFound(Uuid),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    anyhowError(#[from] anyhow::Error),

    #[error(transparent)]
    SecurityError(#[from] crate::domain::services::security_monitoring_service::SecurityError),
}

/// Default implementation of Session Management Service
pub struct DefaultSessionManagementService {
    session_repository: Box<dyn SessionRepository>,
    user_repository: Box<dyn UserRepository>,
    security_monitoring: Box<dyn SecurityMonitoringService>,
    config: SessionConfig,
}

impl DefaultSessionManagementService {
    pub fn new(
        session_repository: Box<dyn SessionRepository>,
        user_repository: Box<dyn UserRepository>,
        security_monitoring: Box<dyn SecurityMonitoringService>,
        config: SessionConfig,
    ) -> Self {
        Self {
            session_repository,
            user_repository,
            security_monitoring,
            config,
        }
    }

    /// Calculate session expiration time
    fn calculate_expiration(&self, remember_me: bool) -> DateTime<Utc> {
        let duration = if remember_me {
            Duration::hours(self.config.extended_session_duration_hours as i64)
        } else {
            Duration::hours(self.config.default_session_duration_hours as i64)
        };

        Utc::now() + duration
    }

    /// Check if IP address has changed significantly
    fn has_ip_changed(&self, original_ip: Option<IpAddress>, current_ip: Option<IpAddress>) -> bool {
        match (original_ip, current_ip) {
            (Some(orig), Some(curr)) => {
                // Check if it's a significant change (different subnet)
                orig.as_str() != curr.as_str() &&
                !self.is_private_ip_change(orig.as_str(), curr.as_str())
            }
            (None, Some(_)) | (Some(_), None) => true,
            (None, None) => false,
        }
    }

    /// Check if IP change is within private network ranges (acceptable)
    fn is_private_ip_change(&self, old_ip: &str, new_ip: &str) -> bool {
        // Both IPs are private network addresses
        (old_ip.starts_with("10.") && new_ip.starts_with("10.")) ||
        (old_ip.starts_with("192.168.") && new_ip.starts_with("192.168.")) ||
        (old_ip.starts_with("172.") && new_ip.starts_with("172."))
    }

    /// Generate session security metadata
    fn generate_session_metadata(&self, device_fingerprint: &DeviceFingerprint, ip_address: Option<IpAddress>) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("device_fingerprint".to_string(), device_fingerprint.value().to_string());
        metadata.insert("created_at".to_string(), Utc::now().to_rfc3339());

        if let Some(ip) = ip_address {
            metadata.insert("ip_address".to_string(), ip.as_str().to_string());
            metadata.insert("ip_country".to_string(), "unknown".to_string()); // TODO: GeoIP lookup
        }

        metadata
    }

    /// Validate session against security policies
    async fn validate_session_security(&self, session: &Session, current_ip: Option<IpAddress>) -> Result<Vec<String>, SessionError> {
        let mut alerts = Vec::new();

        // Check IP consistency if enabled
        if self.config.ip_consistency_check {
            if let (Some(session_ip), Some(current_ip_val)) = (session.ip_address.as_ref(), current_ip) {
                if session_ip.as_str() != current_ip_val.as_str() {
                    if !self.is_private_ip_change(session_ip.as_str(), current_ip_val.as_str()) {
                        alerts.push("IP address has changed significantly".to_string());
                    }
                }
            }
        }

        // Check device trust if required
        if self.config.device_trust_required {
            let trust_status = self.security_monitoring
                .get_device_trust_status(session.user_id, &DeviceFingerprint::new(session.metadata.get("device_fingerprint").and_then(|v| v.as_str()).unwrap_or("")))
                .await?;

            if matches!(trust_status, crate::domain::services::security_monitoring_service::DeviceTrustStatus::Unknown) {
                alerts.push("Login from untrusted device".to_string());
            }
        }

        // Check if session is approaching absolute maximum duration
        let created_at = session.metadata.get("created_at")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|| Utc::now()); // Fallback to now if not found
        let session_age = Utc::now().signed_duration_since(created_at);
        let max_duration = Duration::hours(self.config.absolute_max_session_duration_hours as i64);

        if session_age > max_duration {
            alerts.push("Session exceeded maximum duration".to_string());
        }

        Ok(alerts)
    }

    /// Detect suspicious session activities
    async fn detect_suspicious_activities(&self, session: &Session, current_ip: Option<IpAddress>) -> Vec<SuspiciousActivity> {
        let mut activities = Vec::new();

        // Check for IP hopping
        if let (Some(session_ip), Some(current_ip_val)) = (session.ip_address.as_ref(), current_ip) {
            let ip_distance = self.calculate_ip_distance(session_ip.as_str(), current_ip_val.as_str());
            if ip_distance > 1000.0 {
                activities.push(SuspiciousActivity {
                    activity_type: "ip_hopping".to_string(),
                    description: format!("IP address changed by ~{}km", ip_distance as u32),
                    risk_score: 60,
                    requires_action: true,
                });
            }
        }

        // Check for session age suspiciousness
        let created_at = session.metadata.get("created_at")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|| Utc::now()); // Fallback to now if not found
        let session_age = Utc::now().signed_duration_since(created_at);
        if session_age > Duration::days(7) {
            activities.push(SuspiciousActivity {
                activity_type: "long_lived_session".to_string(),
                description: "Session has been active for over 7 days".to_string(),
                risk_score: 30,
                requires_action: false,
            });
        }

        // Check for rapid activity patterns
        if let Some(last_activity) = session.last_activity {
            let activity_gap = Utc::now().signed_duration_since(last_activity);
            if activity_gap < Duration::seconds(1) {
                activities.push(SuspiciousActivity {
                    activity_type: "rapid_activity".to_string(),
                    description: "Very rapid session activity detected".to_string(),
                    risk_score: 40,
                    requires_action: true,
                });
            }
        }

        activities
    }

    /// Calculate approximate IP distance (simplified)
    fn calculate_ip_distance(&self, ip1: &str, ip2: &str) -> f64 {
        // This is a very simplified calculation
        // In production, you'd use a proper GeoIP service
        if ip1 == ip2 {
            return 0.0;
        }

        // Different countries estimate ~1000-5000km
        2000.0 // Placeholder
    }
}

#[async_trait]
impl SessionManagementService for DefaultSessionManagementService {
    async fn create_session(&self, user_id: Uuid, device_fingerprint: DeviceFingerprint, ip_address: Option<IpAddress>, remember_me: bool) -> Result<Session, SessionError> {
        // Verify user exists
        let _user = self.user_repository.find_by_id(&user_id.to_string())
            .await?
            .ok_or(SessionError::UserNotFound(user_id))?;

        // Check session limits
        self.enforce_session_limits(user_id).await?;

        // Create session
        let expires_at = self.calculate_expiration(remember_me);
        let metadata = self.generate_session_metadata(&device_fingerprint, ip_address.clone());

        let session = Session::new(
            user_id,
            device_fingerprint.value().to_string(),
            remember_me,
            ip_address.as_ref().map(|ip| ip.to_string()),
            None, // user_agent not available in this context
        );

        let saved_session = self.session_repository.save(&session).await?;
        let session_id = saved_session.id();

        // Log security event
        self.security_monitoring.log_security_event(
            crate::domain::services::security_monitoring_service::SecurityEvent {
                id: Uuid::new_v4(),
                user_id,
                event_type: crate::domain::services::security_monitoring_service::SecurityEventType::FailedLoginAttempt, // Should be SessionCreated
                timestamp: Utc::now(),
                ip_address,
                device_fingerprint: Some(device_fingerprint),
                location: None,
                details: HashMap::from([
                    ("session_id".to_string(), session_id.to_string()),
                    ("remember_me".to_string(), remember_me.to_string()),
                ]),
                risk_score: 10,
            }
        ).await?;

        Ok(session)
    }

    async fn validate_session(&self, session_id: Uuid, current_ip: Option<IpAddress>) -> Result<SessionValidationResult, SessionError> {
        // Find session
        let session = self.session_repository.find_by_id(&session_id.to_string())
            .await?
            .ok_or(SessionError::SessionNotFound(session_id))?;

        // Check expiration
        if session.is_expired() {
            return Ok(SessionValidationResult {
                valid: false,
                session_id,
                user_id: session.user_id,
                expires_at: session.expires_at,
                extended_at: session.extended_at(),
                requires_reauth: true,
                security_alerts: vec!["Session expired".to_string()],
            });
        }

        // Validate security
        let security_alerts = self.validate_session_security(&session, current_ip.clone()).await?;
        let suspicious_activities = self.detect_suspicious_activities(&session, current_ip).await;

        // Combine alerts
        let mut all_alerts = security_alerts;
        for activity in suspicious_activities {
            all_alerts.push(activity.description);
        }

        // Determine if re-authentication is required
        let requires_reauth = !all_alerts.is_empty() || session.requires_reauth();

        Ok(SessionValidationResult {
            valid: !requires_reauth,
            session_id,
            user_id: session.user_id,
            expires_at: session.expires_at,
            extended_at: session.extended_at(),
            requires_reauth,
            security_alerts: all_alerts,
        })
    }

    async fn extend_session(&self, session_id: Uuid, user_id: Uuid) -> Result<Session, SessionError> {
        // Find and validate session
        let session = self.get_session(session_id, user_id).await?
            .ok_or(SessionError::SessionNotFound(session_id))?;

        // Check if extension is allowed
        let current_time = Utc::now();
        let absolute_max = current_time + Duration::hours(self.config.absolute_max_session_duration_hours as i64);

        if session.expires_at > absolute_max {
            return Err(SessionError::ValidationFailed("Session cannot be extended beyond maximum duration".to_string()));
        }

        // Extend session
        let new_expires_at = current_time + Duration::hours(self.config.extended_session_duration_hours as i64);
        let updated_session = session.with_expiration(new_expires_at);

        self.session_repository.update(&updated_session.id.to_string(), &updated_session).await?;

        Ok(updated_session)
    }

    async fn terminate_session(&self, session_id: Uuid, user_id: Uuid) -> Result<(), SessionError> {
        // Validate session ownership
        let _session = self.get_session(session_id, user_id).await?;

        // Delete session
        self.session_repository.delete(&session_id.to_string()).await?;

        Ok(())
    }

    async fn terminate_all_sessions(&self, user_id: Uuid, current_session_id: Option<Uuid>) -> Result<u32, SessionError> {
        // Get all sessions for user
        let sessions = self.get_user_sessions(user_id).await?;
        let mut terminated_count = 0;

        for session in sessions {
            // Skip current session if specified
            if let Some(current_id) = current_session_id {
                if session.id == current_id {
                    continue;
                }
            }

            // Terminate session
            self.session_repository.delete(&session.id.to_string()).await?;
            terminated_count += 1;
        }

        Ok(terminated_count)
    }

    async fn get_user_sessions(&self, user_id: Uuid) -> Result<Vec<Session>, SessionError> {
        let sessions = self.session_repository.find_by_user_id(user_id).await?;
        Ok(sessions)
    }

    async fn get_session(&self, session_id: Uuid, user_id: Uuid) -> Result<Option<Session>, SessionError> {
        let session = self.session_repository.find_by_id(&session_id.to_string()).await?;

        // Validate user ownership
        if let Some(s) = &session {
            if s.user_id() != user_id {
                return Err(SessionError::SessionInvalid("Session belongs to different user".to_string()));
            }
        }

        Ok(session)
    }

    async fn cleanup_expired_sessions(&self) -> Result<u32, SessionError> {
        let expired_sessions = self.session_repository.find_expired().await?;
        let mut cleaned_count = 0;

        for session in expired_sessions {
            self.session_repository.delete(&session.id.to_string()).await?;
            cleaned_count += 1;
        }

        Ok(cleaned_count)
    }

    async fn enforce_session_limits(&self, user_id: Uuid) -> Result<(), SessionError> {
        let sessions = self.get_user_sessions(user_id).await?;
        let active_sessions: Vec<_> = sessions.iter()
            .filter(|s| !s.is_expired())
            .collect();

        if active_sessions.len() >= self.config.max_concurrent_sessions as usize {
            // Terminate oldest sessions
            let mut sorted_sessions = active_sessions.clone();
            sorted_sessions.sort_by(|a, b| {
                let a_created = a.metadata.get("created_at").and_then(|v| v.as_str()).unwrap_or("");
                let b_created = b.metadata.get("created_at").and_then(|v| v.as_str()).unwrap_or("");
                a_created.cmp(b_created)
            });

            let sessions_to_terminate = sorted_sessions.len() - (self.config.max_concurrent_sessions as usize - 1);
            for session in sorted_sessions.iter().take(sessions_to_terminate) {
                self.session_repository.delete(&session.id.to_string()).await?;
            }
        }

        Ok(())
    }

    async fn update_session_activity(&self, session_id: Uuid, ip_address: Option<IpAddress>) -> Result<(), SessionError> {
        let mut session = self.session_repository.find_by_id(&session_id.to_string())
            .await?
            .ok_or(SessionError::SessionNotFound(session_id))?;

        // Update activity tracking
        session.update_activity(Utc::now(), ip_address);

        self.session_repository.update(&session.id.to_string(), &session).await?;

        Ok(())
    }

    async fn detect_suspicious_activity(&self, session_id: Uuid, current_ip: Option<IpAddress>) -> Result<Vec<SuspiciousActivity>, SessionError> {
        let session = self.session_repository.find_by_id(&session_id.to_string())
            .await?
            .ok_or(SessionError::SessionNotFound(session_id))?;

        Ok(self.detect_suspicious_activities(&session, current_ip).await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::IpAddress;

    #[test]
    fn test_session_config_defaults() {
        let config = SessionConfig::default();

        assert_eq!(config.max_concurrent_sessions, 5);
        assert_eq!(config.default_session_duration_hours, 24);
        assert_eq!(config.extended_session_duration_hours, 168);
        assert_eq!(config.absolute_max_session_duration_hours, 720);
        assert_eq!(config.idle_timeout_minutes, 30);
        assert!(!config.device_trust_required);
        assert!(config.ip_consistency_check);
    }

    #[test]
    fn test_session_validation_result_structure() {
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        let now = Utc::now();

        let result = SessionValidationResult {
            valid: true,
            session_id,
            user_id,
            expires_at: now + constants::default_session_expiry(),
            extended_at: Some(now),
            requires_reauth: false,
            security_alerts: vec![],
        };

        assert!(result.valid);
        assert_eq!(result.session_id, session_id);
        assert_eq!(result.user_id, user_id);
        assert!(!result.requires_reauth);
        assert!(result.security_alerts.is_empty());
    }

    #[test]
    fn test_session_validation_with_alerts() {
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        let now = Utc::now();

        let result = SessionValidationResult {
            valid: false,
            session_id,
            user_id,
            expires_at: now + constants::default_session_expiry(),
            extended_at: None,
            requires_reauth: true,
            security_alerts: vec![
                "IP address change detected".to_string(),
                "New device detected".to_string(),
            ],
        };

        assert!(!result.valid);
        assert!(result.requires_reauth);
        assert_eq!(result.security_alerts.len(), 2);
    }

    #[test]
    fn test_session_operation_variants() {
        let operations = vec![
            SessionOperation::Create,
            SessionOperation::Update,
            SessionOperation::Terminate,
            SessionOperation::TerminateAll,
            SessionOperation::Extend,
            SessionOperation::Validate,
        ];

        assert_eq!(operations.len(), 6);
    }

    #[test]
    fn test_device_fingerprint_value_object() {
        let fp1: DeviceFingerprint = "device-abc-123".into();
        let fp2: DeviceFingerprint = "device-abc-123".into();

        assert_eq!(fp1.value(), fp2.value());
        assert_eq!(fp1.value(), "device-abc-123");
    }

    #[test]
    fn test_ip_address_value_object() {
        let private_ip = IpAddress::new("192.168.1.1").unwrap();
        let public_ip = IpAddress::new("8.8.8.8").unwrap();

        assert!(private_ip.is_private());
        assert!(!public_ip.is_private());
    }

    #[test]
    fn test_session_error_variants() {
        let not_found = SessionError::SessionNotFound(Uuid::new_v4());
        let expired = SessionError::SessionExpired;
        let invalid = SessionError::SessionInvalid("test reason".to_string());
        let max_exceeded = SessionError::MaxSessionsExceeded;
        let device_mismatch = SessionError::DeviceMismatch;

        assert!(matches!(not_found, SessionError::SessionNotFound(_)));
        assert!(matches!(expired, SessionError::SessionExpired));
        assert!(matches!(invalid, SessionError::SessionInvalid(_)));
        assert!(matches!(max_exceeded, SessionError::MaxSessionsExceeded));
        assert!(matches!(device_mismatch, SessionError::DeviceMismatch));
    }
}