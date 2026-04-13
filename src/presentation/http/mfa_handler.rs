//! HTTP Handlers for Multi-Factor Authentication (MFA) endpoints
//!
//! Implements the 8 MFA endpoints for Phase 2.1

use crate::domain::services::mfa_service::{
    MFAService, SetupMFARequest, VerifyMFARequest, GenerateBackupCodesRequest,
    ListMFADevicesRequest, RemoveMFARequest, MFAPreferenceUpdate,
    SetupMFAResponse, VerifyMFAResponse, GenerateBackupCodesResponse,
    ListMFADevicesResponse, RemoveMFAResponse
};
use crate::domain::services::{AuthenticationService, SecurityMonitoringService};
use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, HeaderMap},
    response::Json,
    middleware,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

// Application state containing MFA service
pub struct AppState {
    pub mfa_service: Arc<dyn MFAService>,
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
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
        }
    }
}

// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// Query parameters for device listing
#[derive(Debug, Deserialize)]
pub struct ListDevicesQuery {
    pub include_inactive: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

// Middleware to extract user ID from authentication token
async fn extract_user_id(
    headers: HeaderMap,
    auth_service: Arc<dyn AuthenticationService>,
) -> Result<Uuid, StatusCode> {
    // Extract Authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate token and extract user ID
    // In a real implementation, this would verify JWT token
    // For now, return a dummy UUID for demonstration
    Ok(Uuid::new_v4())
}

// ===== MFA Endpoint 1: Setup MFA Device =====
pub async fn setup_mfa_device(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<SetupMFARequest>,
) -> Result<Json<ApiResponse<SetupMFAResponse>>, StatusCode> {
    // Validate request
    if let Err(e) = request.validate() {
        return Ok(Json(ApiResponse::error(&format!("Validation error: {}", e))));
    }

    // Extract user ID from auth token
    let user_id = extract_user_id(headers, state.auth_service.clone()).await?;

    // Call MFA service
    match state.mfa_service.setup_mfa_device(request, user_id).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, "MFA device setup initiated successfully"))),
        Err(e) => {
            tracing::error!("Failed to setup MFA device: {}", e);
            Ok(Json(ApiResponse::error("Failed to setup MFA device")))
        }
    }
}

// ===== MFA Endpoint 2: Verify MFA Device =====
pub async fn verify_mfa_device(
    State(state): State<Arc<AppState>>,
    Json(request): Json<VerifyMFARequest>,
) -> Result<Json<ApiResponse<VerifyMFAResponse>>, StatusCode> {
    // Validate request
    if let Err(e) = request.validate() {
        return Ok(Json(ApiResponse::error(&format!("Validation error: {}", e))));
    }

    // Call MFA service
    match state.mfa_service.verify_mfa_device(request).await {
        Ok(response) => {
            let message = if response.success {
                "MFA device verified successfully"
            } else {
                "MFA verification failed"
            };
            Ok(Json(ApiResponse::success(response, message)))
        }
        Err(e) => {
            tracing::error!("Failed to verify MFA device: {}", e);
            Ok(Json(ApiResponse::error("Failed to verify MFA device")))
        }
    }
}

// ===== MFA Endpoint 3: Generate Backup Codes =====
pub async fn generate_backup_codes(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<GenerateBackupCodesRequest>,
) -> Result<Json<ApiResponse<GenerateBackupCodesResponse>>, StatusCode> {
    // Validate request
    if let Err(e) = request.validate() {
        return Ok(Json(ApiResponse::error(&format!("Validation error: {}", e))));
    }

    // Extract user ID from auth token
    let user_id = extract_user_id(headers, state.auth_service.clone()).await?;

    // Validate backup code count
    if request.count < 5 || request.count > 20 {
        return Ok(Json(ApiResponse::error("Number of backup codes must be between 5 and 20")));
    }

    // Call MFA service
    match state.mfa_service.generate_backup_codes(request, user_id).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, "Backup codes generated successfully"))),
        Err(e) => {
            tracing::error!("Failed to generate backup codes: {}", e);
            Ok(Json(ApiResponse::error("Failed to generate backup codes")))
        }
    }
}

// ===== MFA Endpoint 4: List MFA Devices =====
pub async fn list_mfa_devices(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<ListDevicesQuery>,
) -> Result<Json<ApiResponse<ListMFADevicesResponse>>, StatusCode> {
    // Extract user ID from auth token
    let user_id = extract_user_id(headers, state.auth_service.clone()).await?;

    // Create request
    let request = ListMFADevicesRequest {
        user_id,
        include_inactive: query.include_inactive.unwrap_or(false),
    };

    // Call MFA service
    match state.mfa_service.list_mfa_devices(request).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, "MFA devices retrieved successfully"))),
        Err(e) => {
            tracing::error!("Failed to list MFA devices: {}", e);
            Ok(Json(ApiResponse::error("Failed to list MFA devices")))
        }
    }
}

// ===== MFA Endpoint 5: Remove MFA Device =====
pub async fn remove_mfa_device(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(device_id): Path<Uuid>,
    Json(mut request): Json<RemoveMFARequest>,
) -> Result<Json<ApiResponse<RemoveMFAResponse>>, StatusCode> {
    // Set device ID from path
    request.device_id = device_id;

    // Validate request
    if let Err(e) = request.validate() {
        return Ok(Json(ApiResponse::error(&format!("Validation error: {}", e))));
    }

    // Extract user ID from auth token
    let user_id = extract_user_id(headers, state.auth_service.clone()).await?;

    // Call MFA service
    match state.mfa_service.remove_mfa_device(request, user_id).await {
        Ok(response) => Ok(Json(ApiResponse::success(response, "MFA device removed successfully"))),
        Err(e) => {
            tracing::error!("Failed to remove MFA device: {}", e);
            Ok(Json(ApiResponse::error("Failed to remove MFA device")))
        }
    }
}

// ===== MFA Endpoint 6: Update MFA Preferences =====
pub async fn update_mfa_preferences(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(preferences): Json<MFAPreferenceUpdate>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Extract user ID from auth token
    let user_id = extract_user_id(headers, state.auth_service.clone()).await?;

    // Call MFA service
    match state.mfa_service.update_mfa_preferences(preferences, user_id).await {
        Ok(()) => Ok(Json(ApiResponse::success((), "MFA preferences updated successfully"))),
        Err(e) => {
            tracing::error!("Failed to update MFA preferences: {}", e);
            Ok(Json(ApiResponse::error("Failed to update MFA preferences")))
        }
    }
}

// ===== MFA Endpoint 7: Validate MFA During Login =====
pub async fn validate_mfa_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<bool>>, StatusCode> {
    // Extract required fields
    let user_id = payload.get("user_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let verification_code = payload.get("verification_code")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let session_token = payload.get("session_token")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    // Call MFA service
    match state.mfa_service.validate_mfa_verification(user_id, verification_code, session_token).await {
        Ok(success) => {
            let message = if success {
                "MFA verification successful"
            } else {
                "MFA verification failed"
            };
            Ok(Json(ApiResponse::success(success, message)))
        }
        Err(e) => {
            tracing::error!("Failed to validate MFA: {}", e);
            Ok(Json(ApiResponse::error("Failed to validate MFA")))
        }
    }
}

// ===== MFA Endpoint 8: Get MFA Status =====
pub async fn get_mfa_status(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Extract user ID from auth token
    let user_id = extract_user_id(headers, state.auth_service.clone()).await?;

    // Get MFA devices for the user
    let request = ListMFADevicesRequest {
        user_id,
        include_inactive: false,
    };

    match state.mfa_service.list_mfa_devices(request).await {
        Ok(response) => {
            let status_data = serde_json::json!({
                "mfa_enabled": response.active_count > 0,
                "total_devices": response.total_count,
                "active_devices": response.active_count,
                "devices": response.devices,
                "can_setup_totp": true,
                "can_setup_sms": true,
                "can_setup_email": true,
                "backup_codes_available": false // This would be checked from database
            });

            Ok(Json(ApiResponse::success(status_data, "MFA status retrieved successfully")))
        }
        Err(e) => {
            tracing::error!("Failed to get MFA status: {}", e);
            Ok(Json(ApiResponse::error("Failed to get MFA status")))
        }
    }
}

// Utility function to create MFA router
pub fn create_mfa_router(state: Arc<AppState>) -> axum::Router {
    axum::Router::new()
        .route("/api/v1/auth/mfa/setup", axum::routing::post(setup_mfa_device))
        .route("/api/v1/auth/mfa/verify", axum::routing::post(verify_mfa_device))
        .route("/api/v1/auth/mfa/backup-codes/generate", axum::routing::post(generate_backup_codes))
        .route("/api/v1/auth/mfa/devices", axum::routing::get(list_mfa_devices))
        .route("/api/v1/auth/mfa/devices/:device_id", axum::routing::delete(remove_mfa_device))
        .route("/api/v1/auth/mfa/preferences", axum::routing::put(update_mfa_preferences))
        .route("/api/v1/auth/mfa/validate", axum::routing::post(validate_mfa_login))
        .route("/api/v1/auth/mfa/status", axum::routing::get(get_mfa_status))
        .with_state(state)
        .layer(middleware::from_fn(logging_middleware))
}

// Logging middleware for MFA endpoints
async fn logging_middleware(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = std::time::Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed();
    tracing::info!(
        "MFA request: {} {} - Status: {} - Duration: {:?}",
        method,
        uri,
        response.status,
        duration
    );

    response
}

// Implement validation for request types
impl Validate for SetupMFARequest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();

        if self.device_name.is_empty() {
            errors.add(
                "device_name",
                validator::ValidationError::new("Device name is required"),
            );
        }

        if self.device_name.len() > 100 {
            errors.add(
                "device_name",
                validator::ValidationError::new("Device name must be 100 characters or less"),
            );
        }

        // Validate device-specific requirements
        match self.device_type {
            crate::domain::services::mfa_service::MFADeviceType::SMS => {
                if self.phone_number.is_none() || self.phone_number.as_ref().unwrap().is_empty() {
                    errors.add(
                        "phone_number",
                        validator::ValidationError::new("Phone number is required for SMS MFA"),
                    );
                }
            }
            crate::domain::services::mfa_service::MFADeviceType::Email => {
                if self.email_address.is_none() || self.email_address.as_ref().unwrap().is_empty() {
                    errors.add(
                        "email_address",
                        validator::ValidationError::new("Email address is required for email MFA"),
                    );
                }
            }
            _ => {}
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Validate for VerifyMFARequest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();

        if self.verification_code.is_empty() {
            errors.add(
                "verification_code",
                validator::ValidationError::new("Verification code is required"),
            );
        }

        if self.verification_code.len() < 4 || self.verification_code.len() > 10 {
            errors.add(
                "verification_code",
                validator::ValidationError::new("Verification code must be between 4 and 10 characters"),
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Validate for GenerateBackupCodesRequest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();

        if self.count < 5 || self.count > 20 {
            errors.add(
                "count",
                validator::ValidationError::new("Number of backup codes must be between 5 and 20"),
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Validate for RemoveMFARequest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();

        if self.confirmation_code.is_empty() {
            errors.add(
                "confirmation_code",
                validator::ValidationError::new("Confirmation code is required"),
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}