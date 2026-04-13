//! Unit Tests
//!
//! Comprehensive unit tests for Sapiens authentication module.
//! Tests are organized by layer: domain, services, handlers.

// Domain layer tests
pub mod domain;

// Service layer tests
pub mod services;

// Handler layer tests
pub mod handlers;

// Shared test utilities and mocks
pub mod mocks;

// Re-export common test utilities
pub use crate::test_utils::*;
