// ListPasswordResetToken Query
// Query for retrieving paginated PasswordResetToken lists with filtering

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPasswordResetTokenQuery {
    pub page: usize,
    pub page_size: usize,
    pub sort_by: Option<String>,
    pub sort_direction: String,
    pub filters: Option<crate::application::commands::PasswordResetTokenFilters>,
}

impl ListPasswordResetTokenQuery {
    pub fn new() -> Self {
        Self {
            page: 1,
            page_size: 20,
            sort_by: None,
            sort_direction: "asc".to_string(),
            filters: None,
        }
    }

    pub fn with_pagination(mut self, page: usize, page_size: usize) -> Self {
        self.page = page;
        self.page_size = page_size;
        self
    }

    pub fn with_sort(mut self, sort_by: String, sort_direction: String) -> Self {
        self.sort_by = Some(sort_by);
        self.sort_direction = sort_direction;
        self
    }

    pub fn with_filters(mut self, filters: crate::application::commands::PasswordResetTokenFilters) -> Self {
        self.filters = Some(filters);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPasswordResetTokenResponse {
    pub success: bool,
    pub message: String,
    pub password_reset_tokens: Vec<crate::application::commands::PasswordResetTokenDto>,
    pub page: usize,
    pub page_size: usize,
    pub total: u64,
    pub total_pages: usize,
    pub has_next: bool,
    pub has_previous: bool,
}

impl ListPasswordResetTokenResponse {
    pub fn success(
        password_reset_tokens: Vec<crate::application::commands::PasswordResetTokenDto>,
        page: usize,
        page_size: usize,
        total: u64,
    ) -> Self {
        let total_pages = ((total as f64) / (page_size as f64)).ceil() as usize;

        Self {
            success: true,
            message: format!("Retrieved {} password_reset_tokens", password_reset_tokens.len()),
            password_reset_tokens,
            page,
            page_size,
            total,
            total_pages,
            has_next: page < total_pages,
            has_previous: page > 1,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            password_reset_tokens: vec![],
            page: 1,
            page_size: 20,
            total: 0,
            total_pages: 0,
            has_next: false,
            has_previous: false,
        }
    }
}

// Query for listing deleted password_reset_tokens (trash)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDeletedPasswordResetTokenQuery {
    pub page: usize,
    pub page_size: usize,
    pub sort_by: Option<String>,
    pub sort_direction: String,
}

impl ListDeletedPasswordResetTokenQuery {
    pub fn new() -> Self {
        Self {
            page: 1,
            page_size: 20,
            sort_by: None,
            sort_direction: "desc".to_string(), // Sort by deleted_at by default
        }
    }

    pub fn with_pagination(mut self, page: usize, page_size: usize) -> Self {
        self.page = page;
        self.page_size = page_size;
        self
    }

    pub fn with_sort(mut self, sort_by: String, sort_direction: String) -> Self {
        self.sort_by = Some(sort_by);
        self.sort_direction = sort_direction;
        self
    }
}