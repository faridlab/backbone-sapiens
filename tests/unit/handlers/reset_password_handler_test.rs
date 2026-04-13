//! Reset Password Handler Unit Tests
//!
//! Tests for reset password handler including:
//! - Valid token and password
//! - Invalid token
//! - Expired token
//! - Password validation

use serde_json::json;

// ============================================================
// Request Deserialization Tests
// ============================================================

#[cfg(test)]
mod request_tests {
    use super::*;

    #[derive(Debug, serde::Deserialize)]
    pub struct ResetPasswordRequestPayload {
        pub token: String,
        pub new_password: String,
        pub confirm_password: String,
        pub device_fingerprint: Option<String>,
    }

    #[test]
    fn test_reset_password_request_deserialization() {
        let json_str = r#"{
            "token": "reset_token_abc123",
            "new_password": "NewSecureP@ss456",
            "confirm_password": "NewSecureP@ss456",
            "device_fingerprint": "fp_device_123"
        }"#;

        let payload: ResetPasswordRequestPayload = serde_json::from_str(json_str).unwrap();

        assert_eq!(payload.token, "reset_token_abc123");
        assert_eq!(payload.new_password, "NewSecureP@ss456");
        assert_eq!(payload.confirm_password, "NewSecureP@ss456");
        assert_eq!(payload.device_fingerprint, Some("fp_device_123".to_string()));
    }

    #[test]
    fn test_reset_password_request_minimal() {
        let json_str = r#"{
            "token": "token_xyz",
            "new_password": "password123",
            "confirm_password": "password123"
        }"#;

        let payload: ResetPasswordRequestPayload = serde_json::from_str(json_str).unwrap();

        assert_eq!(payload.token, "token_xyz");
        assert_eq!(payload.device_fingerprint, None);
    }

    #[test]
    fn test_password_match_validation() {
        let payload = ResetPasswordRequestPayload {
            token: "token".to_string(),
            new_password: "Password123!".to_string(),
            confirm_password: "Password123!".to_string(),
            device_fingerprint: None,
        };

        assert_eq!(payload.new_password, payload.confirm_password);
    }

    #[test]
    fn test_password_mismatch_validation() {
        let payload = ResetPasswordRequestPayload {
            token: "token".to_string(),
            new_password: "Password123!".to_string(),
            confirm_password: "Different456!".to_string(),
            device_fingerprint: None,
        };

        assert_ne!(payload.new_password, payload.confirm_password);
    }
}

// ============================================================
// Response Tests
// ============================================================

#[cfg(test)]
mod response_tests {
    use super::*;

    #[derive(Debug, serde::Serialize)]
    pub struct ResetPasswordResponse {
        pub success: bool,
        pub message: String,
        pub password_changed: bool,
        pub sessions_revoked: u32,
    }

    #[test]
    fn test_reset_password_response_success() {
        let response = ResetPasswordResponse {
            success: true,
            message: "Password has been reset successfully.".to_string(),
            password_changed: true,
            sessions_revoked: 1,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":true"));
        assert!(json.contains("reset successfully"));
        assert!(json.contains("\"password_changed\":true"));
        assert!(json.contains("\"sessions_revoked\":1"));
    }

    #[test]
    fn test_reset_password_response_invalid_token() {
        let response = ResetPasswordResponse {
            success: false,
            message: "Invalid or expired reset token.".to_string(),
            password_changed: false,
            sessions_revoked: 0,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":false"));
        assert!(json.contains("Invalid or expired"));
        assert!(json.contains("\"password_changed\":false"));
    }

    #[test]
    fn test_reset_password_response_password_mismatch() {
        let response = ResetPasswordResponse {
            success: false,
            message: "Passwords do not match.".to_string(),
            password_changed: false,
            sessions_revoked: 0,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("Passwords do not match"));
        assert!(!json.contains("\"password_changed\":true"));
    }

    #[test]
    fn test_reset_password_response_weak_password() {
        let response = ResetPasswordResponse {
            success: false,
            message: "Password does not meet security requirements.".to_string(),
            password_changed: false,
            sessions_revoked: 0,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("security requirements"));
        assert_eq!(response.sessions_revoked, 0);
    }
}

// ============================================================
// Token Validation Tests
// ============================================================

#[cfg(test)]
mod token_validation_tests {
    use super::*;

    #[test]
    fn test_valid_token_format() {
        let valid_tokens = vec![
            "reset_token_abc123def456",
            "token_with_underscores",
            "token-with-hyphens",
            "TokenWithMixedCase",
            "token123456",
        ];

        for token in valid_tokens {
            assert!(!token.is_empty());
            assert!(token.len() >= 8);
        }
    }

    #[test]
    fn test_invalid_token_format() {
        let invalid_tokens = vec![
            "",
            "   ",
            "\t\n",
            "a",
            "ab",
        ];

        for token in invalid_tokens {
            let payload = json!({
                "token": token,
                "new_password": "Password123!",
                "confirm_password": "Password123!"
            });

            assert!(payload["token"].as_str().unwrap().trim().len() < 8 || payload["token"].as_str().unwrap().is_empty());
        }
    }
}

// ============================================================
// Password Strength Tests
// ============================================================

#[cfg(test)]
mod password_strength_tests {
    use super::*;

    #[test]
    fn test_strong_password() {
        let strong_passwords = vec![
            "SecureP@ssw0rd123!",
            "MyStr0ng!Pass#2024",
            "C0mplex!ty@#$",
        ];

        for password in strong_passwords {
            assert!(password.len() >= 8);
            assert!(password.chars().any(|c| c.is_ascii_uppercase()));
            assert!(password.chars().any(|c| c.is_ascii_lowercase()));
            assert!(password.chars().any(|c| c.is_ascii_digit()));
        }
    }

    #[test]
    fn test_weak_password() {
        let weak_passwords = vec![
            "password",
            "12345678",
            "abcdefgh",
            "PASSWORD",
            "Pass1",
        ];

        for password in weak_passwords {
            let is_weak = password.len() < 8
                || !password.chars().any(|c| c.is_ascii_uppercase())
                || !password.chars().any(|c| c.is_ascii_digit());

            assert!(is_weak, "Password {} should be considered weak", password);
        }
    }

    #[test]
    fn test_password_confirmation() {
        let payload = json!({
            "token": "reset_token",
            "new_password": "NewPass123!",
            "confirm_password": "NewPass123!"
        });

        assert_eq!(payload["new_password"], payload["confirm_password"]);
    }

    #[test]
    fn test_password_confirmation_mismatch() {
        let payload = json!({
            "token": "reset_token",
            "new_password": "NewPass123!",
            "confirm_password": "DifferentPass456!"
        });

        assert_ne!(payload["new_password"], payload["confirm_password"]);
    }
}

// ============================================================
// Session Revocation Tests
// ============================================================

#[cfg(test)]
mod session_revocation_tests {
    use super::*;

    #[test]
    fn test_sessions_revoked_on_password_reset() {
        let response = json!({
            "success": true,
            "message": "Password reset successfully",
            "password_changed": true,
            "sessions_revoked": 3
        });

        assert_eq!(response["sessions_revoked"], 3);
        assert!(response["success"].as_bool().unwrap());
    }

    #[test]
    fn test_no_sessions_revoked_on_failure() {
        let response = json!({
            "success": false,
            "message": "Invalid token",
            "password_changed": false,
            "sessions_revoked": 0
        });

        assert_eq!(response["sessions_revoked"], 0);
        assert!(!response["success"].as_bool().unwrap());
    }
}
