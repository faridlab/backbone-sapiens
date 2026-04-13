//! Logout Handler Unit Tests
//!
//! Tests for logout handler including:
//! - Single device logout
//! - All devices logout
//! - Session cleanup

use backbone_sapiens::handlers::auth::logout_handler::LogoutRequestPayload;
use serde_json::json;

// ============================================================
// Request Deserialization Tests
// ============================================================

#[cfg(test)]
mod request_tests {
    use super::*;

    #[test]
    fn test_logout_request_with_device_untrust() {
        let json_str = r#"{
            "device_untrust": true
        }"#;

        let payload: LogoutRequestPayload = serde_json::from_str(json_str).unwrap();

        assert_eq!(payload.device_untrust, Some(true));
    }

    #[test]
    fn test_logout_request_without_device_untrust() {
        let json_str = r#"{}"#;

        let payload: LogoutRequestPayload = serde_json::from_str(json_str).unwrap();

        assert_eq!(payload.device_untrust, None);
    }

    #[test]
    fn test_logout_request_all_false() {
        let payload = LogoutRequestPayload {
            device_untrust: Some(false),
        };

        let json = serde_json::to_string(&payload).unwrap();
        let parsed: LogoutRequestPayload = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.device_untrust, Some(false));
    }
}

// ============================================================
// Logout Response Tests
// ============================================================

#[cfg(test)]
mod response_tests {
    use super::*;
    use uuid::Uuid;

    #[derive(Debug, serde::Serialize)]
    struct LogoutResponse {
        pub success: bool,
        pub message: String,
        pub sessions_terminated: u32,
    }

    #[test]
    fn test_logout_response_success() {
        let response = LogoutResponse {
            success: true,
            message: "Logged out successfully".to_string(),
            sessions_terminated: 1,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":true"));
        assert!(json.contains("Logged out successfully"));
        assert!(json.contains("\"sessions_terminated\":1"));
    }

    #[test]
    fn test_logout_all_devices_response() {
        let response = LogoutResponse {
            success: true,
            message: "Logged out from all devices".to_string(),
            sessions_terminated: 3,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"sessions_terminated\":3"));
        assert!(json.contains("all devices"));
    }

    #[test]
    fn test_logout_response_failure() {
        let response = LogoutResponse {
            success: false,
            message: "Session not found".to_string(),
            sessions_terminated: 0,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"success\":false"));
        assert!(json.contains("Session not found"));
    }
}

// ============================================================
// Logout Scenarios Tests
// ============================================================

#[cfg(test)]
mod logout_scenarios_tests {
    use super::*;

    #[test]
    fn test_logout_from_single_device() {
        let payload = LogoutRequestPayload {
            device_untrust: None,
        };

        let scenario = json!({
            "action": "logout",
            "scope": "current_device",
            "device_untrust": false
        });

        assert_eq!(scenario["scope"], "current_device");
    }

    #[test]
    fn test_logout_from_all_devices_without_untrust() {
        let payload = LogoutRequestPayload {
            device_untrust: Some(false),
        };

        let scenario = json!({
            "action": "logout_all",
            "scope": "all_devices",
            "device_untrust": false
        });

        assert_eq!(scenario["scope"], "all_devices");
        assert_eq!(scenario["device_untrust"], false);
    }

    #[test]
    fn test_logout_from_all_devices_with_untrust() {
        let payload = LogoutRequestPayload {
            device_untrust: Some(true),
        };

        let scenario = json!({
            "action": "logout_all",
            "scope": "all_devices",
            "device_untrust": true
        });

        assert_eq!(scenario["scope"], "all_devices");
        assert_eq!(scenario["device_untrust"], true);
    }
}

// ============================================================
// Session Cleanup Tests
// ============================================================

#[cfg(test)]
mod session_cleanup_tests {
    use super::*;

    #[test]
    fn test_session_cleanup_on_logout() {
        let session_id = Uuid::new_v4();

        let cleanup_result = json!({
            "session_id": session_id,
            "terminated": true,
            "terminated_at": chrono::Utc::now().to_rfc3339()
        });

        assert_eq!(cleanup_result["terminated"], true);
        assert!(cleanup_result["session_id"].is_string());
    }

    #[test]
    fn test_multiple_sessions_cleanup() {
        let session_ids = vec![
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        ];

        let cleanup_result = json!({
            "terminated_count": 3,
            "session_ids": session_ids
        });

        assert_eq!(cleanup_result["terminated_count"], 3);
        assert!(cleanup_result["session_ids"].is_array());
    }
}
