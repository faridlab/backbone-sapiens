//! Security Monitoring Service
//!
//! Monitors and tracks security-related events including failed login attempts,
//! account lockouts, device trust management, and suspicious activity detection.
//!
//! This service manages:
//! - Failed login attempt tracking (5 attempts = 15 min lockout)
//! - Device fingerprinting and trust management
//! - Geographic anomaly detection
//! - Security event logging and alerting

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use std::net::IpAddr;

use crate::domain::value_objects::{DeviceFingerprint, IpAddress};
use crate::domain::event::security_event_events::{SecurityEventEventPublisher, SecurityEventEvent};
use crate::domain::entity::SecurityEvent as DomainSecurityEvent;
use crate::domain::entity::SecurityEventType as DomainSecurityEventType;
use crate::domain::entity::SecurityEventSeverity;
use crate::domain::entity::AuditMetadata;
use backbone_messaging::EventBus;

/// Security event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    FailedLoginAttempt,
    AccountLocked,
    DeviceUntrusted,
    SuspiciousLocation,
    PasswordResetRequested,
    MfaEnabled,
    MfaDisabled,
    MfaVerificationSucceeded,
    MfaVerificationFailed,
    MfaSetupInitiated,
    MfaBackupCodesGenerated,
    MfaDeviceRemoved,
    MfaPreferencesUpdated,
    PasswordChanged,
    AccountCompromised,
}

/// Security event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: SecurityEventType,
    pub timestamp: DateTime<Utc>,
    pub ip_address: Option<IpAddress>,
    pub device_fingerprint: Option<DeviceFingerprint>,
    pub location: Option<SecurityLocation>,
    pub details: HashMap<String, String>,
    pub risk_score: u8, // 0-100
}

/// Security location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLocation {
    pub country: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_known_location: bool,
}

/// Device trust status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceTrustStatus {
    Unknown,
    Trusted,
    Suspicious,
    Blocked,
    Untrusted,
}

/// Security monitoring service trait
#[async_trait]
pub trait SecurityMonitoringService: Send + Sync {
    /// Record a failed login attempt
    async fn record_failed_attempt(&self, user_id: Uuid) -> Result<(), SecurityError>;

    /// Get number of failed attempts for user
    async fn get_failed_attempts(&self, user_id: Uuid) -> Result<u32, SecurityError>;

    /// Reset failed attempts for user
    async fn reset_failed_attempts(&self, user_id: Uuid) -> Result<(), SecurityError>;

    /// Check if user is locked out
    async fn is_user_locked(&self, user_id: Uuid) -> Result<bool, SecurityError>;

    /// Lock user account
    async fn lock_user_account(&self, user_id: Uuid, reason: &str, duration_minutes: u32) -> Result<(), SecurityError>;

    /// Get device trust status
    async fn get_device_trust_status(&self, user_id: Uuid, device_fingerprint: &DeviceFingerprint) -> Result<DeviceTrustStatus, SecurityError>;

    /// Trust device
    async fn trust_device(&self, user_id: Uuid, device_fingerprint: &DeviceFingerprint) -> Result<(), SecurityError>;

    /// Untrust device
    async fn untrust_device(&self, user_id: Uuid, device_fingerprint: &DeviceFingerprint) -> Result<(), SecurityError>;

    /// Detect suspicious activity
    async fn detect_suspicious_activity(&self, user_id: Uuid, ip_address: Option<IpAddress>, device_fingerprint: Option<DeviceFingerprint>) -> Result<Vec<SecurityRisk>, SecurityError>;

    /// Log security event
    async fn log_security_event(&self, event: SecurityEvent) -> Result<(), SecurityError>;

    /// Get recent security events for user
    async fn get_user_security_events(&self, user_id: Uuid, since: DateTime<Utc>) -> Result<Vec<SecurityEvent>, SecurityError>;

    /// Check for geographic anomalies
    async fn check_geographic_anomaly(&self, user_id: Uuid, current_location: SecurityLocation) -> Result<bool, SecurityError>;
}

/// Security risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRisk {
    pub risk_type: String,
    pub risk_score: u8,
    pub description: String,
    pub recommendation: String,
    pub requires_action: bool,
}

/// Security errors
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Security event not found: {0}")]
    EventNotFound(Uuid),

    #[error("User not found: {0}")]
    UserNotFound(Uuid),

    #[error("Security violation detected: {0}")]
    SecurityViolation(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Default implementation of Security Monitoring Service
pub struct DefaultSecurityMonitoringService {
    // In-memory storage for demo purposes. In production, use persistent storage.
    failed_attempts: std::sync::Mutex<HashMap<Uuid, Vec<DateTime<Utc>>>>,
    locked_accounts: std::sync::Mutex<HashMap<Uuid, DateTime<Utc>>>,
    device_trust: std::sync::Mutex<HashMap<(Uuid, String), DeviceTrustStatus>>,
    security_events: std::sync::Mutex<Vec<SecurityEvent>>,
    known_locations: std::sync::Mutex<HashMap<Uuid, Vec<SecurityLocation>>>,
    event_publisher: Option<SecurityEventEventPublisher>,
}

impl DefaultSecurityMonitoringService {
    pub fn new() -> Self {
        Self {
            failed_attempts: std::sync::Mutex::new(HashMap::new()),
            locked_accounts: std::sync::Mutex::new(HashMap::new()),
            device_trust: std::sync::Mutex::new(HashMap::new()),
            security_events: std::sync::Mutex::new(Vec::new()),
            known_locations: std::sync::Mutex::new(HashMap::new()),
            event_publisher: None,
        }
    }

    /// Create with domain event publishing enabled
    pub fn new_with_events(bus: EventBus<SecurityEventEvent>) -> Self {
        Self {
            failed_attempts: std::sync::Mutex::new(HashMap::new()),
            locked_accounts: std::sync::Mutex::new(HashMap::new()),
            device_trust: std::sync::Mutex::new(HashMap::new()),
            security_events: std::sync::Mutex::new(Vec::new()),
            known_locations: std::sync::Mutex::new(HashMap::new()),
            event_publisher: Some(SecurityEventEventPublisher::new(bus)),
        }
    }

    /// Clean up old failed attempts (older than 15 minutes)
    fn cleanup_old_attempts(&self, user_id: Uuid) {
        let mut attempts = self.failed_attempts.lock().unwrap();
        if let Some(user_attempts) = attempts.get_mut(&user_id) {
            let fifteen_minutes_ago = Utc::now() - Duration::minutes(15);
            user_attempts.retain(|&timestamp| timestamp > fifteen_minutes_ago);

            // Remove user entry if no recent attempts
            if user_attempts.is_empty() {
                attempts.remove(&user_id);
            }
        }
    }

    /// Check if IP is from a known suspicious or invalid network.
    ///
    /// Detects bogon/reserved ranges, documentation ranges, and other IPs that
    /// should never appear in legitimate production traffic. Private IPs (10.x,
    /// 192.168.x, 172.16-31.x) are NOT flagged — they are normal internal addresses.
    fn is_suspicious_ip(&self, ip_address: &Option<IpAddress>) -> bool {
        let ip = match ip_address {
            Some(ip) => ip,
            None => return false,
        };

        // Loopback is not suspicious (local development/testing)
        if ip.is_loopback() {
            return false;
        }

        // Parse to std::net::IpAddr for proper CIDR matching
        let addr = match ip.as_ip_addr() {
            Ok(addr) => addr,
            Err(_) => return true, // Unparseable IP is suspicious
        };

        match addr {
            IpAddr::V4(ipv4) => {
                let octets = ipv4.octets();

                // Bogon/reserved ranges — should never appear in legitimate traffic
                if octets[0] == 0 { return true; }                                       // 0.0.0.0/8
                if octets[0] == 100 && (octets[1] & 0xC0) == 64 { return true; }         // 100.64.0.0/10 CGNAT
                if octets[0] == 169 && octets[1] == 254 { return true; }                  // 169.254.0.0/16 link-local
                if octets[0] >= 224 { return true; }                                      // 224.0.0.0/3 multicast+reserved

                // Documentation/test ranges (RFC 5737, RFC 6890)
                if octets[0] == 192 && octets[1] == 0 && octets[2] == 2 { return true; }    // 192.0.2.0/24 TEST-NET-1
                if octets[0] == 198 && octets[1] == 51 && octets[2] == 100 { return true; } // 198.51.100.0/24 TEST-NET-2
                if octets[0] == 203 && octets[1] == 0 && octets[2] == 113 { return true; }  // 203.0.113.0/24 TEST-NET-3

                // Benchmarking range
                if octets[0] == 198 && (octets[1] & 0xFE) == 18 { return true; }         // 198.18.0.0/15

                false
            }
            IpAddr::V6(ipv6) => {
                let segments = ipv6.segments();

                // Documentation range (RFC 3849)
                if segments[0] == 0x2001 && segments[1] == 0x0db8 { return true; } // 2001:db8::/32

                false
            }
        }
    }

    /// Map local SecurityEventType to domain entity SecurityEventType
    fn map_event_type(local: &SecurityEventType) -> DomainSecurityEventType {
        match local {
            SecurityEventType::FailedLoginAttempt => DomainSecurityEventType::LoginFailed,
            SecurityEventType::AccountLocked => DomainSecurityEventType::AccountLocked,
            SecurityEventType::DeviceUntrusted => DomainSecurityEventType::SuspiciousLogin,
            SecurityEventType::SuspiciousLocation => DomainSecurityEventType::SuspiciousLogin,
            SecurityEventType::PasswordResetRequested => DomainSecurityEventType::PasswordResetRequest,
            SecurityEventType::MfaEnabled => DomainSecurityEventType::MfaEnabled,
            SecurityEventType::MfaDisabled => DomainSecurityEventType::MfaDisabled,
            SecurityEventType::MfaVerificationSucceeded => DomainSecurityEventType::MfaEnabled,
            SecurityEventType::MfaVerificationFailed => DomainSecurityEventType::LoginFailed,
            SecurityEventType::MfaSetupInitiated => DomainSecurityEventType::MfaEnabled,
            SecurityEventType::MfaBackupCodesGenerated => DomainSecurityEventType::MfaEnabled,
            SecurityEventType::MfaDeviceRemoved => DomainSecurityEventType::MfaDisabled,
            SecurityEventType::MfaPreferencesUpdated => DomainSecurityEventType::MfaEnabled,
            SecurityEventType::PasswordChanged => DomainSecurityEventType::PasswordChanged,
            SecurityEventType::AccountCompromised => DomainSecurityEventType::BruteForceAttack,
        }
    }

    /// Map risk score (0-100) to domain severity level
    fn risk_to_severity(risk_score: u8) -> SecurityEventSeverity {
        match risk_score {
            0..=25 => SecurityEventSeverity::Low,
            26..=50 => SecurityEventSeverity::Medium,
            51..=75 => SecurityEventSeverity::High,
            _ => SecurityEventSeverity::Critical,
        }
    }

    /// Convert local SecurityEvent to domain entity for event publishing
    fn to_domain_event(event: &SecurityEvent) -> DomainSecurityEvent {
        DomainSecurityEvent {
            id: event.id,
            user_id: Some(event.user_id),
            session_id: None,
            event_type: Self::map_event_type(&event.event_type),
            severity: Self::risk_to_severity(event.risk_score),
            ip_address: event.ip_address.as_ref().map(|ip| ip.as_string()),
            user_agent: None,
            description: Some(format!("{:?}", event.event_type)),
            details: Some(serde_json::to_value(&event.details).unwrap_or_default()),
            resolved: false,
            resolved_at: None,
            resolved_by_user_id: None,
            resolution_notes: None,
            metadata: AuditMetadata {
                created_at: Some(event.timestamp),
                updated_at: Some(event.timestamp),
                deleted_at: None,
                created_by: Some(event.user_id),
                updated_by: None,
                deleted_by: None,
            },
        }
    }

    /// Calculate security risk score based on various factors
    async fn calculate_risk_score(&self, user_id: Uuid, ip_address: Option<IpAddress>, device_fingerprint: Option<DeviceFingerprint>) -> u8 {
        let mut risk_score = 0;

        // Check for suspicious IP
        if self.is_suspicious_ip(&ip_address) {
            risk_score += 30;
        }

        // Check for unknown device
        if let Some(device) = device_fingerprint {
            let device_trust = self.device_trust.lock().unwrap();
            match device_trust.get(&(user_id, device.as_string())) {
                Some(DeviceTrustStatus::Unknown) => risk_score += 20,
                Some(DeviceTrustStatus::Suspicious) => risk_score += 50,
                Some(DeviceTrustStatus::Blocked) => risk_score += 80,
                _ => {} // Trusted device, no risk
            }
        }

        // Check recent failed attempts
        if let Ok(attempts) = self.get_failed_attempts(user_id).await {
            if attempts > 0 {
                risk_score += (attempts as u8) * 10;
            }
        }

        std::cmp::min(risk_score, 100)
    }

    /// Get known locations for user
    fn get_known_locations(&self, user_id: Uuid) -> Vec<SecurityLocation> {
        self.known_locations.lock().unwrap()
            .get(&user_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Add known location for user
    fn add_known_location(&self, user_id: Uuid, location: SecurityLocation) {
        let mut locations = self.known_locations.lock().unwrap();
        let user_locations = locations.entry(user_id).or_insert_with(Vec::new);

        // Check if location already exists
        if !user_locations.iter().any(|loc| {
            (loc.latitude - location.latitude).abs() < 0.01 &&
            (loc.longitude - location.longitude).abs() < 0.01
        }) {
            user_locations.push(location);
        }

        // Keep only last 10 known locations
        if user_locations.len() > 10 {
            user_locations.remove(0);
        }
    }
}

#[async_trait]
impl SecurityMonitoringService for DefaultSecurityMonitoringService {
    async fn record_failed_attempt(&self, user_id: Uuid) -> Result<(), SecurityError> {
        self.cleanup_old_attempts(user_id);

        let should_lock = {
            let mut attempts = self.failed_attempts.lock().unwrap();
            let user_attempts = attempts.entry(user_id).or_insert_with(Vec::new);
            user_attempts.push(Utc::now());

            // Check if this triggers a lockout
            user_attempts.len() >= 5
        };

        if should_lock {
            let lockout_duration = Duration::minutes(15);
            let locked_until = Utc::now() + lockout_duration;

            {
                let mut locked = self.locked_accounts.lock().unwrap();
                locked.insert(user_id, locked_until);
            } // Mutex guard dropped here

            // Log security event
            self.log_security_event(SecurityEvent {
                id: Uuid::new_v4(),
                user_id,
                event_type: SecurityEventType::AccountLocked,
                timestamp: Utc::now(),
                ip_address: None,
                device_fingerprint: None,
                location: None,
                details: HashMap::from([
                    ("reason".to_string(), "too_many_failed_attempts".to_string()),
                    ("attempts".to_string(), "5".to_string()),
                ]),
                risk_score: 70,
            }).await?;
        }

        Ok(())
    }

    async fn get_failed_attempts(&self, user_id: Uuid) -> Result<u32, SecurityError> {
        self.cleanup_old_attempts(user_id);

        let attempts = self.failed_attempts.lock().unwrap();
        Ok(attempts.get(&user_id).map(|v| v.len() as u32).unwrap_or(0))
    }

    async fn reset_failed_attempts(&self, user_id: Uuid) -> Result<(), SecurityError> {
        let mut attempts = self.failed_attempts.lock().unwrap();
        attempts.remove(&user_id);

        // Also unlock account if it was locked
        let mut locked = self.locked_accounts.lock().unwrap();
        locked.remove(&user_id);

        Ok(())
    }

    async fn is_user_locked(&self, user_id: Uuid) -> Result<bool, SecurityError> {
        let locked = self.locked_accounts.lock().unwrap();

        if let Some(&lockout_until) = locked.get(&user_id) {
            if lockout_until > Utc::now() {
                return Ok(true);
            } else {
                // Lockout expired, remove it by dropping the lock and acquiring a mutable one
                drop(locked);
                self.locked_accounts.lock().unwrap().remove(&user_id);
                return Ok(false);
            }
        }

        Ok(false)
    }

    async fn lock_user_account(&self, user_id: Uuid, reason: &str, duration_minutes: u32) -> Result<(), SecurityError> {
        let lockout_until = Utc::now() + Duration::minutes(duration_minutes as i64);

        {
            let mut locked = self.locked_accounts.lock().unwrap();
            locked.insert(user_id, lockout_until);
        } // Mutex guard dropped here

        // Log security event
        self.log_security_event(SecurityEvent {
            id: Uuid::new_v4(),
            user_id,
            event_type: SecurityEventType::AccountLocked,
            timestamp: Utc::now(),
            ip_address: None,
            device_fingerprint: None,
            location: None,
            details: HashMap::from([
                ("reason".to_string(), reason.to_string()),
                ("duration_minutes".to_string(), duration_minutes.to_string()),
            ]),
            risk_score: 80,
        }).await?;

        Ok(())
    }

    async fn get_device_trust_status(&self, user_id: Uuid, device_fingerprint: &DeviceFingerprint) -> Result<DeviceTrustStatus, SecurityError> {
        let device_trust = self.device_trust.lock().unwrap();
        Ok(device_trust
            .get(&(user_id, device_fingerprint.as_string()))
            .cloned()
            .unwrap_or(DeviceTrustStatus::Unknown))
    }

    async fn trust_device(&self, user_id: Uuid, device_fingerprint: &DeviceFingerprint) -> Result<(), SecurityError> {
        {
            let mut device_trust = self.device_trust.lock().unwrap();
            device_trust.insert(
                (user_id, device_fingerprint.as_string()),
                DeviceTrustStatus::Trusted,
            );
        } // Lock is released here

        // Log security event
        self.log_security_event(SecurityEvent {
            id: Uuid::new_v4(),
            user_id,
            event_type: SecurityEventType::DeviceUntrusted, // Should be DeviceTrusted, but using existing enum
            timestamp: Utc::now(),
            ip_address: None,
            device_fingerprint: Some(device_fingerprint.clone()),
            location: None,
            details: HashMap::from([
                ("action".to_string(), "device_trusted".to_string()),
            ]),
            risk_score: 10,
        }).await?;

        Ok(())
    }

    async fn untrust_device(&self, user_id: Uuid, device_fingerprint: &DeviceFingerprint) -> Result<(), SecurityError> {
        {
            let mut device_trust = self.device_trust.lock().unwrap();
            device_trust.insert(
                (user_id, device_fingerprint.as_string()),
                DeviceTrustStatus::Untrusted,
            );
        } // Lock is released here

        // Log security event
        self.log_security_event(SecurityEvent {
            id: Uuid::new_v4(),
            user_id,
            event_type: SecurityEventType::DeviceUntrusted,
            timestamp: Utc::now(),
            ip_address: None,
            device_fingerprint: Some(device_fingerprint.clone()),
            location: None,
            details: HashMap::from([
                ("action".to_string(), "device_untrusted".to_string()),
            ]),
            risk_score: 30,
        }).await?;

        Ok(())
    }

    async fn detect_suspicious_activity(&self, user_id: Uuid, ip_address: Option<IpAddress>, device_fingerprint: Option<DeviceFingerprint>) -> Result<Vec<SecurityRisk>, SecurityError> {
        let mut risks = Vec::new();

        // Calculate overall risk score
        let risk_score = self.calculate_risk_score(user_id, ip_address.clone(), device_fingerprint.clone()).await;

        if risk_score >= 70 {
            risks.push(SecurityRisk {
                risk_type: "high_risk_session".to_string(),
                risk_score,
                description: "High-risk login attempt detected".to_string(),
                recommendation: "Require additional verification".to_string(),
                requires_action: true,
            });
        }

        // Check for suspicious IP
        if self.is_suspicious_ip(&ip_address) {
            risks.push(SecurityRisk {
                risk_type: "suspicious_ip".to_string(),
                risk_score: 30,
                description: "Login from suspicious IP address".to_string(),
                recommendation: "Monitor for additional suspicious activity".to_string(),
                requires_action: false,
            });
        }

        // Check for unknown device
        if let Some(device) = device_fingerprint {
            if self.get_device_trust_status(user_id, &device).await? == DeviceTrustStatus::Unknown {
                risks.push(SecurityRisk {
                    risk_type: "unknown_device".to_string(),
                    risk_score: 20,
                    description: "Login from unrecognized device".to_string(),
                    recommendation: "Offer device trust option".to_string(),
                    requires_action: false,
                });
            }
        }

        Ok(risks)
    }

    async fn log_security_event(&self, event: SecurityEvent) -> Result<(), SecurityError> {
        // Publish domain event if event bus is configured
        if let Some(ref publisher) = self.event_publisher {
            let domain_event = Self::to_domain_event(&event);
            let _ = publisher.publish_created(domain_event, Some(event.user_id.to_string())).await;
        }

        let mut events = self.security_events.lock().unwrap();
        events.push(event);

        // Keep only last 10000 events to prevent memory leaks
        if events.len() > 10000 {
            events.remove(0);
        }

        Ok(())
    }

    async fn get_user_security_events(&self, user_id: Uuid, since: DateTime<Utc>) -> Result<Vec<SecurityEvent>, SecurityError> {
        let events = self.security_events.lock().unwrap();
        Ok(events
            .iter()
            .filter(|event| event.user_id == user_id && event.timestamp >= since)
            .cloned()
            .collect())
    }

    async fn check_geographic_anomaly(&self, user_id: Uuid, current_location: SecurityLocation) -> Result<bool, SecurityError> {
        let known_locations = self.get_known_locations(user_id);

        if known_locations.is_empty() {
            // First login from this user, add to known locations
            self.add_known_location(user_id, current_location);
            return Ok(false);
        }

        // Check if current location is within 500km of any known location
        let is_near_known_location = known_locations.iter().any(|known| {
            let distance = self.calculate_distance(known, &current_location);
            distance < 500.0 // 500 km threshold
        });

        if !is_near_known_location && !current_location.is_known_location {
            return Ok(true); // Geographic anomaly detected
        }

        Ok(false)
    }
}

impl DefaultSecurityMonitoringService {
    /// Calculate distance between two geographic coordinates (Haversine formula)
    fn calculate_distance(&self, loc1: &SecurityLocation, loc2: &SecurityLocation) -> f64 {
        const EARTH_RADIUS: f64 = 6371.0; // Earth's radius in kilometers

        let lat1_rad = loc1.latitude.to_radians();
        let lat2_rad = loc2.latitude.to_radians();
        let delta_lat = (loc2.latitude - loc1.latitude).to_radians();
        let delta_lon = (loc2.longitude - loc1.longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2) +
                lat1_rad.cos() * lat2_rad.cos() *
                (delta_lon / 2.0).sin().powi(2);

        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS * c
    }
}

// Implement the password_reset_service::SecurityMonitoringService trait for DefaultSecurityMonitoringService
// This allows DefaultSecurityMonitoringService to be used by PasswordResetServiceImpl
// <<< CUSTOM CODE START >>>
// NOTE: password_reset_service::SecurityMonitoringService trait doesn't exist yet
// Uncomment when the trait is added to password_reset_service
/*
#[async_trait::async_trait]
impl crate::domain::services::password_reset_service::SecurityMonitoringService for DefaultSecurityMonitoringService {
    async fn get_recent_failed_attempts(&self, user_id: uuid::Uuid, duration: chrono::Duration) -> anyhow::Result<u32> {
        // Use the existing failed_attempts tracking
        let attempts = self.failed_attempts.lock().unwrap();

        if let Some(user_attempts) = attempts.get(&user_id) {
            let cutoff = chrono::Utc::now() - duration;
            let recent_count = user_attempts
                .iter()
                .filter(|&&timestamp| timestamp > cutoff)
                .count() as u32;
            Ok(recent_count)
        } else {
            Ok(0)
        }
    }

    async fn analyze_ip_address(&self, ip_address: &str) -> anyhow::Result<crate::domain::services::password_reset_service::IpAnalysisResult> {
        let ip = IpAddress::from_unchecked(ip_address);
        let is_suspicious = self.is_suspicious_ip(&Some(ip));

        Ok(crate::domain::services::password_reset_service::IpAnalysisResult {
            is_suspicious,
        })
    }

    async fn analyze_device_fingerprint(&self, user_id: uuid::Uuid, fingerprint: &str) -> anyhow::Result<crate::domain::services::password_reset_service::DeviceAnalysisResult> {
        // Simple device analysis - in a real implementation, this would check device history
        let device_trust_map = self.device_trust.lock().unwrap();
        let is_new_device = !device_trust_map.keys().any(|(uid, _)| *uid == user_id);

        Ok(crate::domain::services::password_reset_service::DeviceAnalysisResult {
            is_new_device,
        })
    }

    async fn check_geographic_anomaly(&self, user_id: uuid::Uuid, original_ip: &str, current_ip: &str) -> anyhow::Result<crate::domain::services::password_reset_service::GeoAnalysisResult> {
        // Simple geographic check - in a real implementation, this would use IP geolocation
        let is_anomaly = original_ip != current_ip && !original_ip.is_empty() && !current_ip.is_empty();

        Ok(crate::domain::services::password_reset_service::GeoAnalysisResult {
            is_anomaly,
        })
    }
}
*/
// <<< CUSTOM CODE END >>>

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::IpAddress;

    #[tokio::test]
    async fn test_failed_attempt_tracking() {
        let service = DefaultSecurityMonitoringService::new();
        let user_id = Uuid::new_v4();

        // Record 3 failed attempts
        for _ in 0..3 {
            service.record_failed_attempt(user_id).await.unwrap();
        }

        assert_eq!(service.get_failed_attempts(user_id).await.unwrap(), 3);

        // Reset attempts
        service.reset_failed_attempts(user_id).await.unwrap();
        assert_eq!(service.get_failed_attempts(user_id).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_account_lockout() {
        let service = DefaultSecurityMonitoringService::new();
        let user_id = Uuid::new_v4();

        // Record 5 failed attempts to trigger lockout
        for _ in 0..5 {
            service.record_failed_attempt(user_id).await.unwrap();
        }

        // User should be locked
        assert!(service.is_user_locked(user_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_device_trust() {
        let service = DefaultSecurityMonitoringService::new();
        let user_id = Uuid::new_v4();
        let device_fingerprint: DeviceFingerprint = "test-device".into();

        // Initial status should be unknown
        assert!(matches!(
            service.get_device_trust_status(user_id, &device_fingerprint).await.unwrap(),
            DeviceTrustStatus::Unknown
        ));

        // Trust the device
        service.trust_device(user_id, &device_fingerprint).await.unwrap();
        assert!(matches!(
            service.get_device_trust_status(user_id, &device_fingerprint).await.unwrap(),
            DeviceTrustStatus::Trusted
        ));
    }

    #[tokio::test]
    async fn test_risk_score_calculation() {
        let service = DefaultSecurityMonitoringService::new();
        let user_id = Uuid::new_v4();

        // Test risk score for suspicious IP (bogon range 0.0.0.0/8)
        let suspicious_ip = IpAddress::new("0.0.0.1").unwrap();
        let risk_score = service.calculate_risk_score(user_id, Some(suspicious_ip), None).await;
        assert!(risk_score >= 30);

        // Test risk score for trusted device
        let device_fingerprint: DeviceFingerprint = "trusted-device".into();
        service.trust_device(user_id, &device_fingerprint).await.unwrap();
        let risk_score = service.calculate_risk_score(user_id, None, Some(device_fingerprint)).await;
        assert_eq!(risk_score, 0);
    }

    #[test]
    fn test_suspicious_ip_detection() {
        let service = DefaultSecurityMonitoringService::new();

        // Bogon/reserved ranges — should be suspicious
        assert!(service.is_suspicious_ip(&Some(IpAddress::new("0.0.0.1").unwrap())));           // 0.0.0.0/8
        assert!(service.is_suspicious_ip(&Some(IpAddress::new("100.64.0.1").unwrap())));        // CGNAT
        assert!(service.is_suspicious_ip(&Some(IpAddress::new("169.254.1.1").unwrap())));       // link-local
        assert!(service.is_suspicious_ip(&Some(IpAddress::new("224.0.0.1").unwrap())));         // multicast
        assert!(service.is_suspicious_ip(&Some(IpAddress::new("240.0.0.1").unwrap())));         // reserved
        assert!(service.is_suspicious_ip(&Some(IpAddress::new("255.255.255.255").unwrap())));   // broadcast

        // Documentation/test ranges — should be suspicious
        assert!(service.is_suspicious_ip(&Some(IpAddress::new("192.0.2.1").unwrap())));         // TEST-NET-1
        assert!(service.is_suspicious_ip(&Some(IpAddress::new("198.51.100.1").unwrap())));      // TEST-NET-2
        assert!(service.is_suspicious_ip(&Some(IpAddress::new("203.0.113.1").unwrap())));       // TEST-NET-3
        assert!(service.is_suspicious_ip(&Some(IpAddress::new("198.18.0.1").unwrap())));        // benchmarking

        // Private IPs — should NOT be suspicious (normal internal traffic)
        assert!(!service.is_suspicious_ip(&Some(IpAddress::new("10.0.0.1").unwrap())));
        assert!(!service.is_suspicious_ip(&Some(IpAddress::new("192.168.1.1").unwrap())));
        assert!(!service.is_suspicious_ip(&Some(IpAddress::new("172.16.0.1").unwrap())));

        // Loopback — should NOT be suspicious
        assert!(!service.is_suspicious_ip(&Some(IpAddress::new("127.0.0.1").unwrap())));

        // Normal public IPs — should NOT be suspicious
        assert!(!service.is_suspicious_ip(&Some(IpAddress::new("8.8.8.8").unwrap())));
        assert!(!service.is_suspicious_ip(&Some(IpAddress::new("1.1.1.1").unwrap())));
        assert!(!service.is_suspicious_ip(&Some(IpAddress::new("142.250.80.46").unwrap())));

        // None — should NOT be suspicious
        assert!(!service.is_suspicious_ip(&None));

        // IPv6 documentation range — should be suspicious
        assert!(service.is_suspicious_ip(&Some(IpAddress::new("2001:db8::1").unwrap())));

        // Normal IPv6 — should NOT be suspicious
        assert!(!service.is_suspicious_ip(&Some(IpAddress::new("2607:f8b0:4004:800::200e").unwrap())));
    }

    #[test]
    fn test_geographic_distance_calculation() {
        let service = DefaultSecurityMonitoringService::new();

        let loc1 = SecurityLocation {
            country: "US".to_string(),
            city: "New York".to_string(),
            latitude: 40.7128,
            longitude: -74.0060,
            is_known_location: false,
        };

        let loc2 = SecurityLocation {
            country: "US".to_string(),
            city: "Los Angeles".to_string(),
            latitude: 34.0522,
            longitude: -118.2437,
            is_known_location: false,
        };

        let distance = service.calculate_distance(&loc1, &loc2);
        // NYC to LA is approximately 3944 km
        assert!((distance - 3944.0).abs() < 50.0); // Allow 50km tolerance
    }
}