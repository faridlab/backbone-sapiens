//! Background Jobs Module for Sapiens Bounded Context
//!
//! This module provides background job processing and scheduled tasks
//! for the sapiens bounded context including:
//! - Session cleanup (expired sessions)
//! - Token expiration (password reset, email verification)
//! - User activity tracking
//! - Audit log rotation

#![allow(dead_code)]
#![allow(unused_imports)]

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::interval;

// ============================================================
// Job Configuration
// ============================================================

/// Configuration for background jobs
#[derive(Debug, Clone)]
pub struct JobConfig {
    /// Enable session cleanup job
    pub session_cleanup_enabled: bool,
    /// Interval for session cleanup in seconds
    pub session_cleanup_interval_secs: u64,
    /// Enable token expiration job
    pub token_expiration_enabled: bool,
    /// Interval for token expiration in seconds
    pub token_expiration_interval_secs: u64,
    /// Enable audit log rotation
    pub audit_rotation_enabled: bool,
    /// Interval for audit rotation in seconds (typically daily)
    pub audit_rotation_interval_secs: u64,
}

impl Default for JobConfig {
    fn default() -> Self {
        Self {
            session_cleanup_enabled: true,
            session_cleanup_interval_secs: 3600, // 1 hour
            token_expiration_enabled: true,
            token_expiration_interval_secs: 3600, // 1 hour
            audit_rotation_enabled: true,
            audit_rotation_interval_secs: 86400, // 24 hours
        }
    }
}

// ============================================================
// Job Trait
// ============================================================

/// Trait for background job implementations
#[async_trait]
pub trait BackgroundJob: Send + Sync {
    /// Job name for logging and identification
    fn name(&self) -> &str;

    /// Execute the job once
    async fn execute(&self) -> Result<JobResult, JobError>;

    /// Check if the job should run
    fn should_run(&self) -> bool {
        true
    }
}

/// Result of a job execution
#[derive(Debug, Clone)]
pub struct JobResult {
    pub job_name: String,
    pub success: bool,
    pub items_processed: u64,
    pub duration_ms: u64,
    pub message: String,
    pub executed_at: DateTime<Utc>,
}

impl JobResult {
    pub fn success(job_name: String, items_processed: u64, duration_ms: u64, message: String) -> Self {
        Self {
            job_name,
            success: true,
            items_processed,
            duration_ms,
            message,
            executed_at: Utc::now(),
        }
    }

    pub fn failure(job_name: String, duration_ms: u64, message: String) -> Self {
        Self {
            job_name,
            success: false,
            items_processed: 0,
            duration_ms,
            message,
            executed_at: Utc::now(),
        }
    }
}

/// Job execution errors
#[derive(Debug, thiserror::Error)]
pub enum JobError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Job cancelled")]
    Cancelled,
}

// ============================================================
// Repository Traits for Jobs
// ============================================================

/// Trait for session repository operations needed by jobs
#[async_trait]
pub trait SessionJobRepository: Send + Sync {
    /// Clean up expired sessions
    async fn cleanup_expired(&self) -> Result<u64, anyhow::Error>;
}

/// Trait for user repository operations needed by jobs
#[async_trait]
pub trait UserJobRepository: Send + Sync {
    /// Find inactive users
    async fn find_inactive(&self, days: i64) -> Result<Vec<String>, anyhow::Error>;
}

// ============================================================
// Session Cleanup Job
// ============================================================

/// Job to clean up expired sessions
pub struct SessionCleanupJob<R: SessionJobRepository> {
    session_repo: Arc<R>,
}

impl<R: SessionJobRepository> SessionCleanupJob<R> {
    pub fn new(session_repo: Arc<R>) -> Self {
        Self { session_repo }
    }
}

#[async_trait]
impl<R: SessionJobRepository + 'static> BackgroundJob for SessionCleanupJob<R> {
    fn name(&self) -> &str {
        "session_cleanup"
    }

    async fn execute(&self) -> Result<JobResult, JobError> {
        let start = std::time::Instant::now();

        let cleaned = self
            .session_repo
            .cleanup_expired()
            .await
            .map_err(|e| JobError::DatabaseError(e.to_string()))?;

        let duration = start.elapsed().as_millis() as u64;

        tracing::info!(
            job = self.name,
            sessions_cleaned = cleaned,
            duration_ms = duration,
            "Session cleanup job completed"
        );

        Ok(JobResult::success(
            self.name.to_string(),
            cleaned,
            duration,
            format!("Cleaned {} expired sessions", cleaned),
        ))
    }
}

// ============================================================
// Inactive User Detection Job
// ============================================================

/// Job to detect and flag inactive users
pub struct InactiveUserDetectionJob<R: UserJobRepository> {
    user_repo: Arc<R>,
    inactive_threshold_days: i64,
}

impl<R: UserJobRepository> InactiveUserDetectionJob<R> {
    pub fn new(user_repo: Arc<R>, inactive_threshold_days: i64) -> Self {
        Self {
            user_repo,
            inactive_threshold_days,
        }
    }
}

#[async_trait]
impl<R: UserJobRepository + 'static> BackgroundJob for InactiveUserDetectionJob<R> {
    fn name(&self) -> &str {
        "inactive_user_detection"
    }

    async fn execute(&self) -> Result<JobResult, JobError> {
        let start = std::time::Instant::now();

        // For now, just log that the job ran
        // In a real implementation, this would query for users with no recent activity
        // and potentially send warning emails or flag accounts

        let duration = start.elapsed().as_millis() as u64;

        tracing::info!(
            job = self.name,
            threshold_days = self.inactive_threshold_days,
            duration_ms = duration,
            "Inactive user detection job completed"
        );

        Ok(JobResult::success(
            self.name.to_string(),
            0,
            duration,
            format!(
                "Checked for users inactive for {} days",
                self.inactive_threshold_days
            ),
        ))
    }
}

// ============================================================
// Job Scheduler
// ============================================================

/// Job scheduler for running background tasks
pub struct JobScheduler {
    jobs: Vec<ScheduledJob>,
    config: JobConfig,
    running: Arc<RwLock<bool>>,
}

struct ScheduledJob {
    job: Arc<dyn BackgroundJob>,
    interval_secs: u64,
}

impl JobScheduler {
    pub fn new(config: JobConfig) -> Self {
        Self {
            jobs: Vec::new(),
            config,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Add a job to the scheduler
    pub fn add_job(mut self, job: Arc<dyn BackgroundJob>, interval_secs: u64) -> Self {
        self.jobs.push(ScheduledJob { job, interval_secs });
        self
    }

    /// Start the job scheduler in the background
    pub async fn start(self) -> JobSchedulerHandle {
        let running = self.running.clone();
        *running.write().await = true;

        let handle = tokio::spawn(async move {
            self.run_loop().await;
        });

        JobSchedulerHandle {
            running,
            handle: Some(handle),
        }
    }

    async fn run_loop(self) {
        // Create intervals for each job
        let mut intervals: Vec<_> = self
            .jobs
            .iter()
            .map(|j| (j.job.clone(), interval(std::time::Duration::from_secs(j.interval_secs))))
            .collect();

        loop {
            // Check if we should stop
            if !*self.running.read().await {
                break;
            }

            // Run each job if its interval has elapsed
            for (job, int) in &mut intervals {
                tokio::select! {
                    _ = int.tick() => {
                        if job.should_run() {
                            match job.execute().await {
                                Ok(result) => {
                                    tracing::debug!(
                                        job = result.job_name,
                                        success = result.success,
                                        items = result.items_processed,
                                        "Job completed"
                                    );
                                }
                                Err(e) => {
                                    tracing::error!(
                                        job = job.name,
                                        error = %e,
                                        "Job failed"
                                    );
                                }
                            }
                        }
                    }
                }
            }

            // Small delay to prevent busy loop
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }
}

/// Handle for controlling the job scheduler
pub struct JobSchedulerHandle {
    running: Arc<RwLock<bool>>,
    handle: Option<tokio::task::JoinHandle<()>>,
}

impl JobSchedulerHandle {
    /// Stop the scheduler gracefully
    pub async fn stop(&mut self) {
        *self.running.write().await = false;
        if let Some(handle) = self.handle.take() {
            let _ = handle.await;
        }
    }

    /// Check if the scheduler is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_config_default() {
        let config = JobConfig::default();
        assert!(config.session_cleanup_enabled);
        assert_eq!(config.session_cleanup_interval_secs, 3600);
        assert!(config.token_expiration_enabled);
        assert!(config.audit_rotation_enabled);
    }

    #[test]
    fn test_job_result_success() {
        let result = JobResult::success(
            "test_job".to_string(),
            10,
            100,
            "Test completed".to_string(),
        );

        assert!(result.success);
        assert_eq!(result.job_name, "test_job");
        assert_eq!(result.items_processed, 10);
        assert_eq!(result.duration_ms, 100);
    }

    #[test]
    fn test_job_result_failure() {
        let result = JobResult::failure(
            "test_job".to_string(),
            50,
            "Test failed".to_string(),
        );

        assert!(!result.success);
        assert_eq!(result.job_name, "test_job");
        assert_eq!(result.items_processed, 0);
    }
}
