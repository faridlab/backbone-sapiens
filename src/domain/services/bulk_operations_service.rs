//! Bulk Operations Service
//!
//! Provides bulk import, export, and advanced search capabilities for user management.

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::constants;
use crate::domain::entity::User;
use crate::domain::value_objects::Email;

/// Bulk operation types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BulkOperationType {
    /// Import users from file
    ImportUsers,
    /// Export users to file
    ExportUsers,
    /// Bulk update users
    BulkUpdate,
    /// Bulk delete users
    BulkDelete,
    /// Bulk operation with custom processing
    Custom(String),
}

/// Bulk operation status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BulkOperationStatus {
    /// Operation is queued
    Queued,
    /// Operation is in progress
    InProgress,
    /// Operation completed successfully
    Completed,
    /// Operation failed
    Failed,
    /// Operation was cancelled
    Cancelled,
}

/// File format for import/export
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileFormat {
    CSV,
    JSON,
    XML,
    Excel,
}

/// Search filters for advanced user search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearchFilters {
    pub email_contains: Option<String>,
    pub first_name_contains: Option<String>,
    pub last_name_contains: Option<String>,
    pub status_filter: Option<String>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub last_login_after: Option<DateTime<Utc>>,
    pub last_login_before: Option<DateTime<Utc>>,
    pub has_email_verified: Option<bool>,
    pub min_login_count: Option<u32>,
    pub max_login_count: Option<u32>,
    pub custom_filters: HashMap<String, serde_json::Value>,
}

/// Search sort options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearchSort {
    pub field: String,
    pub direction: SortDirection,
}

/// Sort direction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Advanced user search request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedUserSearchRequest {
    pub query: Option<String>,
    pub filters: UserSearchFilters,
    pub sort: Vec<UserSearchSort>,
    pub page: u32,
    pub limit: u32,
    pub include_deleted: bool,
    pub include_inactive: bool,
}

/// Advanced user search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedUserSearchResponse {
    pub users: Vec<UserSearchResult>,
    pub total_count: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
    pub search_time_ms: u64,
    pub applied_filters: Vec<String>,
}

/// User search result with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearchResult {
    pub user: User,
    pub relevance_score: f32,
    pub highlight_matches: HashMap<String, Vec<String>>,
    pub last_login_ago: Option<String>,
    pub account_age: Option<String>,
    pub login_frequency: Option<String>,
}

/// Bulk user import request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUserImportRequest {
    pub file_format: FileFormat,
    pub file_data: Vec<u8>,
    pub filename: String,
    pub skip_duplicates: bool,
    pub update_existing: bool,
    pub send_welcome_emails: bool,
    pub default_password_policy: Option<String>,
    pub field_mapping: HashMap<String, String>,
    pub validation_mode: ValidationMode,
}

/// Import validation mode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationMode {
    /// Skip invalid records and continue
    SkipInvalid,
    /// Stop on first error
    Strict,
    /// Try to fix common errors
    AutoFix,
}

/// Bulk user import response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUserImportResponse {
    pub operation_id: Uuid,
    pub total_records: u32,
    pub successful_imports: u32,
    pub failed_imports: u32,
    pub skipped_duplicates: u32,
    pub updated_records: u32,
    pub processing_time_ms: u64,
    pub errors: Vec<ImportError>,
    pub warnings: Vec<String>,
    pub summary: ImportSummary,
}

/// Import error details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportError {
    pub row_number: u32,
    pub field: Option<String>,
    pub error_type: ImportErrorType,
    pub error_message: String,
    pub original_value: Option<String>,
    pub suggested_fix: Option<String>,
}

/// Import error types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImportErrorType {
    ValidationError,
    DuplicateRecord,
    MissingRequiredField,
    InvalidEmail,
    InvalidDateFormat,
    InvalidEnumValue,
    DataTooLong,
    FieldMappingError,
    Other(String),
}

/// Import summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSummary {
    pub new_users_count: u32,
    pub updated_users_count: u32,
    pub skipped_count: u32,
    pub error_count: u32,
    pub processing_rate: f32,
    pub data_quality_score: f32,
}

/// Bulk user export request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUserExportRequest {
    pub file_format: FileFormat,
    pub search_filters: Option<UserSearchFilters>,
    pub include_fields: Vec<String>,
    pub exclude_sensitive_data: bool,
    pub compression: bool,
    pub include_headers: bool,
    pub date_format: String,
    pub max_records: Option<u32>,
}

/// Bulk user export response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUserExportResponse {
    pub operation_id: Uuid,
    pub file_url: String,
    pub file_size_bytes: u64,
    pub exported_records: u32,
    pub file_format: FileFormat,
    pub expires_at: DateTime<Utc>,
    pub processing_time_ms: u64,
    pub compression_used: bool,
    pub export_metadata: ExportMetadata,
}

/// Export metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    pub exported_at: DateTime<Utc>,
    pub exported_by: Uuid,
    pub filters_applied: Vec<String>,
    pub fields_included: Vec<String>,
    pub total_users_in_system: u64,
    pub export_percentage: f32,
}

/// Bulk operation status request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationStatusRequest {
    pub operation_id: Uuid,
}

/// Bulk operation status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationStatusResponse {
    pub operation_id: Uuid,
    pub operation_type: BulkOperationType,
    pub status: BulkOperationStatus,
    pub progress_percentage: f32,
    pub current_step: String,
    pub total_steps: u32,
    pub started_at: DateTime<Utc>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub processed_records: u32,
    pub total_records: u32,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Bulk Operations Service trait
#[async_trait::async_trait]
pub trait BulkOperationsService: Send + Sync {
    /// Perform advanced user search
    async fn advanced_user_search(&self, request: AdvancedUserSearchRequest) -> Result<AdvancedUserSearchResponse>;

    /// Bulk import users
    async fn bulk_import_users(&self, request: BulkUserImportRequest, admin_user_id: Uuid) -> Result<BulkUserImportResponse>;

    /// Bulk export users
    async fn bulk_export_users(&self, request: BulkUserExportRequest, admin_user_id: Uuid) -> Result<BulkUserExportResponse>;

    /// Get bulk operation status
    async fn get_bulk_operation_status(&self, request: BulkOperationStatusRequest) -> Result<BulkOperationStatusResponse>;

    /// Cancel bulk operation
    async fn cancel_bulk_operation(&self, operation_id: Uuid, admin_user_id: Uuid) -> Result<()>;

    /// Download exported file
    async fn download_export_file(&self, operation_id: Uuid) -> Result<Vec<u8>>;

    /// Validate import file
    async fn validate_import_file(&self, file_format: FileFormat, file_data: Vec<u8>) -> Result<FileValidationResult>;

    /// Get export template
    async fn get_export_template(&self, file_format: FileFormat) -> Result<Vec<u8>>;

    /// Get import field mapping
    async fn get_import_field_mapping(&self, file_format: FileFormat) -> Result<Vec<FieldMapping>>;

    /// Preview import data
    async fn preview_import_data(&self, request: BulkUserImportRequest, preview_rows: u32) -> Result<ImportPreview>;
}

/// File validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileValidationResult {
    pub is_valid: bool,
    pub file_format: FileFormat,
    pub estimated_records: u32,
    pub detected_encoding: String,
    pub has_headers: bool,
    pub detected_columns: Vec<String>,
    pub validation_errors: Vec<String>,
    pub warnings: Vec<String>,
    pub suggested_mappings: HashMap<String, String>,
}

/// Field mapping for import/export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMapping {
    pub source_field: String,
    pub target_field: String,
    pub required: bool,
    pub data_type: String,
    pub default_value: Option<String>,
    pub validation_rules: Option<String>,
}

/// Import preview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPreview {
    pub total_rows: u32,
    pub preview_rows: Vec<HashMap<String, serde_json::Value>>,
    pub detected_issues: Vec<String>,
    pub suggested_mappings: HashMap<String, String>,
    pub estimated_processing_time: u32,
}

/// Default implementation of Bulk Operations Service
pub struct DefaultBulkOperationsService {
    user_repository: Arc<dyn UserRepository>,
    search_service: Arc<dyn UserSearchService>,
    file_processor: Arc<dyn FileProcessor>,
    operation_tracker: Arc<dyn BulkOperationTracker>,
}

#[async_trait::async_trait]
impl BulkOperationsService for DefaultBulkOperationsService {
    async fn advanced_user_search(&self, request: AdvancedUserSearchRequest) -> Result<AdvancedUserSearchResponse> {
        let start_time = std::time::Instant::now();

        // Build search query
        let search_query = self.build_search_query(&request)?;

        // Execute search
        let (users, total_count) = self.search_service.search_users(search_query).await?;

        // Process results
        let search_results = self.process_search_results(users, &request).await?;

        let search_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(AdvancedUserSearchResponse {
            users: search_results,
            total_count,
            page: request.page,
            limit: request.limit,
            total_pages: (total_count as f32 / request.limit as f32).ceil() as u32,
            search_time_ms,
            applied_filters: self.extract_applied_filters(&request),
        })
    }

    async fn bulk_import_users(&self, request: BulkUserImportRequest, admin_user_id: Uuid) -> Result<BulkUserImportResponse> {
        let operation_id = Uuid::new_v4();

        // Start operation tracking
        self.operation_tracker.start_operation(operation_id, BulkOperationType::ImportUsers).await?;

        let start_time = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut successful_imports = 0;
        let mut failed_imports = 0;
        let mut skipped_duplicates = 0;
        let mut updated_records = 0;

        // Parse file data
        let records = self.file_processor.parse_file(&request.file_format, &request.file_data).await?;

        for (index, record) in records.iter().enumerate() {
            // Update progress
            let progress = (index as f32 / records.len() as f32) * 100.0;
            self.operation_tracker.update_progress(operation_id, progress).await?;

            // Validate and process record
            match self.process_import_record(record, &request, index as u32 + 1).await {
                Ok(result) => {
                    match result {
                        ImportRecordResult::Created => successful_imports += 1,
                        ImportRecordResult::Updated => updated_records += 1,
                        ImportRecordResult::SkippedDuplicate => skipped_duplicates += 1,
                    }
                },
                Err(error) => {
                    failed_imports += 1;
                    errors.push(ImportError {
                        row_number: index as u32 + 1,
                        field: None,
                        error_type: ImportErrorType::ValidationError,
                        error_message: error.to_string(),
                        original_value: None,
                        suggested_fix: None,
                    });
                }
            }
        }

        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        // Complete operation
        self.operation_tracker.complete_operation(operation_id).await?;

        Ok(BulkUserImportResponse {
            operation_id,
            total_records: records.len() as u32,
            successful_imports,
            failed_imports,
            skipped_duplicates,
            updated_records,
            processing_time_ms,
            errors,
            warnings: Vec::new(),
            summary: ImportSummary {
                new_users_count: successful_imports,
                updated_users_count: updated_records,
                skipped_count: skipped_duplicates,
                error_count: failed_imports,
                processing_rate: records.len() as f32 / (processing_time_ms as f32 / 1000.0),
                data_quality_score: self.calculate_data_quality_score(successful_imports, failed_imports, skipped_duplicates),
            },
        })
    }

    async fn bulk_export_users(&self, request: BulkUserExportRequest, admin_user_id: Uuid) -> Result<BulkUserExportResponse> {
        let operation_id = Uuid::new_v4();

        // Start operation tracking
        self.operation_tracker.start_operation(operation_id, BulkOperationType::ExportUsers).await?;

        let start_time = std::time::Instant::now();

        // Query users based on filters
        let search_query = self.build_export_query(&request)?;
        let (users, total_count) = self.search_service.search_users(search_query).await?;

        // Process and export data
        let file_data = self.file_processor.export_file(
            &request.file_format,
            users,
            &request.include_fields,
            request.exclude_sensitive_data,
            request.include_headers,
        ).await?;

        // Store file and get URL
        let file_url = self.store_export_file(operation_id, &file_data, &request.file_format).await?;

        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        // Complete operation
        self.operation_tracker.complete_operation(operation_id).await?;

        Ok(BulkUserExportResponse {
            operation_id,
            file_url,
            file_size_bytes: file_data.len() as u64,
            exported_records: users.len() as u32,
            file_format: request.file_format,
            expires_at: Utc::now() + constants::default_session_expiry(),
            processing_time_ms,
            compression_used: request.compression,
            export_metadata: ExportMetadata {
                exported_at: Utc::now(),
                exported_by: admin_user_id,
                filters_applied: self.extract_filters_applied(&request.search_filters),
                fields_included: request.include_fields.clone(),
                total_users_in_system: total_count,
                export_percentage: (users.len() as f32 / total_count as f32) * 100.0,
            },
        })
    }

    async fn get_bulk_operation_status(&self, request: BulkOperationStatusRequest) -> Result<BulkOperationStatusResponse> {
        self.operation_tracker.get_operation_status(request.operation_id).await
    }

    async fn cancel_bulk_operation(&self, operation_id: Uuid, admin_user_id: Uuid) -> Result<()> {
        self.operation_tracker.cancel_operation(operation_id).await
    }

    async fn download_export_file(&self, operation_id: Uuid) -> Result<Vec<u8>> {
        self.operation_tracker.get_export_file(operation_id).await
    }

    async fn validate_import_file(&self, file_format: FileFormat, file_data: Vec<u8>) -> Result<FileValidationResult> {
        self.file_processor.validate_file(file_format, file_data).await
    }

    async fn get_export_template(&self, file_format: FileFormat) -> Result<Vec<u8>> {
        self.file_processor.generate_template(file_format).await
    }

    async fn get_import_field_mapping(&self, file_format: FileFormat) -> Result<Vec<FieldMapping>> {
        self.file_processor.get_default_field_mapping(file_format).await
    }

    async fn preview_import_data(&self, request: BulkUserImportRequest, preview_rows: u32) -> Result<ImportPreview> {
        self.file_processor.preview_file(&request.file_format, &request.file_data, preview_rows).await
    }
}

// Placeholder traits for dependencies
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn create(&self, user: User) -> Result<User>;
    async fn update(&self, user: User) -> Result<User>;
}

#[async_trait::async_trait]
pub trait UserSearchService: Send + Sync {
    async fn search_users(&self, query: UserSearchQuery) -> Result<(Vec<User>, u64)>;
}

#[async_trait::async_trait]
pub trait FileProcessor: Send + Sync {
    async fn parse_file(&self, format: &FileFormat, data: &[u8]) -> Result<Vec<HashMap<String, serde_json::Value>>>;
    async fn export_file(&self, format: &FileFormat, users: Vec<User>, fields: &[String], exclude_sensitive: bool, include_headers: bool) -> Result<Vec<u8>>;
    async fn validate_file(&self, format: FileFormat, data: Vec<u8>) -> Result<FileValidationResult>;
    async fn generate_template(&self, format: FileFormat) -> Result<Vec<u8>>;
    async fn get_default_field_mapping(&self, format: FileFormat) -> Result<Vec<FieldMapping>>;
    async fn preview_file(&self, format: &FileFormat, data: &[u8], rows: u32) -> Result<ImportPreview>;
}

#[async_trait::async_trait]
pub trait BulkOperationTracker: Send + Sync {
    async fn start_operation(&self, operation_id: Uuid, operation_type: BulkOperationType) -> Result<()>;
    async fn update_progress(&self, operation_id: Uuid, progress: f32) -> Result<()>;
    async fn complete_operation(&self, operation_id: Uuid) -> Result<()>;
    async fn cancel_operation(&self, operation_id: Uuid) -> Result<()>;
    async fn get_operation_status(&self, operation_id: Uuid) -> Result<BulkOperationStatusResponse>;
    async fn get_export_file(&self, operation_id: Uuid) -> Result<Vec<u8>>;
}

// Internal types and helper methods
pub struct UserSearchQuery {
    pub query: Option<String>,
    pub filters: UserSearchFilters,
    pub sort: Vec<UserSearchSort>,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Clone)]
pub enum ImportRecordResult {
    Created,
    Updated,
    SkippedDuplicate,
}

impl DefaultBulkOperationsService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        search_service: Arc<dyn UserSearchService>,
        file_processor: Arc<dyn FileProcessor>,
        operation_tracker: Arc<dyn BulkOperationTracker>,
    ) -> Self {
        Self {
            user_repository,
            search_service,
            file_processor,
            operation_tracker,
        }
    }

    fn build_search_query(&self, request: &AdvancedUserSearchRequest) -> Result<UserSearchQuery> {
        Ok(UserSearchQuery {
            query: request.query.clone(),
            filters: request.filters.clone(),
            sort: request.sort.clone(),
            page: request.page,
            limit: request.limit,
        })
    }

    async fn process_search_results(&self, users: Vec<User>, request: &AdvancedUserSearchRequest) -> Result<Vec<UserSearchResult>> {
        let mut results = Vec::new();

        for user in users {
            let relevance_score = self.calculate_relevance_score(&user, &request.query);
            let highlight_matches = self.generate_highlight_matches(&user, &request.query);
            let last_login_ago = self.calculate_last_login_ago(&user);
            let account_age = self.calculate_account_age(&user);
            let login_frequency = self.calculate_login_frequency(&user);

            results.push(UserSearchResult {
                user,
                relevance_score,
                highlight_matches,
                last_login_ago,
                account_age,
                login_frequency,
            });
        }

        Ok(results)
    }

    async fn process_import_record(&self, record: &HashMap<String, serde_json::Value>, request: &BulkUserImportRequest, row_number: u32) -> Result<ImportRecordResult> {
        // Map fields from record to User entity
        let email_str = record.get("email")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Email field is missing"))?;

        let email = Email::new(email_str)
            .map_err(|e| anyhow!("Invalid email: {}", e))?;

        // Check for existing user
        if let Some(_existing_user) = self.user_repository.find_by_email(email.as_str()).await? {
            if request.skip_duplicates {
                return Ok(ImportRecordResult::SkippedDuplicate);
            } else if request.update_existing {
                // Update existing user logic here
                return Ok(ImportRecordResult::Updated);
            } else {
                return Err(anyhow!("Duplicate email found"));
            }
        }

        // Create new user logic here
        Ok(ImportRecordResult::Created)
    }

    fn calculate_data_quality_score(&self, successful: u32, failed: u32, skipped: u32) -> f32 {
        let total = successful + failed + skipped;
        if total == 0 {
            return 0.0;
        }
        (successful as f32 / total as f32) * 100.0
    }

    fn calculate_relevance_score(&self, user: &User, query: &Option<String>) -> f32 {
        match query {
            Some(q) if !q.is_empty() => {
                let query_lower = q.to_lowercase();
                let email_lower = user.email.to_lowercase();
                let name_lower = format!("{} {}", user.first_name, user.last_name).to_lowercase();

                let mut score = 0.0;

                if email_lower.contains(&query_lower) {
                    score += 0.8;
                }
                if name_lower.contains(&query_lower) {
                    score += 0.6;
                }

                score.min(1.0)
            },
            _ => 1.0,
        }
    }

    fn generate_highlight_matches(&self, user: &User, query: &Option<String>) -> HashMap<String, Vec<String>> {
        let mut highlights = HashMap::new();

        if let Some(q) = query {
            if !q.is_empty() {
                let query_lower = q.to_lowercase();

                if user.email.to_lowercase().contains(&query_lower) {
                    highlights.insert("email".to_string(), vec![user.email.clone()]);
                }

                let full_name = format!("{} {}", user.first_name, user.last_name);
                if full_name.to_lowercase().contains(&query_lower) {
                    highlights.insert("name".to_string(), vec![full_name]);
                }
            }
        }

        highlights
    }

    fn calculate_last_login_ago(&self, user: &User) -> Option<String> {
        // Calculate based on user's last login timestamp
        // This is a placeholder implementation
        Some("2 days ago".to_string())
    }

    fn calculate_account_age(&self, user: &User) -> Option<String> {
        let now = Utc::now();
        let duration = now.signed_duration_since(user.created_at);

        if duration.num_days() > 365 {
            Some(format!("{} years", duration.num_days() / 365))
        } else if duration.num_days() > 30 {
            Some(format!("{} months", duration.num_days() / 30))
        } else {
            Some(format!("{} days", duration.num_days()))
        }
    }

    fn calculate_login_frequency(&self, user: &User) -> Option<String> {
        // Calculate based on user's login history
        // This is a placeholder implementation
        Some("Weekly".to_string())
    }

    fn extract_applied_filters(&self, request: &AdvancedUserSearchRequest) -> Vec<String> {
        let mut filters = Vec::new();

        if let Some(query) = &request.query {
            if !query.is_empty() {
                filters.push(format!("Search: {}", query));
            }
        }

        if request.filters.email_contains.is_some() {
            filters.push("Email filter applied".to_string());
        }

        if request.filters.status_filter.is_some() {
            filters.push("Status filter applied".to_string());
        }

        filters
    }

    fn build_export_query(&self, request: &BulkUserExportRequest) -> Result<UserSearchQuery> {
        let filters = request.search_filters.clone().unwrap_or_default();

        Ok(UserSearchQuery {
            query: None,
            filters,
            sort: vec![],
            page: 1,
            limit: request.max_records.unwrap_or(10000),
        })
    }

    async fn store_export_file(&self, operation_id: Uuid, file_data: &[u8], format: &FileFormat) -> Result<String> {
        // Store file to storage service and return URL
        // This is a placeholder implementation
        let filename = format!("export_{}_{}.{}",
            operation_id,
            Utc::now().format("%Y%m%d_%H%M%S"),
            match format {
                FileFormat::CSV => "csv",
                FileFormat::JSON => "json",
                FileFormat::XML => "xml",
                FileFormat::Excel => "xlsx",
            }
        );

        Ok(format!("https://storage.example.com/exports/{}", filename))
    }

    fn extract_filters_applied(&self, filters: &Option<UserSearchFilters>) -> Vec<String> {
        let mut applied = Vec::new();

        if let Some(f) = filters {
            if f.email_contains.is_some() {
                applied.push("Email contains".to_string());
            }
            if f.status_filter.is_some() {
                applied.push("Status filter".to_string());
            }
            if f.created_after.is_some() {
                applied.push("Created after".to_string());
            }
            if f.created_before.is_some() {
                applied.push("Created before".to_string());
            }
        }

        applied
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bulk_operation_type() {
        assert_eq!(BulkOperationType::ImportUsers, BulkOperationType::ImportUsers);
        assert_ne!(BulkOperationType::ImportUsers, BulkOperationType::ExportUsers);
    }

    #[test]
    fn test_file_format() {
        assert_eq!(FileFormat::CSV, FileFormat::CSV);
        assert_ne!(FileFormat::CSV, FileFormat::JSON);
    }

    #[test]
    fn test_sort_direction() {
        assert_eq!(SortDirection::Ascending, SortDirection::Ascending);
        assert_ne!(SortDirection::Ascending, SortDirection::Descending);
    }

    #[test]
    fn test_validation_mode() {
        assert_eq!(ValidationMode::SkipInvalid, ValidationMode::SkipInvalid);
        assert_ne!(ValidationMode::SkipInvalid, ValidationMode::Strict);
    }

    #[test]
    fn test_import_error_type() {
        assert_eq!(ImportErrorType::ValidationError, ImportErrorType::ValidationError);
        assert_ne!(ImportErrorType::ValidationError, ImportErrorType::DuplicateRecord);
    }
}