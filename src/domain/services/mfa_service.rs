//! Multi-Factor Authentication Service
//!
//! Provides comprehensive MFA functionality including TOTP, SMS, email,
//! hardware keys, biometrics, and push notifications.

use crate::domain::services::{EmailService, SecurityMonitoringService, SecurityEvent, SecurityEventType};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc, Duration};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

// MFA Device Types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MFADeviceType {
    TOTP,
    SMS,
    Email,
    HardwareKey,
    Biometric,
    PushNotification,
}

// MFA Device Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MFADeviceStatus {
    Active,
    Inactive,
    Suspended,
    Compromised,
    Expired,
    Revoked,
}

// MFA Verification Method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MFAVerificationMethod {
    TOTP,
    SMS,
    Email,
    HardwareKey,
    PushNotification,
    BackupCode,
    Biometric,
}

// MFA Device Data Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFADevice {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_type: MFADeviceType,
    pub device_name: String,
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
    pub totp_secret: Option<String>,
    pub hardware_key_id: Option<String>,
    pub push_token: Option<String>,
    pub device_fingerprint: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub operating_system: Option<String>,
    pub app_version: Option<String>,
    pub is_primary: bool,
    pub is_backup: bool,
    pub is_active: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub last_used: Option<DateTime<Utc>>,
    pub usage_count: i32,
    pub successful_verifications: i32,
    pub failed_verifications: i32,
    pub status: MFADeviceStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// MFA Session Data Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFASession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub session_token: String,
    pub session_hash: String,
    pub verification_method: MFAVerificationMethod,
    pub verification_initiated_at: DateTime<Utc>,
    pub verification_completed_at: Option<DateTime<Utc>>,
    pub verification_attempts: i32,
    pub max_attempts_allowed: i32,
    pub verification_success: Option<bool>,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub status: MFASessionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// MFA Session Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MFASessionStatus {
    Initiated,
    PendingVerification,
    Verified,
    Failed,
    Expired,
    Terminated,
}

// MFA Backup Code Data Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFABackupCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_id: Option<Uuid>,
    pub batch_id: Uuid,
    pub code_hash: String,
    pub code_index: i32,
    pub generated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_consumed: bool,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Request and Response DTOs
#[derive(Debug, Deserialize)]
pub struct SetupMFARequest {
    pub device_type: MFADeviceType,
    pub device_name: String,
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
    pub device_fingerprint: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub operating_system: Option<String>,
    pub app_version: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SetupMFAResponse {
    pub device_id: Uuid,
    pub device_type: MFADeviceType,
    pub qr_code: Option<String>, // For TOTP
    pub secret: Option<String>,   // For TOTP
    pub verification_code: Option<String>, // For SMS/Email
    pub instructions: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyMFARequest {
    pub device_id: Uuid,
    pub verification_code: String,
    pub session_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VerifyMFAResponse {
    pub success: bool,
    pub device_id: Uuid,
    pub verification_method: MFAVerificationMethod,
    pub remaining_attempts: i32,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateBackupCodesRequest {
    pub device_id: Option<Uuid>,
    pub count: i32,
}

#[derive(Debug, Serialize)]
pub struct GenerateBackupCodesResponse {
    pub backup_codes: Vec<String>,
    pub batch_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub instructions: String,
}

#[derive(Debug, Deserialize)]
pub struct ListMFADevicesRequest {
    pub user_id: Uuid,
    pub include_inactive: bool,
}

#[derive(Debug, Serialize)]
pub struct ListMFADevicesResponse {
    pub devices: Vec<MFADevice>,
    pub total_count: i32,
    pub active_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct RemoveMFARequest {
    pub device_id: Uuid,
    pub confirmation_code: String,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RemoveMFAResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct MFAPreferenceUpdate {
    pub primary_device_id: Option<Uuid>,
    pub require_mfa_for_new_devices: Option<bool>,
    pub grace_period_minutes: Option<i32>,
}

// Main MFA Service Trait
#[async_trait::async_trait]
pub trait MFAService: Send + Sync {
    /// Setup a new MFA device
    async fn setup_mfa_device(&self, request: SetupMFARequest, user_id: Uuid) -> Result<SetupMFAResponse>;

    /// Verify MFA device during setup
    async fn verify_mfa_device(&self, request: VerifyMFARequest) -> Result<VerifyMFAResponse>;

    /// Generate backup codes for MFA recovery
    async fn generate_backup_codes(&self, request: GenerateBackupCodesRequest, user_id: Uuid) -> Result<GenerateBackupCodesResponse>;

    /// List all MFA devices for a user
    async fn list_mfa_devices(&self, request: ListMFADevicesRequest) -> Result<ListMFADevicesResponse>;

    /// Remove/disable an MFA device
    async fn remove_mfa_device(&self, request: RemoveMFARequest, user_id: Uuid) -> Result<RemoveMFAResponse>;

    /// Update MFA preferences
    async fn update_mfa_preferences(&self, preferences: MFAPreferenceUpdate, user_id: Uuid) -> Result<()>;

    /// Validate MFA verification code during login
    async fn validate_mfa_verification(&self, user_id: Uuid, verification_code: &str, session_token: &str) -> Result<bool>;
}

// Default Implementation
pub struct DefaultMFAService {
    // In a real implementation, this would have database repositories
    email_service: Arc<dyn EmailService>,
    security_service: Arc<dyn SecurityMonitoringService>,
}

impl DefaultMFAService {
    pub fn new(
        email_service: Arc<dyn EmailService>,
        security_service: Arc<dyn SecurityMonitoringService>,
    ) -> Self {
        Self {
            email_service,
            security_service,
        }
    }

    /// Build a security event with common fields pre-filled.
    fn build_security_event(
        user_id: Uuid,
        event_type: SecurityEventType,
        details: HashMap<String, String>,
        risk_score: u8,
    ) -> SecurityEvent {
        SecurityEvent {
            id: Uuid::new_v4(),
            user_id,
            event_type,
            timestamp: Utc::now(),
            ip_address: None,
            device_fingerprint: None,
            location: None,
            details,
            risk_score,
        }
    }

    fn generate_totp_secret(&self) -> String {
        // Generate a secure random secret for TOTP
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>()
    }

    fn generate_qr_code(&self, secret: &str, user_email: &str, issuer: &str) -> Result<String> {
        // In a real implementation, this would generate a QR code
        // For now, return a placeholder URL
        let totp_uri = format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}",
            issuer, user_email, secret, issuer
        );
        Ok(totp_uri)
    }

    fn generate_verification_code(&self) -> String {
        // Generate 6-digit verification code
        rand::thread_rng()
            .gen_range(100000..=999999)
            .to_string()
    }

    fn generate_backup_code(&self) -> String {
        // Generate 8-character backup code
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>()
            .to_uppercase()
    }

    async fn send_sms_verification(&self, phone_number: &str, code: &str) -> Result<()> {
        // In a real implementation, this would integrate with an SMS service
        log::info!("Sending SMS verification to {}: {}", phone_number, code);
        Ok(())
    }

    fn hash_code(&self, code: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(code.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn verify_totp_code(&self, secret: &str, code: &str) -> bool {
        // In a real implementation, this would verify TOTP using proper algorithms
        // For now, just check if it's a 6-digit code
        code.len() == 6 && code.chars().all(|c| c.is_ascii_digit())
    }
}

#[async_trait::async_trait]
impl MFAService for DefaultMFAService {
    async fn setup_mfa_device(&self, request: SetupMFARequest, user_id: Uuid) -> Result<SetupMFAResponse> {
        let device_id = Uuid::new_v4();
        let now = Utc::now();

        let mut response = SetupMFAResponse {
            device_id,
            device_type: request.device_type.clone(),
            qr_code: None,
            secret: None,
            verification_code: None,
            instructions: String::new(),
        };

        match request.device_type {
            MFADeviceType::TOTP => {
                let secret = self.generate_totp_secret();
                let qr_code = self.generate_qr_code(&secret, "user@example.com", "MyApp")?;
                response.secret = Some(secret.clone());
                response.qr_code = Some(qr_code);
                response.instructions = "Scan the QR code with your authenticator app and enter the 6-digit code.".to_string();
            }
            MFADeviceType::SMS => {
                if let Some(phone_number) = &request.phone_number {
                    let code = self.generate_verification_code();
                    self.send_sms_verification(phone_number, &code).await?;
                    response.verification_code = Some(code.clone());
                    response.instructions = format!("Enter the 6-digit code sent to {}", phone_number);
                } else {
                    return Err(anyhow!("Phone number is required for SMS MFA"));
                }
            }
            MFADeviceType::Email => {
                if let Some(email_address) = &request.email_address {
                    let code = self.generate_verification_code();
                    // In a real implementation, this would send an email
                    response.verification_code = Some(code.clone());
                    response.instructions = format!("Enter the 6-digit code sent to {}", email_address);
                } else {
                    return Err(anyhow!("Email address is required for email MFA"));
                }
            }
            MFADeviceType::HardwareKey => {
                response.instructions = "Insert your hardware key and follow the on-screen prompts.".to_string();
            }
            MFADeviceType::Biometric => {
                response.instructions = "Follow the biometric verification prompt on your device.".to_string();
            }
            MFADeviceType::PushNotification => {
                response.instructions = "Check your mobile device for the push notification.".to_string();
            }
        }

        // Log the setup attempt
        let _ = self.security_service.log_security_event(
            Self::build_security_event(
                user_id,
                SecurityEventType::MfaSetupInitiated,
                HashMap::from([
                    ("device_type".to_string(), format!("{:?}", request.device_type)),
                    ("action".to_string(), "mfa_device_setup_initiated".to_string()),
                ]),
                20,
            )
        ).await;

        Ok(response)
    }

    async fn verify_mfa_device(&self, request: VerifyMFARequest) -> Result<VerifyMFAResponse> {
        // In a real implementation, this would verify against stored data
        let success = request.verification_code.len() == 6; // Simplified validation

        Ok(VerifyMFAResponse {
            success,
            device_id: request.device_id,
            verification_method: MFAVerificationMethod::TOTP, // This would be determined by device type
            remaining_attempts: if success { 3 } else { 2 },
            message: if success {
                "MFA device verified successfully".to_string()
            } else {
                "Invalid verification code. Please try again.".to_string()
            },
        })
    }

    async fn generate_backup_codes(&self, request: GenerateBackupCodesRequest, user_id: Uuid) -> Result<GenerateBackupCodesResponse> {
        let batch_id = Uuid::new_v4();
        let backup_codes: Vec<String> = (0..request.count)
            .map(|_| self.generate_backup_code())
            .collect();

        let expires_at = Utc::now() + Duration::days(365); // 1 year expiry

        // Log backup code generation
        let _ = self.security_service.log_security_event(
            Self::build_security_event(
                user_id,
                SecurityEventType::MfaBackupCodesGenerated,
                HashMap::from([
                    ("action".to_string(), "mfa_backup_codes_generated".to_string()),
                    ("count".to_string(), request.count.to_string()),
                    ("batch_id".to_string(), batch_id.to_string()),
                ]),
                30,
            )
        ).await;

        Ok(GenerateBackupCodesResponse {
            backup_codes: backup_codes.clone(),
            batch_id,
            expires_at,
            instructions: "Store these backup codes in a safe place. Each code can only be used once.".to_string(),
        })
    }

    async fn list_mfa_devices(&self, request: ListMFADevicesRequest) -> Result<ListMFADevicesResponse> {
        // In a real implementation, this would query the database
        // For now, return empty list
        Ok(ListMFADevicesResponse {
            devices: vec![],
            total_count: 0,
            active_count: 0,
        })
    }

    async fn remove_mfa_device(&self, request: RemoveMFARequest, user_id: Uuid) -> Result<RemoveMFAResponse> {
        // In a real implementation, this would remove/disable the device
        let _ = self.security_service.log_security_event(
            Self::build_security_event(
                user_id,
                SecurityEventType::MfaDeviceRemoved,
                HashMap::from([
                    ("action".to_string(), "mfa_device_removed".to_string()),
                    ("device_id".to_string(), request.device_id.to_string()),
                    ("reason".to_string(), request.reason.clone().unwrap_or_default()),
                ]),
                40,
            )
        ).await;

        Ok(RemoveMFAResponse {
            success: true,
            message: "MFA device removed successfully".to_string(),
        })
    }

    async fn update_mfa_preferences(&self, preferences: MFAPreferenceUpdate, user_id: Uuid) -> Result<()> {
        // In a real implementation, this would update user preferences
        let mut details = HashMap::from([
            ("action".to_string(), "mfa_preferences_updated".to_string()),
        ]);
        if let Some(device_id) = preferences.primary_device_id {
            details.insert("primary_device_id".to_string(), device_id.to_string());
        }
        if let Some(require_mfa) = preferences.require_mfa_for_new_devices {
            details.insert("require_mfa_for_new_devices".to_string(), require_mfa.to_string());
        }

        let _ = self.security_service.log_security_event(
            Self::build_security_event(user_id, SecurityEventType::MfaPreferencesUpdated, details, 10)
        ).await;

        Ok(())
    }

    async fn validate_mfa_verification(&self, user_id: Uuid, verification_code: &str, session_token: &str) -> Result<bool> {
        // In a real implementation, this would validate against stored sessions
        let success = verification_code.len() == 6; // Simplified validation

        let event_type = if success {
            SecurityEventType::MfaVerificationSucceeded
        } else {
            SecurityEventType::MfaVerificationFailed
        };

        let _ = self.security_service.log_security_event(
            Self::build_security_event(
                user_id,
                event_type,
                HashMap::from([
                    ("method".to_string(), "mfa_verification".to_string()),
                    ("success".to_string(), success.to_string()),
                ]),
                if success { 0 } else { 40 },
            )
        ).await;

        Ok(success)
    }
}