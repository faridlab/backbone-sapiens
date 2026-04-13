//! Mock Service Implementations
//!
//! Simple mock implementations for testing.

use backbone_sapiens::domain::services::{
    EmailService, PasswordPolicyService, SessionManagementService,
    SessionConfig, SessionValidationResult, SecurityMonitoringService,
};
use backbone_sapiens::domain::services::email_service::{EmailTemplate, EmailDeliveryResult, EmailDeliveryStatus};
use backbone_sapiens::domain::services::password_policy_service::{PasswordValidationResult, PasswordViolation};
use backbone_sapiens::domain::services::security_monitoring_service::{SecurityEvent, SecurityEventType};
use backbone_sapiens::domain::value_objects::{Email, DeviceFingerprint};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;

/// Mock Email Service
#[derive(Clone)]
pub struct MockEmailService {
    sent_emails: Arc<RwLock<Vec<EmailRecord>>>,
}

#[derive(Debug, Clone)]
pub struct EmailRecord {
    pub to: String,
    pub template: String,
    pub sent_at: chrono::DateTime<Utc>,
}

impl MockEmailService {
    pub fn new() -> Self {
        Self {
            sent_emails: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn get_sent_emails(&self) -> Vec<EmailRecord> {
        self.sent_emails.read().await.clone()
    }

    pub async fn clear(&self) {
        self.sent_emails.write().await.clear();
    }
}

impl Default for MockEmailService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmailService for MockEmailService {
    async fn send_email(
        &self,
        to: &Email,
        template: &EmailTemplate,
    ) -> Result<EmailDeliveryResult, Box<dyn std::error::Error + Send + Sync>> {
        let record = EmailRecord {
            to: to.to_string(),
            template: format!("{:?}", template),
            sent_at: Utc::now(),
        };

        self.sent_emails.write().await.push(record);

        Ok(EmailDeliveryResult {
            success: true,
            email_id: Uuid::new_v4().to_string(),
            status: EmailDeliveryStatus::Sent,
            error_message: None,
        })
    }
}

/// Mock Password Policy Service
#[derive(Clone)]
pub struct MockPasswordPolicyService {
    pub strict_mode: Arc<RwLock<bool>>,
}

impl MockPasswordPolicyService {
    pub fn new() -> Self {
        Self {
            strict_mode: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn set_strict_mode(&self, strict: bool) {
        *self.strict_mode.write().await = strict;
    }
}

impl Default for MockPasswordPolicyService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PasswordPolicyService for MockPasswordPolicyService {
    async fn validate_password(
        &self,
        password: &str,
        _user_context: Option<&backbone_sapiens::domain::services::password_policy_service::UserContext>,
    ) -> Result<PasswordValidationResult, Box<dyn std::error::Error + Send + Sync>> {
        let strict = *self.strict_mode.read().await;

        let mut violations = Vec::new();

        if password.len() < 8 {
            violations.push(PasswordViolation::TooShort);
        }

        if !password.chars().any(|c| c.is_ascii_uppercase()) {
            violations.push(PasswordViolation::NoUppercase);
        }

        if !password.chars().any(|c| c.is_ascii_lowercase()) {
            violations.push(PasswordViolation::NoLowercase);
        }

        if !password.chars().any(|c| c.is_ascii_digit()) {
            violations.push(PasswordViolation::NoDigit);
        }

        if strict && !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
            violations.push(PasswordViolation::NoSpecialChar);
        }

        if strict && password.len() < 12 {
            violations.push(PasswordViolation::TooShort);
        }

        let is_valid = violations.is_empty();

        Ok(PasswordValidationResult {
            is_valid,
            violations,
            strength: if is_valid { 100 } else { 50 },
            suggestions: if violations.is_empty() {
                vec![]
            } else {
                vec!["Use a stronger password".to_string()]
            },
        })
    }
}

/// Mock Security Monitoring Service
#[derive(Clone)]
pub struct MockSecurityMonitoringService {
    events: Arc<RwLock<Vec<SecurityEventRecord>>>,
    failed_attempts: Arc<RwLock<HashMap<String, u32>>>,
}

#[derive(Debug, Clone)]
pub struct SecurityEventRecord {
    pub event_type: String,
    pub user_id: Option<Uuid>,
    pub recorded_at: chrono::DateTime<Utc>,
}

impl MockSecurityMonitoringService {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            failed_attempts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_events(&self) -> Vec<SecurityEventRecord> {
        self.events.read().await.clone()
    }

    pub async fn set_failed_attempts(&self, email: &str, attempts: u32) {
        self.failed_attempts.write().await.insert(email.to_string(), attempts);
    }

    pub async fn clear(&self) {
        self.events.write().await.clear();
        self.failed_attempts.write().await.clear();
    }
}

impl Default for MockSecurityMonitoringService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SecurityMonitoringService for MockSecurityMonitoringService {
    async fn record_security_event(
        &self,
        event: SecurityEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let record = SecurityEventRecord {
            event_type: format!("{:?}", event.event_type),
            user_id: event.user_id,
            recorded_at: Utc::now(),
        };

        self.events.write().await.push(record);
        Ok(())
    }

    async fn check_account_lockout(
        &self,
        email: &str,
    ) -> Result<Option<chrono::DateTime<Utc>>, Box<dyn std::error::Error + Send + Sync>> {
        let attempts = self.failed_attempts.read().await.get(email).copied().unwrap_or(0);

        if attempts >= 5 {
            Ok(Some(Utc::now() + chrono::Duration::minutes(15)))
        } else {
            Ok(None)
        }
    }

    async fn record_failed_login(
        &self,
        email: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        *self.failed_attempts.write().await.entry(email.to_string()).or_insert(0) += 1;
        Ok(())
    }

    async fn reset_failed_attempts(
        &self,
        email: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.failed_attempts.write().await.remove(email);
        Ok(())
    }
}

/// Mock Session Management Service
#[derive(Clone)]
pub struct MockSessionManagementService {
    sessions: Arc<RwLock<HashMap<Uuid, SessionValidationResult>>>,
}

impl MockSessionManagementService {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_valid_session(&self, session_id: Uuid, user_id: Uuid) {
        self.sessions.write().await.insert(session_id, SessionValidationResult {
            valid: true,
            user_id,
            requires_reauth: false,
            security_alerts: vec![],
        });
    }

    pub async fn clear(&self) {
        self.sessions.write().await.clear();
    }
}

impl Default for MockSessionManagementService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SessionManagementService for MockSessionManagementService {
    async fn create_session(
        &self,
        _user_id: Uuid,
        _config: SessionConfig,
    ) -> Result<backbone_sapiens::domain::entity::Session, Box<dyn std::error::Error + Send + Sync>> {
        Ok(backbone_sapiens::domain::entity::Session::new(
            Uuid::new_v4(),
            DeviceFingerprint::generate().to_string(),
            false,
            None,
            None,
        ))
    }

    async fn validate_session(
        &self,
        session_id: Uuid,
        _device_fingerprint: Option<&DeviceFingerprint>,
    ) -> Result<SessionValidationResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.sessions.read().await.get(&session_id).cloned().unwrap_or(SessionValidationResult {
            valid: false,
            user_id: Uuid::nil(),
            requires_reauth: false,
            security_alerts: vec!["Session not found".to_string()],
        }))
    }

    async fn get_session(
        &self,
        _session_id: Uuid,
        _user_id: Uuid,
    ) -> Result<Option<backbone_sapiens::domain::entity::Session>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }

    async fn terminate_session(
        &self,
        _session_id: Uuid,
        _user_id: Uuid,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        Ok(true)
    }

    async fn terminate_all_sessions(
        &self,
        _user_id: Uuid,
        _except_session_id: Option<Uuid>,
    ) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(1)
    }

    async fn get_user_sessions(
        &self,
        _user_id: Uuid,
    ) -> Result<Vec<backbone_sapiens::domain::entity::Session>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![])
    }
}
