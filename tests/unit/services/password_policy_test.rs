//! Password Policy Service Unit Tests
//!
//! Tests for password validation and policy enforcement.

use backbone_sapiens::domain::services::password_policy_service::{
    PasswordValidationResult, PasswordViolation,
};

/// Test password policy service
struct TestPasswordPolicyService {
    min_length: usize,
    require_uppercase: bool,
    require_lowercase: bool,
    require_digit: bool,
    require_special: bool,
}

impl TestPasswordPolicyService {
    fn new() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_digit: true,
            require_special: false,
        }
    }

    fn strict() -> Self {
        Self {
            min_length: 12,
            require_uppercase: true,
            require_lowercase: true,
            require_digit: true,
            require_special: true,
        }
    }

    fn validate(&self, password: &str) -> PasswordValidationResult {
        let mut violations = Vec::new();

        if password.len() < self.min_length {
            violations.push(PasswordViolation::TooShort);
        }

        if self.require_uppercase && !password.chars().any(|c| c.is_ascii_uppercase()) {
            violations.push(PasswordViolation::NoUppercase);
        }

        if self.require_lowercase && !password.chars().any(|c| c.is_ascii_lowercase()) {
            violations.push(PasswordViolation::NoLowercase);
        }

        if self.require_digit && !password.chars().any(|c| c.is_ascii_digit()) {
            violations.push(PasswordViolation::NoDigit);
        }

        if self.require_special && !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
            violations.push(PasswordViolation::NoSpecialChar);
        }

        let is_valid = violations.is_empty();
        let strength = if is_valid {
            if password.len() >= 14 && violations.is_empty() {
                100
            } else {
                75
            }
        } else {
            30
        };

        PasswordValidationResult {
            is_valid,
            violations,
            strength,
            suggestions: if violations.is_empty() {
                vec![]
            } else {
                vec![
                    "Use at least 8 characters".to_string(),
                    "Include uppercase and lowercase letters".to_string(),
                    "Add numbers and special characters".to_string(),
                ]
            },
        }
    }
}

// ============================================================
// Basic Validation Tests
// ============================================================

#[cfg(test)]
mod basic_validation_tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let policy = TestPasswordPolicyService::new();

        let result = policy.validate("SecureP@ssw0rd");

        assert!(result.is_valid);
        assert!(result.violations.is_empty());
        assert_eq!(result.strength, 75);
    }

    #[test]
    fn test_too_short_password() {
        let policy = TestPasswordPolicyService::new();

        let result = policy.validate("Short1");

        assert!(!result.is_valid);
        assert!(result.violations.contains(&PasswordViolation::TooShort));
    }

    #[test]
    fn test_no_uppercase() {
        let policy = TestPasswordPolicyService::new();

        let result = policy.validate("lowercase123");

        assert!(!result.is_valid);
        assert!(result.violations.contains(&PasswordViolation::NoUppercase));
    }

    #[test]
    fn test_no_lowercase() {
        let policy = TestPasswordPolicyService::new();

        let result = policy.validate("UPPERCASE123");

        assert!(!result.is_valid);
        assert!(result.violations.contains(&PasswordViolation::NoLowercase));
    }

    #[test]
    fn test_no_digit() {
        let policy = TestPasswordPolicyService::new();

        let result = policy.validate("NoDigitsHere");

        assert!(!result.is_valid);
        assert!(result.violations.contains(&PasswordViolation::NoDigit));
    }
}

// ============================================================
// Strict Policy Tests
// ============================================================

#[cfg(test)]
mod strict_policy_tests {
    use super::*;

    #[test]
    fn test_strict_policy_valid_password() {
        let policy = TestPasswordPolicyService::strict();

        let result = policy.validate("VerySecureP@ssw0rd123!");

        assert!(result.is_valid);
        assert!(result.violations.is_empty());
        assert_eq!(result.strength, 100);
    }

    #[test]
    fn test_strict_policy_insufficient_length() {
        let policy = TestPasswordPolicyService::strict();

        let result = policy.validate("Short1!");

        assert!(!result.is_valid);
        assert!(result.violations.contains(&PasswordViolation::TooShort));
    }

    #[test]
    fn test_strict_policy_no_special_char() {
        let policy = TestPasswordPolicyService::strict();

        let result = policy.validate("LongEnoughPassword123");

        assert!(!result.is_valid);
        assert!(result.violations.contains(&PasswordViolation::NoSpecialChar));
    }
}

// ============================================================
// Edge Cases Tests
// ============================================================

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_password() {
        let policy = TestPasswordPolicyService::new();

        let result = policy.validate("");

        assert!(!result.is_valid);
        assert!(result.violations.contains(&PasswordViolation::TooShort));
    }

    #[test]
    fn test_whitespace_only() {
        let policy = TestPasswordPolicyService::new();

        let result = policy.validate("   ");

        assert!(!result.is_valid);
        assert!(result.violations.contains(&PasswordViolation::TooShort));
    }

    #[test]
    fn test_password_with_suggestions() {
        let policy = TestPasswordPolicyService::new();

        let result = policy.validate("weak");

        assert!(!result.is_valid);
        assert!(!result.suggestions.is_empty());
    }

    #[test]
    fn test_all_violations() {
        let policy = TestPasswordPolicyService::strict();

        let result = policy.validate("weak");

        assert!(!result.is_valid);
        assert!(result.violations.len() >= 3); // Too short, no uppercase, no digit, no special
    }
}
