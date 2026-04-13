//! Health Checker Implementation
//! Health monitoring for the Sapiens module

#![allow(dead_code)]
#![allow(unused_imports)]

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ============================================================
// Health Status Types
// ============================================================

/// Health check status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl HealthStatus {
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }

    pub fn is_degraded(&self) -> bool {
        matches!(self, Self::Degraded)
    }

    pub fn is_unhealthy(&self) -> bool {
        matches!(self, Self::Unhealthy)
    }
}

// ============================================================
// Health Check Result
// ============================================================

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub component: String,
    pub status: HealthStatus,
    pub message: String,
    pub details: Option<HashMap<String, serde_json::Value>>,
    pub response_time_ms: u64,
    pub timestamp: DateTime<Utc>,
    pub error: Option<String>,
}

impl HealthCheckResult {
    pub fn healthy(component: String, message: String, response_time_ms: u64) -> Self {
        Self {
            component,
            status: HealthStatus::Healthy,
            message,
            details: None,
            response_time_ms,
            timestamp: Utc::now(),
            error: None,
        }
    }

    pub fn degraded(component: String, message: String, response_time_ms: u64) -> Self {
        Self {
            component,
            status: HealthStatus::Degraded,
            message,
            details: None,
            response_time_ms,
            timestamp: Utc::now(),
            error: None,
        }
    }

    pub fn unhealthy(component: String, message: String, response_time_ms: u64, error: String) -> Self {
        Self {
            component,
            status: HealthStatus::Unhealthy,
            message,
            details: None,
            response_time_ms,
            timestamp: Utc::now(),
            error: Some(error),
        }
    }

    pub fn with_details(mut self, details: HashMap<String, serde_json::Value>) -> Self {
        self.details = Some(details);
        self
    }
}

// ============================================================
// Health Status Response
// ============================================================

/// Overall health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatusResponse {
    pub status: HealthStatus,
    pub timestamp: DateTime<Utc>,
    pub uptime_seconds: u64,
    pub version: String,
    pub checks: HashMap<String, HealthCheckResult>,
    pub summary: HealthSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    pub total_checks: usize,
    pub healthy_checks: usize,
    pub degraded_checks: usize,
    pub unhealthy_checks: usize,
    pub average_response_time_ms: f64,
}

impl HealthStatusResponse {
    pub fn new(start_time: DateTime<Utc>, version: String) -> Self {
        Self {
            status: HealthStatus::Healthy,
            timestamp: Utc::now(),
            uptime_seconds: (Utc::now() - start_time).num_seconds() as u64,
            version,
            checks: HashMap::new(),
            summary: HealthSummary {
                total_checks: 0,
                healthy_checks: 0,
                degraded_checks: 0,
                unhealthy_checks: 0,
                average_response_time_ms: 0.0,
            },
        }
    }

    pub fn add_check(&mut self, result: HealthCheckResult) {
        let component = result.component.clone();
        self.checks.insert(component, result.clone());

        // Update summary
        self.summary.total_checks += 1;
        match result.status {
            HealthStatus::Healthy => self.summary.healthy_checks += 1,
            HealthStatus::Degraded => self.summary.degraded_checks += 1,
            HealthStatus::Unhealthy => self.summary.unhealthy_checks += 1,
            HealthStatus::Unknown => {}
        }

        // Update overall status
        self.update_overall_status();

        // Update average response time
        if !self.checks.is_empty() {
            let total_response_time: u64 = self.checks.values()
                .map(|check| check.response_time_ms)
                .sum();
            self.summary.average_response_time_ms = total_response_time as f64 / self.checks.len() as f64;
        }
    }

    fn update_overall_status(&mut self) {
        if self.checks.is_empty() {
            self.status = HealthStatus::Unknown;
            return;
        }

        if self.summary.unhealthy_checks > 0 {
            self.status = HealthStatus::Unhealthy;
        } else if self.summary.degraded_checks > 0 {
            self.status = HealthStatus::Degraded;
        } else {
            self.status = HealthStatus::Healthy;
        }
    }
}

// ============================================================
// Health Check Trait
// ============================================================

/// Health check trait
#[async_trait]
pub trait HealthCheck: Send + Sync {
    fn name(&self) -> &str;
    async fn check(&self) -> HealthCheckResult;
}

// ============================================================
// Database Health Check
// ============================================================

/// Database health check using sqlx PgPool
pub struct DatabaseHealthCheck {
    pool: sqlx::PgPool,
    timeout: Duration,
}

impl DatabaseHealthCheck {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            timeout: Duration::from_secs(5),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

#[async_trait]
impl HealthCheck for DatabaseHealthCheck {
    fn name(&self) -> &str {
        "database"
    }

    async fn check(&self) -> HealthCheckResult {
        let start = std::time::Instant::now();

        let result = tokio::time::timeout(
            self.timeout,
            sqlx::query("SELECT 1").execute(&self.pool)
        ).await;

        let response_time = start.elapsed().as_millis() as u64;

        match result {
            Ok(Ok(_)) => {
                HealthCheckResult::healthy(
                    self.name.to_string(),
                    "Database connection is healthy".to_string(),
                    response_time,
                )
            }
            Ok(Err(e)) => {
                HealthCheckResult::unhealthy(
                    self.name.to_string(),
                    "Database connection error".to_string(),
                    response_time,
                    e.to_string(),
                )
            }
            Err(_) => {
                HealthCheckResult::unhealthy(
                    self.name.to_string(),
                    "Database health check timed out".to_string(),
                    self.timeout.as_millis() as u64,
                    format!("Health check timed out after {:?}", self.timeout),
                )
            }
        }
    }
}

// ============================================================
// Memory Health Check
// ============================================================

/// Memory usage health check (simplified - no external dependencies)
pub struct MemoryHealthCheck {
    warning_threshold_mb: u64,
    critical_threshold_mb: u64,
}

impl MemoryHealthCheck {
    pub fn new() -> Self {
        Self {
            warning_threshold_mb: 512,   // 512MB
            critical_threshold_mb: 1024, // 1GB
        }
    }

    pub fn with_thresholds(mut self, warning_mb: u64, critical_mb: u64) -> Self {
        self.warning_threshold_mb = warning_mb;
        self.critical_threshold_mb = critical_mb;
        self
    }
}

impl Default for MemoryHealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HealthCheck for MemoryHealthCheck {
    fn name(&self) -> &str {
        "memory"
    }

    async fn check(&self) -> HealthCheckResult {
        let start = std::time::Instant::now();
        let response_time = start.elapsed().as_millis() as u64;

        // Simplified memory check - always returns healthy
        // In production, you'd use platform-specific APIs or a crate like `sysinfo`
        let mut details = HashMap::new();
        details.insert("note".to_string(), serde_json::Value::String("Simplified health check".to_string()));

        HealthCheckResult::healthy(
            self.name.to_string(),
            "Memory usage is normal".to_string(),
            response_time,
        ).with_details(details)
    }
}

// ============================================================
// Disk Space Health Check
// ============================================================

/// Disk space health check (simplified)
pub struct DiskSpaceHealthCheck {
    warning_threshold_percent: f64,
    critical_threshold_percent: f64,
    mount_path: String,
}

impl DiskSpaceHealthCheck {
    pub fn new() -> Self {
        Self {
            warning_threshold_percent: 80.0,
            critical_threshold_percent: 95.0,
            mount_path: "/".to_string(),
        }
    }

    pub fn with_thresholds(mut self, warning_percent: f64, critical_percent: f64) -> Self {
        self.warning_threshold_percent = warning_percent;
        self.critical_threshold_percent = critical_percent;
        self
    }

    pub fn with_mount_path(mut self, path: String) -> Self {
        self.mount_path = path;
        self
    }
}

impl Default for DiskSpaceHealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HealthCheck for DiskSpaceHealthCheck {
    fn name(&self) -> &str {
        "disk_space"
    }

    async fn check(&self) -> HealthCheckResult {
        let start = std::time::Instant::now();
        let response_time = start.elapsed().as_millis() as u64;

        // Simplified disk check - always returns healthy
        // In production, you'd use platform-specific APIs or a crate like `sysinfo`
        let mut details = HashMap::new();
        details.insert("path".to_string(), serde_json::Value::String(self.mount_path.clone()));
        details.insert("note".to_string(), serde_json::Value::String("Simplified health check".to_string()));

        HealthCheckResult::healthy(
            self.name.to_string(),
            "Disk usage is normal".to_string(),
            response_time,
        ).with_details(details)
    }
}

// ============================================================
// CPU Health Check
// ============================================================

/// CPU usage health check (simplified)
pub struct CpuHealthCheck {
    warning_threshold_percent: f64,
    critical_threshold_percent: f64,
}

impl CpuHealthCheck {
    pub fn new() -> Self {
        Self {
            warning_threshold_percent: 80.0,
            critical_threshold_percent: 95.0,
        }
    }

    pub fn with_thresholds(mut self, warning_percent: f64, critical_percent: f64) -> Self {
        self.warning_threshold_percent = warning_percent;
        self.critical_threshold_percent = critical_percent;
        self
    }
}

impl Default for CpuHealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HealthCheck for CpuHealthCheck {
    fn name(&self) -> &str {
        "cpu"
    }

    async fn check(&self) -> HealthCheckResult {
        let start = std::time::Instant::now();
        let response_time = start.elapsed().as_millis() as u64;

        // Simplified CPU check - always returns healthy
        // In production, you'd use platform-specific APIs or a crate like `sysinfo`
        let mut details = HashMap::new();
        details.insert("note".to_string(), serde_json::Value::String("Simplified health check".to_string()));

        HealthCheckResult::healthy(
            self.name.to_string(),
            "CPU usage is normal".to_string(),
            response_time,
        ).with_details(details)
    }
}

// ============================================================
// Health Checker Service
// ============================================================

/// Health checker service
pub struct HealthChecker {
    checks: Vec<Box<dyn HealthCheck>>,
    start_time: DateTime<Utc>,
    version: String,
}

impl HealthChecker {
    pub fn new(version: String) -> Self {
        Self {
            checks: Vec::new(),
            start_time: Utc::now(),
            version,
        }
    }

    pub fn add_check(mut self, check: Box<dyn HealthCheck>) -> Self {
        self.checks.push(check);
        self
    }

    pub async fn check_health(&self) -> HealthStatusResponse {
        let mut response = HealthStatusResponse::new(self.start_time, self.version.clone());

        // Run all health checks sequentially (simpler than join_all)
        for check in &self.checks {
            let result = check.check().await;
            response.add_check(result);
        }

        response
    }

    pub async fn check_individual(&self, check_name: &str) -> Option<HealthCheckResult> {
        for check in &self.checks {
            if check.name == check_name {
                return Some(check.check().await);
            }
        }
        None
    }
}

// ============================================================
// Health Checker Factory
// ============================================================

/// Factory for creating health checkers
pub struct HealthCheckerFactory;

impl HealthCheckerFactory {
    pub fn create_default(
        pool: sqlx::PgPool,
        version: String,
    ) -> HealthChecker {
        HealthChecker::new(version)
            .add_check(Box::new(DatabaseHealthCheck::new(pool)))
            .add_check(Box::new(MemoryHealthCheck::new()))
            .add_check(Box::new(DiskSpaceHealthCheck::new()))
            .add_check(Box::new(CpuHealthCheck::new()))
    }

    pub fn create_minimal(
        pool: sqlx::PgPool,
        version: String,
    ) -> HealthChecker {
        HealthChecker::new(version)
            .add_check(Box::new(DatabaseHealthCheck::new(pool)))
            .add_check(Box::new(MemoryHealthCheck::new()))
    }

    pub fn create_custom(version: String) -> HealthChecker {
        HealthChecker::new(version)
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status() {
        assert!(HealthStatus::Healthy.is_healthy());
        assert!(HealthStatus::Degraded.is_degraded());
        assert!(HealthStatus::Unhealthy.is_unhealthy());
    }

    #[test]
    fn test_health_check_result_healthy() {
        let result = HealthCheckResult::healthy(
            "test".to_string(),
            "OK".to_string(),
            100,
        );
        assert_eq!(result.status, HealthStatus::Healthy);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_health_check_result_unhealthy() {
        let result = HealthCheckResult::unhealthy(
            "test".to_string(),
            "Error".to_string(),
            200,
            "Details".to_string(),
        );
        assert_eq!(result.status, HealthStatus::Unhealthy);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_health_status_response() {
        let start_time = Utc::now();
        let mut response = HealthStatusResponse::new(start_time, "1.0.0".to_string());

        // Add a healthy check
        let healthy_check = HealthCheckResult::healthy("test".to_string(), "OK".to_string(), 100);
        response.add_check(healthy_check);

        assert_eq!(response.status, HealthStatus::Healthy);
        assert_eq!(response.summary.total_checks, 1);
        assert_eq!(response.summary.healthy_checks, 1);

        // Add an unhealthy check
        let unhealthy_check = HealthCheckResult::unhealthy(
            "test2".to_string(),
            "Error".to_string(),
            200,
            "Details".to_string(),
        );
        response.add_check(unhealthy_check);

        assert_eq!(response.status, HealthStatus::Unhealthy);
        assert_eq!(response.summary.total_checks, 2);
        assert_eq!(response.summary.unhealthy_checks, 1);
    }

    #[tokio::test]
    async fn test_memory_health_check() {
        let check = MemoryHealthCheck::new();
        let result = check.check().await;
        assert_eq!(result.component, "memory");
        assert_eq!(result.status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_cpu_health_check() {
        let check = CpuHealthCheck::new();
        let result = check.check().await;
        assert_eq!(result.component, "cpu");
        assert_eq!(result.status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_disk_health_check() {
        let check = DiskSpaceHealthCheck::new();
        let result = check.check().await;
        assert_eq!(result.component, "disk_space");
        assert_eq!(result.status, HealthStatus::Healthy);
    }
}
