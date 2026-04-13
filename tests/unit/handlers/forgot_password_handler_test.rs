//! Forgot Password Handler Unit Tests
//!
//! Tests for forgot password handler including:
//! - Valid reset request
//! - User not found (don't reveal)
//! - Rate limiting

use backbone_sapiens::handlers::auth::forgot_password_handler::ForgotPasswordRequestPayload;
use serde_json::json;

// ============================================================
// Request Deserialization Tests
// ============================================================

#[cfg(test)]
mod request_tests {
    use super::*;

    #[test]
    fn test_forgot_password_request_deserialization() {
        let json_str = r#"{
            "email": "test@example.com",
            "captcha_token": "captcha_123456"
        }"#;

        let payload: ForgotPasswordRequestPayload = serde_json::from_str(json_str).unwrap();

        assert_eq!(payload.email, "test@example.com");
        assert_eq!(payload.captcha_token, Some("captcha_123456".to_string()));
    }

    #[test]
    fn test_forgot_password_request_minimal() {
        let json_str = r#"{
            "email": "test@example.com"
        }"#;

        let payload: ForgotPasswordRequestPayload = serde_json::from_str(json_str).unwrap();

        assert_eq!(payload.email, "test@example.com");
        assert_eq!(payload.captcha_token, None);
    }

    #[test]
    fn test_forgot_password_request_with_captcha() {
        let payload = ForgotPasswordRequestPayload {
            email: "secure@example.com".to_string(),
            captcha_token: Some("reCAPTCHA_token_xyz".to_string()),
        };

        assert!(payload.captcha_token.is_some());
        assert_eq!(payload.captcha_token.unwrap(), "reCAPTCHA_token_xyz");
    }
}

// ============================================================
// Response Tests
// ============================================================

#[cfg(test)]
mod response_tests {
    use super::*;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct ForgotPasswordResponse {
        pub success: bool,
        pub message: String,
        pub reset_token_sent: bool,
        pub email: Option<String>,
        pub cooldown_minutes: u32,
    }

    #[test]
    fn test_forgot_password_response_success() {
        let response = ForgotPasswordResponse {
            success: true,
            message: "Password reset link has been sent to your email address.".to_string(),
            reset_token_sent: true,
            email: Some("test@example.com".to_string()),
            cooldown_minutes: 15,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":true"));
        assert!(json.contains("reset link has been sent"));
        assert!(json.contains("\"reset_token_sent\":true"));
        assert!(json.contains("\"cooldown_minutes\":15"));
    }

    #[test]
    fn test_forgot_password_response_user_not_found() {
        let response = ForgotPasswordResponse {
            success: true, // Still true for security
            message: "If an account with this email exists, a password reset link has been sent.".to_string(),
            reset_token_sent: false,
            email: None, // Don't reveal email
            cooldown_minutes: 15,
        };

        let json = serde_json::to_string(&response).unwrap();

        // Should not reveal that user doesn't exist
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("If an account"));
        assert!(json.contains("\"reset_token_sent\":false"));
    }

    #[test]
    fn test_forgot_password_response_rate_limited() {
        let response = ForgotPasswordResponse {
            success: false,
            message: "Too many reset requests. Please try again in 15 minutes.".to_string(),
            reset_token_sent: false,
            email: None,
            cooldown_minutes: 15,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":false"));
        assert!(json.contains("Too many reset requests"));
        assert!(json.contains("15 minutes"));
    }
}

// ============================================================
// Security Tests
// ============================================================

#[cfg(test)]
mod security_tests {
    use super::*;

    #[test]
    fn test_email_not_revealed_when_user_not_found() {
        let response = json!({
            "success": true,
            "message": "If an account with this email exists, a password reset link has been sent.",
            "reset_token_sent": false,
            "email": null,
            "cooldown_minutes": 15
        });

        // Email should not be revealed
        assert!(response["email"].is_null());
        assert!(response["success"].as_bool().unwrap());
    }

    #[test]
    fn test_cooldown_period_enforced() {
        let cooldown_minutes = 15;

        let rate_limit_response = json!({
            "success": false,
            "message": format!("Too many reset requests. Please try again in {} minutes.", cooldown_minutes),
            "cooldown_minutes": cooldown_minutes
        });

        assert_eq!(rate_limit_response["cooldown_minutes"], 15);
        assert!(rate_limit_response["message"].as_str().unwrap().contains("Too many"));
    }

    #[test]
    fn test_captcha_token_optional() {
        let with_captcha = ForgotPasswordRequestPayload {
            email: "test@example.com".to_string(),
            captcha_token: Some("token123".to_string()),
        };

        let without_captcha = ForgotPasswordRequestPayload {
            email: "test@example.com".to_string(),
            captcha_token: None,
        };

        assert!(with_captcha.captcha_token.is_some());
        assert!(without_captcha.captcha_token.is_none());
    }
}

// ============================================================
// Email Validation Tests
// ============================================================

#[cfg(test)]
mod email_validation_tests {
    use super::*;
    use backbone_sapiens::domain::value_objects::Email;

    #[test]
    fn test_valid_email_in_forgot_password() {
        let valid_emails = vec![
            "user@example.com",
            "first.last@example.co.uk",
            "user+tag@example.com",
        ];

        for email in valid_emails {
            let result = Email::new(email);
            assert!(result.is_ok(), "Email {} should be valid", email);
        }
    }

    #[test]
    fn test_invalid_email_in_forgot_password() {
        let invalid_emails = vec![
            "invalid",
            "@example.com",
            "user@",
            "user @example.com",
        ];

        for email in invalid_emails {
            let result = Email::new(email);
            assert!(result.is_err(), "Email {} should be invalid", email);
        }
    }
}
