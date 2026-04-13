use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::GenerationMethod;
use super::CompromiseDetectionMethod;
use super::AuditMetadata;

/// Strongly-typed ID for MFABackupCode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MFABackupCodeId(pub Uuid);

impl MFABackupCodeId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for MFABackupCodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for MFABackupCodeId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for MFABackupCodeId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<MFABackupCodeId> for Uuid {
    fn from(id: MFABackupCodeId) -> Self { id.0 }
}

impl AsRef<Uuid> for MFABackupCodeId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for MFABackupCodeId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MFABackupCode {
    pub id: Uuid,
    pub user_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Uuid>,
    pub batch_id: Uuid,
    pub code: String,
    pub code_hash: String,
    pub code_index: i32,
    pub generated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_by: Option<Uuid>,
    pub generation_method: GenerationMethod,
    pub generation_ip: String,
    pub algorithm_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_attempt: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_since_generation: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_risk_score: Option<i32>,
    pub is_consumed: bool,
    pub consumption_attempts: i32,
    pub successful_verification: bool,
    pub is_revoked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_by: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,
    pub compromised: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compromised_detected_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compromise_detection_method: Option<CompromiseDetectionMethod>,
    pub expires_at: DateTime<Utc>,
    pub created_with_expiry: bool,
    pub grace_period_hours: i32,
    pub auto_renew_enabled: bool,
    pub renewal_reminder_sent: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_before_expiry: Option<DateTime<Utc>>,
    pub creation_notification_sent: bool,
    pub expiry_warning_sent: bool,
    pub usage_notification_sent: bool,
    pub security_alert_sent: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_notification_at: Option<DateTime<Utc>>,
    pub gdpr_compliant: bool,
    pub audit_log_required: bool,
    pub data_retention_days: i32,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl MFABackupCode {
    /// Create a builder for MFABackupCode
    pub fn builder() -> MFABackupCodeBuilder {
        MFABackupCodeBuilder::default()
    }

    /// Create a new MFABackupCode with required fields
    pub fn new(user_id: Uuid, batch_id: Uuid, code: String, code_hash: String, code_index: i32, generated_at: DateTime<Utc>, generation_method: GenerationMethod, generation_ip: String, algorithm_version: String, is_consumed: bool, consumption_attempts: i32, successful_verification: bool, is_revoked: bool, compromised: bool, expires_at: DateTime<Utc>, created_with_expiry: bool, grace_period_hours: i32, auto_renew_enabled: bool, renewal_reminder_sent: bool, creation_notification_sent: bool, expiry_warning_sent: bool, usage_notification_sent: bool, security_alert_sent: bool, gdpr_compliant: bool, audit_log_required: bool, data_retention_days: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            device_id: None,
            batch_id,
            code,
            code_hash,
            code_index,
            generated_at,
            generated_by: None,
            generation_method,
            generation_ip,
            algorithm_version,
            used_at: None,
            used_ip: None,
            used_user_agent: None,
            session_id: None,
            verification_attempt: None,
            time_since_generation: None,
            usage_risk_score: None,
            is_consumed,
            consumption_attempts,
            successful_verification,
            is_revoked,
            revoked_at: None,
            revoked_by: None,
            revocation_reason: None,
            compromised,
            compromised_detected_at: None,
            compromise_detection_method: None,
            expires_at,
            created_with_expiry,
            grace_period_hours,
            auto_renew_enabled,
            renewal_reminder_sent,
            last_access_before_expiry: None,
            creation_notification_sent,
            expiry_warning_sent,
            usage_notification_sent,
            security_alert_sent,
            last_notification_at: None,
            gdpr_compliant,
            audit_log_required,
            data_retention_days,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> MFABackupCodeId {
        MFABackupCodeId(self.id)
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


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the device_id field (chainable)
    pub fn with_device_id(mut self, value: Uuid) -> Self {
        self.device_id = Some(value);
        self
    }

    /// Set the generated_by field (chainable)
    pub fn with_generated_by(mut self, value: Uuid) -> Self {
        self.generated_by = Some(value);
        self
    }

    /// Set the used_at field (chainable)
    pub fn with_used_at(mut self, value: DateTime<Utc>) -> Self {
        self.used_at = Some(value);
        self
    }

    /// Set the used_ip field (chainable)
    pub fn with_used_ip(mut self, value: String) -> Self {
        self.used_ip = Some(value);
        self
    }

    /// Set the used_user_agent field (chainable)
    pub fn with_used_user_agent(mut self, value: String) -> Self {
        self.used_user_agent = Some(value);
        self
    }

    /// Set the session_id field (chainable)
    pub fn with_session_id(mut self, value: Uuid) -> Self {
        self.session_id = Some(value);
        self
    }

    /// Set the verification_attempt field (chainable)
    pub fn with_verification_attempt(mut self, value: i32) -> Self {
        self.verification_attempt = Some(value);
        self
    }

    /// Set the time_since_generation field (chainable)
    pub fn with_time_since_generation(mut self, value: i32) -> Self {
        self.time_since_generation = Some(value);
        self
    }

    /// Set the usage_risk_score field (chainable)
    pub fn with_usage_risk_score(mut self, value: i32) -> Self {
        self.usage_risk_score = Some(value);
        self
    }

    /// Set the revoked_at field (chainable)
    pub fn with_revoked_at(mut self, value: DateTime<Utc>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    /// Set the revoked_by field (chainable)
    pub fn with_revoked_by(mut self, value: Uuid) -> Self {
        self.revoked_by = Some(value);
        self
    }

    /// Set the revocation_reason field (chainable)
    pub fn with_revocation_reason(mut self, value: String) -> Self {
        self.revocation_reason = Some(value);
        self
    }

    /// Set the compromised_detected_at field (chainable)
    pub fn with_compromised_detected_at(mut self, value: DateTime<Utc>) -> Self {
        self.compromised_detected_at = Some(value);
        self
    }

    /// Set the compromise_detection_method field (chainable)
    pub fn with_compromise_detection_method(mut self, value: CompromiseDetectionMethod) -> Self {
        self.compromise_detection_method = Some(value);
        self
    }

    /// Set the last_access_before_expiry field (chainable)
    pub fn with_last_access_before_expiry(mut self, value: DateTime<Utc>) -> Self {
        self.last_access_before_expiry = Some(value);
        self
    }

    /// Set the last_notification_at field (chainable)
    pub fn with_last_notification_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_notification_at = Some(value);
        self
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
                "device_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_id = v; }
                }
                "batch_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.batch_id = v; }
                }
                "code" => {
                    if let Ok(v) = serde_json::from_value(value) { self.code = v; }
                }
                "code_hash" => {
                    if let Ok(v) = serde_json::from_value(value) { self.code_hash = v; }
                }
                "code_index" => {
                    if let Ok(v) = serde_json::from_value(value) { self.code_index = v; }
                }
                "generated_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.generated_at = v; }
                }
                "generated_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.generated_by = v; }
                }
                "generation_method" => {
                    if let Ok(v) = serde_json::from_value(value) { self.generation_method = v; }
                }
                "generation_ip" => {
                    if let Ok(v) = serde_json::from_value(value) { self.generation_ip = v; }
                }
                "algorithm_version" => {
                    if let Ok(v) = serde_json::from_value(value) { self.algorithm_version = v; }
                }
                "used_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.used_at = v; }
                }
                "used_ip" => {
                    if let Ok(v) = serde_json::from_value(value) { self.used_ip = v; }
                }
                "used_user_agent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.used_user_agent = v; }
                }
                "session_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.session_id = v; }
                }
                "verification_attempt" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_attempt = v; }
                }
                "time_since_generation" => {
                    if let Ok(v) = serde_json::from_value(value) { self.time_since_generation = v; }
                }
                "usage_risk_score" => {
                    if let Ok(v) = serde_json::from_value(value) { self.usage_risk_score = v; }
                }
                "is_consumed" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_consumed = v; }
                }
                "consumption_attempts" => {
                    if let Ok(v) = serde_json::from_value(value) { self.consumption_attempts = v; }
                }
                "successful_verification" => {
                    if let Ok(v) = serde_json::from_value(value) { self.successful_verification = v; }
                }
                "is_revoked" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_revoked = v; }
                }
                "revoked_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.revoked_at = v; }
                }
                "revoked_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.revoked_by = v; }
                }
                "revocation_reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.revocation_reason = v; }
                }
                "compromised" => {
                    if let Ok(v) = serde_json::from_value(value) { self.compromised = v; }
                }
                "compromised_detected_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.compromised_detected_at = v; }
                }
                "compromise_detection_method" => {
                    if let Ok(v) = serde_json::from_value(value) { self.compromise_detection_method = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "created_with_expiry" => {
                    if let Ok(v) = serde_json::from_value(value) { self.created_with_expiry = v; }
                }
                "grace_period_hours" => {
                    if let Ok(v) = serde_json::from_value(value) { self.grace_period_hours = v; }
                }
                "auto_renew_enabled" => {
                    if let Ok(v) = serde_json::from_value(value) { self.auto_renew_enabled = v; }
                }
                "renewal_reminder_sent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.renewal_reminder_sent = v; }
                }
                "last_access_before_expiry" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_access_before_expiry = v; }
                }
                "creation_notification_sent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.creation_notification_sent = v; }
                }
                "expiry_warning_sent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expiry_warning_sent = v; }
                }
                "usage_notification_sent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.usage_notification_sent = v; }
                }
                "security_alert_sent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.security_alert_sent = v; }
                }
                "last_notification_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_notification_at = v; }
                }
                "gdpr_compliant" => {
                    if let Ok(v) = serde_json::from_value(value) { self.gdpr_compliant = v; }
                }
                "audit_log_required" => {
                    if let Ok(v) = serde_json::from_value(value) { self.audit_log_required = v; }
                }
                "data_retention_days" => {
                    if let Ok(v) = serde_json::from_value(value) { self.data_retention_days = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for MFABackupCode {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "MFABackupCode"
    }
}

impl backbone_core::PersistentEntity for MFABackupCode {
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

impl backbone_orm::EntityRepoMeta for MFABackupCode {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("device_id".to_string(), "uuid".to_string());
        m.insert("batch_id".to_string(), "uuid".to_string());
        m.insert("session_id".to_string(), "uuid".to_string());
        m.insert("generation_method".to_string(), "generation_method".to_string());
        m.insert("compromise_detection_method".to_string(), "compromise_detection_method".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["code", "code_hash", "generation_ip", "algorithm_version"]
    }
}

/// Builder for MFABackupCode entity
///
/// Provides a fluent API for constructing MFABackupCode instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct MFABackupCodeBuilder {
    user_id: Option<Uuid>,
    device_id: Option<Uuid>,
    batch_id: Option<Uuid>,
    code: Option<String>,
    code_hash: Option<String>,
    code_index: Option<i32>,
    generated_at: Option<DateTime<Utc>>,
    generated_by: Option<Uuid>,
    generation_method: Option<GenerationMethod>,
    generation_ip: Option<String>,
    algorithm_version: Option<String>,
    used_at: Option<DateTime<Utc>>,
    used_ip: Option<String>,
    used_user_agent: Option<String>,
    session_id: Option<Uuid>,
    verification_attempt: Option<i32>,
    time_since_generation: Option<i32>,
    usage_risk_score: Option<i32>,
    is_consumed: Option<bool>,
    consumption_attempts: Option<i32>,
    successful_verification: Option<bool>,
    is_revoked: Option<bool>,
    revoked_at: Option<DateTime<Utc>>,
    revoked_by: Option<Uuid>,
    revocation_reason: Option<String>,
    compromised: Option<bool>,
    compromised_detected_at: Option<DateTime<Utc>>,
    compromise_detection_method: Option<CompromiseDetectionMethod>,
    expires_at: Option<DateTime<Utc>>,
    created_with_expiry: Option<bool>,
    grace_period_hours: Option<i32>,
    auto_renew_enabled: Option<bool>,
    renewal_reminder_sent: Option<bool>,
    last_access_before_expiry: Option<DateTime<Utc>>,
    creation_notification_sent: Option<bool>,
    expiry_warning_sent: Option<bool>,
    usage_notification_sent: Option<bool>,
    security_alert_sent: Option<bool>,
    last_notification_at: Option<DateTime<Utc>>,
    gdpr_compliant: Option<bool>,
    audit_log_required: Option<bool>,
    data_retention_days: Option<i32>,
}

impl MFABackupCodeBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the device_id field (optional)
    pub fn device_id(mut self, value: Uuid) -> Self {
        self.device_id = Some(value);
        self
    }

    /// Set the batch_id field (required)
    pub fn batch_id(mut self, value: Uuid) -> Self {
        self.batch_id = Some(value);
        self
    }

    /// Set the code field (required)
    pub fn code(mut self, value: String) -> Self {
        self.code = Some(value);
        self
    }

    /// Set the code_hash field (required)
    pub fn code_hash(mut self, value: String) -> Self {
        self.code_hash = Some(value);
        self
    }

    /// Set the code_index field (required)
    pub fn code_index(mut self, value: i32) -> Self {
        self.code_index = Some(value);
        self
    }

    /// Set the generated_at field (default: `Default::default()`)
    pub fn generated_at(mut self, value: DateTime<Utc>) -> Self {
        self.generated_at = Some(value);
        self
    }

    /// Set the generated_by field (optional)
    pub fn generated_by(mut self, value: Uuid) -> Self {
        self.generated_by = Some(value);
        self
    }

    /// Set the generation_method field (default: `GenerationMethod::default()`)
    pub fn generation_method(mut self, value: GenerationMethod) -> Self {
        self.generation_method = Some(value);
        self
    }

    /// Set the generation_ip field (required)
    pub fn generation_ip(mut self, value: String) -> Self {
        self.generation_ip = Some(value);
        self
    }

    /// Set the algorithm_version field (default: `Default::default()`)
    pub fn algorithm_version(mut self, value: String) -> Self {
        self.algorithm_version = Some(value);
        self
    }

    /// Set the used_at field (optional)
    pub fn used_at(mut self, value: DateTime<Utc>) -> Self {
        self.used_at = Some(value);
        self
    }

    /// Set the used_ip field (optional)
    pub fn used_ip(mut self, value: String) -> Self {
        self.used_ip = Some(value);
        self
    }

    /// Set the used_user_agent field (optional)
    pub fn used_user_agent(mut self, value: String) -> Self {
        self.used_user_agent = Some(value);
        self
    }

    /// Set the session_id field (optional)
    pub fn session_id(mut self, value: Uuid) -> Self {
        self.session_id = Some(value);
        self
    }

    /// Set the verification_attempt field (optional)
    pub fn verification_attempt(mut self, value: i32) -> Self {
        self.verification_attempt = Some(value);
        self
    }

    /// Set the time_since_generation field (optional)
    pub fn time_since_generation(mut self, value: i32) -> Self {
        self.time_since_generation = Some(value);
        self
    }

    /// Set the usage_risk_score field (optional)
    pub fn usage_risk_score(mut self, value: i32) -> Self {
        self.usage_risk_score = Some(value);
        self
    }

    /// Set the is_consumed field (default: `false`)
    pub fn is_consumed(mut self, value: bool) -> Self {
        self.is_consumed = Some(value);
        self
    }

    /// Set the consumption_attempts field (default: `0`)
    pub fn consumption_attempts(mut self, value: i32) -> Self {
        self.consumption_attempts = Some(value);
        self
    }

    /// Set the successful_verification field (default: `false`)
    pub fn successful_verification(mut self, value: bool) -> Self {
        self.successful_verification = Some(value);
        self
    }

    /// Set the is_revoked field (default: `false`)
    pub fn is_revoked(mut self, value: bool) -> Self {
        self.is_revoked = Some(value);
        self
    }

    /// Set the revoked_at field (optional)
    pub fn revoked_at(mut self, value: DateTime<Utc>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    /// Set the revoked_by field (optional)
    pub fn revoked_by(mut self, value: Uuid) -> Self {
        self.revoked_by = Some(value);
        self
    }

    /// Set the revocation_reason field (optional)
    pub fn revocation_reason(mut self, value: String) -> Self {
        self.revocation_reason = Some(value);
        self
    }

    /// Set the compromised field (default: `false`)
    pub fn compromised(mut self, value: bool) -> Self {
        self.compromised = Some(value);
        self
    }

    /// Set the compromised_detected_at field (optional)
    pub fn compromised_detected_at(mut self, value: DateTime<Utc>) -> Self {
        self.compromised_detected_at = Some(value);
        self
    }

    /// Set the compromise_detection_method field (optional)
    pub fn compromise_detection_method(mut self, value: CompromiseDetectionMethod) -> Self {
        self.compromise_detection_method = Some(value);
        self
    }

    /// Set the expires_at field (required)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the created_with_expiry field (default: `true`)
    pub fn created_with_expiry(mut self, value: bool) -> Self {
        self.created_with_expiry = Some(value);
        self
    }

    /// Set the grace_period_hours field (default: `0`)
    pub fn grace_period_hours(mut self, value: i32) -> Self {
        self.grace_period_hours = Some(value);
        self
    }

    /// Set the auto_renew_enabled field (default: `false`)
    pub fn auto_renew_enabled(mut self, value: bool) -> Self {
        self.auto_renew_enabled = Some(value);
        self
    }

    /// Set the renewal_reminder_sent field (default: `false`)
    pub fn renewal_reminder_sent(mut self, value: bool) -> Self {
        self.renewal_reminder_sent = Some(value);
        self
    }

    /// Set the last_access_before_expiry field (optional)
    pub fn last_access_before_expiry(mut self, value: DateTime<Utc>) -> Self {
        self.last_access_before_expiry = Some(value);
        self
    }

    /// Set the creation_notification_sent field (default: `false`)
    pub fn creation_notification_sent(mut self, value: bool) -> Self {
        self.creation_notification_sent = Some(value);
        self
    }

    /// Set the expiry_warning_sent field (default: `false`)
    pub fn expiry_warning_sent(mut self, value: bool) -> Self {
        self.expiry_warning_sent = Some(value);
        self
    }

    /// Set the usage_notification_sent field (default: `false`)
    pub fn usage_notification_sent(mut self, value: bool) -> Self {
        self.usage_notification_sent = Some(value);
        self
    }

    /// Set the security_alert_sent field (default: `false`)
    pub fn security_alert_sent(mut self, value: bool) -> Self {
        self.security_alert_sent = Some(value);
        self
    }

    /// Set the last_notification_at field (optional)
    pub fn last_notification_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_notification_at = Some(value);
        self
    }

    /// Set the gdpr_compliant field (default: `true`)
    pub fn gdpr_compliant(mut self, value: bool) -> Self {
        self.gdpr_compliant = Some(value);
        self
    }

    /// Set the audit_log_required field (default: `true`)
    pub fn audit_log_required(mut self, value: bool) -> Self {
        self.audit_log_required = Some(value);
        self
    }

    /// Set the data_retention_days field (default: `365`)
    pub fn data_retention_days(mut self, value: i32) -> Self {
        self.data_retention_days = Some(value);
        self
    }

    /// Build the MFABackupCode entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<MFABackupCode, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let batch_id = self.batch_id.ok_or_else(|| "batch_id is required".to_string())?;
        let code = self.code.ok_or_else(|| "code is required".to_string())?;
        let code_hash = self.code_hash.ok_or_else(|| "code_hash is required".to_string())?;
        let code_index = self.code_index.ok_or_else(|| "code_index is required".to_string())?;
        let generation_ip = self.generation_ip.ok_or_else(|| "generation_ip is required".to_string())?;
        let expires_at = self.expires_at.ok_or_else(|| "expires_at is required".to_string())?;

        Ok(MFABackupCode {
            id: Uuid::new_v4(),
            user_id,
            device_id: self.device_id,
            batch_id,
            code,
            code_hash,
            code_index,
            generated_at: self.generated_at.unwrap_or(Default::default()),
            generated_by: self.generated_by,
            generation_method: self.generation_method.unwrap_or(GenerationMethod::default()),
            generation_ip,
            algorithm_version: self.algorithm_version.unwrap_or(Default::default()),
            used_at: self.used_at,
            used_ip: self.used_ip,
            used_user_agent: self.used_user_agent,
            session_id: self.session_id,
            verification_attempt: self.verification_attempt,
            time_since_generation: self.time_since_generation,
            usage_risk_score: self.usage_risk_score,
            is_consumed: self.is_consumed.unwrap_or(false),
            consumption_attempts: self.consumption_attempts.unwrap_or(0),
            successful_verification: self.successful_verification.unwrap_or(false),
            is_revoked: self.is_revoked.unwrap_or(false),
            revoked_at: self.revoked_at,
            revoked_by: self.revoked_by,
            revocation_reason: self.revocation_reason,
            compromised: self.compromised.unwrap_or(false),
            compromised_detected_at: self.compromised_detected_at,
            compromise_detection_method: self.compromise_detection_method,
            expires_at,
            created_with_expiry: self.created_with_expiry.unwrap_or(true),
            grace_period_hours: self.grace_period_hours.unwrap_or(0),
            auto_renew_enabled: self.auto_renew_enabled.unwrap_or(false),
            renewal_reminder_sent: self.renewal_reminder_sent.unwrap_or(false),
            last_access_before_expiry: self.last_access_before_expiry,
            creation_notification_sent: self.creation_notification_sent.unwrap_or(false),
            expiry_warning_sent: self.expiry_warning_sent.unwrap_or(false),
            usage_notification_sent: self.usage_notification_sent.unwrap_or(false),
            security_alert_sent: self.security_alert_sent.unwrap_or(false),
            last_notification_at: self.last_notification_at,
            gdpr_compliant: self.gdpr_compliant.unwrap_or(true),
            audit_log_required: self.audit_log_required.unwrap_or(true),
            data_retention_days: self.data_retention_days.unwrap_or(365),
            metadata: AuditMetadata::default(),
        })
    }
}
