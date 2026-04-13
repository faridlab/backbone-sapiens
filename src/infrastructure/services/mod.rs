//! Infrastructure Services - Cross-Module Query Services
//!
//! This module contains query services that expose a minimal public API
//! for other bounded contexts. These services act as an Anti-Corruption Layer,
//! preventing tight coupling between modules.
//!
//! # Purpose
//!
//! When Postman or Bucket modules need user information, they should NOT:
//! - Import Sapiens domain entities directly
//! - Depend on Sapiens internal repository interfaces
//! - Access Sapiens database directly
//!
//! Instead, they use these Query Services which:
//! - Expose only the data other modules need (DTOs, not domain entities)
//! - Provide a stable interface that can evolve independently
//! - Allow Sapiens to change internal implementation without breaking clients
//!
//! # Usage
//!
//! ```rust,ignore
//! use backbone_sapiens::infrastructure::services::UserQueryService;
//!
//! // In Postman module
//! let user_query = UserQueryService::new(sapiens_db_pool);
//!
//! // Query user by ID (returns minimal DTO, not full User entity)
//! if let Some(info) = user_query.get_user_info(&user_id).await? {
//!     send_email(&info.email, "Welcome!");
//! }
//! ```

pub mod user_query;

pub use user_query::{
    UserQueryService,
    UserInfo,
    UserExistsResult,
};
