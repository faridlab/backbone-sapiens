// ListSession Query
// Query for retrieving paginated Session lists with filtering

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSessionQuery {
    pub page: usize,
    pub page_size: usize,
    pub sort_by: Option<String>,
    pub sort_direction: String,
    pub filters: Option<crate::application::commands::SessionFilters>,
}

impl ListSessionQuery {
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

    pub fn with_filters(mut self, filters: crate::application::commands::SessionFilters) -> Self {
        self.filters = Some(filters);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSessionResponse {
    pub success: bool,
    pub message: String,
    pub sessions: Vec<crate::application::commands::SessionDto>,
    pub page: usize,
    pub page_size: usize,
    pub total: u64,
    pub total_pages: usize,
    pub has_next: bool,
    pub has_previous: bool,
}

impl ListSessionResponse {
    pub fn success(
        sessions: Vec<crate::application::commands::SessionDto>,
        page: usize,
        page_size: usize,
        total: u64,
    ) -> Self {
        let total_pages = ((total as f64) / (page_size as f64)).ceil() as usize;

        Self {
            success: true,
            message: format!("Retrieved {} sessions", sessions.len()),
            sessions,
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
            sessions: vec![],
            page: 1,
            page_size: 20,
            total: 0,
            total_pages: 0,
            has_next: false,
            has_previous: false,
        }
    }
}

// Query for listing deleted sessions (trash)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDeletedSessionQuery {
    pub page: usize,
    pub page_size: usize,
    pub sort_by: Option<String>,
    pub sort_direction: String,
}

impl ListDeletedSessionQuery {
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