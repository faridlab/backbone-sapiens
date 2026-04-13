//! Email address value object with validation
//!
//! Provides type-safe email validation and manipulation.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Email address value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, ValidationError> {
        if email.is_empty() {
            return Err(ValidationError("Email cannot be empty"));
        }

        if !email.contains('@') {
            return Err(ValidationError("Invalid email format"));
        }

        if email.len() > 255 {
            return Err(ValidationError("Email too long"));
        }

        Ok(Email(email.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Email {
    fn from(s: String) -> Self {
        // This should ideally use Email::new, but for compatibility we'll accept the string
        Email(s.to_lowercase())
    }
}

impl From<&str> for Email {
    fn from(s: &str) -> Self {
        Email(s.to_lowercase())
    }
}

impl From<&String> for Email {
    fn from(s: &String) -> Self {
        Email(s.to_lowercase())
    }
}

/// Validation error type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(pub &'static str);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        // Valid emails
        assert!(Email::new("test@example.com").is_ok());
        assert!(Email::new("user.name+tag@domain.co.uk").is_ok());

        // Invalid emails
        assert!(Email::new("").is_err());
        assert!(Email::new("invalid-email").is_err());
        assert!(Email::new("toolong".repeat(100).as_str()).is_err());
    }

    #[test]
    fn test_email_case_insensitive() {
        let email1 = Email::new("Test@Example.com").unwrap();
        let email2 = Email::new("test@example.com").unwrap();
        assert_eq!(email1, email2);
    }
}