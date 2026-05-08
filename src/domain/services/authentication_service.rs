//! Authentication Service
//!
//! Core authentication workflows including login, registration, email verification,
//! and security monitoring with account lockout protection.
//!
//! This service handles the business logic for user authentication, including:
//! - Advanced login with MFA support and device tracking
//! - Email verification workflows
//! - Account lockout protection (5 attempts = 15 min)
//! - Device fingerprinting and security monitoring

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use backbone_auth::jwt::{JwtService, Claims};
use backbone_auth::traits::RefreshTokenClaims;

use crate::domain::entity::{User, Session, MFADevice, MFADeviceType, MFADeviceStatus, EmailVerificationToken};
use crate::domain::entity::EmailVerificationType;
use crate::domain::entity::UserDomainEvent;
use crate::domain::repositories::{UserRepository, SessionRepository, MFADeviceRepository, EmailVerificationTokenRepository};
use crate::domain::value_objects::{Email, PasswordHash, DeviceFingerprint};
use backbone_messaging::EventBus;
use crate::domain::services::email_service::EmailService;
use crate::domain::services::security_monitoring_service::{SecurityMonitoringService, SecurityEvent, SecurityEventType};
use crate::domain::services::password_policy_service::PasswordPolicyService;
use sha2::{Sha256, Digest};

/// Authentication result with session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    pub success: bool,
    pub user_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub requires_mfa: bool,
    pub mfa_methods: Vec<MfaMethod>,
    pub error_message: Option<String>,
    pub lockout_until: Option<DateTime<Utc>>,
}

/// MFA method types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    Totp,
    Sms,
    Email,
    BackupCode,
}

/// Login request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: Email,
    pub password: String,
    pub remember_me: bool,
    pub device_fingerprint: Option<DeviceFingerprint>,
    pub mfa_code: Option<String>,
}

/// Registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub email: Email,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub accept_terms: bool,
    pub device_fingerprint: Option<DeviceFingerprint>,
}

/// Email verification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerificationRequest {
    pub email: Email,
    pub verification_token: String,
}

/// Authentication Service trait
#[async_trait]
pub trait AuthenticationService: Send + Sync {
    /// Register a new user with email verification
    async fn register_user(&self, request: RegistrationRequest) -> Result<AuthenticationResult, AuthenticationError>;

    /// Authenticate user with advanced security features
    async fn authenticate_user(&self, request: LoginRequest) -> Result<AuthenticationResult, AuthenticationError>;

    /// Verify email address
    async fn verify_email(&self, request: EmailVerificationRequest) -> Result<AuthenticationResult, AuthenticationError>;

    /// Resend email verification
    async fn resend_verification(&self, email: Email) -> Result<(), AuthenticationError>;

    /// Logout user with session cleanup
    async fn logout_user(&self, session_id: Uuid, device_untrust: bool) -> Result<(), AuthenticationError>;

    /// Refresh JWT token with security validation
    async fn refresh_token(&self, refresh_token: String) -> Result<TokenRefreshResult, AuthenticationError>;
}

/// Token refresh result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRefreshResult {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user_id: Uuid,
}

/// Authentication errors
#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Account locked until {0}")]
    AccountLocked(DateTime<Utc>),

    #[error("Account not verified")]
    AccountNotVerified,

    #[error("Account suspended")]
    AccountSuspended,

    #[error("Too many failed attempts. Try again in {0} minutes")]
    TooManyAttempts(i64),

    #[error("MFA required")]
    MfaRequired,

    #[error("Invalid MFA code")]
    InvalidMfaCode,

    #[error("Invalid email verification token")]
    InvalidVerificationToken,

    #[error("Email already registered")]
    EmailAlreadyRegistered,

    #[error("Password does not meet security requirements")]
    PasswordTooWeak,

    #[error("Device not trusted")]
    DeviceNotTrusted,

    #[error("Session not found")]
    SessionNotFound,

    #[error("Invalid refresh token")]
    InvalidRefreshToken,

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    EmailError(#[from] anyhow::Error),

    #[error(transparent)]
    SecurityError(#[from] crate::domain::services::security_monitoring_service::SecurityError),
}

/// Default implementation of Authentication Service
pub struct DefaultAuthenticationService {
    user_repository: Box<dyn UserRepository>,
    session_repository: Box<dyn SessionRepository>,
    mfa_device_repository: Box<dyn MFADeviceRepository>,
    email_verification_repository: Box<dyn EmailVerificationTokenRepository>,
    email_service: Box<dyn EmailService>,
    security_monitoring: Box<dyn SecurityMonitoringService>,
    password_policy_service: Box<dyn PasswordPolicyService>,
    jwt_service: JwtService,
    event_bus: Option<EventBus<UserDomainEvent>>,
}

impl DefaultAuthenticationService {
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        session_repository: Box<dyn SessionRepository>,
        mfa_device_repository: Box<dyn MFADeviceRepository>,
        email_verification_repository: Box<dyn EmailVerificationTokenRepository>,
        email_service: Box<dyn EmailService>,
        security_monitoring: Box<dyn SecurityMonitoringService>,
        password_policy_service: Box<dyn PasswordPolicyService>,
        jwt_secret: &str,
        event_bus: Option<EventBus<UserDomainEvent>>,
    ) -> Self {
        Self {
            user_repository,
            session_repository,
            mfa_device_repository,
            email_verification_repository,
            email_service,
            security_monitoring,
            password_policy_service,
            jwt_service: JwtService::new(jwt_secret),
            event_bus,
        }
    }

    /// Check if account is locked due to too many failed attempts.
    /// Delegates to SecurityMonitoringService which tracks lockout timestamps.
    async fn is_account_locked(&self, user_id: Uuid) -> Result<bool, AuthenticationError> {
        self.security_monitoring.is_user_locked(user_id).await
            .map_err(AuthenticationError::from)
    }

    /// Publish a domain event to the event bus (fire-and-forget).
    /// If no event bus is configured, the event is silently dropped.
    async fn publish_event(&self, event: UserDomainEvent) {
        if let Some(bus) = &self.event_bus {
            let _ = bus.publish(event).await;
        }
    }

    /// Generate secure JWT tokens
    fn generate_tokens(&self, user_id: Uuid) -> Result<TokenRefreshResult, AuthenticationError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;

        // Generate access token (15 minutes)
        let access_exp = (now + Duration::minutes(15)).timestamp() as usize;
        let access_claims = Claims {
            sub: user_id.to_string(),
            exp: access_exp,
            iat,
            iss: "sapiens".to_string(),
        };

        let access_token = self.jwt_service.create_token(&access_claims)
            .map_err(|e| AuthenticationError::EmailError(e))?;

        // Generate refresh token (7 days)
        let refresh_exp = (now + Duration::days(7)).timestamp() as usize;
        let refresh_claims = RefreshTokenClaims {
            sub: user_id.to_string(),
            exp: refresh_exp,
            iat,
            iss: "sapiens".to_string(),
            token_type: "refresh".to_string(),
        };

        let refresh_token = self.jwt_service.create_refresh_token(&refresh_claims)
            .map_err(|e| AuthenticationError::EmailError(e))?;

        Ok(TokenRefreshResult {
            access_token,
            refresh_token,
            expires_in: 900, // 15 minutes
            user_id,
        })
    }

    /// Create device fingerprint if not provided
    fn create_device_fingerprint(&self, request: &LoginRequest) -> DeviceFingerprint {
        request.device_fingerprint
            .clone()
            .unwrap_or_else(|| DeviceFingerprint::generate())
    }

    /// Validate password using PasswordPolicyService
    async fn validate_password(&self, password: &str, _user_id: Uuid) -> Result<(), AuthenticationError> {
        let validation_result = self.password_policy_service
            .validate_password(password)
            .await;

        if !validation_result.is_valid {
            return Err(AuthenticationError::PasswordTooWeak);
        }

        Ok(())
    }

    /// Derive available MFA methods from user's actual enrolled devices.
    /// Only includes active, verified, non-locked devices.
    fn derive_mfa_methods(devices: &[MFADevice]) -> Vec<MfaMethod> {
        let mut has_totp = false;
        let mut has_sms = false;
        let mut has_email = false;
        let mut methods = Vec::new();

        for device in devices {
            if !device.is_active || device.is_locked || device.verified_at.is_none() {
                continue;
            }
            if device.status != MFADeviceStatus::Active {
                continue;
            }

            match device.device_type {
                MFADeviceType::Totp if !has_totp => {
                    methods.push(MfaMethod::Totp);
                    has_totp = true;
                }
                MFADeviceType::Sms if !has_sms => {
                    methods.push(MfaMethod::Sms);
                    has_sms = true;
                }
                MFADeviceType::Email if !has_email => {
                    methods.push(MfaMethod::Email);
                    has_email = true;
                }
                _ => {}
            }
        }

        // Always offer backup codes when any MFA device is active
        if !methods.is_empty() {
            methods.push(MfaMethod::BackupCode);
        }

        methods
    }

    /// Verify MFA code against user's active devices.
    ///
    /// For TOTP: generates expected codes using SHA-256 hash with ±1 time window
    /// (30-second steps) and compares against the provided code.
    async fn verify_mfa_code(&self, user_id: Uuid, code: &str, devices: &[MFADevice]) -> Result<bool, AuthenticationError> {
        // Code must be exactly 6 ASCII digits
        if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
            return Ok(false);
        }

        let active_devices: Vec<&MFADevice> = devices.iter()
            .filter(|d| d.is_active && !d.is_locked && d.verified_at.is_some())
            .filter(|d| d.status == MFADeviceStatus::Active)
            .collect();

        if active_devices.is_empty() {
            return Ok(false);
        }

        // Try verification against each active TOTP device
        let now = Utc::now().timestamp();
        let time_step: i64 = 30;

        for device in &active_devices {
            if device.device_type != MFADeviceType::Totp {
                continue;
            }
            let secret = match &device.totp_secret {
                Some(s) => s,
                None => continue,
            };

            // Check current window and ±1 for clock drift
            for offset in [-1i64, 0, 1] {
                let counter = (now / time_step) + offset;
                let expected = Self::generate_totp_code(secret, counter);
                if expected == code {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Build a security event with common fields pre-filled.
    fn build_security_event(
        user_id: Uuid,
        event_type: SecurityEventType,
        device_fingerprint: Option<DeviceFingerprint>,
        details: HashMap<String, String>,
        risk_score: u8,
    ) -> SecurityEvent {
        SecurityEvent {
            id: Uuid::new_v4(),
            user_id,
            event_type,
            timestamp: Utc::now(),
            ip_address: None,
            device_fingerprint,
            location: None,
            details,
            risk_score,
        }
    }

    /// Generate a 6-digit TOTP code using SHA-256 hash of secret + counter.
    ///
    /// Note: This uses a simplified TOTP algorithm based on SHA-256.
    /// For full RFC 6238 compliance (HMAC-SHA1, base32 secrets), add the `totp-rs` crate.
    fn generate_totp_code(secret: &str, counter: i64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        hasher.update(counter.to_be_bytes());
        let hash = hasher.finalize();

        // Dynamic truncation: use last nibble as offset
        let offset = (hash[hash.len() - 1] & 0x0f) as usize;
        let code_bytes = &hash[offset..offset + 4];
        let code_u32 = u32::from_be_bytes([
            code_bytes[0] & 0x7f,
            code_bytes[1],
            code_bytes[2],
            code_bytes[3],
        ]);

        format!("{:06}", code_u32 % 1_000_000)
    }
}

#[async_trait]
impl AuthenticationService for DefaultAuthenticationService {
    async fn register_user(&self, request: RegistrationRequest) -> Result<AuthenticationResult, AuthenticationError> {
        // Check if email already exists
        if self.user_repository.exists_by_email(request.email.as_str()).await? {
            return Err(AuthenticationError::EmailAlreadyRegistered);
        }

        // Create new user (fail fast if password hashing fails)
        // Note: Password hashing errors are rare (indicates system issues like RNG failure)
        let user = User::new(
            request.email.to_string(),
            request.password.clone(),
            request.first_name.clone(),
            request.last_name.clone(),
        ).map_err(|hash_error| {
            tracing::error!("Password hashing failed during registration: {}", hash_error);
            AuthenticationError::PasswordTooWeak
        })?;

        let saved_user = self.user_repository.save(&user).await?;
        let user_id = *saved_user.id();
        let registered_at = Utc::now();

        // Validate password strength using the newly created user_id
        self.validate_password(&request.password, user_id).await?;

        // Create email verification token
        let verification_token = EmailVerificationToken::new_account_creation(user_id, request.email.to_string());
        self.email_verification_repository.save(&verification_token).await?;

        // Send verification email
        self.email_service.send_verification_email(
            &request.email,
            verification_token.token(),
        ).await.map_err(|e| AuthenticationError::EmailError(anyhow::anyhow!("Failed to send verification email: {}", e)))?;

        // Publish user registered event
        self.publish_event(UserDomainEvent::Created {
            user_id: user_id.to_string(),
            occurred_at: registered_at,
        }).await;

        Ok(AuthenticationResult {
            success: true,
            user_id: Some(user_id),
            session_id: None,
            requires_mfa: false,
            mfa_methods: vec![],
            error_message: None,
            lockout_until: None,
        })
    }

    async fn authenticate_user(&self, request: LoginRequest) -> Result<AuthenticationResult, AuthenticationError> {
        // Find user by email
        let user = self.user_repository.find_by_email(request.email.as_str())
            .await?
            .ok_or(AuthenticationError::InvalidCredentials)?;

        // Check if account is locked
        if self.is_account_locked(user.id).await? {
            return Err(AuthenticationError::AccountLocked(Utc::now() + Duration::minutes(5)));
        }

        // Check account status
        match user.status {
            crate::domain::entity::UserStatus::PendingVerification => {
                return Err(AuthenticationError::AccountNotVerified);
            }
            crate::domain::entity::UserStatus::Suspended => {
                return Err(AuthenticationError::AccountSuspended);
            }
            crate::domain::entity::UserStatus::Inactive => {
                return Err(AuthenticationError::InvalidCredentials);
            }
            _ => {}
        }

        // Verify password
        if !user.verify_password(&request.password) {
            let failed_at = Utc::now();

            // Record failed attempt
            self.security_monitoring.record_failed_attempt(user.id).await?;

            // Publish login failed event
            self.publish_event(UserDomainEvent::LoginFailed {
                user_id: user.id.to_string(),
                reason: "invalid_credentials".to_string(),
                occurred_at: failed_at,
            }).await;

            return Err(AuthenticationError::InvalidCredentials);
        }

        // Check if MFA is required
        let mfa_devices = self.mfa_device_repository.find_by_user_id(user.id).await?;
        let requires_mfa = !mfa_devices.is_empty();

        if requires_mfa && request.mfa_code.is_none() {
            return Ok(AuthenticationResult {
                success: false,
                user_id: Some(user.id),
                session_id: None,
                requires_mfa: true,
                mfa_methods: Self::derive_mfa_methods(&mfa_devices),
                error_message: Some("MFA code required".to_string()),
                lockout_until: None,
            });
        }

        // Verify MFA code if provided
        if requires_mfa {
            let mfa_code = request.mfa_code
                .as_ref()
                .ok_or(AuthenticationError::MfaRequired)?;

            let mfa_verified = self.verify_mfa_code(user.id, mfa_code, &mfa_devices).await?;

            if !mfa_verified {
                // Record failed MFA attempt (counts toward lockout)
                self.security_monitoring.record_failed_attempt(user.id).await?;

                // Log MFA verification failure
                let _ = self.security_monitoring.log_security_event(
                    Self::build_security_event(
                        user.id,
                        SecurityEventType::MfaVerificationFailed,
                        request.device_fingerprint.clone(),
                        HashMap::from([("reason".to_string(), "invalid_mfa_code".to_string())]),
                        40,
                    )
                ).await;

                return Err(AuthenticationError::InvalidMfaCode);
            }

            // Log successful MFA verification
            let _ = self.security_monitoring.log_security_event(
                Self::build_security_event(
                    user.id,
                    SecurityEventType::MfaVerificationSucceeded,
                    request.device_fingerprint.clone(),
                    HashMap::from([("method".to_string(), "totp".to_string())]),
                    0,
                )
            ).await;
        }

        // Create session
        let device_fingerprint = self.create_device_fingerprint(&request);
        let session = Session::new(
            user.id,
            device_fingerprint.value().to_string(),
            request.remember_me,
            None, // IP address not available in request
            None, // User agent not available in request
        );

        let saved_session = self.session_repository.save(&session).await?;
        let session_id = saved_session.id();
        let logged_in_at = Utc::now();

        // Reset failed attempts on successful login
        self.security_monitoring.reset_failed_attempts(user.id).await?;

        // Update user last login
        self.user_repository.update_last_login(&user.id.to_string()).await?;

        // Publish login success event
        self.publish_event(UserDomainEvent::LoggedIn {
            user_id: user.id.to_string(),
            session_id: session_id.to_string(),
            occurred_at: logged_in_at,
        }).await;

        Ok(AuthenticationResult {
            success: true,
            user_id: Some(user.id),
            session_id: Some(*session_id),
            requires_mfa: false,
            mfa_methods: vec![],
            error_message: None,
            lockout_until: None,
        })
    }

    async fn verify_email(&self, request: EmailVerificationRequest) -> Result<AuthenticationResult, AuthenticationError> {
        // Find verification token
        let token = self.email_verification_repository
            .find_by_token(&request.verification_token)
            .await?
            .ok_or(AuthenticationError::InvalidVerificationToken)?;

        // Check if token is expired
        if token.is_expired() {
            return Err(AuthenticationError::InvalidVerificationToken);
        }

        // Check if token can be attempted
        if !token.can_attempt() {
            return Err(AuthenticationError::InvalidVerificationToken);
        }

        // Update user email verification status
        self.user_repository.verify_email(&token.user_id.to_string()).await?;

        // Delete used token
        self.email_verification_repository.delete(&token.id.to_string()).await?;

        // Capture timestamp at the moment verification completes
        let verified_at = Utc::now();

        // Publish email verified event
        self.publish_event(UserDomainEvent::EmailVerified {
            user_id: token.user_id.to_string(),
            occurred_at: verified_at,
        }).await;

        Ok(AuthenticationResult {
            success: true,
            user_id: Some(token.user_id),
            session_id: None,
            requires_mfa: false,
            mfa_methods: vec![],
            error_message: None,
            lockout_until: None,
        })
    }

    async fn resend_verification(&self, email: Email) -> Result<(), AuthenticationError> {
        // Find user by email
        let user = self.user_repository.find_by_email(email.as_str())
            .await?
            .ok_or(AuthenticationError::InvalidCredentials)?;

        // Check if already verified
        if user.email_verified {
            return Ok(()); // Already verified, no error needed
        }

        // Create new verification token
        let verification_token = EmailVerificationToken::new_account_creation(user.id, email.to_string());
        self.email_verification_repository.save(&verification_token).await?;

        // Send verification email
        self.email_service.send_verification_email(
            &email,
            verification_token.token(),
        ).await.map_err(|e| AuthenticationError::EmailError(anyhow::anyhow!("Failed to send verification email: {}", e)))?;

        Ok(())
    }

    async fn logout_user(&self, session_id: Uuid, device_untrust: bool) -> Result<(), AuthenticationError> {
        // Find session
        let session = self.session_repository.find_by_id(&session_id.to_string())
            .await?
            .ok_or(AuthenticationError::SessionNotFound)?;

        // Delete session
        self.session_repository.delete(&session_id.to_string()).await?;

        // Untrust device if requested
        if device_untrust {
            if let Some(fp) = &session.device_fingerprint {
                let device_fingerprint = crate::domain::value_objects::DeviceFingerprint::new(&fp);
                self.security_monitoring.untrust_device(session.user_id, &device_fingerprint).await?;
            }
        }

        // Capture timestamp at the moment logout completes
        let logged_out_at = Utc::now();

        // Publish logout event
        self.publish_event(UserDomainEvent::LoggedOut {
            user_id: session.user_id.to_string(),
            session_id: session_id.to_string(),
            occurred_at: logged_out_at,
        }).await;

        Ok(())
    }

    async fn refresh_token(&self, refresh_token: String) -> Result<TokenRefreshResult, AuthenticationError> {
        // Validate refresh token signature and expiration
        let claims = self.jwt_service.validate_refresh_token(&refresh_token)
            .map_err(|_| AuthenticationError::InvalidRefreshToken)?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AuthenticationError::InvalidRefreshToken)?;

        // Verify user still exists and is in valid state
        let user = self.user_repository
            .find_by_id(&user_id.to_string())
            .await?
            .ok_or(AuthenticationError::InvalidRefreshToken)?;

        match user.status {
            crate::domain::entity::UserStatus::Suspended => {
                let _ = self.security_monitoring.log_security_event(
                    Self::build_security_event(
                        user_id,
                        SecurityEventType::AccountCompromised,
                        None,
                        HashMap::from([
                            ("action".to_string(), "token_refresh_blocked".to_string()),
                            ("reason".to_string(), "account_suspended".to_string()),
                        ]),
                        80,
                    )
                ).await;

                return Err(AuthenticationError::AccountSuspended);
            }
            crate::domain::entity::UserStatus::Inactive => {
                return Err(AuthenticationError::InvalidRefreshToken);
            }
            crate::domain::entity::UserStatus::PendingVerification => {
                return Err(AuthenticationError::AccountNotVerified);
            }
            crate::domain::entity::UserStatus::Active => {}
        }

        // Check if account is locked
        if self.is_account_locked(user_id).await? {
            return Err(AuthenticationError::AccountLocked(Utc::now() + Duration::minutes(5)));
        }

        self.generate_tokens(user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test device fingerprint generation logic directly.
    /// This tests that:
    /// 1. When request has a device fingerprint, it uses that
    /// 2. When request has no fingerprint, it generates one
    #[test]
    fn test_device_fingerprint_from_request() {
        // Test with provided fingerprint
        let provided_fp: DeviceFingerprint = "test-fingerprint-123".into();
        let request_with_fp = LoginRequest {
            email: Email::new("test@example.com").unwrap(),
            password: "password123".to_string(),
            remember_me: false,
            device_fingerprint: Some(provided_fp.clone()),
            mfa_code: None,
        };

        // Simulate what create_device_fingerprint does
        let result = request_with_fp.device_fingerprint
            .clone()
            .unwrap_or_else(|| DeviceFingerprint::generate());

        assert_eq!(result.value(), provided_fp.value());
    }

    #[test]
    fn test_device_fingerprint_generation_when_none() {
        // Test without provided fingerprint - should generate one
        let request_without_fp = LoginRequest {
            email: Email::new("test@example.com").unwrap(),
            password: "password123".to_string(),
            remember_me: false,
            device_fingerprint: None,
            mfa_code: None,
        };

        // Simulate what create_device_fingerprint does
        let result = request_without_fp.device_fingerprint
            .clone()
            .unwrap_or_else(|| DeviceFingerprint::generate());

        // Generated fingerprint should not be empty
        assert!(!result.value().is_empty());
    }

    #[test]
    fn test_login_request_creation() {
        let request = LoginRequest {
            email: Email::new("user@example.com").unwrap(),
            password: "SecurePassword123!".to_string(),
            remember_me: true,
            device_fingerprint: Some("fp-12345".into()),
            mfa_code: Some("123456".to_string()),
        };

        assert_eq!(request.email.as_str(), "user@example.com");
        assert!(request.remember_me);
        assert!(request.device_fingerprint.is_some());
        assert!(request.mfa_code.is_some());
    }

    #[test]
    fn test_authentication_result_success() {
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        let result = AuthenticationResult {
            success: true,
            user_id: Some(user_id),
            session_id: Some(session_id),
            requires_mfa: false,
            mfa_methods: vec![],
            error_message: None,
            lockout_until: None,
        };

        assert!(result.success);
        assert_eq!(result.user_id, Some(user_id));
        assert!(!result.requires_mfa);
    }

    #[test]
    fn test_authentication_result_mfa_required() {
        let user_id = Uuid::new_v4();

        let result = AuthenticationResult {
            success: false,
            user_id: Some(user_id),
            session_id: None,
            requires_mfa: true,
            mfa_methods: vec![MfaMethod::Totp, MfaMethod::Email],
            error_message: None,
            lockout_until: None,
        };

        assert!(!result.success);
        assert!(result.requires_mfa);
        assert_eq!(result.mfa_methods.len(), 2);
    }

    #[test]
    fn test_authentication_error_variants() {
        let email_error = AuthenticationError::EmailAlreadyRegistered;
        let password_error = AuthenticationError::PasswordTooWeak;
        let invalid_cred = AuthenticationError::InvalidCredentials;

        assert!(matches!(email_error, AuthenticationError::EmailAlreadyRegistered));
        assert!(matches!(password_error, AuthenticationError::PasswordTooWeak));
        assert!(matches!(invalid_cred, AuthenticationError::InvalidCredentials));
    }
}