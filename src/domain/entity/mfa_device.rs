use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::MFADeviceType;
use super::EnrollmentMethod;
use super::MFADeviceStatus;
use super::AuditMetadata;

use crate::domain::state_machine::{MFADeviceStateMachine, MFADeviceState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for MFADevice
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MFADeviceId(pub Uuid);

impl MFADeviceId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for MFADeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for MFADeviceId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for MFADeviceId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<MFADeviceId> for Uuid {
    fn from(id: MFADeviceId) -> Self { id.0 }
}

impl AsRef<Uuid> for MFADeviceId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for MFADeviceId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MFADevice {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_type: MFADeviceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totp_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    pub is_primary: bool,
    pub is_backup: bool,
    pub requires_verification: bool,
    pub auto_approval_enabled: bool,
    pub trusted_duration_hours: i32,
    pub enrolled_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrolled_by: Option<Uuid>,
    pub enrollment_method: EnrollmentMethod,
    pub enrollment_ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_user_agent: Option<String>,
    pub verification_attempts: i32,
    pub backup_codes_generated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<DateTime<Utc>>,
    pub usage_count: i32,
    pub successful_verifications: i32,
    pub failed_verifications: i32,
    pub is_locked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_reason: Option<String>,
    pub risk_score: i32,
    pub(crate) is_active: bool,
    pub status: MFADeviceStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_codes_data: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl MFADevice {
    /// Create a builder for MFADevice
    pub fn builder() -> MFADeviceBuilder {
        MFADeviceBuilder::default()
    }

    /// Create a new MFADevice with required fields
    pub fn new(user_id: Uuid, device_type: MFADeviceType, is_primary: bool, is_backup: bool, requires_verification: bool, auto_approval_enabled: bool, trusted_duration_hours: i32, enrolled_at: DateTime<Utc>, enrollment_method: EnrollmentMethod, enrollment_ip: String, verification_attempts: i32, backup_codes_generated: bool, usage_count: i32, successful_verifications: i32, failed_verifications: i32, is_locked: bool, risk_score: i32, is_active: bool, status: MFADeviceStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            device_type,
            device_name: None,
            phone_number: None,
            email_address: None,
            totp_secret: None,
            secret: None,
            hardware_key_id: None,
            push_token: None,
            device_fingerprint: None,
            manufacturer: None,
            model: None,
            operating_system: None,
            app_version: None,
            is_primary,
            is_backup,
            requires_verification,
            auto_approval_enabled,
            trusted_duration_hours,
            enrolled_at,
            enrolled_by: None,
            enrollment_method,
            enrollment_ip,
            enrollment_user_agent: None,
            verification_attempts,
            backup_codes_generated,
            verified_at: None,
            last_used: None,
            last_used_at: None,
            usage_count,
            successful_verifications,
            failed_verifications,
            is_locked,
            locked_at: None,
            locked_until: None,
            lock_reason: None,
            risk_score,
            is_active,
            status,
            backup_codes_data: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> MFADeviceId {
        MFADeviceId(self.id)
    }

    /// Get when this entity was created
    pub fn created_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.created_at.as_ref()
    }

    /// Get when this entity was last updated
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.updated_at.as_ref()
    }

    /// Check if this entity is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.metadata.deleted_at.is_some()
    }

    /// Check if this entity is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.metadata.deleted_at.is_none()
    }

    /// Get when this entity was deleted
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.deleted_at.as_ref()
    }

    /// Get who created this entity
    pub fn created_by(&self) -> Option<&Uuid> {
        self.metadata.created_by.as_ref()
    }

    /// Get who last updated this entity
    pub fn updated_by(&self) -> Option<&Uuid> {
        self.metadata.updated_by.as_ref()
    }

    /// Get who deleted this entity
    pub fn deleted_by(&self) -> Option<&Uuid> {
        self.metadata.deleted_by.as_ref()
    }

    /// Get the current status
    pub fn status(&self) -> &MFADeviceStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the device_name field (chainable)
    pub fn with_device_name(mut self, value: String) -> Self {
        self.device_name = Some(value);
        self
    }

    /// Set the phone_number field (chainable)
    pub fn with_phone_number(mut self, value: String) -> Self {
        self.phone_number = Some(value);
        self
    }

    /// Set the email_address field (chainable)
    pub fn with_email_address(mut self, value: String) -> Self {
        self.email_address = Some(value);
        self
    }

    /// Set the totp_secret field (chainable)
    pub fn with_totp_secret(mut self, value: String) -> Self {
        self.totp_secret = Some(value);
        self
    }

    /// Set the secret field (chainable)
    pub fn with_secret(mut self, value: String) -> Self {
        self.secret = Some(value);
        self
    }

    /// Set the hardware_key_id field (chainable)
    pub fn with_hardware_key_id(mut self, value: String) -> Self {
        self.hardware_key_id = Some(value);
        self
    }

    /// Set the push_token field (chainable)
    pub fn with_push_token(mut self, value: String) -> Self {
        self.push_token = Some(value);
        self
    }

    /// Set the device_fingerprint field (chainable)
    pub fn with_device_fingerprint(mut self, value: String) -> Self {
        self.device_fingerprint = Some(value);
        self
    }

    /// Set the manufacturer field (chainable)
    pub fn with_manufacturer(mut self, value: String) -> Self {
        self.manufacturer = Some(value);
        self
    }

    /// Set the model field (chainable)
    pub fn with_model(mut self, value: String) -> Self {
        self.model = Some(value);
        self
    }

    /// Set the operating_system field (chainable)
    pub fn with_operating_system(mut self, value: String) -> Self {
        self.operating_system = Some(value);
        self
    }

    /// Set the app_version field (chainable)
    pub fn with_app_version(mut self, value: String) -> Self {
        self.app_version = Some(value);
        self
    }

    /// Set the enrolled_by field (chainable)
    pub fn with_enrolled_by(mut self, value: Uuid) -> Self {
        self.enrolled_by = Some(value);
        self
    }

    /// Set the enrollment_user_agent field (chainable)
    pub fn with_enrollment_user_agent(mut self, value: String) -> Self {
        self.enrollment_user_agent = Some(value);
        self
    }

    /// Set the verified_at field (chainable)
    pub fn with_verified_at(mut self, value: DateTime<Utc>) -> Self {
        self.verified_at = Some(value);
        self
    }

    /// Set the last_used field (chainable)
    pub fn with_last_used(mut self, value: DateTime<Utc>) -> Self {
        self.last_used = Some(value);
        self
    }

    /// Set the last_used_at field (chainable)
    pub fn with_last_used_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_used_at = Some(value);
        self
    }

    /// Set the locked_at field (chainable)
    pub fn with_locked_at(mut self, value: DateTime<Utc>) -> Self {
        self.locked_at = Some(value);
        self
    }

    /// Set the locked_until field (chainable)
    pub fn with_locked_until(mut self, value: DateTime<Utc>) -> Self {
        self.locked_until = Some(value);
        self
    }

    /// Set the lock_reason field (chainable)
    pub fn with_lock_reason(mut self, value: String) -> Self {
        self.lock_reason = Some(value);
        self
    }

    /// Set the backup_codes_data field (chainable)
    pub fn with_backup_codes_data(mut self, value: serde_json::Value) -> Self {
        self.backup_codes_data = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the is_active state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.is_active` directly.
    pub fn transition_to(&mut self, new_state: MFADeviceState) -> Result<(), StateMachineError> {
        let current = self.is_active.to_string().parse::<MFADeviceState>()?;
        let mut sm = MFADeviceStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.is_active = new_state.to_string().parse::<bool>()
            .map_err(|e| StateMachineError::InvalidState(e.to_string()))?;
        Ok(())
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_id = v; }
                }
                "device_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_type = v; }
                }
                "device_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_name = v; }
                }
                "phone_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.phone_number = v; }
                }
                "email_address" => {
                    if let Ok(v) = serde_json::from_value(value) { self.email_address = v; }
                }
                "totp_secret" => {
                    if let Ok(v) = serde_json::from_value(value) { self.totp_secret = v; }
                }
                "secret" => {
                    if let Ok(v) = serde_json::from_value(value) { self.secret = v; }
                }
                "hardware_key_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.hardware_key_id = v; }
                }
                "push_token" => {
                    if let Ok(v) = serde_json::from_value(value) { self.push_token = v; }
                }
                "device_fingerprint" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_fingerprint = v; }
                }
                "manufacturer" => {
                    if let Ok(v) = serde_json::from_value(value) { self.manufacturer = v; }
                }
                "model" => {
                    if let Ok(v) = serde_json::from_value(value) { self.model = v; }
                }
                "operating_system" => {
                    if let Ok(v) = serde_json::from_value(value) { self.operating_system = v; }
                }
                "app_version" => {
                    if let Ok(v) = serde_json::from_value(value) { self.app_version = v; }
                }
                "is_primary" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_primary = v; }
                }
                "is_backup" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_backup = v; }
                }
                "requires_verification" => {
                    if let Ok(v) = serde_json::from_value(value) { self.requires_verification = v; }
                }
                "auto_approval_enabled" => {
                    if let Ok(v) = serde_json::from_value(value) { self.auto_approval_enabled = v; }
                }
                "trusted_duration_hours" => {
                    if let Ok(v) = serde_json::from_value(value) { self.trusted_duration_hours = v; }
                }
                "enrolled_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.enrolled_at = v; }
                }
                "enrolled_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.enrolled_by = v; }
                }
                "enrollment_method" => {
                    if let Ok(v) = serde_json::from_value(value) { self.enrollment_method = v; }
                }
                "enrollment_ip" => {
                    if let Ok(v) = serde_json::from_value(value) { self.enrollment_ip = v; }
                }
                "enrollment_user_agent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.enrollment_user_agent = v; }
                }
                "verification_attempts" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_attempts = v; }
                }
                "backup_codes_generated" => {
                    if let Ok(v) = serde_json::from_value(value) { self.backup_codes_generated = v; }
                }
                "verified_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verified_at = v; }
                }
                "last_used" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_used = v; }
                }
                "last_used_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_used_at = v; }
                }
                "usage_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.usage_count = v; }
                }
                "successful_verifications" => {
                    if let Ok(v) = serde_json::from_value(value) { self.successful_verifications = v; }
                }
                "failed_verifications" => {
                    if let Ok(v) = serde_json::from_value(value) { self.failed_verifications = v; }
                }
                "is_locked" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_locked = v; }
                }
                "locked_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.locked_at = v; }
                }
                "locked_until" => {
                    if let Ok(v) = serde_json::from_value(value) { self.locked_until = v; }
                }
                "lock_reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.lock_reason = v; }
                }
                "risk_score" => {
                    if let Ok(v) = serde_json::from_value(value) { self.risk_score = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "backup_codes_data" => {
                    if let Ok(v) = serde_json::from_value(value) { self.backup_codes_data = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for MFADevice {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "MFADevice"
    }
}

impl backbone_core::PersistentEntity for MFADevice {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.created_at
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.created_at = Some(ts);
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.updated_at
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.updated_at = Some(ts);
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.deleted_at
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.metadata.deleted_at = ts;
    }
}

impl backbone_orm::EntityRepoMeta for MFADevice {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("device_type".to_string(), "mfa_device_type".to_string());
        m.insert("enrollment_method".to_string(), "enrollment_method".to_string());
        m.insert("status".to_string(), "mfa_device_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["enrollment_ip"]
    }
}

/// Builder for MFADevice entity
///
/// Provides a fluent API for constructing MFADevice instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct MFADeviceBuilder {
    user_id: Option<Uuid>,
    device_type: Option<MFADeviceType>,
    device_name: Option<String>,
    phone_number: Option<String>,
    email_address: Option<String>,
    totp_secret: Option<String>,
    secret: Option<String>,
    hardware_key_id: Option<String>,
    push_token: Option<String>,
    device_fingerprint: Option<String>,
    manufacturer: Option<String>,
    model: Option<String>,
    operating_system: Option<String>,
    app_version: Option<String>,
    is_primary: Option<bool>,
    is_backup: Option<bool>,
    requires_verification: Option<bool>,
    auto_approval_enabled: Option<bool>,
    trusted_duration_hours: Option<i32>,
    enrolled_at: Option<DateTime<Utc>>,
    enrolled_by: Option<Uuid>,
    enrollment_method: Option<EnrollmentMethod>,
    enrollment_ip: Option<String>,
    enrollment_user_agent: Option<String>,
    verification_attempts: Option<i32>,
    backup_codes_generated: Option<bool>,
    verified_at: Option<DateTime<Utc>>,
    last_used: Option<DateTime<Utc>>,
    last_used_at: Option<DateTime<Utc>>,
    usage_count: Option<i32>,
    successful_verifications: Option<i32>,
    failed_verifications: Option<i32>,
    is_locked: Option<bool>,
    locked_at: Option<DateTime<Utc>>,
    locked_until: Option<DateTime<Utc>>,
    lock_reason: Option<String>,
    risk_score: Option<i32>,
    is_active: Option<bool>,
    status: Option<MFADeviceStatus>,
    backup_codes_data: Option<serde_json::Value>,
}

impl MFADeviceBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the device_type field (required)
    pub fn device_type(mut self, value: MFADeviceType) -> Self {
        self.device_type = Some(value);
        self
    }

    /// Set the device_name field (optional)
    pub fn device_name(mut self, value: String) -> Self {
        self.device_name = Some(value);
        self
    }

    /// Set the phone_number field (optional)
    pub fn phone_number(mut self, value: String) -> Self {
        self.phone_number = Some(value);
        self
    }

    /// Set the email_address field (optional)
    pub fn email_address(mut self, value: String) -> Self {
        self.email_address = Some(value);
        self
    }

    /// Set the totp_secret field (optional)
    pub fn totp_secret(mut self, value: String) -> Self {
        self.totp_secret = Some(value);
        self
    }

    /// Set the secret field (optional)
    pub fn secret(mut self, value: String) -> Self {
        self.secret = Some(value);
        self
    }

    /// Set the hardware_key_id field (optional)
    pub fn hardware_key_id(mut self, value: String) -> Self {
        self.hardware_key_id = Some(value);
        self
    }

    /// Set the push_token field (optional)
    pub fn push_token(mut self, value: String) -> Self {
        self.push_token = Some(value);
        self
    }

    /// Set the device_fingerprint field (optional)
    pub fn device_fingerprint(mut self, value: String) -> Self {
        self.device_fingerprint = Some(value);
        self
    }

    /// Set the manufacturer field (optional)
    pub fn manufacturer(mut self, value: String) -> Self {
        self.manufacturer = Some(value);
        self
    }

    /// Set the model field (optional)
    pub fn model(mut self, value: String) -> Self {
        self.model = Some(value);
        self
    }

    /// Set the operating_system field (optional)
    pub fn operating_system(mut self, value: String) -> Self {
        self.operating_system = Some(value);
        self
    }

    /// Set the app_version field (optional)
    pub fn app_version(mut self, value: String) -> Self {
        self.app_version = Some(value);
        self
    }

    /// Set the is_primary field (default: `false`)
    pub fn is_primary(mut self, value: bool) -> Self {
        self.is_primary = Some(value);
        self
    }

    /// Set the is_backup field (default: `false`)
    pub fn is_backup(mut self, value: bool) -> Self {
        self.is_backup = Some(value);
        self
    }

    /// Set the requires_verification field (default: `true`)
    pub fn requires_verification(mut self, value: bool) -> Self {
        self.requires_verification = Some(value);
        self
    }

    /// Set the auto_approval_enabled field (default: `false`)
    pub fn auto_approval_enabled(mut self, value: bool) -> Self {
        self.auto_approval_enabled = Some(value);
        self
    }

    /// Set the trusted_duration_hours field (default: `24`)
    pub fn trusted_duration_hours(mut self, value: i32) -> Self {
        self.trusted_duration_hours = Some(value);
        self
    }

    /// Set the enrolled_at field (default: `Default::default()`)
    pub fn enrolled_at(mut self, value: DateTime<Utc>) -> Self {
        self.enrolled_at = Some(value);
        self
    }

    /// Set the enrolled_by field (optional)
    pub fn enrolled_by(mut self, value: Uuid) -> Self {
        self.enrolled_by = Some(value);
        self
    }

    /// Set the enrollment_method field (default: `EnrollmentMethod::default()`)
    pub fn enrollment_method(mut self, value: EnrollmentMethod) -> Self {
        self.enrollment_method = Some(value);
        self
    }

    /// Set the enrollment_ip field (required)
    pub fn enrollment_ip(mut self, value: String) -> Self {
        self.enrollment_ip = Some(value);
        self
    }

    /// Set the enrollment_user_agent field (optional)
    pub fn enrollment_user_agent(mut self, value: String) -> Self {
        self.enrollment_user_agent = Some(value);
        self
    }

    /// Set the verification_attempts field (default: `0`)
    pub fn verification_attempts(mut self, value: i32) -> Self {
        self.verification_attempts = Some(value);
        self
    }

    /// Set the backup_codes_generated field (default: `false`)
    pub fn backup_codes_generated(mut self, value: bool) -> Self {
        self.backup_codes_generated = Some(value);
        self
    }

    /// Set the verified_at field (optional)
    pub fn verified_at(mut self, value: DateTime<Utc>) -> Self {
        self.verified_at = Some(value);
        self
    }

    /// Set the last_used field (optional)
    pub fn last_used(mut self, value: DateTime<Utc>) -> Self {
        self.last_used = Some(value);
        self
    }

    /// Set the last_used_at field (optional)
    pub fn last_used_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_used_at = Some(value);
        self
    }

    /// Set the usage_count field (default: `0`)
    pub fn usage_count(mut self, value: i32) -> Self {
        self.usage_count = Some(value);
        self
    }

    /// Set the successful_verifications field (default: `0`)
    pub fn successful_verifications(mut self, value: i32) -> Self {
        self.successful_verifications = Some(value);
        self
    }

    /// Set the failed_verifications field (default: `0`)
    pub fn failed_verifications(mut self, value: i32) -> Self {
        self.failed_verifications = Some(value);
        self
    }

    /// Set the is_locked field (default: `false`)
    pub fn is_locked(mut self, value: bool) -> Self {
        self.is_locked = Some(value);
        self
    }

    /// Set the locked_at field (optional)
    pub fn locked_at(mut self, value: DateTime<Utc>) -> Self {
        self.locked_at = Some(value);
        self
    }

    /// Set the locked_until field (optional)
    pub fn locked_until(mut self, value: DateTime<Utc>) -> Self {
        self.locked_until = Some(value);
        self
    }

    /// Set the lock_reason field (optional)
    pub fn lock_reason(mut self, value: String) -> Self {
        self.lock_reason = Some(value);
        self
    }

    /// Set the risk_score field (default: `0`)
    pub fn risk_score(mut self, value: i32) -> Self {
        self.risk_score = Some(value);
        self
    }

    /// Set the is_active field (default: `false`)
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Set the status field (default: `MFADeviceStatus::default()`)
    pub fn status(mut self, value: MFADeviceStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the backup_codes_data field (optional)
    pub fn backup_codes_data(mut self, value: serde_json::Value) -> Self {
        self.backup_codes_data = Some(value);
        self
    }

    /// Build the MFADevice entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<MFADevice, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let device_type = self.device_type.ok_or_else(|| "device_type is required".to_string())?;
        let enrollment_ip = self.enrollment_ip.ok_or_else(|| "enrollment_ip is required".to_string())?;

        Ok(MFADevice {
            id: Uuid::new_v4(),
            user_id,
            device_type,
            device_name: self.device_name,
            phone_number: self.phone_number,
            email_address: self.email_address,
            totp_secret: self.totp_secret,
            secret: self.secret,
            hardware_key_id: self.hardware_key_id,
            push_token: self.push_token,
            device_fingerprint: self.device_fingerprint,
            manufacturer: self.manufacturer,
            model: self.model,
            operating_system: self.operating_system,
            app_version: self.app_version,
            is_primary: self.is_primary.unwrap_or(false),
            is_backup: self.is_backup.unwrap_or(false),
            requires_verification: self.requires_verification.unwrap_or(true),
            auto_approval_enabled: self.auto_approval_enabled.unwrap_or(false),
            trusted_duration_hours: self.trusted_duration_hours.unwrap_or(24),
            enrolled_at: self.enrolled_at.unwrap_or(Default::default()),
            enrolled_by: self.enrolled_by,
            enrollment_method: self.enrollment_method.unwrap_or(EnrollmentMethod::default()),
            enrollment_ip,
            enrollment_user_agent: self.enrollment_user_agent,
            verification_attempts: self.verification_attempts.unwrap_or(0),
            backup_codes_generated: self.backup_codes_generated.unwrap_or(false),
            verified_at: self.verified_at,
            last_used: self.last_used,
            last_used_at: self.last_used_at,
            usage_count: self.usage_count.unwrap_or(0),
            successful_verifications: self.successful_verifications.unwrap_or(0),
            failed_verifications: self.failed_verifications.unwrap_or(0),
            is_locked: self.is_locked.unwrap_or(false),
            locked_at: self.locked_at,
            locked_until: self.locked_until,
            lock_reason: self.lock_reason,
            risk_score: self.risk_score.unwrap_or(0),
            is_active: self.is_active.unwrap_or(false),
            status: self.status.unwrap_or(MFADeviceStatus::default()),
            backup_codes_data: self.backup_codes_data,
            metadata: AuditMetadata::default(),
        })
    }
}
