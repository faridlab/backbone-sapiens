//! HTTP Response Utilities
//!
//! Standardized response structures for API endpoints.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Serialize, Deserialize};

/// Standard API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub timestamp: String,
}

/// API error information
#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

/// Standard API response
impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code: "ERROR".to_string(),
                message,
                details: None,
            }),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn error_with_details(message: String, details: serde_json::Value) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code: "ERROR".to_string(),
                message,
                details: Some(details),
            }),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn error_code(message: String, code: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code: code.to_string(),
                message,
                details: None,
            }),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Error response for common HTTP errors
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: Option<String>,
    pub timestamp: String,
}

impl ErrorResponse {
    pub fn new(error: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
            code: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_code(error: &str, message: &str, code: &str) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
            code: Some(code.to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status_code = if self.success {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };

        (status_code, Json(self)).into_response()
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status_code = StatusCode::BAD_REQUEST;
        (status_code, Json(self)).into_response()
    }
}

/// Common success responses
pub mod success {
    use super::*;

    pub fn created<T: Serialize>(data: T) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn updated<T: Serialize>(data: T) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn deleted<T: Serialize>(data: T) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn accepted<T: Serialize>(data: T) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Common error responses
pub mod errors {
    use super::*;

    pub fn bad_request(message: &str) -> ErrorResponse {
        ErrorResponse::new("BAD_REQUEST", message)
    }

    pub fn unauthorized(message: &str) -> ErrorResponse {
        ErrorResponse::new("UNAUTHORIZED", message)
    }

    pub fn forbidden(message: &str) -> ErrorResponse {
        ErrorResponse::new("FORBIDDEN", message)
    }

    pub fn not_found(message: &str) -> ErrorResponse {
        ErrorResponse::new("NOT_FOUND", message)
    }

    pub fn conflict(message: &str) -> ErrorResponse {
        ErrorResponse::new("CONFLICT", message)
    }

    pub fn internal_error(message: &str) -> ErrorResponse {
        ErrorResponse::new("INTERNAL_ERROR", message)
    }

    pub fn validation_error(message: &str, field: &str) -> ErrorResponse {
        ErrorResponse::with_code(
            "VALIDATION_ERROR",
            message,
            &format!("FIELD_{}", field.to_uppercase())
        )
    }

    pub fn rate_limit_exceeded(message: &str) -> ErrorResponse {
        ErrorResponse::with_code("RATE_LIMIT_EXCEEDED", message, "TOO_MANY_REQUESTS")
    }

    pub fn token_expired() -> ErrorResponse {
        ErrorResponse::with_code("TOKEN_EXPIRED", "Authentication token has expired", "EXPIRED_TOKEN")
    }

    pub fn token_invalid() -> ErrorResponse {
        ErrorResponse::with_code("TOKEN_INVALID", "Invalid authentication token", "INVALID_TOKEN")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[derive(Serialize)]
    struct TestData {
        id: u32,
        name: String,
    }

    #[test]
    fn test_success_response() {
        let data = TestData { id: 1, name: "test".to_string() };
        let response = ApiResponse::success(data);

        assert!(response.success);
        assert!(response.error.is_none());
        assert!(response.data.is_some());
    }

    #[test]
    fn test_error_response() {
        let response: ApiResponse<()> = ApiResponse::error("Something went wrong".to_string());

        assert!(!response.success);
        assert!(response.error.is_some());
        assert!(response.data.is_none());
    }

    #[test]
    fn test_error_response_serialization() {
        let response = ErrorResponse::new("BAD_REQUEST", "Invalid input");
        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"error\":\"BAD_REQUEST\""));
        assert!(json.contains("\"message\":\"Invalid input\""));
    }
}