//! Value Object Unit Tests
//!
//! Tests for value objects including Email, PasswordHash, DeviceFingerprint, etc.

use backbone_sapiens::domain::value_objects::{Email, DeviceFingerprint, IpAddress};
use std::net::IpAddr as StdIpAddr;
use std::str::FromStr;

// ============================================================
// Email Value Object Tests
// ============================================================

#[cfg(test)]
mod email_tests {
    use super::*;

    /// Test valid email creation
    #[test]
    fn test_valid_email_creation() {
        let valid_emails = vec![
            "test@example.com",
            "user.name@example.com",
            "user+tag@example.co.uk",
            "user-name@test.example.com",
        ];

        for email in valid_emails {
            let result = Email::new(email);
            assert!(result.is_ok(), "Email {} should be valid", email);
            assert_eq!(result.unwrap().to_string(), email);
        }
    }

    /// Test invalid email creation
    #[test]
    fn test_invalid_email_creation() {
        let invalid_emails = vec![
            "",
            "invalid",
            "@example.com",
            "user@",
            "user name@example.com",
            "user@example",
            "user..name@example.com",
        ];

        for email in invalid_emails {
            let result = Email::new(email);
            assert!(result.is_err(), "Email {} should be invalid", email);
        }
    }

    /// Test email display
    #[test]
    fn test_email_display() {
        let email = Email::new("test@example.com").unwrap();
        assert_eq!(format!("{}", email), "test@example.com");
        assert_eq!(email.to_string(), "test@example.com");
    }

    /// Test email equality
    #[test]
    fn test_email_equality() {
        let email1 = Email::new("test@example.com").unwrap();
        let email2 = Email::new("test@example.com").unwrap();
        let email3 = Email::new("other@example.com").unwrap();

        assert_eq!(email1, email2);
        assert_ne!(email1, email3);
    }
}

// ============================================================
// Device Fingerprint Value Object Tests
// ============================================================

#[cfg(test)]
mod device_fingerprint_tests {
    use super::*;

    /// Test device fingerprint from string
    #[test]
    fn test_device_fingerprint_from_string() {
        let fp_str = "fp_abc123def456";
        let fp = DeviceFingerprint::from(fp_str.to_string());

        assert_eq!(fp.to_string(), fp_str);
    }

    /// Test device fingerprint generation
    #[test]
    fn test_device_fingerprint_generation() {
        let fp1 = DeviceFingerprint::generate();
        let fp2 = DeviceFingerprint::generate();

        // Generated fingerprints should be unique
        assert_ne!(fp1.to_string(), fp2.to_string());

        // Should not be empty
        assert!(!fp1.to_string().is_empty());
        assert!(!fp2.to_string().is_empty());
    }

    /// Test device fingerprint equality
    #[test]
    fn test_device_fingerprint_equality() {
        let fp_str = "fp_test123";
        let fp1 = DeviceFingerprint::from(fp_str.to_string());
        let fp2 = DeviceFingerprint::from(fp_str.to_string());

        assert_eq!(fp1, fp2);
    }
}

// ============================================================
// IP Address Value Object Tests
// ============================================================

#[cfg(test)]
mod ip_address_tests {
    use super::*;

    /// Test IP address from string - IPv4
    #[test]
    fn test_ip_address_from_ipv4() {
        let ip_str = "192.168.1.1";
        let ip = IpAddress::from_std(StdIpAddr::from_str(ip_str).unwrap());

        assert_eq!(ip.to_string(), ip_str);
    }

    /// Test IP address from string - IPv6
    #[test]
    fn test_ip_address_from_ipv6() {
        let ip_str = "::1";
        let ip = IpAddress::from_std(StdIpAddr::from_str(ip_str).unwrap());

        assert_eq!(ip.to_string(), "::1");
    }

    /// Test IP address validation - localhost
    #[test]
    fn test_ip_address_is_local() {
        let localhost = IpAddress::from_std(StdIpAddr::from_str("127.0.0.1").unwrap());
        assert!(localhost.is_loopback());

        let private = IpAddress::from_std(StdIpAddr::from_str("192.168.1.1").unwrap());
        assert!(private.is_private());
    }

    /// Test IP address comparison
    #[test]
    fn test_ip_address_comparison() {
        let ip1 = IpAddress::from_std(StdIpAddr::from_str("192.168.1.1").unwrap());
        let ip2 = IpAddress::from_std(StdIpAddr::from_str("192.168.1.1").unwrap());
        let ip3 = IpAddress::from_std(StdIpAddr::from_str("10.0.0.1").unwrap());

        assert_eq!(ip1, ip2);
        assert_ne!(ip1, ip3);
    }
}

// ============================================================
// UserStatus Enum Tests
// ============================================================

#[cfg(test)]
mod user_status_tests {
    use backbone_sapiens::domain::entity::UserStatus;
    use std::str::FromStr;

    /// Test UserStatus display
    #[test]
    fn test_user_status_display() {
        assert_eq!(format!("{}", UserStatus::Active), "active");
        assert_eq!(format!("{}", UserStatus::Inactive), "inactive");
        assert_eq!(format!("{}", UserStatus::Suspended), "suspended");
        assert_eq!(format!("{}", UserStatus::PendingVerification), "pending_verification");
    }

    /// Test UserStatus from string
    #[test]
    fn test_user_status_from_str() {
        assert_eq!(UserStatus::from_str("active").unwrap(), UserStatus::Active);
        assert_eq!(UserStatus::from_str("ACTIVE").unwrap(), UserStatus::Active);
        assert_eq!(UserStatus::from_str("inactive").unwrap(), UserStatus::Inactive);
        assert_eq!(UserStatus::from_str("suspended").unwrap(), UserStatus::Suspended);
        assert_eq!(UserStatus::from_str("pending_verification").unwrap(), UserStatus::PendingVerification);
    }

    /// Test UserStatus from string - invalid
    #[test]
    fn test_user_status_from_str_invalid() {
        assert!(UserStatus::from_str("unknown").is_err());
        assert!(UserStatus::from_str("").is_err());
    }

    /// Test UserStatus default
    #[test]
    fn test_user_status_default() {
        assert_eq!(UserStatus::default(), UserStatus::PendingVerification);
    }
}

// ============================================================
// DeviceType Enum Tests
// ============================================================

#[cfg(test)]
mod device_type_tests {
    use backbone_sapiens::domain::entity::DeviceType;
    use std::str::FromStr;

    /// Test DeviceType display
    #[test]
    fn test_device_type_display() {
        assert_eq!(format!("{}", DeviceType::Web), "web");
        assert_eq!(format!("{}", DeviceType::Mobile), "mobile");
        assert_eq!(format!("{}", DeviceType::Tablet), "tablet");
        assert_eq!(format!("{}", DeviceType::Desktop), "desktop");
        assert_eq!(format!("{}", DeviceType::Unknown), "unknown");
    }

    /// Test DeviceType from string
    #[test]
    fn test_device_type_from_str() {
        assert_eq!(DeviceType::from_str("web").unwrap(), DeviceType::Web);
        assert_eq!(DeviceType::from_str("WEB").unwrap(), DeviceType::Web);
        assert_eq!(DeviceType::from_str("mobile").unwrap(), DeviceType::Mobile);
        assert_eq!(DeviceType::from_str("tablet").unwrap(), DeviceType::Tablet);
        assert_eq!(DeviceType::from_str("desktop").unwrap(), DeviceType::Desktop);
        assert_eq!(DeviceType::from_str("unknown").unwrap(), DeviceType::Unknown);
    }

    /// Test DeviceType from string - invalid
    #[test]
    fn test_device_type_from_str_invalid() {
        assert!(DeviceType::from_str("invalid").is_err());
    }

    /// Test DeviceType default
    #[test]
    fn test_device_type_default() {
        assert_eq!(DeviceType::default(), DeviceType::Unknown);
    }
}
