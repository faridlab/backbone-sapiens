//! Test Utilities
//!
//! Common utilities and helper functions for authentication tests.

use serde_json::{json, Value};
use uuid::Uuid;
use chrono::Utc;

/// Test user data generator
#[derive(Debug, Clone)]
pub struct TestUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub metadata: Value,
}

impl TestUser {
    pub fn new() -> Self {
        let timestamp = Utc::now().timestamp();
        let id = Uuid::new_v4();

        Self {
            id,
            username: format!("testuser_{}", timestamp),
            email: format!("test{}@auth.test", timestamp),
            password: "SecureP@ssw0rd123!".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            phone: Some(format!("+1555123456{}", timestamp % 1000)),
            metadata: json!({
                "created_at": Utc::now().to_rfc3339(),
                "updated_at": Utc::now().to_rfc3339(),
                "test_user": true,
                "source": "test_suite"
            }),
        }
    }

    pub fn login_payload(&self) -> Value {
        json!({
            "email_or_username": self.email.clone(),
            "password": self.password.clone(),
            "remember_me": false,
            "device_fingerprint": format!("test_fp_{}", self.id)
        })
    }

    pub fn registration_payload(&self) -> Value {
        json!({
            "username": self.username,
            "email": self.email,
            "password": self.password,
            "confirm_password": self.password,
            "first_name": self.first_name,
            "last_name": self.last_name,
            "accept_terms": true,
            "device_fingerprint": format!("test_fp_{}", self.id),
            "newsletter_opt_in": false
        })
    }
}

/// Test session data generator
#[derive(Debug, Clone)]
pub struct TestSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub device_fingerprint: String,
    pub ip_address: String,
    pub user_agent: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub metadata: Value,
}

impl TestSession {
    pub fn new(user_id: Uuid) -> Self {
        let id = Uuid::new_v4();
        let now = Utc::now();

        Self {
            id,
            user_id,
            token_hash: format!("session_token_{}", id),
            device_fingerprint: format!("test_fp_{}", id),
            ip_address: "127.0.0.1".to_string(),
            user_agent: "TestSuite/1.0".to_string(),
            expires_at: now + chrono::Duration::hours(24),
            metadata: json!({
                "created_at": now.to_rfc3339(),
                "updated_at": now.to_rfc3339(),
                "test_session": true
            }),
        }
    }

    pub fn status_payload(&self) -> Value {
        json!({
            "session_id": self.id,
            "user_id": self.user_id,
            "device_fingerprint": self.device_fingerprint
        })
    }
}

/// Test email verification token generator
#[derive(Debug, Clone)]
pub struct TestVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub email: String,
    pub token_type: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub metadata: Value,
}

impl TestVerificationToken {
    pub fn new(user_id: Uuid, email: String) -> Self {
        let id = Uuid::new_v4();
        let now = Utc::now();

        Self {
            id,
            user_id,
            token: format!("verify_token_{}", id),
            email,
            token_type: "email_verification".to_string(),
            expires_at: now + chrono::Duration::hours(24),
            metadata: json!({
                "created_at": now.to_rfc3339(),
                "test_token": true
            }),
        }
    }

    pub fn verification_payload(&self) -> Value {
        json!({
            "token": self.token
        })
    }
}

/// Test password reset token generator
#[derive(Debug, Clone)]
pub struct TestPasswordResetToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub email: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub metadata: Value,
}

impl TestPasswordResetToken {
    pub fn new(user_id: Uuid, email: String) -> Self {
        let id = Uuid::new_v4();
        let now = Utc::now();

        Self {
            id,
            user_id,
            token: format!("reset_token_{}", id),
            email,
            expires_at: now + chrono::Duration::hours(1),
            metadata: json!({
                "created_at": now.to_rfc3339(),
                "test_token": true
            }),
        }
    }

    pub fn reset_request_payload(&self) -> Value {
        json!({
            "email": self.email
        })
    }

    pub fn reset_payload(&self, new_password: &str) -> Value {
        json!({
            "token": self.token,
            "new_password": new_password,
            "confirm_password": new_password
        })
    }
}

/// Mock application state for testing
#[derive(Debug, Clone)]
pub struct MockAppState {
    pub database_url: String,
    pub jwt_secret: String,
    pub rate_limit_enabled: bool,
    pub csrf_protection_enabled: bool,
}

impl MockAppState {
    pub fn new() -> Self {
        Self {
            database_url: "postgresql://postgres:password@localhost:5432/backbonedb".to_string(),
            jwt_secret: "test_jwt_secret_key_for_testing_only".to_string(),
            rate_limit_enabled: true,
            csrf_protection_enabled: false, // Disabled for easier testing
        }
    }

    pub fn with_database_url(url: &str) -> Self {
        let mut state = Self::new();
        state.database_url = url.to_string();
        state
    }

    pub fn with_rate_limiting(enabled: bool) -> Self {
        let mut state = Self::new();
        state.rate_limit_enabled = enabled;
        state
    }

    pub fn with_csrf_protection(enabled: bool) -> Self {
        let mut state = Self::new();
        state.csrf_protection_enabled = enabled;
        state
    }
}

/// HTTP test utilities
pub mod http {
    use axum::{
        body::Body,
        http::{Request, StatusCode, Method},
    };
    use serde_json::Value;
    use std::collections::HashMap;

    /// Create a test HTTP request
    pub fn create_test_request(
        method: Method,
        uri: &str,
        body: Option<Value>,
        headers: Option<HashMap<String, String>>,
    ) -> Request<Body> {
        let mut request_builder = Request::builder()
            .method(method)
            .uri(uri);

        // Add default headers
        request_builder = request_builder.header("Content-Type", "application/json");
        request_builder = request_builder.header("User-Agent", "TestSuite/1.0");

        // Add custom headers
        if let Some(custom_headers) = headers {
            for (key, value) in custom_headers {
                request_builder = request_builder.header(key, value);
            }
        }

        // Add body
        let body = match body {
            Some(json) => Body::from(json.to_string()),
            None => Body::empty(),
        };

        request_builder.body(body).unwrap()
    }

    /// Create a POST request with JSON body
    pub fn create_post_request(uri: &str, body: Value) -> Request<Body> {
        create_test_request(Method::POST, uri, Some(body), None)
    }

    /// Create a GET request
    pub fn create_get_request(uri: &str) -> Request<Body> {
        create_test_request(Method::GET, uri, None, None)
    }

    /// Create a request with custom headers
    pub fn create_request_with_headers(
        method: Method,
        uri: &str,
        body: Option<Value>,
        headers: HashMap<String, String>,
    ) -> Request<Body> {
        create_test_request(method, uri, body, Some(headers))
    }
}

/// Assertion utilities
pub mod assertions {
    use axum::{body::Body, http::StatusCode, response::Response};
    use serde_json::Value;

    /// Assert response status code
    pub fn assert_status(response: &Response<Body>, expected_status: StatusCode) {
        assert_eq!(
            response.status(),
            expected_status,
            "Expected status {}, got {}",
            expected_status,
            response.status()
        );
    }

    /// Assert response is a client error (4xx)
    pub fn assert_client_error(response: &Response<Body>) {
        assert!(
            response.status().is_client_error(),
            "Expected client error (4xx), got {}",
            response.status()
        );
    }

    /// Assert response is a server error (5xx)
    pub fn assert_server_error(response: &Response<Body>) {
        assert!(
            response.status().is_server_error(),
            "Expected server error (5xx), got {}",
            response.status()
        );
    }

    /// Assert response is successful (2xx)
    pub fn assert_success(response: &Response<Body>) {
        assert!(
            response.status().is_success(),
            "Expected success (2xx), got {}",
            response.status()
        );
    }

    /// Assert JSON response contains specific field
    pub async fn assert_json_field(
        response: Response<Body>,
        field_path: &str,
        expected_value: &Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (parts, body) = response.into_parts();
        let bytes = axum::body::to_bytes(body, usize::MAX).await?;
        let response_json: Value = serde_json::from_slice(&bytes)?;

        let actual_value = response_json
            .pointer(field_path)
            .ok_or(format!("Field '{}' not found in response", field_path))?;

        assert_eq!(
            actual_value, expected_value,
            "Expected field '{}' to be {:?}, got {:?}",
            field_path, expected_value, actual_value
        );

        Ok(())
    }

    /// Assert response has JSON content type
    pub fn assert_json_content_type(response: &Response<Body>) {
        let content_type = response
            .headers()
            .get("content-type")
            .expect("Missing content-type header");

        assert_eq!(
            content_type.to_str().unwrap(),
            "application/json",
            "Expected content-type 'application/json', got '{}'",
            content_type.to_str().unwrap()
        );
    }
}

/// Database test utilities
pub mod database {
    use sqlx::PgPool;
    use uuid::Uuid;

    /// Clean up test data from database
    pub async fn cleanup_test_data(
        pool: &PgPool,
        user_ids: &[Uuid],
        session_ids: &[Uuid],
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Delete sessions
        for session_id in session_ids {
            let _ = sqlx::query!("DELETE FROM sessions WHERE id = $1", session_id)
                .execute(pool)
                .await?;
        }

        // Delete users
        for user_id in user_ids {
            let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    /// Create test database connection
    pub async fn create_test_pool() -> Result<PgPool, Box<dyn std::error::Error>> {
        let database_url = "postgresql://postgres:password@localhost:5432/backbonedb";
        Ok(PgPool::connect(database_url).await?)
    }
}