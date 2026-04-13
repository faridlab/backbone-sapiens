use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for PasswordResetSecurity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PasswordResetSecurityId(pub Uuid);

impl PasswordResetSecurityId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for PasswordResetSecurityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PasswordResetSecurityId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for PasswordResetSecurityId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<PasswordResetSecurityId> for Uuid {
    fn from(id: PasswordResetSecurityId) -> Self { id.0 }
}

impl AsRef<Uuid> for PasswordResetSecurityId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for PasswordResetSecurityId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordResetSecurity {
    pub id: Uuid,
    pub max_attempts: i32,
    pub attempts_remaining: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempt_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub suspicious_activity: bool,
    pub risk_score: i32,
    pub security_flags: serde_json::Value,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl PasswordResetSecurity {
    /// Create a builder for PasswordResetSecurity
    pub fn builder() -> PasswordResetSecurityBuilder {
        PasswordResetSecurityBuilder::default()
    }

    /// Create a new PasswordResetSecurity with required fields
    pub fn new(max_attempts: i32, attempts_remaining: i32, expires_at: DateTime<Utc>, suspicious_activity: bool, risk_score: i32, security_flags: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            max_attempts,
            attempts_remaining,
            last_attempt_at: None,
            expires_at,
            suspicious_activity,
            risk_score,
            security_flags,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> PasswordResetSecurityId {
        PasswordResetSecurityId(self.id)
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

    /// Set the last_attempt_at field (chainable)
    pub fn with_last_attempt_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_attempt_at = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "max_attempts" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_attempts = v; }
                }
                "attempts_remaining" => {
                    if let Ok(v) = serde_json::from_value(value) { self.attempts_remaining = v; }
                }
                "last_attempt_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_attempt_at = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "suspicious_activity" => {
                    if let Ok(v) = serde_json::from_value(value) { self.suspicious_activity = v; }
                }
                "risk_score" => {
                    if let Ok(v) = serde_json::from_value(value) { self.risk_score = v; }
                }
                "security_flags" => {
                    if let Ok(v) = serde_json::from_value(value) { self.security_flags = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for PasswordResetSecurity {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "PasswordResetSecurity"
    }
}

impl backbone_core::PersistentEntity for PasswordResetSecurity {
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

impl backbone_orm::EntityRepoMeta for PasswordResetSecurity {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for PasswordResetSecurity entity
///
/// Provides a fluent API for constructing PasswordResetSecurity instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct PasswordResetSecurityBuilder {
    max_attempts: Option<i32>,
    attempts_remaining: Option<i32>,
    last_attempt_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    suspicious_activity: Option<bool>,
    risk_score: Option<i32>,
    security_flags: Option<serde_json::Value>,
}

impl PasswordResetSecurityBuilder {
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

    /// Set the expires_at field (required)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the suspicious_activity field (default: `false`)
    pub fn suspicious_activity(mut self, value: bool) -> Self {
        self.suspicious_activity = Some(value);
        self
    }

    /// Set the risk_score field (default: `0`)
    pub fn risk_score(mut self, value: i32) -> Self {
        self.risk_score = Some(value);
        self
    }

    /// Set the security_flags field (default: `serde_json::json!({})`)
    pub fn security_flags(mut self, value: serde_json::Value) -> Self {
        self.security_flags = Some(value);
        self
    }

    /// Build the PasswordResetSecurity entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<PasswordResetSecurity, String> {
        let expires_at = self.expires_at.ok_or_else(|| "expires_at is required".to_string())?;

        Ok(PasswordResetSecurity {
            id: Uuid::new_v4(),
            max_attempts: self.max_attempts.unwrap_or(3),
            attempts_remaining: self.attempts_remaining.unwrap_or(3),
            last_attempt_at: self.last_attempt_at,
            expires_at,
            suspicious_activity: self.suspicious_activity.unwrap_or(false),
            risk_score: self.risk_score.unwrap_or(0),
            security_flags: self.security_flags.unwrap_or(serde_json::json!({})),
            metadata: AuditMetadata::default(),
        })
    }
}
