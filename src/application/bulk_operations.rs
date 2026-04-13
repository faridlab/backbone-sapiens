//! Advanced Bulk Operations
//!
//! Provides high-performance bulk operations for large datasets
//! including streaming, batching, and error recovery capabilities.

use std::sync::Arc;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;
use futures::StreamExt;
use std::pin::Pin;

use crate::application::commands::{
    BulkCreateUserCommand, BulkCreateUserResponse,
    UserApplicationService, CreateUserCommand,
};

/// Configuration for bulk operations
#[derive(Debug, Clone)]
pub struct BulkOperationConfig {
    /// Maximum batch size for processing
    pub batch_size: usize,
    /// Maximum number of concurrent operations
    pub max_concurrency: usize,
    /// Whether to continue on individual errors
    pub continue_on_error: bool,
    /// Maximum retry attempts for failed operations
    pub max_retries: u32,
    /// Delay between retries (in milliseconds)
    pub retry_delay_ms: u64,
    /// Whether to validate input before processing
    pub validate_input: bool,
}

impl Default for BulkOperationConfig {
    fn default() -> Self {
        Self {
            batch_size: 1000,
            max_concurrency: 10,
            continue_on_error: true,
            max_retries: 3,
            retry_delay_ms: 100,
            validate_input: true,
        }
    }
}

/// Progress reporting for bulk operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationProgress {
    pub total_items: usize,
    pub processed_items: usize,
    pub successful_items: usize,
    pub failed_items: usize,
    pub current_batch: usize,
    pub total_batches: usize,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub estimated_completion: Option<chrono::DateTime<chrono::Utc>>,
}

impl BulkOperationProgress {
    pub fn new(total_items: usize) -> Self {
        Self {
            total_items,
            processed_items: 0,
            successful_items: 0,
            failed_items: 0,
            current_batch: 0,
            total_batches: 0,
            start_time: chrono::Utc::now(),
            estimated_completion: None,
        }
    }

    pub fn completion_percentage(&self) -> f64 {
        if self.total_items == 0 {
            return 100.0;
        }
        (self.processed_items as f64 / self.total_items as f64) * 100.0
    }

    pub fn items_per_second(&self) -> f64 {
        let elapsed = (chrono::Utc::now() - self.start_time).num_seconds() as f64;
        if elapsed > 0.0 {
            self.processed_items as f64 / elapsed
        } else {
            0.0
        }
    }

    pub fn eta_seconds(&self) -> Option<f64> {
        if self.processed_items == 0 {
            return None;
        }

        let elapsed = (chrono::Utc::now() - self.start_time).num_seconds() as f64;
        let items_per_second = self.processed_items as f64 / elapsed;
        let remaining_items = (self.total_items - self.processed_items) as f64;

        if items_per_second > 0.0 {
            Some(remaining_items / items_per_second)
        } else {
            None
        }
    }
}

/// Advanced bulk operations handler
pub struct BulkOperationsHandler {
    user_service: Arc<UserApplicationService>,
    config: BulkOperationConfig,
}

impl BulkOperationsHandler {
    pub fn new(user_service: Arc<UserApplicationService>, config: BulkOperationConfig) -> Self {
        Self { user_service, config }
    }

    /// Process bulk user creation with advanced features
    pub async fn bulk_create_users_streaming<I>(
        &self,
        items: I,
        mut progress_callback: Option<Box<dyn Fn(BulkOperationProgress) + Send + Sync>>,
    ) -> Result<BulkCreateUserResponse>
    where
        I: futures::Stream<Item = Result<CreateUserCommand>>,
    {
        let mut progress = BulkOperationProgress::new(0); // Unknown total for streaming
        let mut all_commands = Vec::new();
        let mut errors = Vec::new();

        // Collect and validate all items first if validation is enabled
        if self.config.validate_input {
            let mut validation_errors = Vec::new();
            let mut items_stream = Box::pin(items);
            let mut item_count = 0;

            while let Some(item_result) = items_stream.next().await {
                item_count += 1;
                match item_result {
                    Ok(command) => {
                        if let Err(e) = self.validate_user_command(&command) {
                            validation_errors.push(format!("Validation error for item {}: {}", item_count, e));
                        } else {
                            all_commands.push(command);
                        }
                    }
                    Err(e) => {
                        validation_errors.push(format!("Error processing item {}: {}", item_count, e));
                    }
                }

                // Report progress during validation
                if let Some(ref callback) = progress_callback {
                    callback(BulkOperationProgress {
                        total_items: item_count,
                        processed_items: item_count,
                        successful_items: all_commands.len(),
                        failed_items: validation_errors.len(),
                        current_batch: 1,
                        total_batches: 1,
                        start_time: progress.start_time,
                        estimated_completion: Some(chrono::Utc::now()),
                    });
                }
            }

            if !validation_errors.is_empty() && !self.config.continue_on_error {
                return Ok(BulkCreateUserResponse::new(
                    0,
                    validation_errors.len(),
                    Vec::new(),
                    validation_errors,
                ));
            }

            errors.extend(validation_errors);
        } else {
            // Collect all items without validation
            let mut items_stream = Box::pin(items);
            let mut item_count = 0;

            while let Some(item_result) = items_stream.next().await {
                item_count += 1;
                match item_result {
                    Ok(command) => all_commands.push(command),
                    Err(e) => {
                        errors.push(format!("Error processing item {}: {}", item_count, e));
                        if !self.config.continue_on_error {
                            break;
                        }
                    }
                }
            }

            progress.total_items = item_count;
        }

        // Process in batches
        let total_batches = (all_commands.len() + self.config.batch_size - 1) / self.config.batch_size;
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrency));

        let mut batch_results = Vec::new();

        for (batch_index, batch_start) in (0..all_commands.len()).step_by(self.config.batch_size).enumerate() {
            let batch_end = std::cmp::min(batch_start + self.config.batch_size, all_commands.len());
            let batch_commands: Vec<CreateUserCommand> = all_commands[batch_start..batch_end].to_vec();

            let batch_result = self.process_user_batch_with_semaphore(
                batch_commands,
                batch_index,
                &semaphore,
            ).await;

            match batch_result {
                Ok(response) => {
                    batch_results.push(response);
                    progress.successful_items += response.created_count;
                    progress.failed_items += response.failed_count;
                    errors.extend(response.errors);
                }
                Err(e) => {
                    let batch_error = format!("Batch {} failed: {}", batch_index + 1, e);
                    errors.push(batch_error);
                    progress.failed_items += batch_commands.len();

                    if !self.config.continue_on_error {
                        break;
                    }
                }
            }

            progress.processed_items += batch_commands.len();
            progress.current_batch = batch_index + 1;
            progress.total_batches = total_batches;

            // Update progress callback
            if let Some(ref callback) = progress_callback {
                // Calculate ETA
                if progress.processed_items > 0 {
                    let elapsed = (chrono::Utc::now() - progress.start_time).num_seconds() as f64;
                    let items_per_second = progress.processed_items as f64 / elapsed;
                    let remaining_items = (progress.total_items - progress.processed_items) as f64;
                    if items_per_second > 0.0 {
                        let eta_seconds = remaining_items / items_per_second;
                        progress.estimated_completion = Some(chrono::Utc::now() + chrono::Duration::seconds(eta_seconds as i64));
                    }
                }

                callback(progress.clone());
            }

            // Add delay between batches to prevent overwhelming the database
            if batch_index < total_batches - 1 {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }

        // Aggregate all results
        let mut total_created = 0;
        let mut total_created_users = Vec::new();

        for batch_result in batch_results {
            total_created += batch_result.created_count;
            total_created_users.extend(batch_result.created_users);
        }

        Ok(BulkCreateUserResponse::new(
            total_created,
            progress.failed_items,
            total_created_users,
            errors,
        ))
    }

    /// Process a single batch with semaphore control
    async fn process_user_batch_with_semaphore(
        &self,
        batch_commands: Vec<CreateUserCommand>,
        batch_index: usize,
        semaphore: &Arc<Semaphore>,
    ) -> Result<BulkCreateUserResponse> {
        let _permit = semaphore.acquire().await.unwrap();

        let bulk_command = BulkCreateUserCommand::new(batch_commands.clone());

        // Implement retry logic
        let mut attempts = 0;
        let mut last_error = None;

        while attempts <= self.config.max_retries {
            match self.user_service.bulk_create_handler().handle(bulk_command.clone()).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);
                    attempts += 1;

                    if attempts <= self.config.max_retries {
                        tracing::warn!(
                            "Batch {} failed (attempt {}/{}), retrying...: {}",
                            batch_index,
                            attempts,
                            self.config.max_retries,
                            last_error.as_ref().unwrap()
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(self.config.retry_delay_ms)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("Unknown error")))
    }

    /// Validate a user command before processing
    fn validate_user_command(&self, command: &CreateUserCommand) -> Result<()> {
        // Validate required fields
        if command.username.trim().is_empty() {
            return Err(anyhow!("Username cannot be empty"));
        }

        if command.email.trim().is_empty() {
            return Err(anyhow!("Email cannot be empty"));
        }

        // Validate email format (basic check)
        if !command.email.contains('@') {
            return Err(anyhow!("Invalid email format"));
        }

        // Validate username format (basic check)
        if command.username.len() < 3 {
            return Err(anyhow!("Username must be at least 3 characters long"));
        }

        Ok(())
    }

    /// Generate performance report for bulk operations
    pub fn generate_performance_report(
        &self,
        progress: &BulkOperationProgress,
    ) -> BulkOperationReport {
        let duration = chrono::Utc::now() - progress.start_time;

        BulkOperationReport {
            total_items: progress.total_items,
            successful_items: progress.successful_items,
            failed_items: progress.failed_items,
            completion_percentage: progress.completion_percentage(),
            duration_seconds: duration.num_seconds(),
            items_per_second: progress.items_per_second(),
            batches_processed: progress.current_batch,
            average_items_per_batch: if progress.current_batch > 0 {
                progress.processed_items as f64 / progress.current_batch as f64
            } else {
                0.0
            },
            config: self.config.clone(),
        }
    }
}

/// Performance report for bulk operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationReport {
    pub total_items: usize,
    pub successful_items: usize,
    pub failed_items: usize,
    pub completion_percentage: f64,
    pub duration_seconds: i64,
    pub items_per_second: f64,
    pub batches_processed: usize,
    pub average_items_per_batch: f64,
    pub config: BulkOperationConfig,
}

impl BulkOperationReport {
    pub fn summary(&self) -> String {
        format!(
            "Bulk Operation Report:\n\
            ------------------\n\
            Total items: {}\n\
            Successful: {} ({:.1}%)\n\
            Failed: {} ({:.1}%)\n\
            Duration: {} seconds\n\
            Rate: {:.2} items/second\n\
            Batches: {}\n\
            Avg items/batch: {:.1}\n\
            ------------------",
            self.total_items,
            self.successful_items,
            self.completion_percentage,
            self.failed_items,
            100.0 - self.completion_percentage,
            self.duration_seconds,
            self.items_per_second,
            self.batches_processed,
            self.average_items_per_batch
        )
    }
}