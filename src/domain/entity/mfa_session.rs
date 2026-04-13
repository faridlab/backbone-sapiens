use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::PrimaryAuthMethod;
use super::MfaTrustLevel;
use super::MFAVerificationMethod;
use super::DeviceTrustStatus;
use super::NetworkQuality;
use super::MFASessionStatus;
use super::AuditMetadata;

/// Strongly-typed ID for MFASession
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MFASessionId(pub Uuid);

impl MFASessionId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for MFASessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for MFASessionId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for MFASessionId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<MFASessionId> for Uuid {
    fn from(id: MFASessionId) -> Self { id.0 }
}

impl AsRef<Uuid> for MFASessionId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for MFASessionId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MFASession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub session_token: String,
    pub session_hash: String,
    pub primary_authentication_method: PrimaryAuthMethod,
    pub mfa_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enforced_at: Option<DateTime<Utc>>,
    pub risk_score: i32,
    pub trust_level: MfaTrustLevel,
    pub adaptive_authentication_triggered: bool,
    pub additional_factors_required: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_risk_score: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_risk_score: Option<i32>,
    pub location_anomaly: bool,
    pub time_anomaly: bool,
    pub verification_method: MFAVerificationMethod,
    pub verification_initiated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_completed_at: Option<DateTime<Utc>>,
    pub verification_attempts: i32,
    pub max_attempts_allowed: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_time_ms: Option<i32>,
    pub device_trust_status: DeviceTrustStatus,
    pub is_terminated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_reason: Option<String>,
    pub ip_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    pub expires_at: DateTime<Utc>,
    pub max_duration_minutes: i32,
    pub idle_timeout_minutes: i32,
    pub auto_renew_enabled: bool,
    pub last_activity_at: DateTime<Utc>,
    pub extension_count: i32,
    pub initiation_time_ms: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_session_time_ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_performance_score: Option<i32>,
    pub network_quality: NetworkQuality,
    pub audit_log_required: bool,
    pub data_retention_hours: i32,
    pub status: MFASessionStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl MFASession {
    /// Create a builder for MFASession
    pub fn builder() -> MFASessionBuilder {
        MFASessionBuilder::default()
    }

    /// Create a new MFASession with required fields
    pub fn new(user_id: Uuid, device_id: Uuid, session_token: String, session_hash: String, primary_authentication_method: PrimaryAuthMethod, mfa_required: bool, risk_score: i32, trust_level: MfaTrustLevel, adaptive_authentication_triggered: bool, additional_factors_required: i32, location_anomaly: bool, time_anomaly: bool, verification_method: MFAVerificationMethod, verification_initiated_at: DateTime<Utc>, verification_attempts: i32, max_attempts_allowed: i32, device_trust_status: DeviceTrustStatus, is_terminated: bool, ip_address: String, expires_at: DateTime<Utc>, max_duration_minutes: i32, idle_timeout_minutes: i32, auto_renew_enabled: bool, last_activity_at: DateTime<Utc>, extension_count: i32, initiation_time_ms: i32, network_quality: NetworkQuality, audit_log_required: bool, data_retention_hours: i32, status: MFASessionStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            device_id,
            session_token,
            session_hash,
            primary_authentication_method,
            mfa_required,
            mfa_enforced_at: None,
            risk_score,
            trust_level,
            adaptive_authentication_triggered,
            additional_factors_required,
            ip_risk_score: None,
            device_risk_score: None,
            location_anomaly,
            time_anomaly,
            verification_method,
            verification_initiated_at,
            verification_completed_at: None,
            verification_attempts,
            max_attempts_allowed,
            verification_success: None,
            verification_time_ms: None,
            device_trust_status,
            is_terminated,
            terminated_at: None,
            termination_reason: None,
            ip_address,
            user_agent: None,
            device_fingerprint: None,
            country: None,
            region: None,
            city: None,
            latitude: None,
            longitude: None,
            expires_at,
            max_duration_minutes,
            idle_timeout_minutes,
            auto_renew_enabled,
            last_activity_at,
            extension_count,
            initiation_time_ms,
            total_session_time_ms: None,
            device_performance_score: None,
            network_quality,
            audit_log_required,
            data_retention_hours,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> MFASessionId {
        MFASessionId(self.id)
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
    pub fn status(&self) -> &MFASessionStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the mfa_enforced_at field (chainable)
    pub fn with_mfa_enforced_at(mut self, value: DateTime<Utc>) -> Self {
        self.mfa_enforced_at = Some(value);
        self
    }

    /// Set the ip_risk_score field (chainable)
    pub fn with_ip_risk_score(mut self, value: i32) -> Self {
        self.ip_risk_score = Some(value);
        self
    }

    /// Set the device_risk_score field (chainable)
    pub fn with_device_risk_score(mut self, value: i32) -> Self {
        self.device_risk_score = Some(value);
        self
    }

    /// Set the verification_completed_at field (chainable)
    pub fn with_verification_completed_at(mut self, value: DateTime<Utc>) -> Self {
        self.verification_completed_at = Some(value);
        self
    }

    /// Set the verification_success field (chainable)
    pub fn with_verification_success(mut self, value: bool) -> Self {
        self.verification_success = Some(value);
        self
    }

    /// Set the verification_time_ms field (chainable)
    pub fn with_verification_time_ms(mut self, value: i32) -> Self {
        self.verification_time_ms = Some(value);
        self
    }

    /// Set the terminated_at field (chainable)
    pub fn with_terminated_at(mut self, value: DateTime<Utc>) -> Self {
        self.terminated_at = Some(value);
        self
    }

    /// Set the termination_reason field (chainable)
    pub fn with_termination_reason(mut self, value: String) -> Self {
        self.termination_reason = Some(value);
        self
    }

    /// Set the user_agent field (chainable)
    pub fn with_user_agent(mut self, value: String) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Set the device_fingerprint field (chainable)
    pub fn with_device_fingerprint(mut self, value: String) -> Self {
        self.device_fingerprint = Some(value);
        self
    }

    /// Set the country field (chainable)
    pub fn with_country(mut self, value: String) -> Self {
        self.country = Some(value);
        self
    }

    /// Set the region field (chainable)
    pub fn with_region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }

    /// Set the city field (chainable)
    pub fn with_city(mut self, value: String) -> Self {
        self.city = Some(value);
        self
    }

    /// Set the latitude field (chainable)
    pub fn with_latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    /// Set the longitude field (chainable)
    pub fn with_longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Set the total_session_time_ms field (chainable)
    pub fn with_total_session_time_ms(mut self, value: i32) -> Self {
        self.total_session_time_ms = Some(value);
        self
    }

    /// Set the device_performance_score field (chainable)
    pub fn with_device_performance_score(mut self, value: i32) -> Self {
        self.device_performance_score = Some(value);
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
                "session_token" => {
                    if let Ok(v) = serde_json::from_value(value) { self.session_token = v; }
                }
                "session_hash" => {
                    if let Ok(v) = serde_json::from_value(value) { self.session_hash = v; }
                }
                "primary_authentication_method" => {
                    if let Ok(v) = serde_json::from_value(value) { self.primary_authentication_method = v; }
                }
                "mfa_required" => {
                    if let Ok(v) = serde_json::from_value(value) { self.mfa_required = v; }
                }
                "mfa_enforced_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.mfa_enforced_at = v; }
                }
                "risk_score" => {
                    if let Ok(v) = serde_json::from_value(value) { self.risk_score = v; }
                }
                "trust_level" => {
                    if let Ok(v) = serde_json::from_value(value) { self.trust_level = v; }
                }
                "adaptive_authentication_triggered" => {
                    if let Ok(v) = serde_json::from_value(value) { self.adaptive_authentication_triggered = v; }
                }
                "additional_factors_required" => {
                    if let Ok(v) = serde_json::from_value(value) { self.additional_factors_required = v; }
                }
                "ip_risk_score" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ip_risk_score = v; }
                }
                "device_risk_score" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_risk_score = v; }
                }
                "location_anomaly" => {
                    if let Ok(v) = serde_json::from_value(value) { self.location_anomaly = v; }
                }
                "time_anomaly" => {
                    if let Ok(v) = serde_json::from_value(value) { self.time_anomaly = v; }
                }
                "verification_method" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_method = v; }
                }
                "verification_initiated_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_initiated_at = v; }
                }
                "verification_completed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_completed_at = v; }
                }
                "verification_attempts" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_attempts = v; }
                }
                "max_attempts_allowed" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_attempts_allowed = v; }
                }
                "verification_success" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_success = v; }
                }
                "verification_time_ms" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_time_ms = v; }
                }
                "device_trust_status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_trust_status = v; }
                }
                "is_terminated" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_terminated = v; }
                }
                "terminated_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.terminated_at = v; }
                }
                "termination_reason" => {
                    if let Ok(v) = serde_json::from_value(value) { self.termination_reason = v; }
                }
                "ip_address" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ip_address = v; }
                }
                "user_agent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_agent = v; }
                }
                "device_fingerprint" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_fingerprint = v; }
                }
                "country" => {
                    if let Ok(v) = serde_json::from_value(value) { self.country = v; }
                }
                "region" => {
                    if let Ok(v) = serde_json::from_value(value) { self.region = v; }
                }
                "city" => {
                    if let Ok(v) = serde_json::from_value(value) { self.city = v; }
                }
                "latitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.latitude = v; }
                }
                "longitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.longitude = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "max_duration_minutes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_duration_minutes = v; }
                }
                "idle_timeout_minutes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.idle_timeout_minutes = v; }
                }
                "auto_renew_enabled" => {
                    if let Ok(v) = serde_json::from_value(value) { self.auto_renew_enabled = v; }
                }
                "last_activity_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_activity_at = v; }
                }
                "extension_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.extension_count = v; }
                }
                "initiation_time_ms" => {
                    if let Ok(v) = serde_json::from_value(value) { self.initiation_time_ms = v; }
                }
                "total_session_time_ms" => {
                    if let Ok(v) = serde_json::from_value(value) { self.total_session_time_ms = v; }
                }
                "device_performance_score" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_performance_score = v; }
                }
                "network_quality" => {
                    if let Ok(v) = serde_json::from_value(value) { self.network_quality = v; }
                }
                "audit_log_required" => {
                    if let Ok(v) = serde_json::from_value(value) { self.audit_log_required = v; }
                }
                "data_retention_hours" => {
                    if let Ok(v) = serde_json::from_value(value) { self.data_retention_hours = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for MFASession {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "MFASession"
    }
}

impl backbone_core::PersistentEntity for MFASession {
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

impl backbone_orm::EntityRepoMeta for MFASession {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("device_id".to_string(), "uuid".to_string());
        m.insert("primary_authentication_method".to_string(), "primary_auth_method".to_string());
        m.insert("trust_level".to_string(), "mfa_trust_level".to_string());
        m.insert("verification_method".to_string(), "mfa_verification_method".to_string());
        m.insert("device_trust_status".to_string(), "device_trust_status".to_string());
        m.insert("network_quality".to_string(), "network_quality".to_string());
        m.insert("status".to_string(), "mfa_session_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["session_token", "session_hash", "ip_address"]
    }
}

/// Builder for MFASession entity
///
/// Provides a fluent API for constructing MFASession instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct MFASessionBuilder {
    user_id: Option<Uuid>,
    device_id: Option<Uuid>,
    session_token: Option<String>,
    session_hash: Option<String>,
    primary_authentication_method: Option<PrimaryAuthMethod>,
    mfa_required: Option<bool>,
    mfa_enforced_at: Option<DateTime<Utc>>,
    risk_score: Option<i32>,
    trust_level: Option<MfaTrustLevel>,
    adaptive_authentication_triggered: Option<bool>,
    additional_factors_required: Option<i32>,
    ip_risk_score: Option<i32>,
    device_risk_score: Option<i32>,
    location_anomaly: Option<bool>,
    time_anomaly: Option<bool>,
    verification_method: Option<MFAVerificationMethod>,
    verification_initiated_at: Option<DateTime<Utc>>,
    verification_completed_at: Option<DateTime<Utc>>,
    verification_attempts: Option<i32>,
    max_attempts_allowed: Option<i32>,
    verification_success: Option<bool>,
    verification_time_ms: Option<i32>,
    device_trust_status: Option<DeviceTrustStatus>,
    is_terminated: Option<bool>,
    terminated_at: Option<DateTime<Utc>>,
    termination_reason: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    device_fingerprint: Option<String>,
    country: Option<String>,
    region: Option<String>,
    city: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    expires_at: Option<DateTime<Utc>>,
    max_duration_minutes: Option<i32>,
    idle_timeout_minutes: Option<i32>,
    auto_renew_enabled: Option<bool>,
    last_activity_at: Option<DateTime<Utc>>,
    extension_count: Option<i32>,
    initiation_time_ms: Option<i32>,
    total_session_time_ms: Option<i32>,
    device_performance_score: Option<i32>,
    network_quality: Option<NetworkQuality>,
    audit_log_required: Option<bool>,
    data_retention_hours: Option<i32>,
    status: Option<MFASessionStatus>,
}

impl MFASessionBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the device_id field (required)
    pub fn device_id(mut self, value: Uuid) -> Self {
        self.device_id = Some(value);
        self
    }

    /// Set the session_token field (required)
    pub fn session_token(mut self, value: String) -> Self {
        self.session_token = Some(value);
        self
    }

    /// Set the session_hash field (required)
    pub fn session_hash(mut self, value: String) -> Self {
        self.session_hash = Some(value);
        self
    }

    /// Set the primary_authentication_method field (required)
    pub fn primary_authentication_method(mut self, value: PrimaryAuthMethod) -> Self {
        self.primary_authentication_method = Some(value);
        self
    }

    /// Set the mfa_required field (required)
    pub fn mfa_required(mut self, value: bool) -> Self {
        self.mfa_required = Some(value);
        self
    }

    /// Set the mfa_enforced_at field (optional)
    pub fn mfa_enforced_at(mut self, value: DateTime<Utc>) -> Self {
        self.mfa_enforced_at = Some(value);
        self
    }

    /// Set the risk_score field (default: `0`)
    pub fn risk_score(mut self, value: i32) -> Self {
        self.risk_score = Some(value);
        self
    }

    /// Set the trust_level field (default: `MfaTrustLevel::default()`)
    pub fn trust_level(mut self, value: MfaTrustLevel) -> Self {
        self.trust_level = Some(value);
        self
    }

    /// Set the adaptive_authentication_triggered field (default: `false`)
    pub fn adaptive_authentication_triggered(mut self, value: bool) -> Self {
        self.adaptive_authentication_triggered = Some(value);
        self
    }

    /// Set the additional_factors_required field (default: `0`)
    pub fn additional_factors_required(mut self, value: i32) -> Self {
        self.additional_factors_required = Some(value);
        self
    }

    /// Set the ip_risk_score field (optional)
    pub fn ip_risk_score(mut self, value: i32) -> Self {
        self.ip_risk_score = Some(value);
        self
    }

    /// Set the device_risk_score field (optional)
    pub fn device_risk_score(mut self, value: i32) -> Self {
        self.device_risk_score = Some(value);
        self
    }

    /// Set the location_anomaly field (default: `false`)
    pub fn location_anomaly(mut self, value: bool) -> Self {
        self.location_anomaly = Some(value);
        self
    }

    /// Set the time_anomaly field (default: `false`)
    pub fn time_anomaly(mut self, value: bool) -> Self {
        self.time_anomaly = Some(value);
        self
    }

    /// Set the verification_method field (required)
    pub fn verification_method(mut self, value: MFAVerificationMethod) -> Self {
        self.verification_method = Some(value);
        self
    }

    /// Set the verification_initiated_at field (required)
    pub fn verification_initiated_at(mut self, value: DateTime<Utc>) -> Self {
        self.verification_initiated_at = Some(value);
        self
    }

    /// Set the verification_completed_at field (optional)
    pub fn verification_completed_at(mut self, value: DateTime<Utc>) -> Self {
        self.verification_completed_at = Some(value);
        self
    }

    /// Set the verification_attempts field (default: `0`)
    pub fn verification_attempts(mut self, value: i32) -> Self {
        self.verification_attempts = Some(value);
        self
    }

    /// Set the max_attempts_allowed field (default: `5`)
    pub fn max_attempts_allowed(mut self, value: i32) -> Self {
        self.max_attempts_allowed = Some(value);
        self
    }

    /// Set the verification_success field (optional)
    pub fn verification_success(mut self, value: bool) -> Self {
        self.verification_success = Some(value);
        self
    }

    /// Set the verification_time_ms field (optional)
    pub fn verification_time_ms(mut self, value: i32) -> Self {
        self.verification_time_ms = Some(value);
        self
    }

    /// Set the device_trust_status field (default: `DeviceTrustStatus::default()`)
    pub fn device_trust_status(mut self, value: DeviceTrustStatus) -> Self {
        self.device_trust_status = Some(value);
        self
    }

    /// Set the is_terminated field (default: `false`)
    pub fn is_terminated(mut self, value: bool) -> Self {
        self.is_terminated = Some(value);
        self
    }

    /// Set the terminated_at field (optional)
    pub fn terminated_at(mut self, value: DateTime<Utc>) -> Self {
        self.terminated_at = Some(value);
        self
    }

    /// Set the termination_reason field (optional)
    pub fn termination_reason(mut self, value: String) -> Self {
        self.termination_reason = Some(value);
        self
    }

    /// Set the ip_address field (required)
    pub fn ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    /// Set the user_agent field (optional)
    pub fn user_agent(mut self, value: String) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Set the device_fingerprint field (optional)
    pub fn device_fingerprint(mut self, value: String) -> Self {
        self.device_fingerprint = Some(value);
        self
    }

    /// Set the country field (optional)
    pub fn country(mut self, value: String) -> Self {
        self.country = Some(value);
        self
    }

    /// Set the region field (optional)
    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }

    /// Set the city field (optional)
    pub fn city(mut self, value: String) -> Self {
        self.city = Some(value);
        self
    }

    /// Set the latitude field (optional)
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    /// Set the longitude field (optional)
    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Set the expires_at field (required)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the max_duration_minutes field (default: `30`)
    pub fn max_duration_minutes(mut self, value: i32) -> Self {
        self.max_duration_minutes = Some(value);
        self
    }

    /// Set the idle_timeout_minutes field (default: `15`)
    pub fn idle_timeout_minutes(mut self, value: i32) -> Self {
        self.idle_timeout_minutes = Some(value);
        self
    }

    /// Set the auto_renew_enabled field (default: `false`)
    pub fn auto_renew_enabled(mut self, value: bool) -> Self {
        self.auto_renew_enabled = Some(value);
        self
    }

    /// Set the last_activity_at field (default: `Default::default()`)
    pub fn last_activity_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_activity_at = Some(value);
        self
    }

    /// Set the extension_count field (default: `0`)
    pub fn extension_count(mut self, value: i32) -> Self {
        self.extension_count = Some(value);
        self
    }

    /// Set the initiation_time_ms field (required)
    pub fn initiation_time_ms(mut self, value: i32) -> Self {
        self.initiation_time_ms = Some(value);
        self
    }

    /// Set the total_session_time_ms field (optional)
    pub fn total_session_time_ms(mut self, value: i32) -> Self {
        self.total_session_time_ms = Some(value);
        self
    }

    /// Set the device_performance_score field (optional)
    pub fn device_performance_score(mut self, value: i32) -> Self {
        self.device_performance_score = Some(value);
        self
    }

    /// Set the network_quality field (default: `NetworkQuality::default()`)
    pub fn network_quality(mut self, value: NetworkQuality) -> Self {
        self.network_quality = Some(value);
        self
    }

    /// Set the audit_log_required field (default: `true`)
    pub fn audit_log_required(mut self, value: bool) -> Self {
        self.audit_log_required = Some(value);
        self
    }

    /// Set the data_retention_hours field (default: `8760`)
    pub fn data_retention_hours(mut self, value: i32) -> Self {
        self.data_retention_hours = Some(value);
        self
    }

    /// Set the status field (default: `MFASessionStatus::default()`)
    pub fn status(mut self, value: MFASessionStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the MFASession entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<MFASession, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let device_id = self.device_id.ok_or_else(|| "device_id is required".to_string())?;
        let session_token = self.session_token.ok_or_else(|| "session_token is required".to_string())?;
        let session_hash = self.session_hash.ok_or_else(|| "session_hash is required".to_string())?;
        let primary_authentication_method = self.primary_authentication_method.ok_or_else(|| "primary_authentication_method is required".to_string())?;
        let mfa_required = self.mfa_required.ok_or_else(|| "mfa_required is required".to_string())?;
        let verification_method = self.verification_method.ok_or_else(|| "verification_method is required".to_string())?;
        let verification_initiated_at = self.verification_initiated_at.ok_or_else(|| "verification_initiated_at is required".to_string())?;
        let ip_address = self.ip_address.ok_or_else(|| "ip_address is required".to_string())?;
        let expires_at = self.expires_at.ok_or_else(|| "expires_at is required".to_string())?;
        let initiation_time_ms = self.initiation_time_ms.ok_or_else(|| "initiation_time_ms is required".to_string())?;

        Ok(MFASession {
            id: Uuid::new_v4(),
            user_id,
            device_id,
            session_token,
            session_hash,
            primary_authentication_method,
            mfa_required,
            mfa_enforced_at: self.mfa_enforced_at,
            risk_score: self.risk_score.unwrap_or(0),
            trust_level: self.trust_level.unwrap_or(MfaTrustLevel::default()),
            adaptive_authentication_triggered: self.adaptive_authentication_triggered.unwrap_or(false),
            additional_factors_required: self.additional_factors_required.unwrap_or(0),
            ip_risk_score: self.ip_risk_score,
            device_risk_score: self.device_risk_score,
            location_anomaly: self.location_anomaly.unwrap_or(false),
            time_anomaly: self.time_anomaly.unwrap_or(false),
            verification_method,
            verification_initiated_at,
            verification_completed_at: self.verification_completed_at,
            verification_attempts: self.verification_attempts.unwrap_or(0),
            max_attempts_allowed: self.max_attempts_allowed.unwrap_or(5),
            verification_success: self.verification_success,
            verification_time_ms: self.verification_time_ms,
            device_trust_status: self.device_trust_status.unwrap_or(DeviceTrustStatus::default()),
            is_terminated: self.is_terminated.unwrap_or(false),
            terminated_at: self.terminated_at,
            termination_reason: self.termination_reason,
            ip_address,
            user_agent: self.user_agent,
            device_fingerprint: self.device_fingerprint,
            country: self.country,
            region: self.region,
            city: self.city,
            latitude: self.latitude,
            longitude: self.longitude,
            expires_at,
            max_duration_minutes: self.max_duration_minutes.unwrap_or(30),
            idle_timeout_minutes: self.idle_timeout_minutes.unwrap_or(15),
            auto_renew_enabled: self.auto_renew_enabled.unwrap_or(false),
            last_activity_at: self.last_activity_at.unwrap_or(Default::default()),
            extension_count: self.extension_count.unwrap_or(0),
            initiation_time_ms,
            total_session_time_ms: self.total_session_time_ms,
            device_performance_score: self.device_performance_score,
            network_quality: self.network_quality.unwrap_or(NetworkQuality::default()),
            audit_log_required: self.audit_log_required.unwrap_or(true),
            data_retention_hours: self.data_retention_hours.unwrap_or(8760),
            status: self.status.unwrap_or(MFASessionStatus::default()),
            metadata: AuditMetadata::default(),
        })
    }
}
