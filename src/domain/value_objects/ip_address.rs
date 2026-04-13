//! IP address value object
//!
//! Provides IP address validation and manipulation.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::{IpAddr, AddrParseError};

/// IP address value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IpAddress(String);

impl IpAddress {
    /// Create a new IP address
    pub fn new(ip: &str) -> Result<Self, ValidationError> {
        if ip.is_empty() {
            return Err(ValidationError("IP address cannot be empty"));
        }

        // Validate IP address format
        if ip.parse::<IpAddr>().is_err() {
            return Err(ValidationError("Invalid IP address format"));
        }

        Ok(IpAddress(ip.to_string()))
    }

    /// Create an IP address without validation (for trusted sources)
    pub fn from_unchecked(ip: &str) -> Self {
        IpAddress(ip.to_string())
    }

    /// Get the IP address as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the IP address as a string reference
    pub fn as_string(&self) -> String {
        self.0.clone()
    }

    /// Consume and return the inner string
    pub fn into_string(self) -> String {
        self.0
    }

    /// Try to parse as std::net::IpAddr
    pub fn as_ip_addr(&self) -> Result<IpAddr, AddrParseError> {
        self.0.parse::<IpAddr>()
    }

    /// Check if this is an IPv4 address
    pub fn is_ipv4(&self) -> bool {
        self.as_ip_addr().map(|ip| ip.is_ipv4()).unwrap_or(false)
    }

    /// Check if this is an IPv6 address
    pub fn is_ipv6(&self) -> bool {
        self.as_ip_addr().map(|ip| ip.is_ipv6()).unwrap_or(false)
    }

    /// Check if this is a private IP address
    pub fn is_private(&self) -> bool {
        self.as_ip_addr().map(|ip| match ip {
            std::net::IpAddr::V4(ipv4) => {
                let octets = ipv4.octets();
                match octets[0] {
                    10 => true, // 10.0.0.0/8
                    172 => octets[1] >= 16 && octets[1] <= 31, // 172.16.0.0/12
                    192 => octets[1] == 168, // 192.168.0.0/16
                    _ => false,
                }
            }
            std::net::IpAddr::V6(ipv6) => {
                ipv6.segments()[0] == 0xfdfe || ipv6.segments()[0] == 0xfdff // fc00::/7
            }
        }).unwrap_or(false)
    }

    /// Check if this is a loopback IP address
    pub fn is_loopback(&self) -> bool {
        self.as_ip_addr().map(|ip| ip.is_loopback()).unwrap_or(false)
    }

    /// Create from string
    pub fn from(s: String) -> Self {
        IpAddress(s)
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for IpAddress {
    fn from(s: String) -> Self {
        IpAddress(s)
    }
}

impl From<&str> for IpAddress {
    fn from(s: &str) -> Self {
        IpAddress(s.to_string())
    }
}

/// Validation error type for IP address
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
    use std::net::Ipv4Addr;

    #[test]
    fn test_ip_address_creation() {
        // Valid IPv4
        assert!(IpAddress::new("192.168.1.1").is_ok());
        assert!(IpAddress::new("127.0.0.1").is_ok());

        // Valid IPv6
        assert!(IpAddress::new("::1").is_ok());
        assert!(IpAddress::new("2001:db8::1").is_ok());

        // Invalid IP
        assert!(IpAddress::new("").is_err());
        assert!(IpAddress::new("invalid").is_err());
        assert!(IpAddress::new("256.256.256.256").is_err());
    }

    #[test]
    fn test_ip_address_properties() {
        let ipv4 = IpAddress::new("192.168.1.1").unwrap();
        assert!(ipv4.is_ipv4());
        assert!(!ipv4.is_ipv6());
        assert!(ipv4.is_private());
        assert!(!ipv4.is_loopback());

        let loopback = IpAddress::new("127.0.0.1").unwrap();
        assert!(loopback.is_loopback());

        let ipv6 = IpAddress::new("2001:db8::1").unwrap();
        assert!(ipv6.is_ipv6());
        assert!(!ipv6.is_ipv4());
    }

    #[test]
    fn test_ip_address_conversions() {
        let ip1: IpAddress = "192.168.1.1".into();
        assert_eq!(ip1.as_str(), "192.168.1.1");

        let ip2: IpAddress = "192.168.1.1".to_string().into();
        assert_eq!(ip2.as_str(), "192.168.1.1");

        assert_eq!(ip1, ip2);
    }

    #[test]
    fn test_ip_addr_parsing() {
        let ip = IpAddress::new("192.168.1.1").unwrap();
        assert!(ip.as_ip_addr().is_ok());

        let parsed = ip.as_ip_addr().unwrap();
        assert_eq!(parsed, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
    }
}