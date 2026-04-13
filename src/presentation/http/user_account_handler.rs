//! HTTP Handlers for User Account Management
//!
//! Handles HTTP requests for user lifecycle operations including suspension,
//! reactivation, email changes, account merging, and GDPR compliance.

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use backbone::response::ApiResponse;
use crate::presentation::dto::UserAccountDto;
use crate::domain::services::user_lifecycle_service::{
    UserLifecycleService, SuspendUserRequest, UnsuspendUserRequest,
    ReactivateUserRequest, ChangeEmailRequest, ForcePasswordChangeRequest,
    MergeAccountsRequest, GetAuditLogRequest, GetAccessHistoryRequest,
    DataSubjectRequestRequest, DataSubjectRequestResponse,
};

/// Application state for user account management
#[derive(Clone)]
pub struct UserAccountAppState {
    pub user_lifecycle_service: Arc<dyn UserLifecycleService>,
}

/// Create a new user account management app state
pub fn new_user_account_state(
    user_lifecycle_service: Arc<dyn UserLifecycleService>,
) -> UserAccountAppState {
    UserAccountAppState {
        user_lifecycle_service,
    }
}

// ==================== SUSPENSION ENDPOINTS ====================

/// Suspend user account
///
/// # Endpoint
/// `POST /api/v1/users/{user_id}/suspend`
///
/// # Description
/// Suspends a user account with specified reason and duration.
/// Requires administrative privileges.
///
/// # Request Headers
/// - `Authorization`: Bearer token (required)
/// - `X-Request-ID`: Request tracking ID (optional)
/// - `X-Admin-User-ID`: Admin user ID (optional)
///
/// # Request Body
/// ```json
/// {
///   "reason": "Policy violation",
///   "duration_hours": 168,
///   "notify_user": true,
///   "evidence": ["violation1.png", "report.pdf"],
///   "review_required": true
/// }
/// ```
///
/// # Responses
/// - `200 OK`: User suspended successfully
/// - `400 Bad Request`: Invalid input
/// - `401 Unauthorized`: Authentication required
/// - `403 Forbidden`: Insufficient privileges
/// - `404 Not Found`: User not found
/// - `409 Conflict`: User already suspended
/// - `500 Internal Server Error`: Server error
///
/// # Response Body
/// ```json
/// {
///   "success": true,
///   "data": {
///     "suspension": {
///       "id": "550e8400-e29b-41d4-a716-446655440000",
///       "user_id": "550e8400-e29b-41d4-a716-446655440001",
///       "suspended_by_user_id": "550e8400-e29b-41d4-a716-446655440002",
///       "suspension_reason": "Policy violation",
///       "suspended_at": "2024-01-15T10:30:00Z",
///       "suspension_end_time": "2024-01-22T10:30:00Z",
///       "permanent": false,
///       "active": true
///     },
///     "message": "User suspended successfully"
///   }
/// }
/// ```
pub async fn suspend_user(
    State(state): State<Arc<UserAccountAppState>>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
    Json(request): Json<Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Validate admin user ID from headers
    let admin_user_id = extract_admin_user_id(&headers)?;

    // Parse request body
    let suspend_request: SuspendUserRequest = serde_json::from_value(request)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Call domain service
    let response = state.user_lifecycle_service
        .suspend_user(suspend_request, admin_user_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to suspend user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse::success(
        json!(response),
        "User suspended successfully"
    )))
}

/// Unsuspend user account
///
/// # Endpoint
/// `POST /api/v1/users/{user_id}/unsuspend`
///
/// # Description
/// Unsuspends a previously suspended user account.
/// Requires administrative privileges.
///
/// # Request Headers
/// - `Authorization`: Bearer token (required)
/// - `X-Request-ID`: Request tracking ID (optional)
///
/// # Request Body
/// ```json
/// {
///   "reason": "Investigation completed",
///   "notify_user": true
/// }
/// ```
///
/// # Responses
/// - `200 OK`: User unsuspended successfully
/// - `400 Bad Request`: Invalid input
/// - `401 Unauthorized`: Authentication required
/// - `403 Forbidden`: Insufficient privileges
/// - `404 Not Found`: User or suspension not found
/// - `500 Internal Server Error`: Server error
///
/// # Response Body
/// ```json
/// {
///   "success": true,
///   "data": {
///     "suspension": {
///       "id": "550e8400-e29b-41d4-a716-446655440000",
///       "user_id": "550e8400-e29b-41d4-a716-446655440001",
///       "suspension_reason": "Policy violation",
///       "suspended_at": "2024-01-15T10:30:00Z",
///       "active": false
///     },
///     "message": "User unsuspended successfully"
///   }
/// }
/// ```
pub async fn unsuspend_user(
    State(state): State<Arc<UserAccountAppState>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Parse request body
    let unsuspend_request: UnsuspendUserRequest = serde_json::from_value(request)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Call domain service
    let response = state.user_lifecycle_service
        .unsuspend_user(unsuspend_request)
        .await
        .map_err(|e| {
            tracing::error!("Failed to unsuspend user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse::success(
        json!(response),
        "User unsuspended successfully"
    )))
}

/// Reactivate user account
///
/// # Endpoint
/// `POST /api/v1/users/{user_id}/reactivate`
///
/// # Description
/// Reactivates an inactive user account (for soft-deleted or expired accounts).
///
/// # Request Headers
/// - `Authorization`: Bearer token (required)
/// - `X-Request-ID`: Request tracking ID (optional)
///
/// # Request Body
/// ```json
/// {
///   "reactivation_type": "admin_initiated",
///   "reason": "Customer request resolved",
///   "send_welcome_email": true
/// }
/// ```
///
/// # Responses
/// - `200 OK`: User reactivated successfully
/// - `400 Bad Request`: Invalid input
/// - `401 Unauthorized`: Authentication required
/// - `404 Not Found`: User not found
/// - `409 Conflict`: User already active
/// - `500 Internal Server Error`: Server error
///
/// # Response Body
/// ```json
/// {
///   "success": true,
///   "data": {
///     "user": {
///       "id": "550e8400-e29b-41d4-a716-446655440001",
///       "email": "user@example.com",
///       "username": "johndoe",
///       "status": "active",
///       "email_verified": true,
///       "last_login_at": "2024-01-15T10:30:00Z",
///       "updated_at": "2024-01-15T10:30:00Z"
///     },
///     "message": "User reactivated successfully"
///   }
/// }
/// ```
pub async fn reactivate_user(
    State(state): State<Arc<UserAccountAppState>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Parse request body
    let reactivate_request: ReactivateUserRequest = serde_json::from_value(request)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Call domain service
    let response = state.user_lifecycle_service
        .reactivate_user(reactivate_request)
        .await
        .map_err(|e| {
            tracing::error!("Failed to reactivate user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse::success(
        json!(response),
        "User reactivated successfully"
    )))
}

// ==================== EMAIL MANAGEMENT ENDPOINTS ====================

/// Change user email
///
/// # Endpoint
/// `POST /api/v1/users/{user_id}/change-email`
///
/// # Description
/// Initiates an email change process with verification.
///
/// # Request Headers
/// - `Authorization`: Bearer token (required)
/// - `X-Request-ID`: Request tracking ID (optional)
///
/// # Request Body
/// ```json
/// {
///   "new_email": "newemail@example.com",
///   "password": "current_password",
///   "send_confirmation_to_old": true,
///   "send_notification_to_new": true
/// }
/// ```
///
/// # Responses
/// - `200 OK`: Email change initiated successfully
/// - `400 Bad Request`: Invalid input or email already exists
/// - `401 Unauthorized`: Authentication required
/// - `403 Forbidden`: Insufficient privileges
/// - `404 Not Found`: User not found
/// - `500 Internal Server Error`: Server error
///
/// # Response Body
/// ```json
/// {
///   "success": true,
///   "data": {
///     "request_id": "550e8400-e29b-41d4-a716-446655440000",
///     "verification_token_expires_at": "2024-01-15T11:30:00Z",
///     "message": "Email change initiated successfully"
///   }
/// }
/// ```
pub async fn change_email(
    State(state): State<Arc<UserAccountAppState>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Parse request body
    let change_request: ChangeEmailRequest = serde_json::from_value(request)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Call domain service
    let response = state.user_lifecycle_service
        .change_email(change_request)
        .await
        .map_err(|e| {
            tracing::error!("Failed to initiate email change: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse::success(
        json!(response),
        "Email change initiated successfully"
    )))
}

/// Force password change
///
/// # Endpoint
/// `POST /api/v1/users/{user_id}/force-password-change`
///
/// # Description
/// Forces a user to change their password on next login.
/// Requires administrative privileges.
///
/// # Request Headers
/// - `Authorization`: Bearer token (required)
/// - `X-Request-ID`: Request tracking ID (optional)
/// - `X-Admin-User-ID`: Admin user ID (optional)
///
/// # Request Body
/// ```json
/// {
///   "reason": "Security precaution",
///   "notify_user": true,
///   "expiration_hours": 24
/// }
/// ```
///
/// # Responses
/// - `200 OK`: Password change requirement set successfully
/// - `400 Bad Request`: Invalid input
/// - `401 Unauthorized`: Authentication required
/// - `403 Forbidden`: Insufficient privileges
/// - `404 Not Found`: User not found
/// - `500 Internal Server Error`: Server error
///
/// # Response Body
/// ```json
/// {
///   "success": true,
///   "data": {
///     "user_id": "550e8400-e29b-41d4-a716-446655440001",
///     "temporary_token": "temp_token_12345",
///     "expires_at": "2024-01-16T10:30:00Z",
///     "message": "Password change requirement set successfully"
///   }
/// }
/// ```
pub async fn force_password_change(
    State(state): State<Arc<UserAccountAppState>>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
    Json(request): Json<Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Validate admin user ID from headers
    let admin_user_id = extract_admin_user_id(&headers)?;

    // Parse request body
    let force_request: ForcePasswordChangeRequest = serde_json::from_value(request)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Call domain service
    let response = state.user_lifecycle_service
        .force_password_change(force_request, admin_user_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to force password change: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse::success(
        json!(response),
        "Password change requirement set successfully"
    )))
}

// ==================== ACCOUNT MANAGEMENT ENDPOINTS ====================

/// Merge user accounts
///
/// # Endpoint
/// `POST /api/v1/users/{user_id}/merge-accounts`
///
/// # Description
/// Merges a source account into the target account.
/// Requires administrative privileges.
///
/// # Request Headers
/// - `Authorization`: Bearer token (required)
/// - `X-Request-ID`: Request tracking ID (optional)
/// - `X-Admin-User-ID`: Admin user ID (optional)
///
/// # Request Body
/// ```json
/// {
///   "source_user_id": "550e8400-e29b-41d4-a716-446655440002",
///   "merge_reason": "Duplicate accounts",
///   "merge_strategy": "target_preferred",
///   "send_notification": true,
///   "confirmation": true
/// }
/// ```
///
/// # Responses
/// - `200 OK`: Accounts merged successfully
/// - `400 Bad Request`: Invalid input
/// - `401 Unauthorized`: Authentication required
/// - `403 Forbidden`: Insufficient privileges
/// - `404 Not Found`: User not found
/// - `409 Conflict`: Merge not allowed
/// - `500 Internal Server Error`: Server error
///
/// # Response Body
/// ```json
/// {
///   "success": true,
///   "data": {
///     "merge_id": "550e8400-e29b-41d4-a716-446655440000",
///     "target_user_id": "550e8400-e29b-41d4-a716-446655440001",
///     "source_user_id": "550e8400-e29b-41d4-a716-446655440002",
///     "merged_data": {
///       "sessions_transferred": 3,
///       "api_keys_transferred": 2,
///       "preferences_merged": 1
///     },
///     "message": "Accounts merged successfully"
///   }
/// }
/// ```
pub async fn merge_accounts(
    State(state): State<Arc<UserAccountAppState>>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
    Json(request): Json<Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Validate admin user ID from headers
    let admin_user_id = extract_admin_user_id(&headers)?;

    // Parse request body
    let merge_request: MergeAccountsRequest = serde_json::from_value(request)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Call domain service
    let response = state.user_lifecycle_service
        .merge_accounts(merge_request, admin_user_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to merge accounts: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse::success(
        json!(response),
        "Accounts merged successfully"
    )))
}

// ==================== AUDIT AND HISTORY ENDPOINTS ====================

/// Get user audit log
///
/// # Endpoint
/// `GET /api/v1/users/{user_id}/audit-log`
///
/// # Description
/// Retrieves the audit log for a specific user.
/// Requires administrative privileges.
///
/// # Request Headers
/// - `Authorization`: Bearer token (required)
/// - `X-Request-ID`: Request tracking ID (optional)
///
/// # Query Parameters
/// - `start_date`: Start date for audit log (ISO 8601)
/// - `end_date`: End date for audit log (ISO 8601)
/// - `page`: Page number (default: 1)
/// - `limit`: Items per page (default: 50, max: 100)
/// - `sort_by`: Sort field (default: event_timestamp)
/// - `sort_order`: Sort order (default: desc)
///
/// # Responses
/// - `200 OK`: Audit log retrieved successfully
/// - `400 Bad Request`: Invalid input
/// - `401 Unauthorized`: Authentication required
/// - `403 Forbidden`: Insufficient privileges
/// - `404 Not Found`: User not found
/// - `500 Internal Server Error`: Server error
///
/// # Response Body
/// ```json
/// {
///   "success": true,
///   "data": {
///     "audit_logs": [
///       {
///         "id": "550e8400-e29b-41d4-a716-446655440000",
///         "user_id": "550e8400-e29b-41d4-a716-446655440001",
///         "entity_type": "user",
///         "entity_id": "550e8400-e29b-41d4-a716-446655440001",
///         "action": "user_suspended",
///         "old_values": {"status": "active"},
///         "new_values": {"status": "suspended"},
///         "performed_by_user_id": "550e8400-e29b-41d4-a716-446655440002",
///         "ip_address": "192.168.1.100",
///         "user_agent": "Mozilla/5.0...",
///         "event_timestamp": "2024-01-15T10:30:00Z"
///       }
///     ],
///     "pagination": {
///       "page": 1,
///       "limit": 50,
///       "total": 1,
///       "total_pages": 1
///     }
///   }
/// }
/// ```
pub async fn get_user_audit_log(
    State(state): State<Arc<UserAccountAppState>>,
    Path(user_id): Path<Uuid>,
    axum::extract::Query(params): axum::extract::Query<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Parse query parameters
    let audit_request = GetAuditLogRequest {
        user_id,
        start_date: params.get("start_date")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc)),
        end_date: params.get("end_date")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc)),
        page: params.get("page")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u32,
        limit: params.get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(50) as u32,
        sort_by: params.get("sort_by")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        sort_order: params.get("sort_order")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
    };

    // Call domain service
    let response = state.user_lifecycle_service
        .get_user_audit_log(audit_request)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user audit log: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse::success(
        json!(response),
        "Audit log retrieved successfully"
    )))
}

/// Get user access history
///
/// # Endpoint
/// `GET /api/v1/users/{user_id}/access-history`
///
/// # Description
/// Retrieves the access history for a specific user.
///
/// # Request Headers
/// - `Authorization`: Bearer token (required)
/// - `X-Request-ID`: Request tracking ID (optional)
///
/// # Query Parameters
/// - `start_date`: Start date for access history (ISO 8601)
/// - `end_date`: End date for access history (ISO 8601)
/// - `page`: Page number (default: 1)
/// - `limit`: Items per page (default: 50, max: 100)
/// - `include_device_details`: Include device details (default: true)
/// - `include_location_details`: Include location details (default: true)
///
/// # Responses
/// - `200 OK`: Access history retrieved successfully
/// - `400 Bad Request`: Invalid input
/// - `401 Unauthorized`: Authentication required
/// - `404 Not Found`: User not found
/// - `500 Internal Server Error`: Server error
///
/// # Response Body
/// ```json
/// {
///   "success": true,
///   "data": {
///     "access_history": [
///       {
///         "id": "550e8400-e29b-41d4-a716-446655440000",
///         "user_id": "550e8400-e29b-41d4-a716-446655440001",
///         "session_id": "550e8400-e29b-41d4-a716-446655440003",
///         "login_time": "2024-01-15T10:30:00Z",
///         "logout_time": "2024-01-15T18:45:00Z",
///         "duration_minutes": 495,
///         "device_fingerprint": "fp123456789",
///         "device_type": "web",
///         "ip_address": "192.168.1.100",
///         "country": "US",
///         "city": "New York",
///         "user_agent": "Mozilla/5.0...",
///         "login_successful": true
///       }
///     ],
///     "pagination": {
///       "page": 1,
///       "limit": 50,
///       "total": 1,
///       "total_pages": 1
///     },
///     "statistics": {
///       "total_logins": 1,
///       "successful_logins": 1,
///       "failed_logins": 0,
///       "unique_devices": 1,
///       "unique_countries": 1,
///       "average_session_duration": 495,
///       "last_login": "2024-01-15T10:30:00Z"
///     }
///   }
/// }
/// ```
pub async fn get_user_access_history(
    State(state): State<Arc<UserAccountAppState>>,
    Path(user_id): Path<Uuid>,
    axum::extract::Query(params): axum::extract::Query<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Parse query parameters
    let history_request = GetAccessHistoryRequest {
        user_id,
        start_date: params.get("start_date")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc)),
        end_date: params.get("end_date")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc)),
        page: params.get("page")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u32,
        limit: params.get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(50) as u32,
        include_device_details: params.get("include_device_details")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
        include_location_details: params.get("include_location_details")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
    };

    // Call domain service
    let response = state.user_lifecycle_service
        .get_user_access_history(history_request)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user access history: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse::success(
        json!(response),
        "Access history retrieved successfully"
    )))
}

// ==================== GDPR COMPLIANCE ENDPOINTS ====================

/// Create data subject request
///
/// # Endpoint
/// `POST /api/v1/users/{user_id}/data-subject-request`
///
/// # Description
/// Creates a GDPR data subject request (data export, deletion, or restriction).
///
/// # Request Headers
/// - `Authorization`: Bearer token (required)
/// - `X-Request-ID`: Request tracking ID (optional)
///
/// # Request Body
/// ```json
/// {
///   "request_type": "data_export",
///   "reason": "Customer data request",
///   "requested_data_types": ["profile", "sessions", "audit_log"],
///   "format_preference": "json",
///   "delivery_method": "email"
/// }
/// ```
///
/// # Responses
/// - `200 OK`: Data subject request created successfully
/// - `400 Bad Request`: Invalid input
/// - `401 Unauthorized`: Authentication required
/// - `404 Not Found`: User not found
/// - `500 Internal Server Error`: Server error
///
/// # Response Body
/// ```json
/// {
///   "success": true,
///   "data": {
///     "request_id": "550e8400-e29b-41d4-a716-446655440000",
///     "user_id": "550e8400-e29b-41d4-a716-446655440001",
///     "request_type": "data_export",
///     "status": "pending",
///     "priority": "normal",
///     "created_at": "2024-01-15T10:30:00Z",
///     "expected_completion_date": "2024-01-22T10:30:00Z",
///     "message": "Data subject request created successfully"
///   }
/// }
/// ```
pub async fn create_data_subject_request(
    State(state): State<Arc<UserAccountAppState>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Parse request body
    let data_request: DataSubjectRequestRequest = serde_json::from_value(request)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Call domain service
    let response = state.user_lifecycle_service
        .create_data_subject_request(data_request, user_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create data subject request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse::success(
        json!(response),
        "Data subject request created successfully"
    )))
}

/// Get data subject request status
///
/// # Endpoint
/// `GET /api/v1/data-subject-requests/{request_id}`
///
/// # Description
/// Retrieves the status of a GDPR data subject request.
///
/// # Request Headers
/// - `Authorization`: Bearer token (required)
/// - `X-Request-ID`: Request tracking ID (optional)
///
/// # Responses
/// - `200 OK`: Request status retrieved successfully
/// - `401 Unauthorized`: Authentication required
/// - `404 Not Found`: Request not found
/// - `500 Internal Server Error`: Server error
///
/// # Response Body
/// ```json
/// {
///   "success": true,
///   "data": {
///     "request_id": "550e8400-e29b-41d4-a716-446655440000",
///     "user_id": "550e8400-e29b-41d4-a716-446655440001",
///     "request_type": "data_export",
///     "status": "completed",
///     "priority": "normal",
///     "created_at": "2024-01-15T10:30:00Z",
///     "updated_at": "2024-01-15T14:30:00Z",
///     "completed_at": "2024-01-15T14:30:00Z",
///     "processed_by_user_id": "550e8400-e29b-41d4-a716-446655440002",
///     "download_url": "https://example.com/exports/data_export_12345.zip",
///     "expires_at": "2024-01-22T10:30:00Z"
///   }
/// }
/// ```
pub async fn get_data_subject_request_status(
    State(state): State<Arc<UserAccountAppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Call domain service
    let response = state.user_lifecycle_service
        .get_data_subject_request_status(request_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get data subject request status: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse::success(
        json!(response),
        "Request status retrieved successfully"
    )))
}

/// Download data export
///
/// # Endpoint
/// `GET /api/v1/data-subject-requests/{request_id}/download`
///
/// # Description
/// Downloads a completed data export file.
///
/// # Request Headers
/// - `Authorization`: Bearer token (required)
/// - `X-Request-ID`: Request tracking ID (optional)
///
/// # Responses
/// - `200 OK`: File downloaded successfully
/// - `401 Unauthorized`: Authentication required
/// - `404 Not Found`: Request not found or export not available
/// - `410 Gone`: Export has expired
/// - `500 Internal Server Error`: Server error
///
/// # Response Body
/// The exported data file (JSON, CSV, or ZIP format)
pub async fn download_data_export(
    State(state): State<Arc<UserAccountAppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<axum::response::Response, StatusCode> {
    // Call domain service
    let export_data = state.user_lifecycle_service
        .download_data_export(request_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to download data export: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Return file response
    Ok(axum::response::Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", export_data.content_type)
        .header("Content-Disposition", format!("attachment; filename=\"{}\"", export_data.filename))
        .body(axum::body::Body::from(export_data.data))
        .unwrap())
}

// ==================== HELPER FUNCTIONS ====================

/// Extract admin user ID from headers
fn extract_admin_user_id(headers: &HeaderMap) -> Result<Uuid, StatusCode> {
    headers
        .get("X-Admin-User-ID")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.parse().ok())
        .ok_or(StatusCode::UNAUTHORIZED)
}