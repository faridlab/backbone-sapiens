use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::PasswordResetStatus;
use super::AuditMetadata;

/// Strongly-typed ID for PasswordReset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PasswordResetId(pub Uuid);

impl PasswordResetId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for PasswordResetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PasswordResetId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for PasswordResetId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<PasswordResetId> for Uuid {
    fn from(id: PasswordResetId) -> Self { id.0 }
}

impl AsRef<Uuid> for PasswordResetId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for PasswordResetId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordReset {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub token_hash: String,
    pub email: String,
    pub status: PasswordResetStatus,
    pub max_attempts: i32,
    pub attempts_remaining: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempt_at: Option<DateTime<Utc>>,
    pub request_metadata: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_details_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_details_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl PasswordReset {
    /// Create a builder for PasswordReset
    pub fn builder() -> PasswordResetBuilder {
        PasswordResetBuilder::default()
    }

    /// Create a new PasswordReset with required fields
    pub fn new(user_id: Uuid, token: String, token_hash: String, email: String, status: PasswordResetStatus, max_attempts: i32, attempts_remaining: i32, request_metadata: serde_json::Value, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            token,
            token_hash,
            email,
            status,
            max_attempts,
            attempts_remaining,
            last_attempt_at: None,
            request_metadata,
            verification_details_id: None,
            completion_details_id: None,
            security_id: None,
            device_fingerprint: None,
            ip_address: None,
            user_agent: None,
            verified_at: None,
            used_at: None,
            expires_at,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> PasswordResetId {
        PasswordResetId(self.id)
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
    pub fn status(&self) -> &PasswordResetStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the last_attempt_at field (chainable)
    pub fn with_last_attempt_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_attempt_at = Some(value);
        self
    }

    /// Set the verification_details_id field (chainable)
    pub fn with_verification_details_id(mut self, value: Uuid) -> Self {
        self.verification_details_id = Some(value);
        self
    }

    /// Set the completion_details_id field (chainable)
    pub fn with_completion_details_id(mut self, value: Uuid) -> Self {
        self.completion_details_id = Some(value);
        self
    }

    /// Set the security_id field (chainable)
    pub fn with_security_id(mut self, value: Uuid) -> Self {
        self.security_id = Some(value);
        self
    }

    /// Set the device_fingerprint field (chainable)
    pub fn with_device_fingerprint(mut self, value: String) -> Self {
        self.device_fingerprint = Some(value);
        self
    }

    /// Set the ip_address field (chainable)
    pub fn with_ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    /// Set the user_agent field (chainable)
    pub fn with_user_agent(mut self, value: String) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Set the verified_at field (chainable)
    pub fn with_verified_at(mut self, value: DateTime<Utc>) -> Self {
        self.verified_at = Some(value);
        self
    }

    /// Set the used_at field (chainable)
    pub fn with_used_at(mut self, value: DateTime<Utc>) -> Self {
        self.used_at = Some(value);
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
                "token" => {
                    if let Ok(v) = serde_json::from_value(value) { self.token = v; }
                }
                "token_hash" => {
                    if let Ok(v) = serde_json::from_value(value) { self.token_hash = v; }
                }
                "email" => {
                    if let Ok(v) = serde_json::from_value(value) { self.email = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "max_attempts" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_attempts = v; }
                }
                "attempts_remaining" => {
                    if let Ok(v) = serde_json::from_value(value) { self.attempts_remaining = v; }
                }
                "last_attempt_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_attempt_at = v; }
                }
                "request_metadata" => {
                    if let Ok(v) = serde_json::from_value(value) { self.request_metadata = v; }
                }
                "verification_details_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verification_details_id = v; }
                }
                "completion_details_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.completion_details_id = v; }
                }
                "security_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.security_id = v; }
                }
                "device_fingerprint" => {
                    if let Ok(v) = serde_json::from_value(value) { self.device_fingerprint = v; }
                }
                "ip_address" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ip_address = v; }
                }
                "user_agent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_agent = v; }
                }
                "verified_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verified_at = v; }
                }
                "used_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.used_at = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for PasswordReset {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "PasswordReset"
    }
}

impl backbone_core::PersistentEntity for PasswordReset {
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

impl backbone_orm::EntityRepoMeta for PasswordReset {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("verification_details_id".to_string(), "uuid".to_string());
        m.insert("completion_details_id".to_string(), "uuid".to_string());
        m.insert("security_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "password_reset_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["token", "token_hash", "email"]
    }
}

/// Builder for PasswordReset entity
///
/// Provides a fluent API for constructing PasswordReset instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct PasswordResetBuilder {
    user_id: Option<Uuid>,
    token: Option<String>,
    token_hash: Option<String>,
    email: Option<String>,
    status: Option<PasswordResetStatus>,
    max_attempts: Option<i32>,
    attempts_remaining: Option<i32>,
    last_attempt_at: Option<DateTime<Utc>>,
    request_metadata: Option<serde_json::Value>,
    verification_details_id: Option<Uuid>,
    completion_details_id: Option<Uuid>,
    security_id: Option<Uuid>,
    device_fingerprint: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    verified_at: Option<DateTime<Utc>>,
    used_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
}

impl PasswordResetBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the token field (required)
    pub fn token(mut self, value: String) -> Self {
        self.token = Some(value);
        self
    }

    /// Set the token_hash field (required)
    pub fn token_hash(mut self, value: String) -> Self {
        self.token_hash = Some(value);
        self
    }

    /// Set the email field (required)
    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    /// Set the status field (default: `PasswordResetStatus::default()`)
    pub fn status(mut self, value: PasswordResetStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the max_attempts field (default: `3`)
    pub fn max_attempts(mut self, value: i32) -> Self {
        self.max_attempts = Some(value);
        self
    }

    /// Set the attempts_remaining field (default: `3`)
    pub fn attempts_remaining(mut self, value: i32) -> Self {
        self.attempts_remaining = Some(value);
        self
    }

    /// Set the last_attempt_at field (optional)
    pub fn last_attempt_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_attempt_at = Some(value);
        self
    }

    /// Set the request_metadata field (default: `serde_json::json!({})`)
    pub fn request_metadata(mut self, value: serde_json::Value) -> Self {
        self.request_metadata = Some(value);
        self
    }

    /// Set the verification_details_id field (optional)
    pub fn verification_details_id(mut self, value: Uuid) -> Self {
        self.verification_details_id = Some(value);
        self
    }

    /// Set the completion_details_id field (optional)
    pub fn completion_details_id(mut self, value: Uuid) -> Self {
        self.completion_details_id = Some(value);
        self
    }

    /// Set the security_id field (optional)
    pub fn security_id(mut self, value: Uuid) -> Self {
        self.security_id = Some(value);
        self
    }

    /// Set the device_fingerprint field (optional)
    pub fn device_fingerprint(mut self, value: String) -> Self {
        self.device_fingerprint = Some(value);
        self
    }

    /// Set the ip_address field (optional)
    pub fn ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    /// Set the user_agent field (optional)
    pub fn user_agent(mut self, value: String) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Set the verified_at field (optional)
    pub fn verified_at(mut self, value: DateTime<Utc>) -> Self {
        self.verified_at = Some(value);
        self
    }

    /// Set the used_at field (optional)
    pub fn used_at(mut self, value: DateTime<Utc>) -> Self {
        self.used_at = Some(value);
        self
    }

    /// Set the expires_at field (required)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Build the PasswordReset entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<PasswordReset, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let token = self.token.ok_or_else(|| "token is required".to_string())?;
        let token_hash = self.token_hash.ok_or_else(|| "token_hash is required".to_string())?;
        let email = self.email.ok_or_else(|| "email is required".to_string())?;
        let expires_at = self.expires_at.ok_or_else(|| "expires_at is required".to_string())?;

        Ok(PasswordReset {
            id: Uuid::new_v4(),
            user_id,
            token,
            token_hash,
            email,
            status: self.status.unwrap_or(PasswordResetStatus::default()),
            max_attempts: self.max_attempts.unwrap_or(3),
            attempts_remaining: self.attempts_remaining.unwrap_or(3),
            last_attempt_at: self.last_attempt_at,
            request_metadata: self.request_metadata.unwrap_or(serde_json::json!({})),
            verification_details_id: self.verification_details_id,
            completion_details_id: self.completion_details_id,
            security_id: self.security_id,
            device_fingerprint: self.device_fingerprint,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            verified_at: self.verified_at,
            used_at: self.used_at,
            expires_at,
            metadata: AuditMetadata::default(),
        })
    }
}
