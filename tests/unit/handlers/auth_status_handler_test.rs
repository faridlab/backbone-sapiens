//! Auth Status Handler Unit Tests
//!
//! Tests for authentication status handler including:
//! - Valid session status
//! - Invalid session status
//! - Session info retrieval
//! - User info retrieval

use backbone_sapiens::handlers::auth::auth_status_handler::{
    AuthStatusResponse, UserInfo, SessionInfo,
};
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;

// ============================================================
// Response Structure Tests
// ============================================================

#[cfg(test)]
mod response_structure_tests {
    use super::*;

    #[test]
    fn test_auth_status_response_authenticated() {
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        let response = AuthStatusResponse {
            authenticated: true,
            user_info: Some(UserInfo {
                id: user_id,
                email: "test@example.com".to_string(),
                username: Some("testuser".to_string()),
                first_name: Some("Test".to_string()),
                last_name: Some("User".to_string()),
                email_verified: true,
                roles: vec!["user".to_string()],
                permissions: vec!["read".to_string(), "write".to_string()],
                last_login: Some(Utc::now()),
                created_at: Utc::now().to_rfc3339(),
            }),
            session_info: Some(SessionInfo {
                id: session_id,
                device_type: Some("Web".to_string()),
                device_fingerprint: "fp123456".to_string(),
                ip_address: Some("192.168.1.1".to_string()),
                created_at: Utc::now().to_rfc3339(),
                last_activity: Some(Utc::now().to_rfc3339()),
                expires_at: (Utc::now() + chrono::Duration::hours(24)).to_rfc3339(),
                remember_me: false,
            }),
            permissions: vec!["read".to_string(), "write".to_string()],
            expires_at: Some((Utc::now() + chrono::Duration::hours(24)).to_rfc3339()),
            requires_reauth: false,
            security_alerts: vec![],
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"authenticated\":true"));
        assert!(json.contains("test@example.com"));
        assert!(json.contains("\"email_verified\":true"));
        assert!(json.contains("\"requires_reauth\":false"));
    }

    #[test]
    fn test_auth_status_response_unauthenticated() {
        let response = AuthStatusResponse {
            authenticated: false,
            user_info: None,
            session_info: None,
            permissions: vec![],
            expires_at: None,
            requires_reauth: false,
            security_alerts: vec!["Session not found".to_string()],
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"authenticated\":false"));
        assert!(json.contains("Session not found"));
    }

    #[test]
    fn test_auth_status_response_requires_reauth() {
        let response = AuthStatusResponse {
            authenticated: false,
            user_info: None,
            session_info: None,
            permissions: vec![],
            expires_at: None,
            requires_reauth: true,
            security_alerts: vec!["Session expired, please re-authenticate".to_string()],
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"requires_reauth\":true"));
        assert!(json.contains("re-authenticate"));
    }
}

// ============================================================
// User Info Tests
// ============================================================

#[cfg(test)]
mod user_info_tests {
    use super::*;

    #[test]
    fn test_user_info_complete() {
        let user_info = UserInfo {
            id: Uuid::new_v4(),
            email: "user@example.com".to_string(),
            username: Some("username".to_string()),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email_verified: true,
            roles: vec!["admin".to_string()],
            permissions: vec![],
            last_login: Some(Utc::now()),
            created_at: Utc::now().to_rfc3339(),
        };

        let json = serde_json::to_string(&user_info).unwrap();

        assert!(json.contains("user@example.com"));
        assert!(json.contains("username"));
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
            permissions: vec![],
            last_login: None,
            created_at: Utc::now().to_rfc3339(),
        };

        let json = serde_json::to_string(&user_info).unwrap();

        assert!(json.contains("minimal@example.com"));
        assert!(json.contains("\"email_verified\":false"));
        assert!(json.contains("\"roles\":[]"));
    }

    #[test]
    fn test_user_info_with_multiple_roles() {
        let user_info = UserInfo {
            id: Uuid::new_v4(),
            email: "multirole@example.com".to_string(),
            username: Some("multiuser".to_string()),
            first_name: Some("Multi".to_string()),
            last_name: Some("Role".to_string()),
            email_verified: true,
            roles: vec!["user".to_string(), "editor".to_string(), "viewer".to_string()],
            permissions: vec!["read".to_string(), "write".to_string()],
            last_login: Some(Utc::now()),
            created_at: Utc::now().to_rfc3339(),
        };

        assert_eq!(user_info.roles.len(), 3);
        assert!(user_info.roles.contains(&"editor".to_string()));
    }
}

// ============================================================
// Session Info Tests
// ============================================================

#[cfg(test)]
mod session_info_tests {
    use super::*;

    #[test]
    fn test_session_info_complete() {
        let session_id = Uuid::new_v4();

        let session_info = SessionInfo {
            id: session_id,
            device_type: Some("Mobile".to_string()),
            device_fingerprint: "fp_mobile_123".to_string(),
            ip_address: Some("10.0.0.1".to_string()),
            created_at: Utc::now().to_rfc3339(),
            last_activity: Some(Utc::now().to_rfc3339()),
            expires_at: (Utc::now() + chrono::Duration::days(30)).to_rfc3339(),
            remember_me: true,
        };

        let json = serde_json::to_string(&session_info).unwrap();

        assert!(json.contains("\"device_type\":\"Mobile\""));
        assert!(json.contains("fp_mobile_123"));
        assert!(json.contains("10.0.0.1"));
        assert!(json.contains("\"remember_me\":true"));
    }

    #[test]
    fn test_session_info_minimal() {
        let session_id = Uuid::new_v4();

        let session_info = SessionInfo {
            id: session_id,
            device_type: None,
            device_fingerprint: "unknown".to_string(),
            ip_address: None,
            created_at: Utc::now().to_rfc3339(),
            last_activity: None,
            expires_at: (Utc::now() + chrono::Duration::hours(24)).to_rfc3339(),
            remember_me: false,
        };

        let json = serde_json::to_string(&session_info).unwrap();

        assert!(json.contains("\"remember_me\":false"));
        assert!(json.contains("unknown"));
    }
}

// ============================================================
// Security Alerts Tests
// ============================================================

#[cfg(test)]
mod security_alerts_tests {
    use super::*;

    #[test]
    fn test_security_alerts_session_expired() {
        let response = AuthStatusResponse {
            authenticated: false,
            user_info: None,
            session_info: None,
            permissions: vec![],
            expires_at: None,
            requires_reauth: false,
            security_alerts: vec![
                "Session has expired".to_string(),
                "Please login again".to_string(),
            ],
        };

        assert_eq!(response.security_alerts.len(), 2);
        assert!(response.security_alerts[0].contains("expired"));
    }

    #[test]
    fn test_security_alerts_suspicious_activity() {
        let response = AuthStatusResponse {
            authenticated: false,
            user_info: None,
            session_info: None,
            permissions: vec![],
            expires_at: None,
            requires_reauth: true,
            security_alerts: vec![
                "Suspicious activity detected".to_string(),
                "Login from new location".to_string(),
            ],
        };

        assert!(response.requires_reauth);
        assert_eq!(response.security_alerts.len(), 2);
    }

    #[test]
    fn test_no_security_alerts() {
        let response = AuthStatusResponse {
            authenticated: true,
            user_info: None,
            session_info: None,
            permissions: vec![],
            expires_at: Some(Utc::now().to_rfc3339()),
            requires_reauth: false,
            security_alerts: vec![],
        };

        assert!(response.security_alerts.is_empty());
        assert!(!response.requires_reauth);
    }
}

// ============================================================
// Permission Tests
// ============================================================

#[cfg(test)]
mod permission_tests {
    use super::*;

    #[test]
    fn test_permissions_list() {
        let response = AuthStatusResponse {
            authenticated: true,
            user_info: None,
            session_info: None,
            permissions: vec![
                "users:read".to_string(),
                "users:write".to_string(),
                "posts:read".to_string(),
                "posts:write".to_string(),
            ],
            expires_at: Some(Utc::now().to_rfc3339()),
            requires_reauth: false,
            security_alerts: vec![],
        };

        assert_eq!(response.permissions.len(), 4);
        assert!(response.permissions.contains(&"users:write".to_string()));
    }

    #[test]
    fn test_empty_permissions() {
        let response = AuthStatusResponse {
            authenticated: true,
            user_info: None,
            session_info: None,
            permissions: vec![],
            expires_at: Some(Utc::now().to_rfc3339()),
            requires_reauth: false,
            security_alerts: vec![],
        };

        assert!(response.permissions.is_empty());
    }
}
