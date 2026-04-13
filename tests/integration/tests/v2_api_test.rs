//! V2.0 API Integration Tests
//!
//! Comprehensive tests for v2.0 feature API endpoints:
//! - Temporary Permission API
//! - Impersonation Session API
//! - Security Event API
//! - Data Export API
//! - Anonymization Record API
//! - Resource Permission API

use async_trait::async_trait;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Instant;
use uuid::Uuid;

use crate::integration::framework::{ApiTest, Test, TestError, TestResult};
use crate::integration::helpers::{CommonUtils, TestSetupManager};

// ============================================================
// Temporary Permission API DTOs
// ============================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTemporaryPermissionRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub permission_id: Uuid,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub granted_by: Uuid,
    pub granted_at: String,
    pub expires_at: String,
    pub reason: String,
    pub status: String,
    pub metadata: Value,
}

impl CreateTemporaryPermissionRequest {
    pub fn new(
        user_id: Uuid,
        permission_id: Uuid,
        granted_by: Uuid,
        duration_days: i64,
        reason: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            permission_id,
            resource_type: None,
            resource_id: None,
            granted_by,
            granted_at: now.to_rfc3339(),
            expires_at: (now + Duration::days(duration_days)).to_rfc3339(),
            reason,
            status: "active".to_string(),
            metadata: json!({
                "created_at": now.to_rfc3339(),
                "updated_at": now.to_rfc3339()
            }),
        }
    }
}

// ============================================================
// Impersonation Session API DTOs
// ============================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateImpersonationSessionRequest {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub target_user_id: Uuid,
    pub started_at: String,
    pub expires_at: String,
    pub reason: String,
    pub ip_address: String,
    pub user_agent: String,
    pub status: String,
    pub metadata: Value,
}

impl CreateImpersonationSessionRequest {
    pub fn new(
        admin_id: Uuid,
        target_user_id: Uuid,
        duration_minutes: i64,
        reason: String,
        ip_address: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            admin_id,
            target_user_id,
            started_at: now.to_rfc3339(),
            expires_at: (now + Duration::minutes(duration_minutes)).to_rfc3339(),
            reason,
            ip_address,
            user_agent: "TestSuite/1.0".to_string(),
            status: "active".to_string(),
            metadata: json!({
                "created_at": now.to_rfc3339(),
                "updated_at": now.to_rfc3339()
            }),
        }
    }
}

// ============================================================
// Security Event API DTOs
// ============================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSecurityEventRequest {
    pub id: Uuid,
    pub event_type: String,
    pub severity: String,
    pub description: String,
    pub user_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub occurred_at: String,
    pub resolved: bool,
    pub metadata: Value,
}

impl CreateSecurityEventRequest {
    pub fn new(
        event_type: String,
        severity: String,
        description: String,
        user_id: Option<Uuid>,
        ip_address: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            event_type,
            severity,
            description,
            user_id,
            ip_address,
            user_agent: Some("TestSuite/1.0".to_string()),
            occurred_at: now.to_rfc3339(),
            resolved: false,
            metadata: json!({
                "created_at": now.to_rfc3339(),
                "updated_at": now.to_rfc3339()
            }),
        }
    }
}

// ============================================================
// Data Export API DTOs
// ============================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDataExportRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub requested_by: Uuid,
    pub status: String,
    pub export_format: String,
    pub included_entities: Vec<String>,
    pub requested_at: String,
    pub expires_at: String,
    pub filters: Value,
    pub metadata: Value,
}

impl CreateDataExportRequest {
    pub fn new(
        user_id: Uuid,
        requested_by: Uuid,
        export_format: String,
        included_entities: Vec<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            requested_by,
            status: "pending".to_string(),
            export_format,
            included_entities,
            requested_at: now.to_rfc3339(),
            expires_at: (now + Duration::days(7)).to_rfc3339(),
            filters: json!({}),
            metadata: json!({
                "created_at": now.to_rfc3339(),
                "updated_at": now.to_rfc3339()
            }),
        }
    }
}

// ============================================================
// Anonymization Record API DTOs
// ============================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnonymizationRecordRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub fields_anonymized: Vec<String>,
    pub anonymization_method: String,
    pub performed_by: Uuid,
    pub performed_at: String,
    pub reason: String,
    pub retention_years: i32,
    pub metadata: Value,
}

impl CreateAnonymizationRecordRequest {
    pub fn new(
        user_id: Uuid,
        entity_type: String,
        entity_id: Uuid,
        fields_anonymized: Vec<String>,
        anonymization_method: String,
        performed_by: Uuid,
        reason: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            entity_type,
            entity_id,
            fields_anonymized,
            anonymization_method,
            performed_by,
            performed_at: now.to_rfc3339(),
            reason,
            retention_years: 7,
            metadata: json!({
                "created_at": now.to_rfc3339(),
                "updated_at": now.to_rfc3339()
            }),
        }
    }
}

// ============================================================
// Resource Permission API DTOs
// ============================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResourcePermissionRequest {
    pub id: Uuid,
    pub resource_type: String,
    pub resource_id: String,
    pub user_id: Option<Uuid>,
    pub role_id: Option<Uuid>,
    pub permission: String,
    pub granted_by: Uuid,
    pub granted_at: String,
    pub expires_at: Option<String>,
    pub conditions: Value,
    pub metadata: Value,
}

impl CreateResourcePermissionRequest {
    pub fn new(
        resource_type: String,
        resource_id: String,
        user_id: Option<Uuid>,
        role_id: Option<Uuid>,
        permission: String,
        granted_by: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            resource_type,
            resource_id,
            user_id,
            role_id,
            permission,
            granted_by,
            granted_at: now.to_rfc3339(),
            expires_at: None,
            conditions: json!({}),
            metadata: json!({
                "created_at": now.to_rfc3339(),
                "updated_at": now.to_rfc3339()
            }),
        }
    }
}

// ============================================================
// V2.0 API Test Implementation
// ============================================================

/// Integration test for v2.0 feature API endpoints
pub struct V2ApiTest {
    /// Test name
    name: String,

    /// API test client
    api: ApiTest,

    /// Setup manager
    setup_manager: TestSetupManager,

    /// Common utilities
    common_utils: CommonUtils,

    /// Test IDs
    test_user_id: Option<Uuid>,
    test_admin_id: Option<Uuid>,
    test_permission_id: Option<Uuid>,
    test_auth_token: Option<String>,

    /// Created resource IDs for cleanup
    created_temp_permission_ids: Vec<Uuid>,
    created_impersonation_ids: Vec<Uuid>,
    created_security_event_ids: Vec<Uuid>,
    created_data_export_ids: Vec<Uuid>,
    created_anonymization_ids: Vec<Uuid>,
    created_resource_permission_ids: Vec<Uuid>,
}

impl V2ApiTest {
    /// Create a new V2 API test instance
    pub fn new() -> Self {
        let setup_manager = TestSetupManager::new();
        let api_url = setup_manager.config.api_base_url.clone();

        Self {
            name: "v2_api_test".to_string(),
            api: ApiTest::new("v2_api", &api_url),
            setup_manager,
            common_utils: CommonUtils::default(),
            test_user_id: None,
            test_admin_id: None,
            test_permission_id: None,
            test_auth_token: None,
            created_temp_permission_ids: Vec::new(),
            created_impersonation_ids: Vec::new(),
            created_security_event_ids: Vec::new(),
            created_data_export_ids: Vec::new(),
            created_anonymization_ids: Vec::new(),
            created_resource_permission_ids: Vec::new(),
        }
    }

    // ============================================================
    // Temporary Permission API Tests
    // ============================================================

    /// Test: POST /temporary-permissions - Create temporary permission
    async fn test_create_temporary_permission_success(&mut self) -> TestResult {
        let start = Instant::now();
        let test_name = "Create Temporary Permission - Success";

        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let granted_by = self.test_admin_id.unwrap_or(Uuid::new_v4());

        let request = CreateTemporaryPermissionRequest::new(
            user_id,
            permission_id,
            granted_by,
            7,
            "Project access for sprint".to_string(),
        );

        let input = serde_json::to_value(&request).unwrap_or(json!({}));

        let response = match self
            .api
            .post("/api/v1/temporary-permissions", &request, None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64())
                    .with_input(input);
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        if response.status_code == 201 || response.status_code == 200 {
            // Track for cleanup
            if let Ok(body) = serde_json::from_str::<Value>(&response.body) {
                if let Some(id) = body.get("id").and_then(|v| v.as_str()) {
                    if let Ok(uuid) = Uuid::parse_str(id) {
                        self.created_temp_permission_ids.push(uuid);
                    }
                }
            }

            TestResult::success(test_name, "Temporary permission created successfully")
                .with_duration(start.elapsed().as_secs_f64())
                .with_input(input)
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 201/200, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_input(input)
            .with_output(output)
        }
    }

    /// Test: POST /temporary-permissions - Validate duration limit
    async fn test_create_temporary_permission_exceeds_max_duration(&self) -> TestResult {
        let start = Instant::now();
        let test_name = "Create Temporary Permission - Exceeds Max Duration";

        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let granted_by = self.test_admin_id.unwrap_or(Uuid::new_v4());

        let request = CreateTemporaryPermissionRequest::new(
            user_id,
            permission_id,
            granted_by,
            31, // Exceeds 30-day limit
            "Long term access".to_string(),
        );

        let input = serde_json::to_value(&request).unwrap_or(json!({}));

        let response = match self
            .api
            .post("/api/v1/temporary-permissions", &request, None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64())
                    .with_input(input);
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        // Expect rejection (400 or 422)
        if response.status_code == 400 || response.status_code == 422 {
            TestResult::success(test_name, "Correctly rejected excessive duration")
                .with_duration(start.elapsed().as_secs_f64())
                .with_input(input)
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 400/422, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_input(input)
            .with_output(output)
        }
    }

    /// Test: GET /temporary-permissions/:id - Get temporary permission
    async fn test_get_temporary_permission(&mut self) -> TestResult {
        let start = Instant::now();
        let test_name = "Get Temporary Permission - Success";

        // First create a permission
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let granted_by = self.test_admin_id.unwrap_or(Uuid::new_v4());

        let create_request = CreateTemporaryPermissionRequest::new(
            user_id,
            permission_id,
            granted_by,
            7,
            "Test access".to_string(),
        );

        let create_response = match self
            .api
            .post("/api/v1/temporary-permissions", &create_request, None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Failed to create: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        if !create_response.is_success() {
            return TestResult::failure(
                test_name,
                format!("Failed to create: status {}", create_response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64());
        }

        let created: Value = match serde_json::from_str(&create_response.body) {
            Ok(v) => v,
            Err(e) => {
                return TestResult::failure(test_name, format!("Failed to parse: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        let temp_perm_id = created
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Now get the permission
        let response = match self
            .api
            .get(&format!("/api/v1/temporary-permissions/{}", temp_perm_id), None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Get request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        if response.status_code == 200 {
            TestResult::success(test_name, "Retrieved temporary permission")
                .with_duration(start.elapsed().as_secs_f64())
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 200, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_output(output)
        }
    }

    // ============================================================
    // Impersonation Session API Tests
    // ============================================================

    /// Test: POST /impersonation-sessions - Start impersonation
    async fn test_start_impersonation_success(&mut self) -> TestResult {
        let start = Instant::now();
        let test_name = "Start Impersonation - Success";

        let admin_id = self.test_admin_id.unwrap_or(Uuid::new_v4());
        let target_user_id = self.test_user_id.unwrap_or(Uuid::new_v4());

        // Ensure different IDs
        let target_user_id = if target_user_id == admin_id {
            Uuid::new_v4()
        } else {
            target_user_id
        };

        let request = CreateImpersonationSessionRequest::new(
            admin_id,
            target_user_id,
            60,
            "Support troubleshooting".to_string(),
            "127.0.0.1".to_string(),
        );

        let input = serde_json::to_value(&request).unwrap_or(json!({}));

        let response = match self
            .api
            .post("/api/v1/impersonation-sessions", &request, None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64())
                    .with_input(input);
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        if response.status_code == 201 || response.status_code == 200 {
            // Track for cleanup
            if let Ok(body) = serde_json::from_str::<Value>(&response.body) {
                if let Some(id) = body.get("id").and_then(|v| v.as_str()) {
                    if let Ok(uuid) = Uuid::parse_str(id) {
                        self.created_impersonation_ids.push(uuid);
                    }
                }
            }

            TestResult::success(test_name, "Impersonation session started successfully")
                .with_duration(start.elapsed().as_secs_f64())
                .with_input(input)
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 201/200, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_input(input)
            .with_output(output)
        }
    }

    /// Test: POST /impersonation-sessions - Prevent self-impersonation
    async fn test_prevent_self_impersonation(&self) -> TestResult {
        let start = Instant::now();
        let test_name = "Prevent Self-Impersonation";

        let admin_id = self.test_admin_id.unwrap_or(Uuid::new_v4());

        let request = CreateImpersonationSessionRequest::new(
            admin_id,
            admin_id, // Same ID - should fail
            60,
            "Trying to impersonate self".to_string(),
            "127.0.0.1".to_string(),
        );

        let input = serde_json::to_value(&request).unwrap_or(json!({}));

        let response = match self
            .api
            .post("/api/v1/impersonation-sessions", &request, None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64())
                    .with_input(input);
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        // Expect rejection (400 or 422)
        if response.status_code == 400 || response.status_code == 422 {
            TestResult::success(test_name, "Correctly prevented self-impersonation")
                .with_duration(start.elapsed().as_secs_f64())
                .with_input(input)
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 400/422, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_input(input)
            .with_output(output)
        }
    }

    /// Test: POST /impersonation-sessions/:id/end - End impersonation
    async fn test_end_impersonation_session(&mut self) -> TestResult {
        let start = Instant::now();
        let test_name = "End Impersonation Session - Success";

        // First create a session
        let admin_id = self.test_admin_id.unwrap_or(Uuid::new_v4());
        let target_user_id = Uuid::new_v4();

        let create_request = CreateImpersonationSessionRequest::new(
            admin_id,
            target_user_id,
            60,
            "Test session".to_string(),
            "127.0.0.1".to_string(),
        );

        let create_response = match self
            .api
            .post("/api/v1/impersonation-sessions", &create_request, None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Failed to create: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        if !create_response.is_success() {
            return TestResult::failure(
                test_name,
                format!("Failed to create: status {}", create_response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64());
        }

        let created: Value = match serde_json::from_str(&create_response.body) {
            Ok(v) => v,
            Err(e) => {
                return TestResult::failure(test_name, format!("Failed to parse: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        let session_id = created
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // End the session
        let response = match self
            .api
            .post(
                &format!("/api/v1/impersonation-sessions/{}/end", session_id),
                &json!({}),
                None,
            )
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("End request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        if response.status_code == 200 || response.status_code == 202 {
            TestResult::success(test_name, "Impersonation session ended successfully")
                .with_duration(start.elapsed().as_secs_f64())
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 200/202, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_output(output)
        }
    }

    // ============================================================
    // Security Event API Tests
    // ============================================================

    /// Test: POST /security-events - Create security event
    async fn test_create_security_event_success(&mut self) -> TestResult {
        let start = Instant::now();
        let test_name = "Create Security Event - Success";

        let user_id = Some(self.test_user_id.unwrap_or(Uuid::new_v4()));

        let request = CreateSecurityEventRequest::new(
            "failed_login".to_string(),
            "medium".to_string(),
            "Multiple failed login attempts detected".to_string(),
            user_id,
            Some("192.168.1.100".to_string()),
        );

        let input = serde_json::to_value(&request).unwrap_or(json!({}));

        let response = match self
            .api
            .post("/api/v1/security-events", &request, None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64())
                    .with_input(input);
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        if response.status_code == 201 || response.status_code == 200 {
            // Track for cleanup
            if let Ok(body) = serde_json::from_str::<Value>(&response.body) {
                if let Some(id) = body.get("id").and_then(|v| v.as_str()) {
                    if let Ok(uuid) = Uuid::parse_str(id) {
                        self.created_security_event_ids.push(uuid);
                    }
                }
            }

            TestResult::success(test_name, "Security event created successfully")
                .with_duration(start.elapsed().as_secs_f64())
                .with_input(input)
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 201/200, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_input(input)
            .with_output(output)
        }
    }

    /// Test: POST /security-events/:id/resolve - Resolve security event
    async fn test_resolve_security_event(&mut self) -> TestResult {
        let start = Instant::now();
        let test_name = "Resolve Security Event - Success";

        // First create an event
        let request = CreateSecurityEventRequest::new(
            "brute_force_attack".to_string(),
            "high".to_string(),
            "Brute force attack detected".to_string(),
            Some(Uuid::new_v4()),
            Some("10.0.0.50".to_string()),
        );

        let create_response = match self
            .api
            .post("/api/v1/security-events", &request, None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Failed to create: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        if !create_response.is_success() {
            return TestResult::failure(
                test_name,
                format!("Failed to create: status {}", create_response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64());
        }

        let created: Value = match serde_json::from_str(&create_response.body) {
            Ok(v) => v,
            Err(e) => {
                return TestResult::failure(test_name, format!("Failed to parse: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        let event_id = created.get("id").and_then(|v| v.as_str()).unwrap_or("");

        // Resolve the event
        let resolve_request = json!({
            "resolved_by": self.test_admin_id.unwrap_or(Uuid::new_v4()),
            "resolution_notes": "False positive - legitimate user activity"
        });

        let response = match self
            .api
            .post(
                &format!("/api/v1/security-events/{}/resolve", event_id),
                &resolve_request,
                None,
            )
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Resolve request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        if response.status_code == 200 {
            TestResult::success(test_name, "Security event resolved successfully")
                .with_duration(start.elapsed().as_secs_f64())
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 200, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_output(output)
        }
    }

    /// Test: GET /security-events - List with severity filter
    async fn test_list_security_events_by_severity(&self) -> TestResult {
        let start = Instant::now();
        let test_name = "List Security Events - By Severity";

        let response = match self
            .api
            .get("/api/v1/security-events?severity=high&limit=10", None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        if response.status_code == 200 {
            TestResult::success(test_name, "Retrieved security events by severity")
                .with_duration(start.elapsed().as_secs_f64())
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 200, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_output(output)
        }
    }

    // ============================================================
    // Data Export API Tests
    // ============================================================

    /// Test: POST /data-exports - Request data export
    async fn test_request_data_export_success(&mut self) -> TestResult {
        let start = Instant::now();
        let test_name = "Request Data Export - Success";

        let user_id = self.test_user_id.unwrap_or(Uuid::new_v4());
        let requested_by = self.test_admin_id.unwrap_or(Uuid::new_v4());

        let request = CreateDataExportRequest::new(
            user_id,
            requested_by,
            "json".to_string(),
            vec!["User".to_string(), "Session".to_string()],
        );

        let input = serde_json::to_value(&request).unwrap_or(json!({}));

        let response = match self.api.post("/api/v1/data-exports", &request, None).await {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64())
                    .with_input(input);
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        if response.status_code == 201 || response.status_code == 200 {
            if let Ok(body) = serde_json::from_str::<Value>(&response.body) {
                if let Some(id) = body.get("id").and_then(|v| v.as_str()) {
                    if let Ok(uuid) = Uuid::parse_str(id) {
                        self.created_data_export_ids.push(uuid);
                    }
                }
            }

            TestResult::success(test_name, "Data export requested successfully")
                .with_duration(start.elapsed().as_secs_f64())
                .with_input(input)
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 201/200, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_input(input)
            .with_output(output)
        }
    }

    /// Test: GET /data-exports/:id/download - Download export
    async fn test_download_data_export(&mut self) -> TestResult {
        let start = Instant::now();
        let test_name = "Download Data Export - Success";

        // First create an export request
        let user_id = self.test_user_id.unwrap_or(Uuid::new_v4());
        let requested_by = self.test_admin_id.unwrap_or(Uuid::new_v4());

        let request = CreateDataExportRequest::new(
            user_id,
            requested_by,
            "json".to_string(),
            vec!["User".to_string()],
        );

        let create_response = match self.api.post("/api/v1/data-exports", &request, None).await {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Failed to create: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        if !create_response.is_success() {
            return TestResult::failure(
                test_name,
                format!("Failed to create: status {}", create_response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64());
        }

        let created: Value = match serde_json::from_str(&create_response.body) {
            Ok(v) => v,
            Err(e) => {
                return TestResult::failure(test_name, format!("Failed to parse: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        let export_id = created.get("id").and_then(|v| v.as_str()).unwrap_or("");

        // Try to download (may return 202 if still processing)
        let response = match self
            .api
            .get(&format!("/api/v1/data-exports/{}/download", export_id), None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Download request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        // 200 (ready), 202 (processing), or 404 (not ready) are all valid
        if response.status_code == 200 || response.status_code == 202 || response.status_code == 404 {
            TestResult::success(test_name, "Download endpoint responded correctly")
                .with_duration(start.elapsed().as_secs_f64())
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 200/202/404, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_output(output)
        }
    }

    // ============================================================
    // Anonymization Record API Tests
    // ============================================================

    /// Test: POST /anonymization-records - Create anonymization record
    async fn test_create_anonymization_record_success(&mut self) -> TestResult {
        let start = Instant::now();
        let test_name = "Create Anonymization Record - Success";

        let user_id = self.test_user_id.unwrap_or(Uuid::new_v4());
        let entity_id = Uuid::new_v4();
        let performed_by = self.test_admin_id.unwrap_or(Uuid::new_v4());

        let request = CreateAnonymizationRecordRequest::new(
            user_id,
            "User".to_string(),
            entity_id,
            vec!["email".to_string(), "phone".to_string(), "address".to_string()],
            "gdpr_redaction".to_string(),
            performed_by,
            "GDPR right to be forgotten".to_string(),
        );

        let input = serde_json::to_value(&request).unwrap_or(json!({}));

        let response = match self
            .api
            .post("/api/v1/anonymization-records", &request, None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64())
                    .with_input(input);
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        if response.status_code == 201 || response.status_code == 200 {
            if let Ok(body) = serde_json::from_str::<Value>(&response.body) {
                if let Some(id) = body.get("id").and_then(|v| v.as_str()) {
                    if let Ok(uuid) = Uuid::parse_str(id) {
                        self.created_anonymization_ids.push(uuid);
                    }
                }
            }

            TestResult::success(test_name, "Anonymization record created successfully")
                .with_duration(start.elapsed().as_secs_f64())
                .with_input(input)
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 201/200, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_input(input)
            .with_output(output)
        }
    }

    // ============================================================
    // Resource Permission API Tests
    // ============================================================

    /// Test: POST /resource-permissions - Grant resource permission
    async fn test_grant_resource_permission_success(&mut self) -> TestResult {
        let start = Instant::now();
        let test_name = "Grant Resource Permission - Success";

        let user_id = self.test_user_id.unwrap_or(Uuid::new_v4());
        let granted_by = self.test_admin_id.unwrap_or(Uuid::new_v4());

        let request = CreateResourcePermissionRequest::new(
            "Project".to_string(),
            Uuid::new_v4().to_string(),
            Some(user_id),
            None,
            "read".to_string(),
            granted_by,
        );

        let input = serde_json::to_value(&request).unwrap_or(json!({}));

        let response = match self
            .api
            .post("/api/v1/resource-permissions", &request, None)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64())
                    .with_input(input);
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        if response.status_code == 201 || response.status_code == 200 {
            if let Ok(body) = serde_json::from_str::<Value>(&response.body) {
                if let Some(id) = body.get("id").and_then(|v| v.as_str()) {
                    if let Ok(uuid) = Uuid::parse_str(id) {
                        self.created_resource_permission_ids.push(uuid);
                    }
                }
            }

            TestResult::success(test_name, "Resource permission granted successfully")
                .with_duration(start.elapsed().as_secs_f64())
                .with_input(input)
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 201/200, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_input(input)
            .with_output(output)
        }
    }

    /// Test: GET /resource-permissions/check - Check resource permission
    async fn test_check_resource_permission(&self) -> TestResult {
        let start = Instant::now();
        let test_name = "Check Resource Permission - Success";

        let user_id = self.test_user_id.unwrap_or(Uuid::new_v4());
        let resource_id = Uuid::new_v4().to_string();

        let response = match self
            .api
            .get(
                &format!(
                    "/api/v1/resource-permissions/check?user_id={}&resource_type=Project&resource_id={}&permission=read",
                    user_id, resource_id
                ),
                None,
            )
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return TestResult::failure(test_name, format!("Request failed: {}", e))
                    .with_duration(start.elapsed().as_secs_f64());
            }
        };

        let output = json!({
            "status_code": response.status_code,
            "body": response.body
        });

        if response.status_code == 200 {
            TestResult::success(test_name, "Permission check completed")
                .with_duration(start.elapsed().as_secs_f64())
                .with_output(output)
        } else {
            TestResult::failure(
                test_name,
                format!("Expected 200, got {}", response.status_code),
            )
            .with_duration(start.elapsed().as_secs_f64())
            .with_output(output)
        }
    }

    /// Cleanup created resources
    async fn cleanup_created_resources(&mut self) -> Result<(), TestError> {
        // Cleanup temporary permissions
        for id in self.created_temp_permission_ids.drain(..) {
            let _ = self
                .api
                .delete(&format!("/api/v1/temporary-permissions/{}", id), None)
                .await;
        }

        // Cleanup impersonation sessions
        for id in self.created_impersonation_ids.drain(..) {
            let _ = self
                .api
                .delete(&format!("/api/v1/impersonation-sessions/{}", id), None)
                .await;
        }

        // Cleanup security events
        for id in self.created_security_event_ids.drain(..) {
            let _ = self
                .api
                .delete(&format!("/api/v1/security-events/{}", id), None)
                .await;
        }

        // Cleanup data exports
        for id in self.created_data_export_ids.drain(..) {
            let _ = self
                .api
                .delete(&format!("/api/v1/data-exports/{}", id), None)
                .await;
        }

        // Cleanup anonymization records
        for id in self.created_anonymization_ids.drain(..) {
            let _ = self
                .api
                .delete(&format!("/api/v1/anonymization-records/{}", id), None)
                .await;
        }

        // Cleanup resource permissions
        for id in self.created_resource_permission_ids.drain(..) {
            let _ = self
                .api
                .delete(&format!("/api/v1/resource-permissions/{}", id), None)
                .await;
        }

        Ok(())
    }
}

#[async_trait]
impl Test for V2ApiTest {
    fn name(&self) -> &str {
        &self.name
    }

    async fn setup(&mut self) -> Result<(), TestError> {
        self.setup_manager.setup().await?;

        // Create test user and admin IDs
        self.test_user_id = Some(Uuid::new_v4());
        self.test_admin_id = Some(Uuid::new_v4());
        self.test_permission_id = Some(Uuid::new_v4());

        // Simulate admin session
        let (_user_id, token) = self
            .common_utils
            .simulate_admin_session("v2_setup")
            .map_err(|e| TestError::SetupFailed(e))?;

        self.test_auth_token = Some(token.clone());
        self.api.add_header("Authorization", &format!("Bearer {}", token));

        Ok(())
    }

    async fn teardown(&mut self) -> Result<(), TestError> {
        // Cleanup created resources
        self.cleanup_created_resources().await?;

        // Teardown setup manager
        self.setup_manager.teardown().await?;

        Ok(())
    }

    async fn run_tests(&mut self) -> Vec<TestResult> {
        vec![
            // Temporary Permission tests
            self.test_create_temporary_permission_success().await,
            self.test_create_temporary_permission_exceeds_max_duration().await,
            self.test_get_temporary_permission().await,
            // Impersonation Session tests
            self.test_start_impersonation_success().await,
            self.test_prevent_self_impersonation().await,
            self.test_end_impersonation_session().await,
            // Security Event tests
            self.test_create_security_event_success().await,
            self.test_resolve_security_event().await,
            self.test_list_security_events_by_severity().await,
            // Data Export tests
            self.test_request_data_export_success().await,
            self.test_download_data_export().await,
            // Anonymization Record tests
            self.test_create_anonymization_record_success().await,
            // Resource Permission tests
            self.test_grant_resource_permission_success().await,
            self.test_check_resource_permission().await,
        ]
    }
}

impl Default for V2ApiTest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2_api_test_creation() {
        let test = V2ApiTest::new();
        assert_eq!(test.name(), "v2_api_test");
    }

    #[test]
    fn test_create_temporary_permission_request() {
        let user_id = Uuid::new_v4();
        let permission_id = Uuid::new_v4();
        let granted_by = Uuid::new_v4();

        let request = CreateTemporaryPermissionRequest::new(
            user_id,
            permission_id,
            granted_by,
            7,
            "Test".to_string(),
        );

        assert_eq!(request.user_id, user_id);
        assert_eq!(request.permission_id, permission_id);
        assert_eq!(request.status, "active");
    }

    #[test]
    fn test_create_impersonation_session_request() {
        let admin_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();

        let request =
            CreateImpersonationSessionRequest::new(admin_id, target_user_id, 60, "Test".to_string(), "127.0.0.1".to_string());

        assert_eq!(request.admin_id, admin_id);
        assert_eq!(request.target_user_id, target_user_id);
        assert_eq!(request.status, "active");
    }
}
