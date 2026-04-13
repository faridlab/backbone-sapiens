use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::Theme;
use super::AuditMetadata;

/// Strongly-typed ID for UserSettings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserSettingsId(pub Uuid);

impl UserSettingsId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for UserSettingsId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserSettingsId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for UserSettingsId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<UserSettingsId> for Uuid {
    fn from(id: UserSettingsId) -> Self { id.0 }
}

impl AsRef<Uuid> for UserSettingsId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for UserSettingsId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub theme: Theme,
    pub language: String,
    pub timezone: String,
    pub notifications_enabled: bool,
    pub email_notifications: bool,
    pub sms_notifications: bool,
    pub mfa_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_settings: Option<serde_json::Value>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl UserSettings {
    /// Create a builder for UserSettings
    pub fn builder() -> UserSettingsBuilder {
        UserSettingsBuilder::default()
    }

    /// Create a new UserSettings with required fields
    pub fn new(user_id: Uuid, theme: Theme, language: String, timezone: String, notifications_enabled: bool, email_notifications: bool, sms_notifications: bool, mfa_enabled: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            theme,
            language,
            timezone,
            notifications_enabled,
            email_notifications,
            sms_notifications,
            mfa_enabled,
            custom_settings: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> UserSettingsId {
        UserSettingsId(self.id)
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

    /// Set the custom_settings field (chainable)
    pub fn with_custom_settings(mut self, value: serde_json::Value) -> Self {
        self.custom_settings = Some(value);
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
                "theme" => {
                    if let Ok(v) = serde_json::from_value(value) { self.theme = v; }
                }
                "language" => {
                    if let Ok(v) = serde_json::from_value(value) { self.language = v; }
                }
                "timezone" => {
                    if let Ok(v) = serde_json::from_value(value) { self.timezone = v; }
                }
                "notifications_enabled" => {
                    if let Ok(v) = serde_json::from_value(value) { self.notifications_enabled = v; }
                }
                "email_notifications" => {
                    if let Ok(v) = serde_json::from_value(value) { self.email_notifications = v; }
                }
                "sms_notifications" => {
                    if let Ok(v) = serde_json::from_value(value) { self.sms_notifications = v; }
                }
                "mfa_enabled" => {
                    if let Ok(v) = serde_json::from_value(value) { self.mfa_enabled = v; }
                }
                "custom_settings" => {
                    if let Ok(v) = serde_json::from_value(value) { self.custom_settings = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for UserSettings {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "UserSettings"
    }
}

impl backbone_core::PersistentEntity for UserSettings {
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

impl backbone_orm::EntityRepoMeta for UserSettings {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("theme".to_string(), "theme".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["language", "timezone"]
    }
}

/// Builder for UserSettings entity
///
/// Provides a fluent API for constructing UserSettings instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct UserSettingsBuilder {
    user_id: Option<Uuid>,
    theme: Option<Theme>,
    language: Option<String>,
    timezone: Option<String>,
    notifications_enabled: Option<bool>,
    email_notifications: Option<bool>,
    sms_notifications: Option<bool>,
    mfa_enabled: Option<bool>,
    custom_settings: Option<serde_json::Value>,
}

impl UserSettingsBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the theme field (default: `Theme::default()`)
    pub fn theme(mut self, value: Theme) -> Self {
        self.theme = Some(value);
        self
    }

    /// Set the language field (default: `Default::default()`)
    pub fn language(mut self, value: String) -> Self {
        self.language = Some(value);
        self
    }

    /// Set the timezone field (default: `Default::default()`)
    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }

    /// Set the notifications_enabled field (default: `true`)
    pub fn notifications_enabled(mut self, value: bool) -> Self {
        self.notifications_enabled = Some(value);
        self
    }

    /// Set the email_notifications field (default: `true`)
    pub fn email_notifications(mut self, value: bool) -> Self {
        self.email_notifications = Some(value);
        self
    }

    /// Set the sms_notifications field (default: `false`)
    pub fn sms_notifications(mut self, value: bool) -> Self {
        self.sms_notifications = Some(value);
        self
    }

    /// Set the mfa_enabled field (default: `false`)
    pub fn mfa_enabled(mut self, value: bool) -> Self {
        self.mfa_enabled = Some(value);
        self
    }

    /// Set the custom_settings field (optional)
    pub fn custom_settings(mut self, value: serde_json::Value) -> Self {
        self.custom_settings = Some(value);
        self
    }

    /// Build the UserSettings entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<UserSettings, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;

        Ok(UserSettings {
            id: Uuid::new_v4(),
            user_id,
            theme: self.theme.unwrap_or(Theme::default()),
            language: self.language.unwrap_or(Default::default()),
            timezone: self.timezone.unwrap_or(Default::default()),
            notifications_enabled: self.notifications_enabled.unwrap_or(true),
            email_notifications: self.email_notifications.unwrap_or(true),
            sms_notifications: self.sms_notifications.unwrap_or(false),
            mfa_enabled: self.mfa_enabled.unwrap_or(false),
            custom_settings: self.custom_settings,
            metadata: AuditMetadata::default(),
        })
    }
}
