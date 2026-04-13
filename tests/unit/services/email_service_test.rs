//! Email Service Unit Tests
//!
//! Tests for email sending and template rendering.

use backbone_sapiens::domain::value_objects::Email;
use crate::unit::mocks::MockEmailService;
use backbone_sapiens::domain::services::email_service::{EmailTemplate, EmailDeliveryResult, EmailDeliveryStatus};
use uuid::Uuid;

// ============================================================
// Email Sending Tests
// ============================================================

#[cfg(test)]
mod email_sending_tests {
    use super::*;

    #[tokio::test]
    async fn test_send_email_success() {
        let service = MockEmailService::new();

        let to = Email::new("recipient@example.com").unwrap();
        let template = EmailTemplate::EmailVerification {
            verification_url: "https://example.com/verify/token123".to_string(),
            user_name: "John Doe".to_string(),
        };

        let result = service.send_email(&to, &template).await;

        assert!(result.is_ok());
        let delivery_result = result.unwrap();
        assert!(delivery_result.success);
        assert_eq!(delivery_result.status, EmailDeliveryStatus::Sent);
    }

    #[tokio::test]
    async fn test_send_email_records_sent() {
        let service = MockEmailService::new();

        let to = Email::new("recipient@example.com").unwrap();
        let template = EmailTemplate::PasswordReset {
            reset_url: "https://example.com/reset/token123".to_string(),
            user_name: "John Doe".to_string(),
            expiry_hours: 1,
        };

        service.send_email(&to, &template).await.unwrap();

        let emails = service.get_sent_emails().await;
        assert_eq!(emails.len(), 1);
        assert_eq!(emails[0].to, "recipient@example.com");
    }

    #[tokio::test]
    async fn test_send_multiple_emails() {
        let service = MockEmailService::new();

        let recipients = vec![
            "user1@example.com",
            "user2@example.com",
            "user3@example.com",
        ];

        for recipient in recipients {
            let to = Email::new(recipient).unwrap();
            let template = EmailTemplate::EmailVerification {
                verification_url: "https://example.com/verify".to_string(),
                user_name: "User".to_string(),
            };
            service.send_email(&to, &template).await.unwrap();
        }

        let emails = service.get_sent_emails().await;
        assert_eq!(emails.len(), 3);
    }

    #[tokio::test]
    async fn test_clear_sent_emails() {
        let service = MockEmailService::new();

        let to = Email::new("recipient@example.com").unwrap();
        let template = EmailTemplate::EmailVerification {
            verification_url: "https://example.com/verify".to_string(),
            user_name: "John Doe".to_string(),
        };

        service.send_email(&to, &template).await.unwrap();
        service.clear().await;

        let emails = service.get_sent_emails().await;
        assert_eq!(emails.len(), 0);
    }
}

// ============================================================
// Email Template Tests
// ============================================================

#[cfg(test)]
mod email_template_tests {
    use super::*;

    #[test]
    fn test_email_verification_template() {
        let template = EmailTemplate::EmailVerification {
            verification_url: "https://example.com/verify/abc123".to_string(),
            user_name: "Jane Doe".to_string(),
        };

        match template {
            EmailTemplate::EmailVerification { verification_url, user_name } => {
                assert_eq!(verification_url, "https://example.com/verify/abc123");
                assert_eq!(user_name, "Jane Doe");
            }
            _ => panic!("Expected EmailVerification template"),
        }
    }

    #[test]
    fn test_password_reset_template() {
        let template = EmailTemplate::PasswordReset {
            reset_url: "https://example.com/reset/xyz789".to_string(),
            user_name: "Bob Smith".to_string(),
            expiry_hours: 24,
        };

        match template {
            EmailTemplate::PasswordReset { reset_url, user_name, expiry_hours } => {
                assert_eq!(reset_url, "https://example.com/reset/xyz789");
                assert_eq!(user_name, "Bob Smith");
                assert_eq!(expiry_hours, 24);
            }
            _ => panic!("Expected PasswordReset template"),
        }
    }

    #[test]
    fn test_welcome_email_template() {
        let template = EmailTemplate::Welcome {
            user_name: "Alice".to_string(),
            login_url: "https://example.com/login".to_string(),
        };

        match template {
            EmailTemplate::Welcome { user_name, login_url } => {
                assert_eq!(user_name, "Alice");
                assert_eq!(login_url, "https://example.com/login");
            }
            _ => panic!("Expected Welcome template"),
        }
    }
}

// ============================================================
// Email Validation Tests
// ============================================================

#[cfg(test)]
mod email_validation_tests {
    use super::*;

    #[test]
    fn test_valid_email_creation() {
        let valid_emails = vec![
            "simple@example.com",
            "very.common@example.com",
            "disposable.style.email.with+symbol@example.com",
            "other.email-with-hyphen@example.com",
            "fully-qualified-domain@example.com",
        ];

        for email in valid_emails {
            let result = Email::new(email);
            assert!(result.is_ok(), "Email {} should be valid", email);
        }
    }

    #[test]
    fn test_invalid_email_creation() {
        let invalid_emails = vec![
            "",
            "plainaddress",
            "@no-local-part.com",
            "missing-at-sign.net",
            "missing@domain",
            "spaces in@address.com",
        ];

        for email in invalid_emails {
            let result = Email::new(email);
            assert!(result.is_err(), "Email {} should be invalid", email);
        }
    }

    #[test]
    fn test_email_equality() {
        let email1 = Email::new("test@example.com").unwrap();
        let email2 = Email::new("test@example.com").unwrap();
        let email3 = Email::new("other@example.com").unwrap();

        assert_eq!(email1, email2);
        assert_ne!(email1, email3);
    }

    #[test]
    fn test_email_display() {
        let email = Email::new("test@example.com").unwrap();
        assert_eq!(format!("{}", email), "test@example.com");
        assert_eq!(email.to_string(), "test@example.com");
    }
}
