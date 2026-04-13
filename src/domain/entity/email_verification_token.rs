use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::EmailVerificationType;
use super::EmailVerificationTokenStatus;
use super::AuditMetadata;

use crate::domain::state_machine::{EmailVerificationTokenStateMachine, EmailVerificationTokenState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for EmailVerificationToken
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmailVerificationTokenId(pub Uuid);

impl EmailVerificationTokenId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for EmailVerificationTokenId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmailVerificationTokenId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for EmailVerificationTokenId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<EmailVerificationTokenId> for Uuid {
    fn from(id: EmailVerificationTokenId) -> Self { id.0 }
}

impl AsRef<Uuid> for EmailVerificationTokenId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for EmailVerificationTokenId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub email: String,
    pub token_type: EmailVerificationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_email: Option<String>,
    pub expires_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<DateTime<Utc>>,
    pub attempts: i32,
    pub max_attempts: i32,
    pub(crate) status: EmailVerificationTokenStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl EmailVerificationToken {
    /// Create a builder for EmailVerificationToken
    pub fn builder() -> EmailVerificationTokenBuilder {
        EmailVerificationTokenBuilder::default()
    }

    /// Create a new EmailVerificationToken with required fields
    pub fn new(user_id: Uuid, token: String, email: String, token_type: EmailVerificationType, expires_at: DateTime<Utc>, attempts: i32, max_attempts: i32, status: EmailVerificationTokenStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            token,
            email,
            token_type,
            old_email: None,
            expires_at,
            verified_at: None,
            attempts,
            max_attempts,
            status,
            ip_address: None,
            user_agent: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> EmailVerificationTokenId {
        EmailVerificationTokenId(self.id)
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
    pub fn status(&self) -> &EmailVerificationTokenStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the old_email field (chainable)
    pub fn with_old_email(mut self, value: String) -> Self {
        self.old_email = Some(value);
        self
    }

    /// Set the verified_at field (chainable)
    pub fn with_verified_at(mut self, value: DateTime<Utc>) -> Self {
        self.verified_at = Some(value);
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

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: EmailVerificationTokenState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<EmailVerificationTokenState>()?;
        let mut sm = EmailVerificationTokenStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<EmailVerificationTokenStatus>()
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
                "token" => {
                    if let Ok(v) = serde_json::from_value(value) { self.token = v; }
                }
                "email" => {
                    if let Ok(v) = serde_json::from_value(value) { self.email = v; }
                }
                "token_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.token_type = v; }
                }
                "old_email" => {
                    if let Ok(v) = serde_json::from_value(value) { self.old_email = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "verified_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.verified_at = v; }
                }
                "attempts" => {
                    if let Ok(v) = serde_json::from_value(value) { self.attempts = v; }
                }
                "max_attempts" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_attempts = v; }
                }
                "ip_address" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ip_address = v; }
                }
                "user_agent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_agent = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for EmailVerificationToken {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "EmailVerificationToken"
    }
}

impl backbone_core::PersistentEntity for EmailVerificationToken {
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

impl backbone_orm::EntityRepoMeta for EmailVerificationToken {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("token_type".to_string(), "email_verification_type".to_string());
        m.insert("status".to_string(), "email_verification_token_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["token", "email"]
    }
}

/// Builder for EmailVerificationToken entity
///
/// Provides a fluent API for constructing EmailVerificationToken instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct EmailVerificationTokenBuilder {
    user_id: Option<Uuid>,
    token: Option<String>,
    email: Option<String>,
    token_type: Option<EmailVerificationType>,
    old_email: Option<String>,
    expires_at: Option<DateTime<Utc>>,
    verified_at: Option<DateTime<Utc>>,
    attempts: Option<i32>,
    max_attempts: Option<i32>,
    status: Option<EmailVerificationTokenStatus>,
    ip_address: Option<String>,
    user_agent: Option<String>,
}

impl EmailVerificationTokenBuilder {
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

    /// Set the email field (required)
    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    /// Set the token_type field (required)
    pub fn token_type(mut self, value: EmailVerificationType) -> Self {
        self.token_type = Some(value);
        self
    }

    /// Set the old_email field (optional)
    pub fn old_email(mut self, value: String) -> Self {
        self.old_email = Some(value);
        self
    }

    /// Set the expires_at field (required)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the verified_at field (optional)
    pub fn verified_at(mut self, value: DateTime<Utc>) -> Self {
        self.verified_at = Some(value);
        self
    }

    /// Set the attempts field (default: `0`)
    pub fn attempts(mut self, value: i32) -> Self {
        self.attempts = Some(value);
        self
    }

    /// Set the max_attempts field (default: `5`)
    pub fn max_attempts(mut self, value: i32) -> Self {
        self.max_attempts = Some(value);
        self
    }

    /// Set the status field (default: `EmailVerificationTokenStatus::default()`)
    pub fn status(mut self, value: EmailVerificationTokenStatus) -> Self {
        self.status = Some(value);
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

    /// Build the EmailVerificationToken entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<EmailVerificationToken, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let token = self.token.ok_or_else(|| "token is required".to_string())?;
        let email = self.email.ok_or_else(|| "email is required".to_string())?;
        let token_type = self.token_type.ok_or_else(|| "token_type is required".to_string())?;
        let expires_at = self.expires_at.ok_or_else(|| "expires_at is required".to_string())?;

        Ok(EmailVerificationToken {
            id: Uuid::new_v4(),
            user_id,
            token,
            email,
            token_type,
            old_email: self.old_email,
            expires_at,
            verified_at: self.verified_at,
            attempts: self.attempts.unwrap_or(0),
            max_attempts: self.max_attempts.unwrap_or(5),
            status: self.status.unwrap_or(EmailVerificationTokenStatus::default()),
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            metadata: AuditMetadata::default(),
        })
    }
}
