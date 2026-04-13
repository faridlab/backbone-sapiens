//! Password strength value object
//!
//! Provides password strength assessment and validation.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Password strength levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PasswordStrength {
    /// Very weak password (e.g., "123", "password")
    VeryWeak,
    /// Weak password (e.g., common words, short length)
    Weak,
    /// Moderate password (mixed case, some numbers)
    Moderate,
    /// Strong password (mixed case, numbers, symbols)
    Strong,
    /// Very strong password (long, complex, high entropy)
    VeryStrong,
}

impl PasswordStrength {
    /// Calculate password strength from a password string
    pub fn calculate(password: &str) -> Self {
        let mut score = 0;

        // Length contribution
        if password.len() >= 8 {
            score += 1;
        }
        if password.len() >= 12 {
            score += 1;
        }
        if password.len() >= 16 {
            score += 1;
        }

        // Character variety
        if password.chars().any(|c| c.is_lowercase()) {
            score += 1;
        }
        if password.chars().any(|c| c.is_uppercase()) {
            score += 1;
        }
        if password.chars().any(|c| c.is_numeric()) {
            score += 1;
        }
        if password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
            score += 1;
        }

        // Avoid common patterns
        if !Self::is_common_pattern(password) {
            score += 1;
        }

        match score {
            0..=2 => PasswordStrength::VeryWeak,
            3..=4 => PasswordStrength::Weak,
            5..=6 => PasswordStrength::Moderate,
            7..=8 => PasswordStrength::Strong,
            9..=10 => PasswordStrength::VeryStrong,
            _ => PasswordStrength::VeryStrong,
        }
    }

    /// Check if password matches common patterns
    fn is_common_pattern(password: &str) -> bool {
        let common_patterns = [
            "password", "123456", "qwerty", "abc123", "password123",
            "admin", "letmein", "welcome", "monkey", "dragon"
        ];

        let lowercase = password.to_lowercase();
        common_patterns.iter().any(|pattern| lowercase.contains(pattern))
            || password.chars().collect::<std::collections::HashSet<_>>().len() < 4
    }

    /// Get numerical score (0-100)
    pub fn score(&self) -> u8 {
        match self {
            PasswordStrength::VeryWeak => 20,
            PasswordStrength::Weak => 40,
            PasswordStrength::Moderate => 60,
            PasswordStrength::Strong => 80,
            PasswordStrength::VeryStrong => 100,
        }
    }

    /// Get overall strength score (alias for score)
    pub fn overall_score(&self) -> u8 {
        self.score()
    }

    /// Check if password meets minimum strength requirements
    pub fn meets_minimum(&self, minimum: Self) -> bool {
        let order = [
            PasswordStrength::VeryWeak,
            PasswordStrength::Weak,
            PasswordStrength::Moderate,
            PasswordStrength::Strong,
            PasswordStrength::VeryStrong,
        ];

        let self_index = order.iter().position(|s| s == self).unwrap_or(0);
        let min_index = order.iter().position(|s| s == &minimum).unwrap_or(0);
        self_index >= min_index
    }
}

impl fmt::Display for PasswordStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordStrength::VeryWeak => write!(f, "Very Weak"),
            PasswordStrength::Weak => write!(f, "Weak"),
            PasswordStrength::Moderate => write!(f, "Moderate"),
            PasswordStrength::Strong => write!(f, "Strong"),
            PasswordStrength::VeryStrong => write!(f, "Very Strong"),
        }
    }
}

impl Default for PasswordStrength {
    fn default() -> Self {
        PasswordStrength::Weak
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_strength_calculation() {
        // Score: len>=8(+1) + lowercase(+1) + uppercase(+1) + digit(+1) + special(+1) + not_common(+1) + len>=12(+1) + len>=16(+1)
        // VeryWeak: 0-2, Weak: 3-4, Moderate: 5-6, Strong: 7-8, VeryStrong: 9+

        // "123" - len<8, digits only = score 1 = VeryWeak
        assert_eq!(PasswordStrength::calculate("123"), PasswordStrength::VeryWeak);
        // "password" - len>=8(+1), lowercase(+1), common pattern(+0) = score 2 = VeryWeak
        assert_eq!(PasswordStrength::calculate("password"), PasswordStrength::VeryWeak);
        // "xyzlongpw" - len>=8(+1), lowercase(+1), not_common(+1) = score 3 = Weak
        assert_eq!(PasswordStrength::calculate("xyzlongpw"), PasswordStrength::Weak);
        // "Password1" - contains "password" (common) = score 4 = Weak
        assert_eq!(PasswordStrength::calculate("Password1"), PasswordStrength::Weak);
        // "Xyztrek1" - len>=8(+1), lower(+1), upper(+1), digit(+1), not_common(+1) = 5 = Moderate
        assert_eq!(PasswordStrength::calculate("Xyztrek1"), PasswordStrength::Moderate);
        // "Xyztr3k123!" - 11 chars: len>=8(+1), lower(+1), upper(+1), digit(+1), special(+1), not_common(+1) = 6 = Moderate
        assert_eq!(PasswordStrength::calculate("Xyztr3k123!"), PasswordStrength::Moderate);
        // "Xyztr3k123!ab" - 13 chars: +len>=12(+1) = 7 = Strong
        assert_eq!(PasswordStrength::calculate("Xyztr3k123!ab"), PasswordStrength::Strong);
        // "VeryComplexXyz123!@#" - 20 chars: all categories + len>=16 = 8 = Strong
        assert_eq!(PasswordStrength::calculate("VeryComplexXyz123!@#"), PasswordStrength::Strong);
    }

    #[test]
    fn test_strength_scores() {
        assert_eq!(PasswordStrength::VeryWeak.score(), 20);
        assert_eq!(PasswordStrength::Weak.score(), 40);
        assert_eq!(PasswordStrength::Moderate.score(), 60);
        assert_eq!(PasswordStrength::Strong.score(), 80);
        assert_eq!(PasswordStrength::VeryStrong.score(), 100);
    }

    #[test]
    fn test_minimum_requirements() {
        let strong = PasswordStrength::Strong;
        let moderate = PasswordStrength::Moderate;

        assert!(strong.meets_minimum(PasswordStrength::Moderate));
        assert!(!moderate.meets_minimum(PasswordStrength::Strong));
        assert!(moderate.meets_minimum(PasswordStrength::Moderate));
    }
}