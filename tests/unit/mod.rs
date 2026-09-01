//! Unit Tests
//!
//! Comprehensive unit tests for Sapiens authentication module.
//! Tests are organized by layer: domain, services.
//! (The old handler-layer tests died with the unmounted auth handlers they
//! tested; the public auth surface is probed end-to-end by
//! tests/auth_hardening_probes.rs against the gated router.)

// Domain layer tests
pub mod domain;

// Service layer tests
pub mod services;

// Shared test utilities and mocks
pub mod mocks;

// Re-export common test utilities
pub use crate::test_utils::*;
