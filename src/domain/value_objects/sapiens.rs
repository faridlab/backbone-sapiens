// Sapiens Value Objects
// Shared value objects for Sapiens domain

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

// SapiensId Value Object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SapiensId(String);

impl SapiensId {
    pub fn new(id: &str) -> Result<Self, SapiensIdError> {
        if id.is_empty() {
            return Err(SapiensIdError::Empty);
        }

        // Validate UUID format (basic validation)
        let parts: Vec<&str> = id.split('-').collect();
        if parts.len() != 5 {
            return Err(SapiensIdError::InvalidFormat);
        }

        Ok(SapiensId(id.to_string()))
    }

    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Default for SapiensId {
    fn default() -> Self {
        Self::generate()
    }
}

impl fmt::Display for SapiensId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SapiensId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for SapiensId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SapiensIdError {
    #[error("Sapiens ID cannot be empty")]
    Empty,
    #[error("Invalid Sapiens ID format")]
    InvalidFormat,
}

// SapiensName Value Object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SapiensName(String);

impl SapiensName {
    pub fn new(name: &str) -> Result<Self, SapiensNameError> {
        let trimmed = name.trim();

        if trimmed.is_empty() {
            return Err(SapiensNameError::Empty);
        }

        if trimmed.len() > 100 {
            return Err(SapiensNameError::TooLong);
        }

        // Validate allowed characters: alphanumeric, spaces, hyphens, underscores
        if !trimmed.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || c == '-' || c == '_') {
            return Err(SapiensNameError::InvalidCharacters);
        }

        Ok(SapiensName(trimmed.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for SapiensName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SapiensName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for SapiensName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SapiensNameError {
    #[error("Sapiens name cannot be empty")]
    Empty,
    #[error("Sapiens name cannot exceed 100 characters")]
    TooLong,
    #[error("Sapiens name contains invalid characters")]
    InvalidCharacters,
}

// SapiensStatus Value Object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SapiensStatus {
    #[default]
    Active,
    Inactive,
    Suspended,
    Archived,
}

impl SapiensStatus {
    pub fn value(&self) -> &'static str {
        match self {
            Self::Active => "ACTIVE",
            Self::Inactive => "INACTIVE",
            Self::Suspended => "SUSPENDED",
            Self::Archived => "ARCHIVED",
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn is_inactive(&self) -> bool {
        matches!(self, Self::Inactive)
    }

    pub fn is_suspended(&self) -> bool {
        matches!(self, Self::Suspended)
    }

    pub fn is_archived(&self) -> bool {
        matches!(self, Self::Archived)
    }

    pub fn can_transition_to(&self, target: &SapiensStatus) -> bool {
        use SapiensStatus::*;

        match (self, target) {
            // From any state to same state
            (s, t) if s == t => true,

            // From Active
            (Active, Inactive) => true,
            (Active, Suspended) => true,
            (Active, Archived) => true,

            // From Inactive
            (Inactive, Active) => true,
            (Inactive, Suspended) => true,
            (Inactive, Archived) => true,

            // From Suspended
            (Suspended, Active) => true,
            (Suspended, Inactive) => true,
            (Suspended, Archived) => true,

            // From Archived (can only transition back to Inactive)
            (Archived, Inactive) => true,

            // All other transitions are invalid
            _ => false,
        }
    }

    pub fn all_statuses() -> Vec<&'static str> {
        vec!["ACTIVE", "INACTIVE", "SUSPENDED", "ARCHIVED"]
    }
}


impl fmt::Display for SapiensStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl From<&str> for SapiensStatus {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "ACTIVE" => Self::Active,
            "INACTIVE" => Self::Inactive,
            "SUSPENDED" => Self::Suspended,
            "ARCHIVED" => Self::Archived,
            _ => Self::Active, // Default fallback
        }
    }
}

impl From<String> for SapiensStatus {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

// SapiensTimestamp Value Object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SapiensTimestamp(DateTime<Utc>);

impl SapiensTimestamp {
    pub fn new(timestamp: DateTime<Utc>) -> Self {
        Self(timestamp)
    }

    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn from_timestamp(timestamp: i64) -> Option<Self> {
        DateTime::from_timestamp(timestamp, 0).map(Self)
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    pub fn iso8601(&self) -> String {
        self.0.to_rfc3339()
    }

    pub fn is_future(&self) -> bool {
        self.0 > Utc::now()
    }

    pub fn is_past(&self) -> bool {
        self.0 < Utc::now()
    }

    pub fn add_days(&self, days: i64) -> Self {
        Self(self.0 + chrono::Duration::days(days))
    }

    pub fn add_hours(&self, hours: i64) -> Self {
        Self(self.0 + chrono::Duration::hours(hours))
    }
}

impl fmt::Display for SapiensTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.iso8601())
    }
}

impl Default for SapiensTimestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for SapiensTimestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }
}

impl From<i64> for SapiensTimestamp {
    fn from(timestamp: i64) -> Self {
        Self(DateTime::from_timestamp(timestamp, 0).unwrap_or_default())
    }
}

// SapiensVersion Value Object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SapiensVersion(i64);

impl SapiensVersion {
    pub fn new(version: i64) -> Result<Self, SapiensVersionError> {
        if version < 0 {
            return Err(SapiensVersionError::Negative);
        }

        Ok(SapiensVersion(version))
    }

    pub fn initial() -> Self {
        Self(1)
    }

    pub fn first() -> Self {
        Self(1)
    }

    pub fn value(&self) -> i64 {
        self.0
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    pub fn is_first(&self) -> bool {
        self.0 == 1
    }

    pub fn greater_than(&self, other: &SapiensVersion) -> bool {
        self.0 > other.0
    }
}

impl fmt::Display for SapiensVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for SapiensVersion {
    fn default() -> Self {
        Self::initial()
    }
}

impl From<i64> for SapiensVersion {
    fn from(version: i64) -> Self {
        Self(version.max(0))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SapiensVersionError {
    #[error("Sapiens version cannot be negative")]
    Negative,
}

// Metadata Value Object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    data: std::collections::HashMap<String, String>,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }

    pub fn from_map(data: std::collections::HashMap<String, String>) -> Result<Self, MetadataError> {
        if data.is_empty() {
            return Err(MetadataError::Empty);
        }

        // Validate key-value pairs
        for (key, value) in &data {
            if key.is_empty() {
                return Err(MetadataError::EmptyKey);
            }
            if key.len() > 50 {
                return Err(MetadataError::KeyTooLong);
            }
            if value.len() > 500 {
                return Err(MetadataError::ValueTooLong);
            }
        }

        Ok(Self { data })
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: std::collections::HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<(), MetadataError> {
        if key.is_empty() {
            return Err(MetadataError::EmptyKey);
        }
        if key.len() > 50 {
            return Err(MetadataError::KeyTooLong);
        }
        if value.len() > 500 {
            return Err(MetadataError::ValueTooLong);
        }

        self.data.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.data.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &String> {
        self.data.values()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.data.iter()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn to_map(&self) -> std::collections::HashMap<String, String> {
        self.data.clone()
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("Metadata cannot be empty")]
    Empty,
    #[error("Metadata key cannot be empty")]
    EmptyKey,
    #[error("Metadata key cannot exceed 50 characters")]
    KeyTooLong,
    #[error("Metadata value cannot exceed 500 characters")]
    ValueTooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sapiens_id() {
        // Valid UUID
        let id = SapiensId::new("123e4567-e89b-12d3-a456-426614174000").unwrap();
        assert_eq!(id.value(), "123e4567-e89b-12d3-a456-426614174000");

        // Invalid UUID format
        assert!(matches!(
            SapiensId::new("invalid-uuid"),
            Err(SapiensIdError::InvalidFormat)
        ));

        // Empty ID
        assert!(matches!(
            SapiensId::new(""),
            Err(SapiensIdError::Empty)
        ));
    }

    #[test]
    fn test_sapiens_name() {
        // Valid names
        let name = SapiensName::new("Test Sapiens").unwrap();
        assert_eq!(name.value(), "Test Sapiens");
        assert_eq!(name.length(), 12);

        let name = SapiensName::new("sapiens-test_123").unwrap();
        assert_eq!(name.value(), "sapiens-test_123");

        // Empty name
        assert!(matches!(
            SapiensName::new(""),
            Err(SapiensNameError::Empty)
        ));

        // Too long name
        let long_name = "a".repeat(101);
        assert!(matches!(
            SapiensName::new(&long_name),
            Err(SapiensNameError::TooLong)
        ));

        // Invalid characters
        assert!(matches!(
            SapiensName::new("test@sapiens"),
            Err(SapiensNameError::InvalidCharacters)
        ));
    }

    #[test]
    fn test_sapiens_status() {
        let status = SapiensStatus::Active;
        assert!(status.is_active());
        assert!(!status.is_inactive());

        // Test transitions
        assert!(status.can_transition_to(&SapiensStatus::Inactive));
        assert!(status.can_transition_to(&SapiensStatus::Suspended));
        assert!(status.can_transition_to(&SapiensStatus::Archived));
        assert!(!SapiensStatus::Archived.can_transition_to(&SapiensStatus::Active));
    }

    #[test]
    fn test_sapiens_timestamp() {
        let now = SapiensTimestamp::now();
        assert!(!now.is_future());
        // Note: is_past() may return true immediately due to timing, so we skip that check for 'now'

        let future = now.add_days(1);
        assert!(future.is_future());

        let past = now.add_hours(-1);
        assert!(past.is_past());
    }

    #[test]
    fn test_sapiens_version() {
        let version = SapiensVersion::initial();
        assert!(version.is_first());
        assert_eq!(version.value(), 1);

        let next = version.next();
        assert_eq!(next.value(), 2);
        assert!(next.greater_than(&version));
    }

    #[test]
    fn test_metadata() {
        let mut metadata = Metadata::new();
        assert!(metadata.is_empty());

        metadata.insert("key".to_string(), "value".to_string()).unwrap();
        assert_eq!(metadata.len(), 1);
        assert_eq!(metadata.get("key"), Some(&"value".to_string()));

        // Test invalid key
        assert!(matches!(
            metadata.insert("".to_string(), "value".to_string()),
            Err(MetadataError::EmptyKey)
        ));

        // Test invalid value
        let long_value = "a".repeat(501);
        assert!(matches!(
            metadata.insert("key".to_string(), long_value),
            Err(MetadataError::ValueTooLong)
        ));
    }
}