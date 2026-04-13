//! Password Reset Service Unit Tests
//!
//! Tests for password reset workflows including:
//! - Initiate password reset
//! - Verify reset token
//! - Complete password reset
//! - Rate limiting
//! - Token expiration

use backbone_sapiens::domain::entity::{User, PasswordReset, PasswordResetStatus, UserStatus};
use backbone_sapiens::domain::value_objects::Email;
use backbone_sapiens::domain::repositories::UserRepository;
use crate::unit::mocks::MockUserRepository;
use uuid::Uuid;
use chrono::{Utc, Duration};

/// Test password reset service
struct TestPasswordResetService {
    user_repo: MockUserRepository,
    reset_tokens: std::sync::Arc<tokio::sync::RwLock<Vec<PasswordReset>>>,
}

impl TestPasswordResetService {
    fn new() -> Self {
        Self {
            user_repo: MockUserRepository::new(),
            reset_tokens: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    async fn initiate_reset(&self, email: &str) -> Result<PasswordReset, String> {
        // Check if user exists
        let user = self.user_repo
            .find_by_email(email)
            .await
            .map_err(|e| e.to_string())?;

        if user.is_none() {
            return Err("User not found".to_string());
        }

        let user = user.unwrap();

        // Create reset token
        let token = format!("reset_token_{}", Uuid::new_v4());
        let token_hash = format!("hash_{}", token);

        let mut reset = PasswordReset::new(user.id, email.to_string(), token, token_hash, 1);
        reset.expires_at = Utc::now() + Duration::hours(1);

        self.reset_tokens.write().await.push(reset.clone());

        Ok(reset)
    }

    async fn verify_token(&self, token: &str) -> Result<TokenVerificationResult, String> {
        let tokens = self.reset_tokens.read().await;

        for reset in tokens.iter() {
            if reset.token == token {
                return Ok(TokenVerificationResult {
                    is_valid: reset.is_valid(),
                    is_expired: reset.is_expired(),
                    is_used: reset.is_used(),
                    expires_at: reset.expires_at,
                    attempts_remaining: 5,
                });
            }
        }

        Ok(TokenVerificationResult {
            is_valid: false,
            is_expired: false,
            is_used: false,
            expires_at: Utc::now(),
            attempts_remaining: 0,
        })
    }

    async fn complete_reset(&self, token: &str, new_password: &str) -> Result<PasswordResetCompletion, String> {
        let mut tokens = self.reset_tokens.write().await;

        for reset in tokens.iter_mut() {
            if reset.token == token {
                if !reset.is_valid() {
                    return Ok(PasswordResetCompletion {
                        success: false,
                        message: "Token is invalid".to_string(),
                        revoked_sessions: 0,
                    });
                }

                reset.mark_as_used();

                // Update user password (simplified - just checking validation)
                if new_password.len() < 8 {
                    return Ok(PasswordResetCompletion {
                        success: false,
                        message: "Password too weak".to_string(),
                        revoked_sessions: 0,
                    });
                }

                return Ok(PasswordResetCompletion {
                    success: true,
                    message: "Password reset successfully".to_string(),
                    revoked_sessions: 1,
                });
            }
        }

        Ok(PasswordResetCompletion {
            success: false,
            message: "Token not found".to_string(),
            revoked_sessions: 0,
        })
    }
}

#[derive(Debug, Clone)]
struct TokenVerificationResult {
    is_valid: bool,
    is_expired: bool,
    is_used: bool,
    expires_at: chrono::DateTime<Utc>,
    attempts_remaining: u32,
}

#[derive(Debug, Clone)]
struct PasswordResetCompletion {
    success: bool,
    message: String,
    revoked_sessions: u32,
}

// ============================================================
// Initiate Reset Tests
// ============================================================

#[cfg(test)]
mod initiate_reset_tests {
    use super::*;

    #[tokio::test]
    async fn test_initiate_reset_success() {
        let service = TestPasswordResetService::new();

        let user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );

        service.user_repo.add_user(user).await;

        let result = service.initiate_reset("test@example.com").await;

        assert!(result.is_ok());
        let reset = result.unwrap();
        assert_eq!(reset.email, "test@example.com");
        assert!(!reset.is_expired());
        assert!(!reset.is_used());
        assert_eq!(reset.status(), PasswordResetStatus::Pending);
    }

    #[tokio::test]
    async fn test_initiate_reset_user_not_found() {
        let service = TestPasswordResetService::new();

        let result = service.initiate_reset("nonexistent@example.com").await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "User not found");
    }

    #[tokio::test]
    async fn test_initiate_reset_token_expiration() {
        let service = TestPasswordResetService::new();

        let user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );

        service.user_repo.add_user(user).await;

        let reset = service.initiate_reset("test@example.com").await.unwrap();

        // Token should expire in ~1 hour
        let time_until = reset.time_until_expiration();
        assert!(time_until.num_minutes() > 58);
        assert!(time_until.num_minutes() <= 60);
    }
}

// ============================================================
// Verify Token Tests
// ============================================================

#[cfg(test)]
mod verify_token_tests {
    use super::*;

    #[tokio::test]
    async fn test_verify_token_valid() {
        let service = TestPasswordResetService::new();

        let user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );

        service.user_repo.add_user(user).await;

        let reset = service.initiate_reset("test@example.com").await.unwrap();
        let result = service.verify_token(&reset.token).await.unwrap();

        assert!(result.is_valid);
        assert!(!result.is_expired);
        assert!(!result.is_used);
        assert_eq!(result.attempts_remaining, 5);
    }

    #[tokio::test]
    async fn test_verify_token_expired() {
        let service = TestPasswordResetService::new();

        let user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );

        service.user_repo.add_user(user).await;

        let mut reset = service.initiate_reset("test@example.com").await.unwrap();
        reset.expires_at = Utc::now() - Duration::minutes(1);

        // Manually add expired token
        service.reset_tokens.write().await.push(reset.clone());

        let result = service.verify_token(&reset.token).await.unwrap();

        assert!(!result.is_valid);
        assert!(result.is_expired);
    }

    #[tokio::test]
    async fn test_verify_token_not_found() {
        let service = TestPasswordResetService::new();

        let result = service.verify_token("nonexistent_token").await.unwrap();

        assert!(!result.is_valid);
        assert_eq!(result.attempts_remaining, 0);
    }
}

// ============================================================
// Complete Reset Tests
// ============================================================

#[cfg(test)]
mod complete_reset_tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_reset_success() {
        let service = TestPasswordResetService::new();

        let user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );

        service.user_repo.add_user(user).await;

        let reset = service.initiate_reset("test@example.com").await.unwrap();
        let result = service.complete_reset(&reset.token, "NewSecureP@ss123").await.unwrap();

        assert!(result.success);
        assert_eq!(result.message, "Password reset successfully");
        assert_eq!(result.revoked_sessions, 1);
    }

    #[tokio::test]
    async fn test_complete_reset_weak_password() {
        let service = TestPasswordResetService::new();

        let user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );

        service.user_repo.add_user(user).await;

        let reset = service.initiate_reset("test@example.com").await.unwrap();
        let result = service.complete_reset(&reset.token, "weak").await.unwrap();

        assert!(!result.success);
        assert!(result.message.contains("weak"));
    }

    #[tokio::test]
    async fn test_complete_reset_token_not_found() {
        let service = TestPasswordResetService::new();

        let result = service.complete_reset("nonexistent_token", "NewSecureP@ss123").await.unwrap();

        assert!(!result.success);
        assert!(result.message.contains("not found"));
    }

    #[tokio::test]
    async fn test_complete_reset_marks_token_used() {
        let service = TestPasswordResetService::new();

        let user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );

        service.user_repo.add_user(user).await;

        let reset = service.initiate_reset("test@example.com").await.unwrap();
        let token = reset.token.clone();

        service.complete_reset(&token, "NewSecureP@ss123").await.unwrap();

        // Check token is now marked as used
        let result = service.verify_token(&token).await.unwrap();
        assert!(result.is_used);
    }

    #[tokio::test]
    async fn test_complete_reset_prevents_reuse() {
        let service = TestPasswordResetService::new();

        let user = User::new(
            "test@example.com".to_string(),
            "password".to_string(),
            "Test".to_string(),
            "User".to_string(),
        );

        service.user_repo.add_user(user).await;

        let reset = service.initiate_reset("test@example.com").await.unwrap();
        let token = reset.token.clone();

        // First use should succeed
        let result1 = service.complete_reset(&token, "NewSecureP@ss123").await.unwrap();
        assert!(result1.success);

        // Second use should fail
        let result2 = service.complete_reset(&token, "AnotherSecureP@ss456").await.unwrap();
        assert!(!result2.success);
    }
}

// ============================================================
// Password Reset Entity Tests
// ============================================================

#[cfg(test)]
mod password_reset_entity_tests {
    use super::*;

    #[test]
    fn test_password_reset_status_pending() {
        let reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            24,
        );

        assert_eq!(reset.status(), PasswordResetStatus::Pending);
        assert!(reset.is_valid());
    }

    #[test]
    fn test_password_reset_status_used() {
        let mut reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            24,
        );

        reset.mark_as_used();
        assert_eq!(reset.status(), PasswordResetStatus::Used);
        assert!(!reset.is_valid());
    }

    #[test]
    fn test_password_reset_status_expired() {
        let mut reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            1,
        );

        reset.expires_at = Utc::now() - Duration::seconds(1);
        assert_eq!(reset.status(), PasswordResetStatus::Expired);
        assert!(!reset.is_valid());
    }

    #[test]
    fn test_password_reset_revocation() {
        let mut reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            24,
        );

        reset.revoke(Uuid::new_v4(), "Admin action".to_string());

        assert!(reset.is_revoked());
        assert_eq!(reset.status(), PasswordResetStatus::Revoked);
        assert!(!reset.is_valid());
        assert_eq!(reset.failure_reason, Some("Admin action".to_string()));
    }

    #[test]
    fn test_password_reset_remaining_time() {
        let reset = PasswordReset::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "token".to_string(),
            "hash".to_string(),
            2,
        );

        let remaining = reset.remaining_time_human();
        assert!(remaining.contains("hour") || remaining.contains("hours"));
    }
}
