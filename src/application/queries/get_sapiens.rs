// Get Sapiens Query
// Query handler for retrieving Sapiens entities

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::domain::entity::Sapiens;
use crate::domain::repositories::{SapiensRepository, PaginationParams, SortParams};
use crate::domain::value_objects::{SapiensId, SapiensStatus};
use crate::domain::{DomainError, DomainResult};

// Query DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSapiensQuery {
    pub id: String,
}

impl GetSapiensQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn validate(&self) -> DomainResult<()> {
        if self.id.trim().is_empty() {
            return Err(DomainError::ValidationError {
                message: "Sapiens ID cannot be empty".to_string(),
            });
        }

        // Basic UUID format validation
        let id = SapiensId::new(&self.id).map_err(|_| DomainError::ValidationError {
            message: "Invalid Sapiens ID format".to_string(),
        })?;

        // If we get here, the ID is valid
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSapienssQuery {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub filters: Option<SapiensFilters>,
}

impl ListSapienssQuery {
    pub fn new() -> Self {
        Self {
            page: None,
            page_size: None,
            sort_by: None,
            sort_direction: None,
            filters: None,
        }
    }

    pub fn with_pagination(mut self, page: usize, page_size: usize) -> Self {
        self.page = Some(page);
        self.page_size = Some(page_size);
        self
    }

    pub fn with_sort(mut self, sort_by: String, sort_direction: String) -> Self {
        self.sort_by = Some(sort_by);
        self.sort_direction = Some(sort_direction);
        self
    }

    pub fn with_filters(mut self, filters: SapiensFilters) -> Self {
        self.filters = Some(filters);
        self
    }

    pub fn validate(&self) -> DomainResult<()> {
        if let Some(page) = self.page {
            if page == 0 {
                return Err(DomainError::ValidationError {
                    message: "Page number must be greater than 0".to_string(),
                });
            }
        }

        if let Some(page_size) = self.page_size {
            if page_size == 0 {
                return Err(DomainError::ValidationError {
                    message: "Page size must be greater than 0".to_string(),
                });
            }
            if page_size > 100 {
                return Err(DomainError::ValidationError {
                    message: "Page size cannot exceed 100".to_string(),
                });
            }
        }

        if let Some(ref sort_by) = self.sort_by {
            let valid_sort_fields = vec!["id", "name", "status", "created_at", "updated_at", "created_by"];
            if !valid_sort_fields.contains(&sort_by.as_str()) {
                return Err(DomainError::ValidationError {
                    message: format!("Invalid sort field: {}", sort_by),
                });
            }
        }

        if let Some(ref sort_direction) = self.sort_direction {
            let valid_directions = vec!["asc", "ascending", "desc", "descending"];
            if !valid_directions.contains(&sort_direction.to_lowercase().as_str()) {
                return Err(DomainError::ValidationError {
                    message: format!("Invalid sort direction: {}", sort_direction),
                });
            }
        }

        Ok(())
    }

    pub fn get_pagination_params(&self) -> PaginationParams {
        PaginationParams::new(
            self.page.unwrap_or(1),
            self.page_size.unwrap_or(20),
        )
    }

    pub fn get_sort_params(&self) -> SortParams {
        use crate::domain::repositories::{SortDirection, SortField};

        let field = match self.sort_by.as_ref().map(|s| s.as_str()) {
            Some("id") => SortField::Id,
            Some("name") => SortField::Name,
            Some("status") => SortField::Status,
            Some("updated_at") => SortField::UpdatedAt,
            Some("created_by") => SortField::CreatedBy,
            _ => SortField::CreatedAt, // Default
        };

        let direction = match self.sort_direction.as_ref().map(|s| s.to_lowercase().as_str()) {
            Some("desc") | Some("descending") => SortDirection::Descending,
            _ => SortDirection::Ascending, // Default
        };

        SortParams::new(field, direction)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSapienssQuery {
    pub query: String,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
}

impl SearchSapienssQuery {
    pub fn new(query: String) -> Self {
        Self {
            query,
            page: None,
            page_size: None,
            sort_by: None,
            sort_direction: None,
        }
    }

    pub fn with_pagination(mut self, page: usize, page_size: usize) -> Self {
        self.page = Some(page);
        self.page_size = Some(page_size);
        self
    }

    pub fn with_sort(mut self, sort_by: String, sort_direction: String) -> Self {
        self.sort_by = Some(sort_by);
        self.sort_direction = Some(sort_direction);
        self
    }

    pub fn validate(&self) -> DomainResult<()> {
        if self.query.trim().is_empty() {
            return Err(DomainError::ValidationError {
                message: "Search query cannot be empty".to_string(),
            });
        }

        if self.query.len() > 1000 {
            return Err(DomainError::ValidationError {
                message: "Search query cannot exceed 1000 characters".to_string(),
            });
        }

        // Validate pagination and sorting (reuse validation from ListSapienssQuery)
        let list_query = ListSapienssQuery {
            page: self.page,
            page_size: self.page_size,
            sort_by: self.sort_by.clone(),
            sort_direction: self.sort_direction.clone(),
            filters: None,
        };

        list_query.validate()
    }

    pub fn get_pagination_params(&self) -> PaginationParams {
        PaginationParams::new(
            self.page.unwrap_or(1),
            self.page_size.unwrap_or(20),
        )
    }

    pub fn get_sort_params(&self) -> SortParams {
        let list_query = ListSapienssQuery {
            page: None,
            page_size: None,
            sort_by: self.sort_by.clone(),
            sort_direction: self.sort_direction.clone(),
            filters: None,
        };

        list_query.get_sort_params()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SapiensFilters {
    pub status: Option<String>,
    pub tags: Option<Vec<String>>,
    pub created_by: Option<String>,
    pub created_after: Option<String>, // ISO 8601 datetime
    pub created_before: Option<String>, // ISO 8601 datetime
    pub updated_after: Option<String>, // ISO 8601 datetime
    pub updated_before: Option<String>, // ISO 8601 datetime
    pub metadata: Option<HashMap<String, String>>,
}

impl SapiensFilters {
    pub fn new() -> Self {
        Self {
            status: None,
            tags: None,
            created_by: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            metadata: None,
        }
    }

    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn with_created_by(mut self, created_by: String) -> Self {
        self.created_by = Some(created_by);
        self
    }

    pub fn with_date_range(
        mut self,
        created_after: Option<String>,
        created_before: Option<String>,
    ) -> Self {
        self.created_after = created_after;
        self.created_before = created_before;
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn validate(&self) -> DomainResult<()> {
        // Validate status
        if let Some(ref status) = self.status {
            let valid_statuses = vec!["ACTIVE", "INACTIVE", "SUSPENDED", "ARCHIVED"];
            if !valid_statuses.contains(&status.to_uppercase().as_str()) {
                return Err(DomainError::ValidationError {
                    message: format!("Invalid status: {}", status),
                });
            }
        }

        // Validate tags
        if let Some(ref tags) = self.tags {
            if tags.len() > 50 {
                return Err(DomainError::ValidationError {
                    message: "Cannot filter by more than 50 tags".to_string(),
                });
            }

            for tag in tags {
                if tag.trim().is_empty() {
                    return Err(DomainError::ValidationError {
                        message: "Filter tags cannot be empty".to_string(),
                    });
                }
                if tag.len() > 50 {
                    return Err(DomainError::ValidationError {
                        message: "Filter tag cannot exceed 50 characters".to_string(),
                    });
                }
            }
        }

        // Validate date formats (basic ISO 8601 validation)
        for date_field in [
            &self.created_after,
            &self.created_before,
            &self.updated_after,
            &self.updated_before,
        ] {
            if let Some(date_str) = date_field {
                if let Err(_) = chrono::DateTime::parse_from_rfc3339(date_str) {
                    return Err(DomainError::ValidationError {
                        message: format!("Invalid date format: {}. Expected ISO 8601 format", date_str),
                    });
                }
            }
        }

        // Validate metadata
        if let Some(ref metadata) = self.metadata {
            if metadata.len() > 20 {
                return Err(DomainError::ValidationError {
                    message: "Cannot filter by more than 20 metadata key-value pairs".to_string(),
                });
            }

            for (key, value) in metadata {
                if key.is_empty() {
                    return Err(DomainError::ValidationError {
                        message: "Metadata filter keys cannot be empty".to_string(),
                    });
                }
                if key.len() > 50 {
                    return Err(DomainError::ValidationError {
                        message: "Metadata filter key cannot exceed 50 characters".to_string(),
                    });
                }
                if value.len() > 500 {
                    return Err(DomainError::ValidationError {
                        message: "Metadata filter value cannot exceed 500 characters".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    pub fn to_repository_filters(&self) -> crate::domain::repositories::SapiensFilters {
        use crate::domain::repositories::SapiensFilters as RepoFilters;

        let mut filters = RepoFilters::new();

        if let Some(ref status) = self.status {
            filters.status = Some(match status.to_uppercase().as_str() {
                "ACTIVE" => SapiensStatus::Active,
                "INACTIVE" => SapiensStatus::Inactive,
                "SUSPENDED" => SapiensStatus::Suspended,
                "ARCHIVED" => SapiensStatus::Archived,
                _ => SapiensStatus::Active, // Default fallback
            });
        }

        if let Some(ref tags) = self.tags {
            filters.tags = Some(tags.clone());
        }

        if let Some(ref created_by) = self.created_by {
            filters.created_by = Some(created_by.clone());
        }

        if let Some(ref created_after) = self.created_after {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(created_after) {
                filters.created_after = Some(dt.with_timezone(&chrono::Utc));
            }
        }

        if let Some(ref created_before) = self.created_before {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(created_before) {
                filters.created_before = Some(dt.with_timezone(&chrono::Utc));
            }
        }

        if let Some(ref updated_after) = self.updated_after {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(updated_after) {
                filters.updated_after = Some(dt.with_timezone(&chrono::Utc));
            }
        }

        if let Some(ref updated_before) = self.updated_before {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(updated_before) {
                filters.updated_before = Some(dt.with_timezone(&chrono::Utc));
            }
        }

        if let Some(ref metadata) = self.metadata {
            filters.metadata = Some(metadata.clone());
        }

        filters
    }
}

impl Default for SapiensFilters {
    fn default() -> Self {
        Self::new()
    }
}

// Response DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSapiensResponse {
    pub success: bool,
    pub sapiens: Option<SapiensDto>,
    pub message: String,
}

impl GetSapiensResponse {
    pub fn success(sapiens: Sapiens) -> Self {
        Self {
            success: true,
            sapiens: Some(SapiensDto::from(sapiens)),
            message: "Sapiens retrieved successfully".to_string(),
        }
    }

    pub fn not_found(id: String) -> Self {
        Self {
            success: false,
            sapiens: None,
            message: format!("Sapiens with ID '{}' not found", id),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            sapiens: None,
            message,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSapienssResponse {
    pub success: bool,
    pub sapienss: Vec<SapiensDto>,
    pub total: u64,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
    pub has_next: bool,
    pub has_previous: bool,
    pub message: String,
}

impl ListSapienssResponse {
    pub fn success(
        paginated_result: crate::domain::repositories::PaginatedResult<Sapiens>,
    ) -> Self {
        let sapienss: Vec<SapiensDto> = paginated_result
            .items
            .into_iter()
            .map(SapiensDto::from)
            .collect();

        Self {
            success: true,
            sapienss,
            total: paginated_result.total,
            page: paginated_result.page,
            page_size: paginated_result.page_size,
            total_pages: paginated_result.total_pages,
            has_next: paginated_result.has_next,
            has_previous: paginated_result.has_previous,
            message: "Sapienss retrieved successfully".to_string(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            sapienss: Vec::new(),
            total: 0,
            page: 1,
            page_size: 20,
            total_pages: 0,
            has_next: false,
            has_previous: false,
            message,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SapiensDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub created_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,
}

impl From<Sapiens> for SapiensDto {
    fn from(sapiens: Sapiens) -> Self {
        Self {
            id: sapiens.id.value().to_string(),
            name: sapiens.name.to_string(),
            description: sapiens.description().to_string(),
            status: sapiens.status.to_string(),
            tags: sapiens.tags().clone(),
            metadata: sapiens.metadata().to_map(),
            created_by: sapiens.created_by().to_string(),
            created_at: *sapiens.created_at,
            updated_at: *sapiens.updated_at(),
            deleted_at: sapiens.deleted_at().map(|dt| *dt),
            version: sapiens.version().value(),
        }
    }
}

// Query Handler Traits
#[async_trait]
pub trait GetSapiensHandler: Send + Sync {
    async fn handle(&self, query: GetSapiensQuery) -> DomainResult<GetSapiensResponse>;
}

#[async_trait]
pub trait ListSapienssHandler: Send + Sync {
    async fn handle(&self, query: ListSapienssQuery) -> DomainResult<ListSapienssResponse>;
}

#[async_trait]
pub trait SearchSapienssHandler: Send + Sync {
    async fn handle(&self, query: SearchSapienssQuery) -> DomainResult<ListSapienssResponse>;
}

// Default Query Handler Implementations
pub struct DefaultGetSapiensHandler {
    repository: Box<dyn SapiensRepository>,
}

impl DefaultGetSapiensHandler {
    pub fn new(repository: Box<dyn SapiensRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl GetSapiensHandler for DefaultGetSapiensHandler {
    async fn handle(&self, query: GetSapiensQuery) -> DomainResult<GetSapiensResponse> {
        // Validate query
        query.validate()?;

        let sapiens_id = SapiensId::new(&query.id)
            .map_err(|e| DomainError::ValidationError { message: e.to_string() })?;

        // Fetch from repository
        match self.repository.find_by_id(&sapiens_id).await {
            Ok(Some(sapiens)) => Ok(GetSapiensResponse::success(sapiens)),
            Ok(None) => Ok(GetSapiensResponse::not_found(query.id)),
            Err(e) => Err(DomainError::from(e)),
        }
    }
}

pub struct DefaultListSapienssHandler {
    repository: Box<dyn SapiensRepository>,
}

impl DefaultListSapienssHandler {
    pub fn new(repository: Box<dyn SapiensRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ListSapienssHandler for DefaultListSapienssHandler {
    async fn handle(&self, query: ListSapienssQuery) -> DomainResult<ListSapienssResponse> {
        // Validate query
        query.validate()?;

        let pagination = query.get_pagination_params();
        let sort = query.get_sort_params();

        // Fetch from repository
        let result = if let Some(filters) = &query.filters {
            filters.validate()?;
            let repo_filters = filters.to_repository_filters();
            self.repository
                .find_with_filters(repo_filters, pagination, sort)
                .await
        } else {
            self.repository.find_all(pagination, sort).await
        };

        match result {
            Ok(paginated_result) => Ok(ListSapienssResponse::success(paginated_result)),
            Err(e) => Err(DomainError::from(e)),
        }
    }
}

pub struct DefaultSearchSapienssHandler {
    repository: Box<dyn SapiensRepository>,
}

impl DefaultSearchSapienssHandler {
    pub fn new(repository: Box<dyn SapiensRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl SearchSapienssHandler for DefaultSearchSapienssHandler {
    async fn handle(&self, query: SearchSapienssQuery) -> DomainResult<ListSapienssResponse> {
        // Validate query
        query.validate()?;

        let pagination = query.get_pagination_params();
        let sort = query.get_sort_params();

        // Search in repository
        match self.repository.search(&query.query, pagination, sort).await {
            Ok(paginated_result) => Ok(ListSapienssResponse::success(paginated_result)),
            Err(e) => Err(DomainError::from(e)),
        }
    }
}

// Handler Factory
pub struct SapiensQueryHandlerFactory;

impl SapiensQueryHandlerFactory {
    pub fn create_get_handler(
        repository: Box<dyn SapiensRepository>,
    ) -> Box<dyn GetSapiensHandler> {
        Box::new(DefaultGetSapiensHandler::new(repository))
    }

    pub fn create_list_handler(
        repository: Box<dyn SapiensRepository>,
    ) -> Box<dyn ListSapienssHandler> {
        Box::new(DefaultListSapienssHandler::new(repository))
    }

    pub fn create_search_handler(
        repository: Box<dyn SapiensRepository>,
    ) -> Box<dyn SearchSapienssHandler> {
        Box::new(DefaultSearchSapienssHandler::new(repository))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::PaginationParams;
    use crate::domain::value_objects::{SapiensName, Metadata};
    use async_trait::async_trait;

    // Mock repository for testing
    struct MockRepository {
        should_fail: bool,
        should_return_none: bool,
    }

    impl MockRepository {
        fn new() -> Self {
            Self {
                should_fail: false,
                should_return_none: false,
            }
        }

        fn with_failure(mut self) -> Self {
            self.should_fail = true;
            self
        }

        fn with_none(mut self) -> Self {
            self.should_return_none = true;
            self
        }
    }

    #[async_trait]
    impl SapiensRepository for MockRepository {
        async fn save(&self, _sapiens: &Sapiens) -> crate::domain::repositories::RepositoryResult<()> {
            Ok(())
        }

        async fn find_by_id(&self, _id: &SapiensId) -> crate::domain::repositories::RepositoryResult<Option<Sapiens>> {
            if self.should_fail {
                Err(crate::domain::repositories::RepositoryError::DatabaseError {
                    message: "Database error".to_string(),
                })
            } else if self.should_return_none {
                Ok(None)
            } else {
                // Return a test sapiens
                let sapiens = Sapiens::create(
                    SapiensName::new("Test Sapiens").unwrap(),
                    "Test Description".to_string(),
                    vec!["test".to_string()],
                    Metadata::new(),
                    "test_user".to_string(),
                ).unwrap();
                Ok(Some(sapiens))
            }
        }

        async fn delete(&self, _id: &SapiensId, _hard_delete: bool) -> crate::domain::repositories::RepositoryResult<()> {
            Ok(())
        }

        async fn find_all(
            &self,
            _pagination: PaginationParams,
            _sort: crate::domain::repositories::SortParams,
        ) -> crate::domain::repositories::RepositoryResult<crate::domain::repositories::PaginatedResult<Sapiens>> {
            Ok(crate::domain::repositories::PaginatedResult::empty(1, 20))
        }

        async fn find_with_filters(
            &self,
            _filters: crate::domain::repositories::SapiensFilters,
            _pagination: PaginationParams,
            _sort: crate::domain::repositories::SortParams,
        ) -> crate::domain::repositories::RepositoryResult<crate::domain::repositories::PaginatedResult<Sapiens>> {
            Ok(crate::domain::repositories::PaginatedResult::empty(1, 20))
        }

        async fn find_by_status(
            &self,
            _status: SapiensStatus,
            _pagination: PaginationParams,
            _sort: crate::domain::repositories::SortParams,
        ) -> crate::domain::repositories::RepositoryResult<crate::domain::repositories::PaginatedResult<Sapiens>> {
            Ok(crate::domain::repositories::PaginatedResult::empty(1, 20))
        }

        async fn find_by_tags(
            &self,
            _tags: Vec<String>,
            _pagination: PaginationParams,
            _sort: crate::domain::repositories::SortParams,
        ) -> crate::domain::repositories::RepositoryResult<crate::domain::repositories::PaginatedResult<Sapiens>> {
            Ok(crate::domain::repositories::PaginatedResult::empty(1, 20))
        }

        async fn find_by_created_by(
            &self,
            _created_by: &str,
            _pagination: PaginationParams,
            _sort: crate::domain::repositories::SortParams,
        ) -> crate::domain::repositories::RepositoryResult<crate::domain::repositories::PaginatedResult<Sapiens>> {
            Ok(crate::domain::repositories::PaginatedResult::empty(1, 20))
        }

        async fn search(
            &self,
            _query: &str,
            _pagination: PaginationParams,
            _sort: crate::domain::repositories::SortParams,
        ) -> crate::domain::repositories::RepositoryResult<crate::domain::repositories::PaginatedResult<Sapiens>> {
            Ok(crate::domain::repositories::PaginatedResult::empty(1, 20))
        }

        async fn save_batch(&self, _sapienss: &[Sapiens]) -> crate::domain::repositories::RepositoryResult<()> {
            Ok(())
        }

        async fn delete_batch(&self, _ids: &[SapiensId], _hard_delete: bool) -> crate::domain::repositories::RepositoryResult<()> {
            Ok(())
        }

        async fn exists(&self, _id: &SapiensId) -> crate::domain::repositories::RepositoryResult<bool> {
            Ok(false)
        }

        async fn count(&self, _filters: Option<crate::domain::repositories::SapiensFilters>) -> crate::domain::repositories::RepositoryResult<u64> {
            Ok(0)
        }

        async fn find_deleted(
            &self,
            _pagination: PaginationParams,
            _sort: crate::domain::repositories::SortParams,
        ) -> crate::domain::repositories::RepositoryResult<crate::domain::repositories::PaginatedResult<Sapiens>> {
            Ok(crate::domain::repositories::PaginatedResult::empty(1, 20))
        }

        async fn restore(&self, _id: &SapiensId) -> crate::domain::repositories::RepositoryResult<()> {
            Ok(())
        }

        async fn find_by_metadata(
            &self,
            _metadata_key: &str,
            _metadata_value: Option<&str>,
            _pagination: PaginationParams,
            _sort: crate::domain::repositories::SortParams,
        ) -> crate::domain::repositories::RepositoryResult<crate::domain::repositories::PaginatedResult<Sapiens>> {
            Ok(crate::domain::repositories::PaginatedResult::empty(1, 20))
        }

        async fn find_by_date_range(
            &self,
            _start_date: chrono::DateTime<chrono::Utc>,
            _end_date: chrono::DateTime<chrono::Utc>,
            _date_field: crate::domain::repositories::SortField,
            _pagination: PaginationParams,
            _sort: crate::domain::repositories::SortParams,
        ) -> crate::domain::repositories::RepositoryResult<crate::domain::repositories::PaginatedResult<Sapiens>> {
            Ok(crate::domain::repositories::PaginatedResult::empty(1, 20))
        }

        async fn get_status_counts(&self) -> crate::domain::repositories::RepositoryResult<std::collections::HashMap<SapiensStatus, u64>> {
            Ok(std::collections::HashMap::new())
        }

        async fn get_tag_counts(&self) -> crate::domain::repositories::RepositoryResult<std::collections::HashMap<String, u64>> {
            Ok(std::collections::HashMap::new())
        }

        async fn get_recently_created(&self, _days: i64, _limit: Option<usize>) -> crate::domain::repositories::RepositoryResult<Vec<Sapiens>> {
            Ok(Vec::new())
        }

        async fn health_check(&self) -> crate::domain::repositories::RepositoryResult<bool> {
            Ok(true)
        }

        async fn connection_pool_status(&self) -> crate::domain::repositories::RepositoryResult<std::collections::HashMap<String, serde_json::Value>> {
            Ok(std::collections::HashMap::new())
        }
    }

    #[tokio::test]
    async fn test_get_sapiens_query_validation() {
        // Valid query
        let valid_query = GetSapiensQuery::new("123e4567-e89b-12d3-a456-426614174000".to_string());
        assert!(valid_query.validate().is_ok());

        // Invalid query - empty ID
        let invalid_query = GetSapiensQuery::new("".to_string());
        assert!(invalid_query.validate().is_err());

        // Invalid query - bad format
        let invalid_query = GetSapiensQuery::new("invalid-uuid".to_string());
        assert!(invalid_query.validate().is_err());
    }

    #[tokio::test]
    async fn test_get_sapiens_handler_success() {
        let repository = Box::new(MockRepository::new());
        let handler = DefaultGetSapiensHandler::new(repository);

        let query = GetSapiensQuery::new("123e4567-e89b-12d3-a456-426614174000".to_string());
        let result = handler.handle(query).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.success);
        assert!(response.sapiens.is_some());
    }

    #[tokio::test]
    async fn test_get_sapiens_handler_not_found() {
        let repository = Box::new(MockRepository::new().with_none());
        let handler = DefaultGetSapiensHandler::new(repository);

        let query = GetSapiensQuery::new("123e4567-e89b-12d3-a456-426614174000".to_string());
        let result = handler.handle(query).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.success);
        assert!(response.sapiens.is_none());
        assert!(response.message.contains("not found"));
    }

    #[tokio::test]
    async fn test_list_sapienss_query_validation() {
        // Valid query
        let valid_query = ListSapienssQuery::new()
            .with_pagination(1, 20)
            .with_sort("name".to_string(), "asc".to_string());
        assert!(valid_query.validate().is_ok());

        // Invalid query - page 0
        let invalid_query = ListSapienssQuery::new().with_pagination(0, 20);
        assert!(invalid_query.validate().is_err());

        // Invalid query - page size 0
        let invalid_query = ListSapienssQuery::new().with_pagination(1, 0);
        assert!(invalid_query.validate().is_err());

        // Invalid query - page size too large
        let invalid_query = ListSapienssQuery::new().with_pagination(1, 101);
        assert!(invalid_query.validate().is_err());

        // Invalid sort field
        let invalid_query = ListSapienssQuery::new()
            .with_sort("invalid_field".to_string(), "asc".to_string());
        assert!(invalid_query.validate().is_err());

        // Invalid sort direction
        let invalid_query = ListSapienssQuery::new()
            .with_sort("name".to_string(), "invalid".to_string());
        assert!(invalid_query.validate().is_err());
    }

    #[tokio::test]
    async fn test_sapiens_filters_validation() {
        // Valid filters
        let valid_filters = SapiensFilters::new()
            .with_status("ACTIVE".to_string())
            .with_tags(vec!["test".to_string()])
            .with_created_by("user".to_string());
        assert!(valid_filters.validate().is_ok());

        // Invalid status
        let invalid_filters = SapiensFilters::new().with_status("INVALID".to_string());
        assert!(invalid_filters.validate().is_err());

        // Too many tags
        let too_many_tags = (0..51).map(|i| format!("tag{}", i)).collect();
        let invalid_filters = SapiensFilters::new().with_tags(too_many_tags);
        assert!(invalid_filters.validate().is_err());

        // Empty tag
        let invalid_filters = SapiensFilters::new().with_tags(vec!["".to_string()]);
        assert!(invalid_filters.validate().is_err());

        // Invalid date format
        let invalid_filters = SapiensFilters::new()
            .with_date_range(Some("invalid-date".to_string()), None);
        assert!(invalid_filters.validate().is_err());
    }

    #[tokio::test]
    async fn test_search_sapienss_query_validation() {
        // Valid query
        let valid_query = SearchSapienssQuery::new("test".to_string())
            .with_pagination(1, 20);
        assert!(valid_query.validate().is_ok());

        // Empty query
        let invalid_query = SearchSapienssQuery::new("".to_string());
        assert!(invalid_query.validate().is_err());

        // Query too long
        let long_query = "a".repeat(1001);
        let invalid_query = SearchSapienssQuery::new(long_query);
        assert!(invalid_query.validate().is_err());
    }

    #[tokio::test]
    async fn test_sapiens_dto_conversion() {
        let sapiens = Sapiens::create(
            SapiensName::new("Test Sapiens").unwrap(),
            "Test Description".to_string(),
            vec!["test".to_string()],
            {
                let mut metadata = Metadata::new();
                metadata.insert("env".to_string(), "test".to_string()).unwrap();
                metadata
            },
            "test_user".to_string(),
        ).unwrap();

        let dto = SapiensDto::from(sapiens);

        assert_eq!(dto.name, "Test Sapiens");
        assert_eq!(dto.description, "Test Description");
        assert_eq!(dto.status, "ACTIVE");
        assert_eq!(dto.tags, vec!["test"]);
        assert_eq!(dto.created_by, "test_user");
        assert!(dto.metadata.contains_key("env"));
    }
}