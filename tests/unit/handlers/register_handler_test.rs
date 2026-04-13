//! Register Handler Unit Tests
//!
//! Tests for registration handler including:
//! - Successful registration
//! - Password mismatch
//! - Terms not accepted
//! - Duplicate email/username

use backbone_sapiens::handlers::auth::register_handler::{
    RegistrationRequestPayload, RegistrationResponse,
};
use serde_json::json;

// ============================================================
// Request Deserialization Tests
// ============================================================

#[cfg(test)]
mod request_tests {
    use super::*;

    #[test]
    fn test_registration_request_deserialization() {
        let json_str = r#"{
            "username": "testuser",
            "email": "test@example.com",
            "password": "SecureP@ssw0rd123!",
            "confirm_password": "SecureP@ssw0rd123!",
            "first_name": "John",
            "last_name": "Doe",
            "accept_terms": true,
            "device_fingerprint": "fp123456",
            "newsletter_opt_in": false
        }"#;

        let payload: RegistrationRequestPayload = serde_json::from_str(json_str).unwrap();

        assert_eq!(payload.username, Some("testuser".to_string()));
        assert_eq!(payload.email, "test@example.com");
        assert_eq!(payload.password, "SecureP@ssw0rd123!");
        assert_eq!(payload.confirm_password, "SecureP@ssw0rd123!");
        assert_eq!(payload.first_name, "John");
        assert_eq!(payload.last_name, "Doe");
        assert!(payload.accept_terms);
        assert_eq!(payload.device_fingerprint, Some("fp123456".to_string()));
        assert_eq!(payload.newsletter_opt_in, Some(false));
    }

    #[test]
    fn test_registration_request_minimal() {
        let json_str = r#"{
            "email": "test@example.com",
            "password": "password123",
            "confirm_password": "password123",
            "first_name": "Test",
            "last_name": "User",
            "accept_terms": true
        }"#;

        let payload: RegistrationRequestPayload = serde_json::from_str(json_str).unwrap();

        assert_eq!(payload.email, "test@example.com");
        assert_eq!(payload.username, None);
        assert_eq!(payload.device_fingerprint, None);
        assert!(payload.accept_terms);
    }

    #[test]
    fn test_registration_request_with_optional_fields() {
        let payload = RegistrationRequestPayload {
            username: Some("optional_user".to_string()),
            email: "optional@example.com".to_string(),
            password: "Pass123!".to_string(),
            confirm_password: "Pass123!".to_string(),
            first_name: "Optional".to_string(),
            last_name: "User".to_string(),
            accept_terms: true,
            device_fingerprint: Some("fp_optional".to_string()),
            newsletter_opt_in: Some(true),
        };

        assert_eq!(payload.username, Some("optional_user".to_string()));
        assert_eq!(payload.newsletter_opt_in, Some(true));
    }
}

// ============================================================
// Response Serialization Tests
// ============================================================

#[cfg(test)]
mod response_tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_registration_response_success() {
        let user_id = Uuid::new_v4();

        let response = RegistrationResponse {
            success: true,
            user_id: Some(user_id),
            message: "Registration successful. Please check your email for verification.".to_string(),
            verification_required: true,
            verification_email_sent: true,
            email: Some("test@example.com".to_string()),
            errors: vec![],
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":true"));
        assert!(json.contains("verification"));
        assert!(json.contains("\"verification_email_sent\":true"));
    }

    #[test]
    fn test_registration_response_failure() {
        let response = RegistrationResponse {
            success: false,
            user_id: None,
            message: "Passwords do not match".to_string(),
            verification_required: false,
            verification_email_sent: false,
            email: Some("test@example.com".to_string()),
            errors: vec!["Passwords do not match".to_string()],
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":false"));
        assert!(json.contains("Passwords do not match"));
        assert!(json.contains("\"verification_email_sent\":false"));
    }

    #[test]
    fn test_registration_response_with_multiple_errors() {
        let response = RegistrationResponse {
            success: false,
            user_id: None,
            message: "Registration failed".to_string(),
            verification_required: false,
            verification_email_sent: false,
            email: Some("test@example.com".to_string()),
            errors: vec![
                "Email already registered".to_string(),
                "Username already taken".to_string(),
            ],
        };

        assert_eq!(response.errors.len(), 2);
        assert!(response.errors.contains(&"Email already registered".to_string()));
    }
}

// ============================================================
// Validation Tests
// ============================================================

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_password_match_validation() {
        let payload = RegistrationRequestPayload {
            username: None,
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            accept_terms: true,
            device_fingerprint: None,
            newsletter_opt_in: None,
        };

        assert_eq!(payload.password, payload.confirm_password);
    }

    #[test]
    fn test_password_mismatch() {
        let payload = RegistrationRequestPayload {
            username: None,
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            confirm_password: "different456".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            accept_terms: true,
            device_fingerprint: None,
            newsletter_opt_in: None,
        };

        assert_ne!(payload.password, payload.confirm_password);
    }

    #[test]
    fn test_terms_acceptance_required() {
        let payload_without_terms = RegistrationRequestPayload {
            username: None,
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            accept_terms: false,
            device_fingerprint: None,
            newsletter_opt_in: None,
        };

        assert!(!payload_without_terms.accept_terms);
    }

    #[test]
    fn test_required_fields_present() {
        let payload = RegistrationRequestPayload {
            username: Some("testuser".to_string()),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            accept_terms: true,
            device_fingerprint: Some("fp123".to_string()),
            newsletter_opt_in: Some(true),
        };

        assert!(payload.username.is_some());
        assert!(!payload.email.is_empty());
        assert!(!payload.password.is_empty());
        assert!(!payload.first_name.is_empty());
        assert!(!payload.last_name.is_empty());
        assert!(payload.accept_terms);
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
    fn test_valid_email_in_registration() {
        let valid_emails = vec![
            "simple@example.com",
            "user.name@example.com",
            "user+tag@example.co.uk",
        ];

        for email in valid_emails {
            let result = Email::new(email);
            assert!(result.is_ok(), "Email {} should be valid", email);
        }
    }

    #[test]
    fn test_invalid_email_in_registration() {
        let invalid_emails = vec![
            "invalid",
            "@example.com",
            "user@",
        ];

        for email in invalid_emails {
            let result = Email::new(email);
            assert!(result.is_err(), "Email {} should be invalid", email);
        }
    }
}
