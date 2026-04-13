//! Integration Test Framework for Sapiens Module
//!
//! This module implements a layered integration test framework following
//! the architecture described in docs/integration-test-framework-guide.md
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │         Test Runner / CLI               │
//! ├─────────────────────────────────────────┤
//! │         Test Configuration              │
//! │         (config.rs)                     │
//! ├─────────────────────────────────────────┤
//! │         Specific Test Classes           │
//! │         (tests/*.rs)                    │
//! ├─────────────────────────────────────────┤
//! │         Base Test Classes               │
//! │         (framework/base_test.rs)        │
//! ├─────────────────────────────────────────┤
//! │         Common Utilities                │
//! │         (helpers/*.rs)                  │
//! └─────────────────────────────────────────┘
//! ```

pub mod framework;
pub mod helpers;
pub mod tests;
pub mod config;

// Re-export commonly used types
pub use framework::{Test, TestResult, TestSuiteResult, TestError, ApiTest, ApiResponse};
pub use helpers::{CommonUtils, JwtTokenManager};
pub use config::{TestRegistry, TestRunner, TestType};
