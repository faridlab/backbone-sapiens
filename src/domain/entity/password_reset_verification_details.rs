use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for PasswordResetVerificationDetails
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PasswordResetVerificationDetailsId(pub Uuid);

impl PasswordResetVerificationDetailsId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for PasswordResetVerificationDetailsId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PasswordResetVerificationDetailsId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for PasswordResetVerificationDetailsId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<PasswordResetVerificationDetailsId> for Uuid {
    fn from(id: PasswordResetVerificationDetailsId) -> Self { id.0 }
}

impl AsRef<Uuid> for PasswordResetVerificationDetailsId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for PasswordResetVerificationDetailsId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordResetVerificationDetails {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_ip: Option<String>,
    pub verification_attempts: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempt_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geolocation: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl PasswordResetVerificationDetails {
    /// Create a builder for PasswordResetVerificationDetails
    pub fn builder() -> PasswordResetVerificationDetailsBuilder {
        PasswordResetVerificationDetailsBuilder::default()
    }

    /// Create a new PasswordResetVerificationDetails with required fields
    pub fn new(verification_attempts: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            verified_at: None,
            verification_ip: None,
            verification_attempts,
            last_attempt_at: None,
            verification_method: None,
            device_fingerprint: None,
            geolocation: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> PasswordResetVerificationDetailsId {
        PasswordResetVerificationDetailsId(self.id)
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

    /// Set the verified_at field (chainable)
    pub fn with_verified_at(mut self, value: DateTime<Utc>) -> Self {
        self.verified_at = Some(value);
        self
    }

    /// Set the verification_ip field (chainable)
    pub fn with_verification_ip(mut self, value: String) -> Self {
        self.verification_ip = Some(value);
        self
    }

    /// Set the last_attempt_at field (chainable)
    pub fn with_last_attempt_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_attempt_at = Some(value);
        self
    }

    /// Set the verification_method field (chainable)
    pub fn with_verification_method(mut self, value: String) -> Self {
        self.verification_method = Some(value);
        self
    }

    /// Set the device_fingerprint field (chainable)
    pub fn with_device_fingerprint(mut self, value: String) -> Self {
        self.device_fingerprint = Some(value);
        self
    }

    /// Set the geolocation field (chainable)
    pub fn with_geolocation(mut self, value: serde_json::Value) -> Self {
        self.geolocation = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "verified_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verified_at = v; }
                }
                "verification_ip" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_ip = v; }
                }
                "verification_attempts" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_attempts = v; }
                }
                "last_attempt_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_attempt_at = v; }
                }
                "verification_method" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_method = v; }
                }
                "device_fingerprint" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_fingerprint = v; }
                }
                "geolocation" => {
                    if let Ok(v) = serde_json::from_value(value) { self.geolocation = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for PasswordResetVerificationDetails {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "PasswordResetVerificationDetails"
    }
}

impl backbone_core::PersistentEntity for PasswordResetVerificationDetails {
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

impl backbone_orm::EntityRepoMeta for PasswordResetVerificationDetails {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for PasswordResetVerificationDetails entity
///
/// Provides a fluent API for constructing PasswordResetVerificationDetails instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct PasswordResetVerificationDetailsBuilder {
    verified_at: Option<DateTime<Utc>>,
    verification_ip: Option<String>,
    verification_attempts: Option<i32>,
    last_attempt_at: Option<DateTime<Utc>>,
    verification_method: Option<String>,
    device_fingerprint: Option<String>,
    geolocation: Option<serde_json::Value>,
}

impl PasswordResetVerificationDetailsBuilder {
    /// Set the verified_at field (optional)
    pub fn verified_at(mut self, value: DateTime<Utc>) -> Self {
        self.verified_at = Some(value);
        self
    }

    /// Set the verification_ip field (optional)
    pub fn verification_ip(mut self, value: String) -> Self {
        self.verification_ip = Some(value);
        self
    }

    /// Set the verification_attempts field (default: `0`)
    pub fn verification_attempts(mut self, value: i32) -> Self {
        self.verification_attempts = Some(value);
        self
    }

    /// Set the last_attempt_at field (optional)
    pub fn last_attempt_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_attempt_at = Some(value);
        self
    }

    /// Set the verification_method field (optional)
    pub fn verification_method(mut self, value: String) -> Self {
        self.verification_method = Some(value);
        self
    }

    /// Set the device_fingerprint field (optional)
    pub fn device_fingerprint(mut self, value: String) -> Self {
        self.device_fingerprint = Some(value);
        self
    }

    /// Set the geolocation field (optional)
    pub fn geolocation(mut self, value: serde_json::Value) -> Self {
        self.geolocation = Some(value);
        self
    }

    /// Build the PasswordResetVerificationDetails entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<PasswordResetVerificationDetails, String> {

        Ok(PasswordResetVerificationDetails {
            id: Uuid::new_v4(),
            verified_at: self.verified_at,
            verification_ip: self.verification_ip,
            verification_attempts: self.verification_attempts.unwrap_or(0),
            last_attempt_at: self.last_attempt_at,
            verification_method: self.verification_method,
            device_fingerprint: self.device_fingerprint,
            geolocation: self.geolocation,
            metadata: AuditMetadata::default(),
        })
    }
}
