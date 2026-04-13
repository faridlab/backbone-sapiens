use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::DeviceTrustType;
use super::TrustLevel;
use super::AuditMetadata;

/// Strongly-typed ID for DeviceTrust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeviceTrustId(pub Uuid);

impl DeviceTrustId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for DeviceTrustId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for DeviceTrustId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for DeviceTrustId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<DeviceTrustId> for Uuid {
    fn from(id: DeviceTrustId) -> Self { id.0 }
}

impl AsRef<Uuid> for DeviceTrustId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for DeviceTrustId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeviceTrust {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_fingerprint: String,
    pub device_name: String,
    pub device_type: DeviceTrustType,
    pub trust_level: TrustLevel,
    pub is_trusted: bool,
    pub trusted_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: DateTime<Utc>,
    pub usage_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>,
    pub risk_score: f64,
    pub requires_mfa: bool,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl DeviceTrust {
    /// Create a builder for DeviceTrust
    pub fn builder() -> DeviceTrustBuilder {
        DeviceTrustBuilder::default()
    }

    /// Create a new DeviceTrust with required fields
    pub fn new(user_id: Uuid, device_fingerprint: String, device_name: String, device_type: DeviceTrustType, trust_level: TrustLevel, is_trusted: bool, trusted_at: DateTime<Utc>, last_used_at: DateTime<Utc>, usage_count: i32, risk_score: f64, requires_mfa: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            device_fingerprint,
            device_name,
            device_type,
            trust_level,
            is_trusted,
            trusted_at,
            expires_at: None,
            last_used_at,
            usage_count,
            ip_address: None,
            location: None,
            risk_score,
            requires_mfa,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> DeviceTrustId {
        DeviceTrustId(self.id)
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

    /// Set the expires_at field (chainable)
    pub fn with_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the ip_address field (chainable)
    pub fn with_ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    /// Set the location field (chainable)
    pub fn with_location(mut self, value: serde_json::Value) -> Self {
        self.location = Some(value);
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
                "device_fingerprint" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_fingerprint = v; }
                }
                "device_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_name = v; }
                }
                "device_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_type = v; }
                }
                "trust_level" => {
                    if let Ok(v) = serde_json::from_value(value) { self.trust_level = v; }
                }
                "is_trusted" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_trusted = v; }
                }
                "trusted_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.trusted_at = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "last_used_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_used_at = v; }
                }
                "usage_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.usage_count = v; }
                }
                "ip_address" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ip_address = v; }
                }
                "location" => {
                    if let Ok(v) = serde_json::from_value(value) { self.location = v; }
                }
                "risk_score" => {
                    if let Ok(v) = serde_json::from_value(value) { self.risk_score = v; }
                }
                "requires_mfa" => {
                    if let Ok(v) = serde_json::from_value(value) { self.requires_mfa = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for DeviceTrust {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "DeviceTrust"
    }
}

impl backbone_core::PersistentEntity for DeviceTrust {
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

impl backbone_orm::EntityRepoMeta for DeviceTrust {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("device_type".to_string(), "device_trust_type".to_string());
        m.insert("trust_level".to_string(), "trust_level".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["device_fingerprint", "device_name"]
    }
}

/// Builder for DeviceTrust entity
///
/// Provides a fluent API for constructing DeviceTrust instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct DeviceTrustBuilder {
    user_id: Option<Uuid>,
    device_fingerprint: Option<String>,
    device_name: Option<String>,
    device_type: Option<DeviceTrustType>,
    trust_level: Option<TrustLevel>,
    is_trusted: Option<bool>,
    trusted_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    last_used_at: Option<DateTime<Utc>>,
    usage_count: Option<i32>,
    ip_address: Option<String>,
    location: Option<serde_json::Value>,
    risk_score: Option<f64>,
    requires_mfa: Option<bool>,
}

impl DeviceTrustBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the device_fingerprint field (required)
    pub fn device_fingerprint(mut self, value: String) -> Self {
        self.device_fingerprint = Some(value);
        self
    }

    /// Set the device_name field (required)
    pub fn device_name(mut self, value: String) -> Self {
        self.device_name = Some(value);
        self
    }

    /// Set the device_type field (required)
    pub fn device_type(mut self, value: DeviceTrustType) -> Self {
        self.device_type = Some(value);
        self
    }

    /// Set the trust_level field (default: `TrustLevel::default()`)
    pub fn trust_level(mut self, value: TrustLevel) -> Self {
        self.trust_level = Some(value);
        self
    }

    /// Set the is_trusted field (default: `true`)
    pub fn is_trusted(mut self, value: bool) -> Self {
        self.is_trusted = Some(value);
        self
    }

    /// Set the trusted_at field (default: `Utc::now()`)
    pub fn trusted_at(mut self, value: DateTime<Utc>) -> Self {
        self.trusted_at = Some(value);
        self
    }

    /// Set the expires_at field (optional)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the last_used_at field (default: `Utc::now()`)
    pub fn last_used_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_used_at = Some(value);
        self
    }

    /// Set the usage_count field (default: `0`)
    pub fn usage_count(mut self, value: i32) -> Self {
        self.usage_count = Some(value);
        self
    }

    /// Set the ip_address field (optional)
    pub fn ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    /// Set the location field (optional)
    pub fn location(mut self, value: serde_json::Value) -> Self {
        self.location = Some(value);
        self
    }

    /// Set the risk_score field (default: `0_f64`)
    pub fn risk_score(mut self, value: f64) -> Self {
        self.risk_score = Some(value);
        self
    }

    /// Set the requires_mfa field (default: `false`)
    pub fn requires_mfa(mut self, value: bool) -> Self {
        self.requires_mfa = Some(value);
        self
    }

    /// Build the DeviceTrust entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<DeviceTrust, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let device_fingerprint = self.device_fingerprint.ok_or_else(|| "device_fingerprint is required".to_string())?;
        let device_name = self.device_name.ok_or_else(|| "device_name is required".to_string())?;
        let device_type = self.device_type.ok_or_else(|| "device_type is required".to_string())?;

        Ok(DeviceTrust {
            id: Uuid::new_v4(),
            user_id,
            device_fingerprint,
            device_name,
            device_type,
            trust_level: self.trust_level.unwrap_or(TrustLevel::default()),
            is_trusted: self.is_trusted.unwrap_or(true),
            trusted_at: self.trusted_at.unwrap_or(Utc::now()),
            expires_at: self.expires_at,
            last_used_at: self.last_used_at.unwrap_or(Utc::now()),
            usage_count: self.usage_count.unwrap_or(0),
            ip_address: self.ip_address,
            location: self.location,
            risk_score: self.risk_score.unwrap_or(0_f64),
            requires_mfa: self.requires_mfa.unwrap_or(false),
            metadata: AuditMetadata::default(),
        })
    }
}
