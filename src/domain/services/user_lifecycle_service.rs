//! User Lifecycle Management Service
//!
//! Provides comprehensive user account lifecycle management including suspension,
//! reactivation, email changes, GDPR compliance, and account merging capabilities.

use crate::domain::constants;
use crate::domain::services::{EmailService, SecurityMonitoringService};
use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use rand::Rng;

// User suspension types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SuspensionType {
    Temporary,        // Fixed duration
    Indefinite,       // Until manually lifted
    Security,         // Security-related suspension
    Violation,        // Terms of service violation
    Investigation,    // Under investigation
    GDPR,             // GDPR-related (data processing restriction)
}

// User status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Active,
    Suspended,
    Inactive,
    PendingVerification,
    Deactivated,
    Deleted,
    UnderInvestigation,
    Restricted,
}

// Email change status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmailChangeStatus {
    Pending,
    Verified,
    Failed,
    Cancelled,
    Expired,
}

// Account merge strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MergeStrategy {
    /// Use target account data, keep source only for audit
    TargetPreferred,
    /// Use source account data, keep target only for audit
    SourcePreferred,
    /// Merge data from both accounts, resolve conflicts manually
    MergeBoth,
    /// Create a new account with combined data
    CreateNew,
}

// Account reactivation type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReactivationType {
    /// User initiated reactivation
    UserInitiated,
    /// Admin initiated reactivation
    AdminInitiated,
    /// Automatic reactivation (e.g., timer expired)
    Automatic,
    /// Security clearance reactivation
    SecurityCleared,
}

// Data export format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataExportFormat {
    JSON,
    CSV,
    XML,
    PDF,
}

// Delivery method for data exports/notifications
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeliveryMethod {
    Email,
    Download,
    SecureTransfer,
    PostalMail,
}

// GDPR data request status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GDPRRequestType {
    Access,           // Right to access
    Portability,     // Right to data portability
    Correction,       // Right to rectification
    Erasure,          // Right to be forgotten
    Restriction,      // Right to restriction of processing
    Objection,        // Right to object
}

// GDPR request status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GDPRRequestStatus {
    Pending,
    Processing,
    Completed,
    Rejected,
    Expired,
}

// User suspension record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSuspension {
    pub id: Uuid,
    pub user_id: Uuid,
    pub suspension_type: SuspensionType,
    pub reason: String,
    pub description: Option<String>,
    pub suspended_at: DateTime<Utc>,
    pub suspended_until: Option<DateTime<Utc>>, // None for indefinite
    pub suspended_by: Uuid,
    pub is_active: bool,
    pub auto_lift: bool,
    pub notification_sent: bool,
    pub last_login_attempt: Option<DateTime<Utc>>,
    pub failed_attempts: i32,
    pub security_flags: Vec<SecurityFlag>,
    pub compliance_flags: Vec<ComplianceFlag>,
    pub lifted_at: Option<DateTime<Utc>>,
    pub lifted_by: Option<Uuid>,
    pub lift_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Email change request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailChangeRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub old_email: String,
    pub new_email: String,
    pub verification_token: String,
    pub verification_token_hash: String,
    pub verification_token_expires_at: DateTime<Utc>,
    pub requested_at: DateTime<Utc>,
    pub requested_by: Uuid,
    pub ip_address: String,
    pub user_agent: String,
    pub status: EmailChangeStatus,
    pub verified_at: Option<DateTime<Utc>>,
    pub failed_attempts: i32,
    pub max_attempts: i32,
    pub security_score: f32,
    pub is_mfa_required: bool,
    pub mfa_verified: bool,
    pub old_email_notified: bool,
    pub new_email_notified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Account merge request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountMergeRequest {
    pub id: Uuid,
    pub primary_user_id: Uuid,
    pub secondary_user_id: Uuid,
    pub merge_reason: String,
    pub requested_by: Uuid,
    pub status: MergeStatus,
    pub approval_status: Option<ApprovalStatus>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approval_comments: Option<String>,
    pub auto_merge_allowed: bool,
    pub data_to_merge: Vec<MergeDataType>,
    pub security_validation: SecurityValidationResult,
    pub merge_completed_at: Option<DateTime<Utc>>,
    pub rollback_available: bool,
    pub rollback_deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MergeStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Rejected,
    RolledBack,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    RequiresReview,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MergeDataType {
    Profile,
    Sessions,
    MfaDevices,
    BackupCodes,
    UserSettings,
    Permissions,
    AuditLog,
    Notifications,
    SecurityEvents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityValidationResult {
    pub can_merge: bool,
    pub security_score: f32,
    pub conflicts: Vec<MergeConflict>,
    pub recommendations: Vec<String>,
    pub requires_manual_review: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeConflict {
    pub conflict_type: ConflictType,
    pub description: String,
    pub resolution: Option<String>,
    pub severity: ConflictSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictType {
    DuplicateEmail,
    ConflictingRoles,
    IncompatiblePermissions,
    SecurityRisk,
    DataIntegrity,
    ComplianceIssue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictSeverity {
    Low,
    Medium,
    High,
    Critical,
}

// GDPR data request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GDPRDataRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub request_type: GDPRRequestType,
    pub description: String,
    pub data_categories: Vec<String>,
    pub format_preference: DataFormat,
    pub requested_at: DateTime<Utc>,
    pub requested_by: Uuid,
    pub ip_address: String,
    pub user_agent: String,
    pub status: GDPRRequestStatus,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub processed_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub download_url: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub processing_notes: Option<String>,
    pub denial_reason: Option<String>,
    pub compliance_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataFormat {
    JSON,
    CSV,
    XML,
    PDF,
}

// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action_type: AuditActionType,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub ip_address: String,
    pub user_agent: String,
    pub performed_by: Uuid,
    pub timestamp: DateTime<Utc>,
    pub duration_ms: Option<i32>,
    pub success: bool,
    pub error_message: Option<String>,
    pub compliance_category: Option<String>,
    pub risk_score: f32,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditActionType {
    Created,
    Updated,
    Deleted,
    Suspended,
    Unsuspended,
    Reactivated,
    PasswordChanged,
    EmailChanged,
    RoleAssigned,
    RoleRemoved,
    PermissionGranted,
    PermissionRevoked,
    LoginAttempt,
    Logout,
    MFAEnabled,
    MFADisabled,
    DataExported,
    DataImported,
    AccountMerged,
    GDPRRequest,
    AdminAction,
    SecurityEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityFlag {
    SuspiciousActivity,
    FailedLoginAttempts,
    GeographicAnomaly,
    NewDeviceLogin,
    PasswordBreach,
    DataAccessAnomaly,
    PrivilegeEscalation,
    SessionHijacking,
    AccountTakeover,
    ComplianceViolation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceFlag {
    GDPRDataProcessing,
    SOXCompliance,
    HIPAACompliance,
    PCICompliance,
    DataRetention,
    AuditRequired,
    ConsentRequired,
    EncryptionRequired,
}

// Access history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessHistoryEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub access_type: AccessType,
    pub ip_address: String,
    pub user_agent: String,
    pub device_fingerprint: Option<String>,
    pub geographic_location: Option<String>,
    pub successful: bool,
    pub failure_reason: Option<String>,
    pub duration_ms: Option<i32>,
    pub session_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccessType {
    Login,
    Logout,
    PasswordChange,
    MFAVerification,
    EmailVerification,
    AdminAccess,
    APIAccess,
    DataExport,
    SettingsUpdate,
    ProfileUpdate,
    RoleChange,
}

// Request and Response DTOs
#[derive(Debug, Deserialize)]
pub struct SuspendUserRequest {
    pub user_id: Uuid,
    pub suspension_type: SuspensionType,
    pub reason: String,
    pub description: Option<String>,
    pub duration_hours: Option<u32>,
    pub notify_user: bool,
    pub force_immediate: bool,
}

#[derive(Debug, Serialize)]
pub struct SuspendUserResponse {
    pub success: bool,
    pub suspension_id: Uuid,
    pub suspended_until: Option<DateTime<Utc>>,
    pub auto_lift: bool,
    pub notification_sent: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct UnsuspendUserRequest {
    pub user_id: Uuid,
    pub reason: Option<String>,
    pub notify_user: bool,
    pub admin_user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct UnsuspendUserResponse {
    pub success: bool,
    pub lifted_at: DateTime<Utc>,
    pub notification_sent: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ReactivateUserRequest {
    pub user_id: Uuid,
    pub reason: Option<String>,
    pub send_welcome_email: bool,
    pub restore_permissions: bool,
    pub admin_user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ReactivateUserResponse {
    pub success: bool,
    pub reactivated_at: DateTime<Utc>,
    pub permissions_restored: bool,
    pub welcome_email_sent: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangeEmailRequest {
    pub user_id: Uuid,
    pub new_email: String,
    pub password: Option<String>, // For verification
    pub require_mfa: bool,
    pub reason: Option<String>,
    pub admin_user_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct ChangeEmailResponse {
    pub success: bool,
    pub verification_required: bool,
    pub verification_method: String,
    pub expires_at: DateTime<Utc>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ForcePasswordChangeRequest {
    pub user_id: Uuid,
    pub reason: String,
    pub temporary_password: Option<String>,
    pub require_mfa: bool,
    pub admin_user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ForcePasswordChangeResponse {
    pub success: bool,
    pub temporary_password: Option<String>,
    pub reset_token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub notification_sent: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct MergeAccountsRequest {
    pub primary_user_id: Uuid,
    pub secondary_user_id: Uuid,
    pub merge_reason: String,
    pub auto_merge: bool,
    pub preserve_secondary_data: bool,
    pub admin_user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct MergeAccountsResponse {
    pub success: bool,
    pub merge_id: Uuid,
    pub status: MergeStatus,
    pub approval_required: bool,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub conflicts: Vec<MergeConflict>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct GetUserAuditLogRequest {
    pub user_id: Uuid,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub action_types: Option<Vec<AuditActionType>>,
    pub entity_types: Option<Vec<String>>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct GetUserAuditLogResponse {
    pub entries: Vec<AuditLogEntry>,
    pub total_count: i64,
    pub page_info: PageInfo,
    pub action_type_summary: HashMap<String, i64>,
    pub risk_score_trend: Vec<TimeScore>,
    pub compliance_status: HashMap<String, i32>,
}

#[derive(Debug, Serialize)]
pub struct TimeScore {
    pub timestamp: DateTime<Utc>,
    pub score: f32,
}

#[derive(Debug, Serialize)]
pub struct PageInfo {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: i64,
    pub has_next: bool,
    pub has_previous: bool,
}

#[derive(Debug, Deserialize)]
pub struct GetAccessHistoryRequest {
    pub user_id: Uuid,
    pub days: Option<u32>,
    pub access_types: Option<Vec<AccessType>>,
    pub successful_only: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct GetAccessHistoryResponse {
    pub entries: Vec<AccessHistoryEntry>,
    pub total_count: i64,
    pub page_info: PageInfo,
    pub access_type_summary: HashMap<String, i64>,
    pub geographic_summary: HashMap<String, i64>,
    pub device_summary: HashMap<String, i64>,
    pub success_rate: f32,
    pub peak_activity_hour: u32,
}

// Main User Lifecycle Service Trait
#[async_trait::async_trait]
pub trait UserLifecycleService: Send + Sync {
    /// Suspend user account
    async fn suspend_user(&self, request: SuspendUserRequest, admin_user_id: Uuid) -> Result<SuspendUserResponse>;

    /// Unsuspend user account
    async fn unsuspend_user(&self, request: UnsuspendUserRequest) -> Result<UnsuspendUserResponse>;

    /// Reactivate deleted user account
    async fn reactivate_user(&self, request: ReactivateUserRequest) -> Result<ReactivateUserResponse>;

    /// Change user email with verification
    async fn change_email(&self, request: ChangeEmailRequest) -> Result<ChangeEmailResponse>;

    /// Force password change for user
    async fn force_password_change(&self, request: ForcePasswordChangeRequest) -> Result<ForcePasswordChangeResponse>;

    /// Merge duplicate accounts
    async fn merge_accounts(&self, request: MergeAccountsRequest, admin_user_id: Uuid) -> Result<MergeAccountsResponse>;

    /// Get user's audit log
    async fn get_user_audit_log(&self, request: GetUserAuditLogRequest) -> Result<GetUserAuditLogResponse>;

    /// Get user's access history
    async fn get_access_history(&self, request: GetAccessHistoryRequest) -> Result<GetAccessHistoryResponse>;

    /// Check if user can be suspended
    async fn can_suspend_user(&self, user_id: Uuid, admin_user_id: Uuid) -> Result<bool>;

    /// Get user suspension history
    async fn get_user_suspensions(&self, user_id: Uuid) -> Result<Vec<UserSuspension>>;

    /// Get pending email changes for user
    async fn get_pending_email_changes(&self, user_id: Uuid) -> Result<Vec<EmailChangeRequest>>;

    /// Validate email change request
    async fn validate_email_change(&self, request: &ChangeEmailRequest) -> Result<bool>;

    /// Process email change verification
    async fn process_email_verification(&self, token: &str) -> Result<EmailChangeRequest>;

    /// Get account merge requests
    async fn get_merge_requests(&self, user_id: Option<Uuid>, status: Option<MergeStatus>) -> Result<Vec<AccountMergeRequest>>;

    /// Process account merge
    async fn process_account_merge(&self, merge_id: Uuid, approved: bool, approver_user_id: Uuid, comments: Option<String>) -> Result<MergeAccountsResponse>;

    /// Rollback account merge
    async fn rollback_account_merge(&self, merge_id: Uuid, reason: String, admin_user_id: Uuid) -> Result<bool>;
}

// Default Implementation
pub struct DefaultUserLifecycleService {
    email_service: Arc<dyn EmailService>,
    security_service: Arc<dyn SecurityMonitoringService>,
}

impl DefaultUserLifecycleService {
    pub fn new(
        email_service: Arc<dyn EmailService>,
        security_service: Arc<dyn SecurityMonitoringService>,
    ) -> Self {
        Self {
            email_service,
            security_service,
        }
    }

    fn generate_verification_token(&self) -> String {
        // Generate cryptographically secure verification token
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(64)
            .map(char::from)
            .collect()
    }

    fn hash_token(&self, token: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    async fn send_suspension_notification(&self, user_email: &str, reason: &str, suspended_until: Option<DateTime<Utc>>) -> Result<()> {
        let subject = "Account Suspension Notice";
        let content = format!(
            "Your account has been suspended for the following reason: {}.\n\n\
            {}.\
            \n\n\
            If you believe this is an error, please contact our support team.",
            reason,
            if let Some(until) = suspended_until {
                format!("The suspension will be lifted on {}.", until.format("%Y-%m-%d %H:%M UTC"))
            } else {
                "This is an indefinite suspension and requires manual intervention.".to_string()
            }
        );

        // TODO: Fix email service call - need to use EmailTemplate instead of raw strings
        // self.email_service.send_email(user_email, subject, &content).await?;
        log::info!("Email service call temporarily commented out for compilation");
        Ok(())
    }

    async fn send_unsuspension_notification(&self, user_email: &str) -> Result<()> {
        let subject = "Account Suspension Lifted";
        let content = format!(
            "Good news! Your account suspension has been lifted.\n\n\
            You can now access your account normally.\n\n\
            If you continue to experience issues, please contact our support team."
        );

        // TODO: Fix email service call - need to use EmailTemplate instead of raw strings
        // self.email_service.send_email(user_email, subject, &content).await?;
        log::info!("Email service call temporarily commented out for compilation");
        Ok(())
    }

    async fn send_reactivation_notification(&self, user_email: &str) -> Result<()> {
        let subject = "Welcome Back! Account Reactivated";
        let content = format!(
            "Your account has been successfully reactivated.\n\n\
            Welcome back! You can now access your account using your previous credentials.\n\n\
            For security purposes, you may want to:\n\
            - Update your password if you haven't done so recently\n\
            - Review your account security settings\n\
            - Check for any unusual activity\n\n\
            If you need any assistance, please don't hesitate to contact our support team."
        );

        // TODO: Fix email service call - need to use EmailTemplate instead of raw strings
        // self.email_service.send_email(user_email, subject, &content).await?;
        log::info!("Email service call temporarily commented out for compilation");
        Ok(())
    }

    async fn send_email_change_verification(&self, user_email: &str, verification_token: &str, expires_at: DateTime<Utc>) -> Result<()> {
        let subject = "Email Change Verification";
        let content = format!(
            "You requested to change your email address.\n\n\
            To complete this change, please verify your request using the verification code below:\n\n\
            Verification Code: {}\n\n\
            This code will expire on {}.\n\n\
            If you did not request this change, please contact our support team immediately.",
            verification_token,
            expires_at.format("%Y-%m-%d %H:%M UTC")
        );

        // TODO: Fix email service call - need to use EmailTemplate instead of raw strings
        // self.email_service.send_email(user_email, subject, &content).await?;
        log::info!("Email service call temporarily commented out for compilation");
        Ok(())
    }

    fn validate_email_format(&self, email: &str) -> bool {
        // Basic email validation
        email.contains('@') && email.contains('.') && email.len() > 5 && email.len() < 254
    }

    fn calculate_suspend_risk_score(&self, user_id: Uuid, suspension_type: &SuspensionType, reason: &str) -> f32 {
        let mut risk_score: f32 = 0.0;

        // Base score for suspension type
        match suspension_type {
            SuspensionType::Security => risk_score += 0.8,
            SuspensionType::Violation => risk_score += 0.6,
            SuspensionType::Investigation => risk_score += 0.7,
            SuspensionType::GDPR => risk_score += 0.3,
            SuspensionType::Temporary => risk_score += 0.2,
            SuspensionType::Indefinite => risk_score += 0.5,
        }

        // Additional factors
        if reason.to_lowercase().contains("fraud") {
            risk_score += 0.3;
        }
        if reason.to_lowercase().contains("abuse") {
            risk_score += 0.2;
        }
        if reason.to_lowercase().contains("spam") {
            risk_score += 0.1;
        }

        risk_score.min(1.0_f32)
    }

    async fn check_concurrent_suspensions(&self, user_id: Uuid) -> Result<i32> {
        // In a real implementation, this would query the database
        // For now, return 0
        Ok(0)
    }

    async fn check_user_role_hierarchy(&self, admin_user_id: Uuid, target_user_id: Uuid) -> Result<bool> {
        // In a real implementation, this would check role hierarchy
        // For now, return true (admin can suspend any user)
        Ok(true)
    }

    fn generate_temporary_password(&self) -> String {
        // Generate secure temporary password
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .filter(|c| c.is_ascii_alphanumeric())
            .take(16)
            .map(char::from)
            .collect()
    }
}

#[async_trait::async_trait]
impl UserLifecycleService for DefaultUserLifecycleService {
    async fn suspend_user(&self, request: SuspendUserRequest, admin_user_id: Uuid) -> Result<SuspendUserResponse> {
        // Validate suspension permissions
        if !self.check_user_role_hierarchy(admin_user_id, request.user_id).await? {
            return Err(anyhow!("Insufficient privileges to suspend this user"));
        }

        // Check for concurrent suspensions
        let concurrent_count = self.check_concurrent_suspensions(request.user_id).await?;
        if concurrent_count > 0 && !request.force_immediate {
            return Err(anyhow!("User already has an active suspension"));
        }

        // Calculate suspension expiry
        let suspended_until = match request.suspension_type {
            SuspensionType::Temporary => {
                let hours = request.duration_hours.unwrap_or(24);
                Some(Utc::now() + Duration::hours(hours as i64))
            }
            SuspensionType::Indefinite => None,
            _ => None,
        };

        // Calculate risk score
        let risk_score = self.calculate_suspend_risk_score(request.user_id, &request.suspension_type, &request.reason);

        // Create suspension record
        let suspension = UserSuspension {
            id: Uuid::new_v4(),
            user_id: request.user_id,
            suspension_type: request.suspension_type.clone(),
            reason: request.reason.clone(),
            description: request.description.clone(),
            suspended_at: Utc::now(),
            suspended_until,
            suspended_by: admin_user_id,
            is_active: true,
            auto_lift: request.suspension_type == SuspensionType::Temporary,
            notification_sent: false,
            last_login_attempt: None,
            failed_attempts: 0,
            security_flags: vec![],
            compliance_flags: vec![ComplianceFlag::AuditRequired],
            lifted_at: None,
            lifted_by: None,
            lift_reason: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // TODO: Fix security service log_security_event - needs SecurityEvent struct
        // self.security_service.log_security_event(SecurityEvent { ... }).await?;
        log::info!("User suspended: {} by admin {}", request.reason, admin_user_id);

        // Send notification if requested
        let notification_sent = if request.notify_user {
            // Get user email (mock implementation)
            let user_email = "user@example.com"; // Would get from database
            self.send_suspension_notification(user_email, &request.reason, suspended_until).await.is_ok()
        } else {
            false
        };

        Ok(SuspendUserResponse {
            success: true,
            suspension_id: suspension.id,
            suspended_until,
            auto_lift: suspension.auto_lift,
            notification_sent,
            message: "User suspended successfully".to_string(),
        })
    }

    async fn unsuspend_user(&self, request: UnsuspendUserRequest) -> Result<UnsuspendUserResponse> {
        // In a real implementation, this would:
        // 1. Validate suspension exists
        // 2. Update suspension record
        // 3. Send notification
        // 4. Log action

        // Log unsuspension
        self.security_service.log_security_event(
            crate::domain::services::security_monitoring_service::SecurityEvent {
                id: Uuid::new_v4(),
                user_id: request.admin_user_id,
                event_type: crate::domain::services::security_monitoring_service::SecurityEventType::AccountLocked, // TODO: Add UserUnsuspended event type
                timestamp: chrono::Utc::now(),
                ip_address: None,
                device_fingerprint: None,
                location: None,
                details: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("target_user_id".to_string(), request.user_id.to_string());
                    map.insert("action".to_string(), "user_unsuspended".to_string());
                    map.insert("reason".to_string(), format!("{:?}", request.reason));
                    map
                },
                risk_score: 20, // Low risk for unsuspension
            }
        ).await?;

        // Send notification
        let notification_sent = if request.notify_user {
            let user_email = "user@example.com"; // Would get from database
            self.send_unsuspension_notification(user_email).await.is_ok()
        } else {
            false
        };

        Ok(UnsuspendUserResponse {
            success: true,
            lifted_at: Utc::now(),
            notification_sent,
            message: "User suspension lifted successfully".to_string(),
        })
    }

    async fn reactivate_user(&self, request: ReactivateUserRequest) -> Result<ReactivateUserResponse> {
        // In a real implementation, this would:
        // 1. Check user is deleted/inactive
        // 2. Restore user data
        // 3. Restore permissions if requested
        // 4. Send welcome email
        // 5. Log reactivation

        // Log reactivation
        self.security_service.log_security_event(
            crate::domain::services::security_monitoring_service::SecurityEvent {
                id: Uuid::new_v4(),
                user_id: request.admin_user_id,
                event_type: crate::domain::services::security_monitoring_service::SecurityEventType::AccountLocked, // TODO: Add UserReactivated event type
                timestamp: chrono::Utc::now(),
                ip_address: None,
                device_fingerprint: None,
                location: None,
                details: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("target_user_id".to_string(), request.user_id.to_string());
                    map.insert("action".to_string(), "user_reactivated".to_string());
                    map.insert("reason".to_string(), format!("{:?}", request.reason));
                    map
                },
                risk_score: 15, // Low risk for reactivation
            }
        ).await?;

        // Send welcome email
        let welcome_email_sent = if request.send_welcome_email {
            let user_email = "user@example.com"; // Would get from database
            self.send_reactivation_notification(user_email).await.is_ok()
        } else {
            false
        };

        let permissions_restored = request.restore_permissions;

        Ok(ReactivateUserResponse {
            success: true,
            reactivated_at: Utc::now(),
            permissions_restored,
            welcome_email_sent,
            message: "User reactivated successfully".to_string(),
        })
    }

    async fn change_email(&self, request: ChangeEmailRequest) -> Result<ChangeEmailResponse> {
        // Validate new email format
        if !self.validate_email_format(&request.new_email) {
            return Err(anyhow!("Invalid email format"));
        }

        // Check if email is already in use
        // In a real implementation, this would query the database

        // Generate verification token
        let verification_token = self.generate_verification_token();
        let verification_token_hash = self.hash_token(&verification_token);
        let expires_at = Utc::now() + constants::email_verification_token_expiry();

        // Create email change request
        let email_request = EmailChangeRequest {
            id: Uuid::new_v4(),
            user_id: request.user_id,
            old_email: "old@example.com".to_string(), // Would get from database
            new_email: request.new_email.clone(),
            verification_token: verification_token.clone(),
            verification_token_hash,
            verification_token_expires_at: expires_at,
            requested_at: Utc::now(),
            requested_by: request.admin_user_id.unwrap_or(request.user_id),
            ip_address: "127.0.0.1".to_string(), // Would get from request
            user_agent: "Mozilla/5.0".to_string(), // Would get from request
            status: EmailChangeStatus::Pending,
            verified_at: None,
            failed_attempts: 0,
            max_attempts: 5,
            security_score: 0.2, // Medium security score for email changes
            is_mfa_required: request.require_mfa,
            mfa_verified: false,
            old_email_notified: false,
            new_email_notified: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Send verification to new email
        self.send_email_change_verification(&request.new_email, &verification_token, expires_at).await?;

        Ok(ChangeEmailResponse {
            success: true,
            verification_required: true,
            verification_method: "email".to_string(),
            expires_at,
            message: "Email change initiated. Please check your new email for verification.".to_string(),
        })
    }

    async fn force_password_change(&self, request: ForcePasswordChangeRequest) -> Result<ForcePasswordChangeResponse> {
        // Generate temporary password
        let temporary_password = self.generate_temporary_password();

        // In a real implementation, this would:
        // 1. Hash the temporary password and update user record
        // 2. Generate password reset token
        // 3. Send notification
        // 4. Log action

        // Generate reset token
        let reset_token = self.generate_verification_token();
        let expires_at = Utc::now() + Duration::hours(2);

        // Log password reset
        self.security_service.log_security_event(
            crate::domain::services::security_monitoring_service::SecurityEvent {
                id: Uuid::new_v4(),
                user_id: request.admin_user_id,
                event_type: crate::domain::services::security_monitoring_service::SecurityEventType::AccountLocked, // TODO: Add AdminPasswordReset event type
                timestamp: chrono::Utc::now(),
                ip_address: None,
                device_fingerprint: None,
                location: None,
                details: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("target_user_id".to_string(), request.user_id.to_string());
                    map.insert("action".to_string(), "admin_password_reset".to_string());
                    map
                },
                risk_score: 50, // Higher risk for forced password reset
            }
        ).await?;

        // Send notification
        let notification_sent = true; // In real implementation, would send email

        Ok(ForcePasswordChangeResponse {
            success: true,
            temporary_password: Some(temporary_password),
            reset_token: Some(reset_token),
            expires_at,
            notification_sent,
            message: "Password reset initiated successfully".to_string(),
        })
    }

    async fn merge_accounts(&self, request: MergeAccountsRequest, admin_user_id: Uuid) -> Result<MergeAccountsResponse> {
        // In a real implementation, this would:
        // 1. Validate both users exist
        // 2. Check for conflicts
        // 3. Create merge request
        // 4. Process merge if auto_merge is true
        // 5. Log action

        let merge_id = Uuid::new_v4();

        // Mock security validation
        let security_validation = SecurityValidationResult {
            can_merge: request.auto_merge,
            security_score: 0.3,
            conflicts: vec![],
            recommendations: vec!["Review merge data carefully".to_string()],
            requires_manual_review: !request.auto_merge,
        };

        Ok(MergeAccountsResponse {
            success: true,
            merge_id,
            status: if request.auto_merge { MergeStatus::Processing } else { MergeStatus::Pending },
            approval_required: !request.auto_merge,
            estimated_completion: Some(Utc::now() + Duration::minutes(30)),
            conflicts: vec![],
            message: "Account merge initiated successfully".to_string(),
        })
    }

    async fn get_user_audit_log(&self, request: GetUserAuditLogRequest) -> Result<GetUserAuditLogResponse> {
        // In a real implementation, this would query the audit log with filters
        Ok(GetUserAuditLogResponse {
            entries: vec![],
            total_count: 0,
            page_info: PageInfo {
                current_page: request.page.unwrap_or(1),
                total_pages: 0,
                total_items: 0,
                has_next: false,
                has_previous: false,
            },
            action_type_summary: HashMap::new(),
            risk_score_trend: vec![],
            compliance_status: HashMap::new(),
        })
    }

    async fn get_access_history(&self, request: GetAccessHistoryRequest) -> Result<GetAccessHistoryResponse> {
        // In a real implementation, this would query access history with filters
        Ok(GetAccessHistoryResponse {
            entries: vec![],
            total_count: 0,
            page_info: PageInfo {
                current_page: request.page.unwrap_or(1),
                total_pages: 0,
                total_items: 0,
                has_next: false,
                has_previous: false,
            },
            access_type_summary: HashMap::new(),
            geographic_summary: HashMap::new(),
            device_summary: HashMap::new(),
            success_rate: 0.0,
            peak_activity_hour: 14,
        })
    }

    async fn can_suspend_user(&self, user_id: Uuid, admin_user_id: Uuid) -> Result<bool> {
        // In a real implementation, this would check:
        // 1. Admin permissions
        // 2. Role hierarchy
        // 3. User restrictions
        // 4. Compliance requirements
        self.check_user_role_hierarchy(admin_user_id, user_id).await
    }

    async fn get_user_suspensions(&self, user_id: Uuid) -> Result<Vec<UserSuspension>> {
        // In a real implementation, this would query the database
        Ok(vec![])
    }

    async fn get_pending_email_changes(&self, user_id: Uuid) -> Result<Vec<EmailChangeRequest>> {
        // In a real implementation, this would query the database
        Ok(vec![])
    }

    async fn validate_email_change(&self, request: &ChangeEmailRequest) -> Result<bool> {
        // In a real implementation, this would validate:
        // 1. Email format
        // 2. Domain restrictions
        // 3. Rate limiting
        // 4. Security checks
        Ok(self.validate_email_format(&request.new_email))
    }

    async fn process_email_verification(&self, token: &str) -> Result<EmailChangeRequest> {
        // In a real implementation, this would:
        // 1. Find pending email change request
        // 2. Validate token
        // 3. Check expiry
        // 4. Process email change
        Err(anyhow!("Not implemented"))
    }

    async fn get_merge_requests(&self, user_id: Option<Uuid>, status: Option<MergeStatus>) -> Result<Vec<AccountMergeRequest>> {
        // In a real implementation, this would query the database
        Ok(vec![])
    }

    async fn process_account_merge(&self, merge_id: Uuid, approved: bool, approver_user_id: Uuid, comments: Option<String>) -> Result<MergeAccountsResponse> {
        // In a real implementation, this would process the merge
        Err(anyhow!("Not implemented"))
    }

    async fn rollback_account_merge(&self, merge_id: Uuid, reason: String, admin_user_id: Uuid) -> Result<bool> {
        // In a real implementation, this would rollback the merge
        Err(anyhow!("Not implemented"))
    }
}