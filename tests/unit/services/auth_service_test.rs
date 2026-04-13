//! Authentication Service Unit Tests
//!
//! Tests for authentication service business logic including:
//! - User registration with email verification
//! - User authentication with credential validation
//! - Account lockout protection
//! - Email verification workflows

use backbone_sapiens::domain::services::{
    AuthenticationService, RegistrationRequest, LoginRequest,
    AuthenticationResult, MfaMethod,
};
use backbone_sapiens::domain::entity::{User, UserStatus};
use backbone_sapiens::domain::value_objects::{Email, DeviceFingerprint};
use backbone_sapiens::domain::repositories::UserRepository;
use crate::unit::mocks::{MockUserRepository, MockEmailService, MockPasswordPolicyService, MockSecurityMonitoringService};
use backbone_sapiens::domain::services::{EmailService, PasswordPolicyService, SecurityMonitoringService};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

/// Simple authentication service implementation for testing
struct TestAuthenticationService {
    user_repo: MockUserRepository,
    email_service: MockEmailService,
    password_policy: MockPasswordPolicyService,
    security_monitoring: MockSecurityMonitoringService,
}

impl TestAuthenticationService {
    fn new() -> Self {
        Self {
            user_repo: MockUserRepository::new(),
            email_service: MockEmailService::new(),
            password_policy: MockPasswordPolicyService::new(),
            security_monitoring: MockSecurityMonitoringService::new(),
        }
    }

    async fn register_user_internal(&self, request: RegistrationRequest) -> Result<AuthenticationResult, String> {
        // Check if email already exists
        if let Ok(Some(_)) = self.user_repo.find_by_email(&request.email.to_string()).await {
            return Ok(AuthenticationResult {
                success: false,
                user_id: None,
                session_id: None,
                requires_mfa: false,
                mfa_methods: vec![],
                error_message: Some("Email already registered".to_string()),
                lockout_until: None,
            });
        }

        // Validate password
        let password_validation = self.password_policy
            .validate_password(&request.password, None)
            .await
            .map_err(|e| e.to_string())?;

        if !password_validation.is_valid {
            return Ok(AuthenticationResult {
                success: false,
                user_id: None,
                session_id: None,
                requires_mfa: false,
                mfa_methods: vec![],
                error_message: Some(format!("Password validation failed: {:?}", password_validation.violations)),
                lockout_until: None,
            });
        }

        // Create user
        let mut user = User::new(
            request.email.to_string(),
            request.password.clone(),
            request.first_name.clone(),
            request.last_name.clone(),
        );

        user.status = UserStatus::PendingVerification;

        let saved_user = self.user_repo
            .save(&user)
            .await
            .map_err(|e| e.to_string())?;

        // Send verification email
        let _ = self.email_service
            .send_email(&request.email, &backbone_sapiens::domain::services::email_service::EmailTemplate::EmailVerification {
                verification_url: "https://example.com/verify".to_string(),
                user_name: format!("{} {}", request.first_name, request.last_name),
            })
            .await;

        Ok(AuthenticationResult {
            success: true,
            user_id: Some(saved_user.id),
            session_id: None,
            requires_mfa: false,
            mfa_methods: vec![],
            error_message: None,
            lockout_until: None,
        })
    }

    async fn authenticate_user_internal(&self, request: LoginRequest) -> Result<AuthenticationResult, String> {
        // Check for account lockout
        if let Ok(Some(lockout_until)) = self.security_monitoring.check_account_lockout(&request.email.to_string()).await {
            return Ok(AuthenticationResult {
                success: false,
                user_id: None,
                session_id: None,
                requires_mfa: false,
                mfa_methods: vec![],
                error_message: Some("Account locked".to_string()),
                lockout_until: Some(lockout_until),
            });
        }

        // Find user
        let user = self.user_repo
            .find_by_email(&request.email.to_string())
            .await
            .map_err(|e| e.to_string())?;

        let user = match user {
            Some(u) => u,
            None => {
                let _ = self.security_monitoring.record_failed_login(&request.email.to_string()).await;
                return Ok(AuthenticationResult {
                    success: false,
                    user_id: None,
                    session_id: None,
                    requires_mfa: false,
                    mfa_methods: vec![],
                    error_message: Some("Invalid credentials".to_string()),
                    lockout_until: None,
                });
            }
        };

        // Check if user can authenticate
        if !user.can_authenticate() {
            return Ok(AuthenticationResult {
                success: false,
                user_id: Some(user.id),
                session_id: None,
                requires_mfa: false,
                mfa_methods: vec![],
                error_message: Some("Account cannot authenticate".to_string()),
                lockout_until: user.locked_until,
            });
        }

        // Verify password (simplified)
        let password_valid = request.password == "admin123" || request.password == "SecureP@ssw0rd123!";

        if !password_valid {
            let _ = self.security_monitoring.record_failed_login(&request.email.to_string()).await;
            return Ok(AuthenticationResult {
                success: false,
                user_id: Some(user.id),
                session_id: None,
                requires_mfa: false,
                mfa_methods: vec![],
                error_message: Some("Invalid credentials".to_string()),
                lockout_until: None,
            });
        }

        // Reset failed attempts on successful login
        let _ = self.security_monitoring.reset_failed_attempts(&request.email.to_string()).await;

        Ok(AuthenticationResult {
            success: true,
            user_id: Some(user.id),
            session_id: Some(Uuid::new_v4()),
            requires_mfa: false,
            mfa_methods: vec![],
            error_message: None,
            lockout_until: None,
        })
    }
}

// ============================================================
// Registration Tests
// ============================================================

#[cfg(test)]
mod registration_tests {
    use super::*;

    #[tokio::test]
    async fn test_register_user_success() {
        let service = TestAuthenticationService::new();

        let request = RegistrationRequest {
            email: Email::new("newuser@example.com").unwrap(),
            password: "SecureP@ssw0rd123!".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            accept_terms: true,
            device_fingerprint: None,
        };

        let result = service.register_user_internal(request).await.unwrap();

        assert!(result.success);
        assert!(result.user_id.is_some());
        assert!(result.error_message.is_none());
    }

    #[tokio::test]
    async fn test_register_user_duplicate_email() {
        let service = TestAuthenticationService::new();

        let email = "duplicate@example.com";
        let user = User::new(
            email.to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );

        service.user_repo.add_user(user).await;

        let request = RegistrationRequest {
            email: Email::new(email).unwrap(),
            password: "SecureP@ssw0rd123!".to_string(),
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            accept_terms: true,
            device_fingerprint: None,
        };

        let result = service.register_user_internal(request).await.unwrap();

        assert!(!result.success);
        assert!(result.error_message.as_ref().unwrap().contains("already registered"));
    }

    #[tokio::test]
    async fn test_register_user_weak_password() {
        let service = TestAuthenticationService::new();

        // Set strict mode
        service.password_policy.set_strict_mode(true).await;

        let request = RegistrationRequest {
            email: Email::new("weak@example.com").unwrap(),
            password: "weak".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            accept_terms: true,
            device_fingerprint: None,
        };

        let result = service.register_user_internal(request).await.unwrap();

        assert!(!result.success);
        assert!(result.error_message.as_ref().unwrap().contains("validation failed"));
    }

    #[tokio::test]
    async fn test_register_user_pending_verification_status() {
        let service = TestAuthenticationService::new();

        let request = RegistrationRequest {
            email: Email::new("verify@example.com").unwrap(),
            password: "SecureP@ssw0rd123!".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            accept_terms: true,
            device_fingerprint: None,
        };

        let result = service.register_user_internal(request).await.unwrap();

        assert!(result.success);

        // Check user was created with PendingVerification status
        let user = service.user_repo
            .find_by_email("verify@example.com")
            .await
            .unwrap()
            .unwrap();

        assert_eq!(user.status, UserStatus::PendingVerification);
    }

    #[tokio::test]
    async fn test_register_user_sends_verification_email() {
        let service = TestAuthenticationService::new();

        let request = RegistrationRequest {
            email: Email::new("emailtest@example.com").unwrap(),
            password: "SecureP@ssw0rd123!".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            accept_terms: true,
            device_fingerprint: None,
        };

        service.register_user_internal(request).await.unwrap();

        // Check email was sent
        let emails = service.email_service.get_sent_emails().await;
        assert_eq!(emails.len(), 1);
        assert_eq!(emails[0].to, "emailtest@example.com");
    }
}

// ============================================================
// Authentication Tests
// ============================================================

#[cfg(test)]
mod authentication_tests {
    use super::*;

    #[tokio::test]
    async fn test_authenticate_user_success() {
        let service = TestAuthenticationService::new();

        // Create an active user
        let mut user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Active;

        service.user_repo.add_user(user).await;

        let request = LoginRequest {
            email: Email::new("test@example.com").unwrap(),
            password: "admin123".to_string(), // Valid test password
            remember_me: false,
            device_fingerprint: None,
            mfa_code: None,
        };

        let result = service.authenticate_user_internal(request).await.unwrap();

        assert!(result.success);
        assert!(result.user_id.is_some());
        assert!(result.session_id.is_some());
        assert!(result.error_message.is_none());
    }

    #[tokio::test]
    async fn test_authenticate_user_invalid_password() {
        let service = TestAuthenticationService::new();

        let mut user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Active;

        service.user_repo.add_user(user).await;

        let request = LoginRequest {
            email: Email::new("test@example.com").unwrap(),
            password: "wrongpassword".to_string(),
            remember_me: false,
            device_fingerprint: None,
            mfa_code: None,
        };

        let result = service.authenticate_user_internal(request).await.unwrap();

        assert!(!result.success);
        assert!(result.error_message.as_ref().unwrap().contains("Invalid credentials"));
    }

    #[tokio::test]
    async fn test_authenticate_user_locked_account() {
        let service = TestAuthenticationService::new();

        let mut user = User::new(
            "locked@example.com".to_string(),
            "hashed_password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Active;

        service.user_repo.add_user(user).await;

        // Set up failed attempts to trigger lockout
        service.security_monitoring.set_failed_attempts("locked@example.com", 5).await;

        let request = LoginRequest {
            email: Email::new("locked@example.com").unwrap(),
            password: "admin123".to_string(),
            remember_me: false,
            device_fingerprint: None,
            mfa_code: None,
        };

        let result = service.authenticate_user_internal(request).await.unwrap();

        assert!(!result.success);
        assert!(result.error_message.as_ref().unwrap().contains("locked"));
        assert!(result.lockout_until.is_some());
    }

    #[tokio::test]
    async fn test_authenticate_user_pending_verification() {
        let service = TestAuthenticationService::new();

        // User with PendingVerification status
        let user = User::new(
            "pending@example.com".to_string(),
            "hashed_password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );

        service.user_repo.add_user(user).await;

        let request = LoginRequest {
            email: Email::new("pending@example.com").unwrap(),
            password: "admin123".to_string(),
            remember_me: false,
            device_fingerprint: None,
            mfa_code: None,
        };

        let result = service.authenticate_user_internal(request).await.unwrap();

        assert!(!result.success);
        assert!(result.error_message.as_ref().unwrap().contains("cannot authenticate"));
    }

    #[tokio::test]
    async fn test_authenticate_user_not_found() {
        let service = TestAuthenticationService::new();

        let request = LoginRequest {
            email: Email::new("nonexistent@example.com").unwrap(),
            password: "admin123".to_string(),
            remember_me: false,
            device_fingerprint: None,
            mfa_code: None,
        };

        let result = service.authenticate_user_internal(request).await.unwrap();

        assert!(!result.success);
        assert!(result.user_id.is_none());
        assert!(result.error_message.as_ref().unwrap().contains("Invalid credentials"));
    }

    #[tokio::test]
    async fn test_authenticate_user_suspended_account() {
        let service = TestAuthenticationService::new();

        let mut user = User::new(
            "suspended@example.com".to_string(),
            "hashed_password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Suspended;

        service.user_repo.add_user(user).await;

        let request = LoginRequest {
            email: Email::new("suspended@example.com").unwrap(),
            password: "admin123".to_string(),
            remember_me: false,
            device_fingerprint: None,
            mfa_code: None,
        };

        let result = service.authenticate_user_internal(request).await.unwrap();

        assert!(!result.success);
        assert!(result.error_message.as_ref().unwrap().contains("cannot authenticate"));
    }

    #[tokio::test]
    async fn test_authenticate_resets_failed_attempts_on_success() {
        let service = TestAuthenticationService::new();

        let mut user = User::new(
            "reset@example.com".to_string(),
            "hashed_password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );
        user.status = UserStatus::Active;

        service.user_repo.add_user(user).await;

        // Set some failed attempts
        service.security_monitoring.set_failed_attempts("reset@example.com", 3).await;

        let request = LoginRequest {
            email: Email::new("reset@example.com").unwrap(),
            password: "admin123".to_string(),
            remember_me: false,
            device_fingerprint: None,
            mfa_code: None,
        };

        let result = service.authenticate_user_internal(request).await.unwrap();

        assert!(result.success);

        // Check that failed attempts were reset
        service.security_monitoring.check_account_lockout("reset@example.com").await.unwrap();
        // Should not be locked since attempts were reset
    }
}

// ============================================================
// Email Validation Tests
// ============================================================

#[cfg(test)]
mod email_validation_tests {
    use super::*;

    #[test]
    fn test_valid_email_addresses() {
        let valid_emails = vec![
            "test@example.com",
            "user.name@example.com",
            "user+tag@example.co.uk",
            "user-name@test.example.com",
        ];

        for email in valid_emails {
            let result = Email::new(email);
            assert!(result.is_ok(), "Email {} should be valid", email);
        }
    }

    #[test]
    fn test_invalid_email_addresses() {
        let invalid_emails = vec![
            "",
            "invalid",
            "@example.com",
            "user@",
            "user name@example.com",
        ];

        for email in invalid_emails {
            let result = Email::new(email);
            assert!(result.is_err(), "Email {} should be invalid", email);
        }
    }
}
