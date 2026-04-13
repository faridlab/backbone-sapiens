use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::UserStatus;
use super::AuditMetadata;

use crate::domain::state_machine::{UserStateMachine, UserState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for User
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for UserId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<UserId> for Uuid {
    fn from(id: UserId) -> Self { id.0 }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for UserId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub(crate) status: UserStatus,
    pub email_verified: bool,
    pub failed_login_attempts: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<DateTime<Utc>>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl User {
    /// Create a builder for User
    pub fn builder() -> UserBuilder {
        UserBuilder::default()
    }

    /// Create a new User with required fields
    pub fn new(username: String, email: String, password_hash: String, status: UserStatus, email_verified: bool, failed_login_attempts: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            status,
            email_verified,
            failed_login_attempts,
            locked_until: None,
            last_login: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> UserId {
        UserId(self.id)
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
    pub fn status(&self) -> &UserStatus {
        &self.status
    }


    /// Verify a password against the stored hash
    ///
    /// Uses argon2 for secure password verification.
    pub fn verify_password(&self, password: &str) -> bool {
        use argon2::PasswordVerifier;
        use argon2::password_hash::PasswordHash;
        let parsed_hash = PasswordHash::new(&self.password_hash);
        match parsed_hash {
            Ok(hash) => {
                let argon2 = argon2::Argon2::default();
                argon2.verify_password(password.as_bytes(), &hash).is_ok()
            }
            Err(_) => false,
        }
    }

    /// Hash a password for storage
    ///
    /// Uses argon2 with default parameters.
    pub fn hash_password(password: &str) -> Result<String, String> {
        use argon2::PasswordHasher;
        use argon2::password_hash::{SaltString, rand_core::OsRng};
        let argon2 = argon2::Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        argon2.hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| e.to_string())
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the locked_until field (chainable)
    pub fn with_locked_until(mut self, value: DateTime<Utc>) -> Self {
        self.locked_until = Some(value);
        self
    }

    /// Set the last_login field (chainable)
    pub fn with_last_login(mut self, value: DateTime<Utc>) -> Self {
        self.last_login = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: UserState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<UserState>()?;
        let mut sm = UserStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<UserStatus>()
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
                "username" => {
                    if let Ok(v) = serde_json::from_value(value) { self.username = v; }
                }
                "email" => {
                    if let Ok(v) = serde_json::from_value(value) { self.email = v; }
                }
                "password_hash" => {
                    if let Ok(v) = serde_json::from_value(value) { self.password_hash = v; }
                }
                "email_verified" => {
                    if let Ok(v) = serde_json::from_value(value) { self.email_verified = v; }
                }
                "failed_login_attempts" => {
                    if let Ok(v) = serde_json::from_value(value) { self.failed_login_attempts = v; }
                }
                "locked_until" => {
                    if let Ok(v) = serde_json::from_value(value) { self.locked_until = v; }
                }
                "last_login" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_login = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for User {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "User"
    }
}

impl backbone_core::PersistentEntity for User {
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

impl backbone_orm::EntityRepoMeta for User {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "user_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["username", "email", "password_hash"]
    }
}

/// Builder for User entity
///
/// Provides a fluent API for constructing User instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct UserBuilder {
    username: Option<String>,
    email: Option<String>,
    password_hash: Option<String>,
    status: Option<UserStatus>,
    email_verified: Option<bool>,
    failed_login_attempts: Option<i32>,
    locked_until: Option<DateTime<Utc>>,
    last_login: Option<DateTime<Utc>>,
}

impl UserBuilder {
    /// Set the username field (required)
    pub fn username(mut self, value: String) -> Self {
        self.username = Some(value);
        self
    }

    /// Set the email field (required)
    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    /// Set the password_hash field (required)
    pub fn password_hash(mut self, value: String) -> Self {
        self.password_hash = Some(value);
        self
    }

    /// Set the status field (default: `UserStatus::default()`)
    pub fn status(mut self, value: UserStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the email_verified field (default: `false`)
    pub fn email_verified(mut self, value: bool) -> Self {
        self.email_verified = Some(value);
        self
    }

    /// Set the failed_login_attempts field (default: `0`)
    pub fn failed_login_attempts(mut self, value: i32) -> Self {
        self.failed_login_attempts = Some(value);
        self
    }

    /// Set the locked_until field (optional)
    pub fn locked_until(mut self, value: DateTime<Utc>) -> Self {
        self.locked_until = Some(value);
        self
    }

    /// Set the last_login field (optional)
    pub fn last_login(mut self, value: DateTime<Utc>) -> Self {
        self.last_login = Some(value);
        self
    }

    /// Build the User entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<User, String> {
        let username = self.username.ok_or_else(|| "username is required".to_string())?;
        let email = self.email.ok_or_else(|| "email is required".to_string())?;
        let password_hash = self.password_hash.ok_or_else(|| "password_hash is required".to_string())?;

        Ok(User {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            status: self.status.unwrap_or(UserStatus::default()),
            email_verified: self.email_verified.unwrap_or(false),
            failed_login_attempts: self.failed_login_attempts.unwrap_or(0),
            locked_until: self.locked_until,
            last_login: self.last_login,
            metadata: AuditMetadata::default(),
        })
    }
}
