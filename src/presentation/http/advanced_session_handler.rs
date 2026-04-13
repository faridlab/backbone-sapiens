//! HTTP Handlers for Advanced Session Management endpoints
//!
//! Implements the 8 advanced session management endpoints for Phase 2.2

use crate::domain::services::advanced_session_service::{
    AdvancedSessionService, CreateSessionRequest, ValidateSessionRequest,
    ListSessionsRequest, TerminateSessionRequest, ExtendSessionRequest,
    SessionHistoryRequest, CreateSessionResponse, ValidateSessionResponse,
    ListSessionsResponse, TerminateSessionResponse, ExtendSessionResponse,
    SessionHistoryResponse, DeviceInfoResponse, GeographicLocation,
    DeviceType, SessionStatus
};
use crate::domain::services::{AuthenticationService, SecurityMonitoringService};
use axum::{
    extract::{Path, Query, State, Request},
    http::{StatusCode, HeaderMap},
    response::Json,
    middleware,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

// Application state for advanced session management
pub struct AdvancedSessionAppState {
    pub session_service: Arc<dyn AdvancedSessionService>,
    pub auth_service: Arc<dyn AuthenticationService>,
    pub security_service: Arc<dyn SecurityMonitoringService>,
}

// Common response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: String, // For tracing
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
            request_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
            request_id: Uuid::new_v4().to_string(),
        }
    }
}

// Query parameters for session listing
#[derive(Debug, Deserialize)]
pub struct ListSessionsQuery {
    pub include_terminated: Option<bool>,
    pub device_type: Option<DeviceType>,
    pub status: Option<SessionStatus>,
    pub from_date: Option<String>, // ISO 8601 date string
    pub to_date: Option<String>,     // ISO 8601 date string
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

// Query parameters for session history
#[derive(Debug, Deserialize)]
pub struct SessionHistoryQuery {
    pub days: Option<u32>,
    pub include_security_events: Option<bool>,
    pub device_id: Option<Uuid>,
}

// Request body for extending session
#[derive(Debug, Deserialize)]
pub struct ExtendSessionBody {
    pub extend_minutes: Option<u32>,
    pub reason: Option<String>,
}

// Request body for terminating session
#[derive(Debug, Deserialize)]
pub struct TerminateSessionBody {
    pub reason: String,
    pub notify_user: Option<bool>,
    pub force_terminate: Option<bool>,
}

// Request body for terminating all sessions
#[derive(Debug, Deserialize)]
pub struct TerminateAllSessionsBody {
    pub reason: String,
    pub notify_users: Option<bool>,
}

// Request body for session analytics
#[derive(Debug, Deserialize)]
pub struct SessionAnalyticsBody {
    pub days: Option<u32>,
    pub include_device_breakdown: Option<bool>,
    pub include_geographic_breakdown: Option<bool>,
}

// Middleware to extract user ID and session information
async fn extract_session_context(
    headers: HeaderMap,
    auth_service: Arc<dyn AuthenticationService>,
) -> Result<(Uuid, Option<Uuid>), StatusCode> {
    // Extract Authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // In a real implementation, this would validate JWT token and extract user ID
    // For now, return mock data
    let user_id = Uuid::new_v4();
    let session_id = Some(Uuid::new_v4()); // Extract from token in real implementation

    Ok((user_id, session_id))
}

// ===== Session Endpoint 1: Create Session (Advanced Login) =====
pub async fn create_session(
    State(state): State<Arc<AdvancedSessionAppState>>,
    headers: HeaderMap,
    Json(mut request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<CreateSessionResponse>>, StatusCode> {
    // Extract user ID from auth token (during login, this might be pre-auth)
    let (user_id, _) = extract_session_context(headers, state.auth_service.clone()).await?;

    // Parse request fields
    let ip_address = request.get("ip_address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let user_agent = request.get("user_agent")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let login_method = request.get("login_method")
        .and_then(|v| v.as_str())
        .unwrap_or("password");

    let device_fingerprint = request.get("device_fingerprint")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let remember_me = request.get("remember_me")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // Parse geographic location if provided
    let geographic_location = request.get("geographic_location")
        .and_then(|v| serde_json::from_value::<GeographicLocation>(v.clone()).ok());

    // Create session request
    let session_request = CreateSessionRequest {
        user_id,
        ip_address: ip_address.to_string(),
        user_agent: user_agent.to_string(),
        login_method: login_method.to_string(),
        device_fingerprint: device_fingerprint.to_string(),
        geographic_location,
        remember_me,
    };

    // Call session service
    match state.session_service.create_session(session_request).await {
        Ok(response) => {
            // Log successful session creation
            tracing::info!("Session created successfully for user: {}", user_id);
            Ok(Json(ApiResponse::success(response, "Session created successfully")))
        }
        Err(e) => {
            tracing::error!("Failed to create session: {}", e);
            Ok(Json(ApiResponse::error("Failed to create session")))
        }
    }
}

// ===== Session Endpoint 2: Validate Session =====
pub async fn validate_session(
    State(state): State<Arc<AdvancedSessionAppState>>,
    Json(mut request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<ValidateSessionResponse>>, StatusCode> {
    // Extract session token from request
    let session_token = request.get("session_token")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let ip_address = request.get("ip_address")
        .and_then(|v| v.as_str());

    let user_agent = request.get("user_agent")
        .and_then(|v| v.as_str());

    // Create validation request
    let validation_request = ValidateSessionRequest {
        session_token: session_token.to_string(),
        ip_address: ip_address.map(|s| s.to_string()),
        user_agent: user_agent.map(|s| s.to_string()),
    };

    // Call session service
    match state.session_service.validate_session(validation_request).await {
        Ok(response) => {
            if response.valid {
                Ok(Json(ApiResponse::success(response, "Session is valid")))
            } else {
                Ok(Json(ApiResponse::success(response, "Session is invalid or expired")))
            }
        }
        Err(e) => {
            tracing::error!("Failed to validate session: {}", e);
            Ok(Json(ApiResponse::error("Failed to validate session")))
        }
    }
}

// ===== Session Endpoint 3: List User Sessions =====
pub async fn list_sessions(
    State(state): State<Arc<AdvancedSessionAppState>>,
    headers: HeaderMap,
    Query(query): Query<ListSessionsQuery>,
) -> Result<Json<ApiResponse<ListSessionsResponse>>, StatusCode> {
    // Extract user ID from auth token
    let (user_id, _) = extract_session_context(headers, state.auth_service.clone()).await?;

    // Parse date filters
    let from_date = query.from_date
        .and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));

    let to_date = query.to_date
        .and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));

    // Create list request
    let list_request = ListSessionsRequest {
        user_id,
        include_terminated: query.include_terminated.unwrap_or(false),
        device_type: query.device_type,
        status: query.status,
        from_date,
        to_date,
        page: query.page.unwrap_or(1),
        limit: query.limit.unwrap_or(20).min(100), // Max 100 per page
    };

    // Call session service
    match state.session_service.list_sessions(list_request).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, "Sessions retrieved successfully"))),
        Err(e) => {
            tracing::error!("Failed to list sessions: {}", e);
            Ok(Json(ApiResponse::error("Failed to list sessions")))
        }
    }
}

// ===== Session Endpoint 4: Terminate Specific Session =====
pub async fn terminate_session(
    State(state): State<Arc<AdvancedSessionAppState>>,
    headers: HeaderMap,
    Path(session_id): Path<Uuid>,
    Json(body): Json<TerminateSessionBody>,
) -> Result<Json<ApiResponse<TerminateSessionResponse>>, StatusCode> {
    // Extract admin user ID from auth token
    let (admin_user_id, _) = extract_session_context(headers, state.auth_service.clone()).await?;

    // Create termination request
    let termination_request = TerminateSessionRequest {
        session_id,
        reason: body.reason,
        notify_user: body.notify_user.unwrap_or(true),
        force_terminate: body.force_terminate.unwrap_or(false),
    };

    // Call session service
    match state.session_service.terminate_session(termination_request, admin_user_id).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, "Session terminated successfully"))),
        Err(e) => {
            tracing::error!("Failed to terminate session: {}", e);
            Ok(Json(ApiResponse::error("Failed to terminate session")))
        }
    }
}

// ===== Session Endpoint 5: Terminate Current Session =====
pub async fn terminate_current_session(
    State(state): State<Arc<AdvancedSessionAppState>>,
    headers: HeaderMap,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<TerminateSessionResponse>>, StatusCode> {
    // Extract session context
    let (user_id, session_id) = extract_session_context(headers, state.auth_service.clone()).await?;

    let reason = body.get("reason")
        .and_then(|v| v.as_str())
        .unwrap_or("User requested logout");

    let notify_user = body.get("notify_user")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // Create termination request
    let termination_request = TerminateSessionRequest {
        session_id: session_id.ok_or(StatusCode::BAD_REQUEST)?,
        reason: reason.to_string(),
        notify_user,
        force_terminate: true,
    };

    // Call session service
    match state.session_service.terminate_session(termination_request, user_id).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, "Current session terminated successfully"))),
        Err(e) => {
            tracing::error!("Failed to terminate current session: {}", e);
            Ok(Json(ApiResponse::error("Failed to terminate current session")))
        }
    }
}

// ===== Session Endpoint 6: Extend Session =====
pub async fn extend_session(
    State(state): State<Arc<AdvancedSessionAppState>>,
    Json(body): Json<ExtendSessionBody>,
) -> Result<Json<ApiResponse<ExtendSessionResponse>>, StatusCode> {
    // Extract session token from request
    let session_token = body.extend_minutes
        .as_ref()
        .map(|_| "dummy_session_token") // In real implementation, extract from headers
        .unwrap_or("dummy_session_token");

    // Create extension request
    let extension_request = ExtendSessionRequest {
        session_token: session_token.to_string(),
        extend_minutes: body.extend_minutes,
        reason: body.reason,
    };

    // Call session service
    match state.session_service.extend_session(extension_request).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, "Session extended successfully"))),
        Err(e) => {
            tracing::error!("Failed to extend session: {}", e);
            Ok(Json(ApiResponse::error("Failed to extend session")))
        }
    }
}

// ===== Session Endpoint 7: Get Session History =====
pub async fn get_session_history(
    State(state): State<Arc<AdvancedSessionAppState>>,
    headers: HeaderMap,
    Query(query): Query<SessionHistoryQuery>,
) -> Result<Json<ApiResponse<SessionHistoryResponse>>, StatusCode> {
    // Extract user ID from auth token
    let (user_id, _) = extract_session_context(headers, state.auth_service.clone()).await?;

    // Create history request
    let history_request = SessionHistoryRequest {
        user_id,
        days: query.days.unwrap_or(30).min(365), // Max 365 days
        include_security_events: query.include_security_events.unwrap_or(true),
        device_id: query.device_id,
    };

    // Call session service
    match state.session_service.get_session_history(history_request).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, "Session history retrieved successfully"))),
        Err(e) => {
            tracing::error!("Failed to get session history: {}", e);
            Ok(Json(ApiResponse::error("Failed to get session history")))
        }
    }
}

// ===== Session Endpoint 8: Get Device Information =====
pub async fn get_device_info(
    State(state): State<Arc<AdvancedSessionAppState>>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<DeviceInfoResponse>>, StatusCode> {
    // Extract session context
    let (_, session_id) = extract_session_context(headers, state.auth_service.clone()).await?;

    // Get device info for current session
    match state.session_service.get_device_info(session_id.ok_or(StatusCode::BAD_REQUEST)?).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, "Device information retrieved successfully"))),
        Err(e) => {
            tracing::error!("Failed to get device info: {}", e);
            Ok(Json(ApiResponse::error("Failed to get device information")))
        }
    }
}

// ===== Session Endpoint 9: Terminate All Sessions (Additional Endpoint) =====
pub async fn terminate_all_sessions(
    State(state): State<Arc<AdvancedSessionAppState>>,
    headers: HeaderMap,
    Json(body): Json<TerminateAllSessionsBody>,
) -> Result<Json<ApiResponse<TerminateSessionResponse>>, StatusCode> {
    // Extract admin user ID from auth token
    let (admin_user_id, _) = extract_session_context(headers, state.auth_service.clone()).await?;

    // Extract target user ID from request or use admin user ID
    let target_user_id = body.get("user_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .unwrap_or(admin_user_id);

    // Call session service
    match state.session_service.terminate_all_sessions(target_user_id, &body.reason, admin_user_id).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, "All sessions terminated successfully"))),
        Err(e) => {
            tracing::error!("Failed to terminate all sessions: {}", e);
            Ok(Json(ApiResponse::error("Failed to terminate all sessions")))
        }
    }
}

// ===== Session Endpoint 10: Get Session Analytics =====
pub async fn get_session_analytics(
    State(state): State<Arc<AdvancedSessionAppState>>,
    headers: HeaderMap,
    Json(body): Json<SessionAnalyticsBody>,
) -> Result<Json<ApiResponse<crate::domain::services::advanced_session_service::SessionAnalytics>>, StatusCode> {
    // Extract user ID from auth token
    let (user_id, _) = extract_session_context(headers, state.auth_service.clone()).await?;

    // Call session service
    match state.session_service.get_session_analytics(user_id, body.days).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, "Session analytics retrieved successfully"))),
        Err(e) => {
            tracing::error!("Failed to get session analytics: {}", e);
            Ok(Json(ApiResponse::error("Failed to get session analytics")))
        }
    }
}

// Utility function to create advanced session router
pub fn create_advanced_session_router(state: Arc<AdvancedSessionAppState>) -> axum::Router {
    axum::Router::new()
        .route("/api/v1/sessions", axum::routing::post(create_session))
        .route("/api/v1/sessions", axum::routing::get(list_sessions))
        .route("/api/v1/sessions/validate", axum::routing::post(validate_session))
        .route("/api/v1/sessions/:id", axum::routing::delete(terminate_session))
        .route("/api/v1/sessions/current", axum::routing::delete(terminate_current_session))
        .route("/api/v1/sessions/extend", axum::routing::post(extend_session))
        .route("/api/v1/sessions/history", axum::routing::get(get_session_history))
        .route("/api/v1/sessions/device-info", axum::routing::get(get_device_info))
        .route("/api/v1/sessions/terminate-all", axum::routing::delete(terminate_all_sessions))
        .route("/api/v1/sessions/analytics", axum::routing::post(get_session_analytics))
        .with_state(state)
        .layer(middleware::from_fn(session_logging_middleware))
}

// Logging middleware for session endpoints
async fn session_logging_middleware(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = std::time::Instant::now();

    // Extract user agent and IP for logging
    let user_agent = req.headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    let ip_address = req.headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .or_else(|| req.headers().get("x-real-ip").and_then(|h| h.to_str().ok()))
        .unwrap_or("unknown");

    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status;

    tracing::info!(
        "Session request: {} {} - Status: {} - Duration: {:?} - IP: {} - User-Agent: {}",
        method,
        uri,
        status,
        duration,
        ip_address,
        user_agent
    );

    response
}

// Implement validation for request types
impl Validate for ExtendSessionBody {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();

        if let Some(extend_minutes) = self.extend_minutes {
            if extend_minutes < 1 || extend_minutes > 1440 { // Max 24 hours
                errors.add(
                    "extend_minutes",
                    validator::ValidationError::new("Extension time must be between 1 and 1440 minutes"),
                );
            }
        }

        if let Some(reason) = &self.reason {
            if reason.len() > 500 {
                errors.add(
                    "reason",
                    validator::ValidationError::new("Reason must be 500 characters or less"),
                );
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Validate for TerminateSessionBody {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();

        if self.reason.is_empty() {
            errors.add(
                "reason",
                validator::ValidationError::new("Termination reason is required"),
            );
        }

        if self.reason.len() > 500 {
            errors.add(
                "reason",
                validator::ValidationError::new("Reason must be 500 characters or less"),
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Validate for SessionAnalyticsBody {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();

        if let Some(days) = self.days {
            if days < 1 || days > 365 {
                errors.add(
                    "days",
                    validator::ValidationError::new("Days must be between 1 and 365"),
                );
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}