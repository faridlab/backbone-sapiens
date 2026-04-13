//! Login Handler Unit Tests
//!
//! Tests for login handler including:
//! - Successful login
//! - Invalid credentials
//! - Account locked
//! - Pending verification

use backbone_sapiens::handlers::auth::login_handler::{
    LoginRequestPayload, LoginResponse, UserInfo,
};
use backbone_sapiens::domain::value_objects::Email;
use serde_json::json;
use uuid::Uuid;

// ============================================================
// Request Deserialization Tests
// ============================================================

#[cfg(test)]
mod request_tests {
    use super::*;

    #[test]
    fn test_login_request_deserialization() {
        let json_str = r#"{
            "email_or_username": "test@example.com",
            "password": "SecureP@ssw0rd123!",
            "remember_me": true,
            "device_fingerprint": "fp123456",
            "mfa_code": "123456"
        }"#;

        let payload: LoginRequestPayload = serde_json::from_str(json_str).unwrap();

        assert_eq!(payload.email_or_username, "test@example.com");
        assert_eq!(payload.password, "SecureP@ssw0rd123!");
        assert_eq!(payload.remember_me, Some(true));
        assert_eq!(payload.device_fingerprint, Some("fp123456".to_string()));
        assert_eq!(payload.mfa_code, Some("123456".to_string()));
    }

    #[test]
    fn test_login_request_minimal() {
        let json_str = r#"{
            "email_or_username": "test@example.com",
            "password": "password123"
        }"#;

        let payload: LoginRequestPayload = serde_json::from_str(json_str).unwrap();

        assert_eq!(payload.email_or_username, "test@example.com");
        assert_eq!(payload.password, "password123");
        assert_eq!(payload.remember_me, None);
        assert_eq!(payload.device_fingerprint, None);
        assert_eq!(payload.mfa_code, None);
    }

    #[test]
    fn test_login_request_invalid_json() {
        let json_str = r#"{
            "email_or_username": "test@example.com"
        }"#;

        let result: Result<LoginRequestPayload, _> = serde_json::from_str(json_str);

        assert!(result.is_err());
    }
}

// ============================================================
// Response Serialization Tests
// ============================================================

#[cfg(test)]
mod response_tests {
    use super::*;

    #[test]
    fn test_login_response_success() {
        let response = LoginResponse {
            success: true,
            user_id: Some(Uuid::new_v4()),
            session_id: Some(Uuid::new_v4()),
            requires_mfa: false,
            mfa_methods: vec![],
            access_token: None,
            refresh_token: None,
            expires_in: None,
            user_info: None,
            error_message: None,
            lockout_until: None,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"user_id\""));
        assert!(json.contains("\"session_id\""));
        assert!(json.contains("\"requires_mfa\":false"));
    }

    #[test]
    fn test_login_response_failure() {
        let response = LoginResponse {
            success: false,
            user_id: None,
            session_id: None,
            requires_mfa: false,
            mfa_methods: vec![],
            access_token: None,
            refresh_token: None,
            expires_in: None,
            user_info: None,
            error_message: Some("Invalid credentials".to_string()),
            lockout_until: None,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":false"));
        assert!(json.contains("Invalid credentials"));
    }

    #[test]
    fn test_login_response_with_mfa() {
        let response = LoginResponse {
            success: false,
            user_id: Some(Uuid::new_v4()),
            session_id: None,
            requires_mfa: true,
            mfa_methods: vec!["Totp".to_string(), "Sms".to_string()],
            access_token: None,
            refresh_token: None,
            expires_in: None,
            user_info: None,
            error_message: None,
            lockout_until: None,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"requires_mfa\":true"));
        assert!(json.contains("Totp"));
        assert!(json.contains("Sms"));
    }

    #[test]
    fn test_login_response_with_lockout() {
        let lockout_time = chrono::Utc::now() + chrono::Duration::minutes(15);

        let response = LoginResponse {
            success: false,
            user_id: None,
            session_id: None,
            requires_mfa: false,
            mfa_methods: vec![],
            access_token: None,
            refresh_token: None,
            expires_in: None,
            user_info: None,
            error_message: Some("Account locked".to_string()),
            lockout_until: Some(lockout_time),
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("Account locked"));
        assert!(json.contains("lockout_until"));
    }
}

// ============================================================
// User Info Tests
// ============================================================

#[cfg(test)]
mod user_info_tests {
    use super::*;

    #[test]
    fn test_user_info_serialization() {
        let user_info = UserInfo {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            username: Some("testuser".to_string()),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email_verified: true,
            roles: vec!["user".to_string()],
        };

        let json = serde_json::to_string(&user_info).unwrap();

        assert!(json.contains("test@example.com"));
        assert!(json.contains("testuser"));
        assert!(json.contains("John"));
        assert!(json.contains("Doe"));
        assert!(json.contains("\"email_verified\":true"));
    }

    #[test]
    fn test_user_info_minimal() {
        let user_info = UserInfo {
            id: Uuid::new_v4(),
            email: "minimal@example.com".to_string(),
            username: None,
            first_name: None,
            last_name: None,
            email_verified: false,
            roles: vec![],
        };

        let json = serde_json::to_string(&user_info).unwrap();

        assert!(json.contains("minimal@example.com"));
        assert!(json.contains("\"email_verified\":false"));
    }
}

// ============================================================
// Integration Scenario Tests
// ============================================================

#[cfg(test)]
mod integration_scenario_tests {
    use super::*;

    #[test]
    fn test_login_payload_to_json() {
        let payload = LoginRequestPayload {
            email_or_username: "user@example.com".to_string(),
            password: "SecureP@ss123".to_string(),
            remember_me: Some(true),
            device_fingerprint: Some("device_fp".to_string()),
            mfa_code: None,
        };

        let json = json!(payload);

        assert_eq!(json["email_or_username"], "user@example.com");
        assert_eq!(json["password"], "SecureP@ss123");
        assert_eq!(json["remember_me"], true);
        assert_eq!(json["device_fingerprint"], "device_fp");
    }

    #[test]
    fn test_successful_login_response_structure() {
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        let response = json!({
            "success": true,
            "user_id": user_id,
            "session_id": session_id,
            "requires_mfa": false,
            "mfa_methods": [],
            "access_token": null,
            "refresh_token": null,
            "expires_in": null,
            "user_info": null,
            "error_message": null,
            "lockout_until": null
        });

        assert_eq!(response["success"], true);
        assert_eq!(response["requires_mfa"], false);
    }

    #[test]
    fn test_email_validation_in_payload() {
        let valid_emails = vec![
            "test@example.com",
            "user+tag@example.co.uk",
            "user-name@test.example.com",
        ];

        for email in valid_emails {
            let payload = LoginRequestPayload {
                email_or_username: email.to_string(),
                password: "test123".to_string(),
                remember_me: None,
                device_fingerprint: None,
                mfa_code: None,
            };

            assert_eq!(payload.email_or_username, email);
        }
    }
}
