# Sapiens Integration Tests

Comprehensive integration test suite for the Sapiens module, covering all 13 domain entities with 11+ CRUD endpoints each.

## Quick Start

```bash
# 1. Start the API server (in a separate terminal)
cd apps/backbone && cargo run --bin backbone

# 2. Run all integration tests
cargo test --package backbone-sapiens --test run_integration_tests -- --ignored --nocapture

# 3. Run specific test suites
cargo test --package backbone-sapiens --test run_integration_tests run_user_api_tests -- --ignored --nocapture
cargo test --package backbone-sapiens --test run_integration_tests run_role_api_tests -- --ignored --nocapture
cargo test --package backbone-sapiens --test run_integration_tests run_api_tests_only -- --ignored --nocapture
```

## Directory Structure

```
tests/
├── README.md                          # This file
├── integration_tests.rs               # Unit tests for module structure
├── http_crud_tests.rs                 # HTTP CRUD unit tests
├── run_integration_tests.rs           # Main integration test runner
│
├── integration/                       # Integration test framework
│   ├── mod.rs                         # Module exports
│   ├── config.rs                      # Test registry and configuration
│   │
│   ├── framework/                     # Core test framework
│   │   ├── mod.rs                     # Framework exports
│   │   ├── base_test.rs               # Test trait, TestResult, TestSuiteResult
│   │   └── api_test.rs                # HTTP client for API testing
│   │
│   ├── helpers/                       # Test utilities
│   │   ├── mod.rs                     # Helper exports
│   │   ├── common_utils.rs            # ID generation, data utilities
│   │   ├── jwt_manager.rs             # JWT token generation
│   │   └── setup_manager.rs           # Test setup/teardown
│   │
│   └── tests/                         # Entity-specific tests
│       ├── mod.rs                     # Test exports
│       ├── crud_test_base.rs          # Generic CRUD test framework
│       ├── user_api_test.rs           # User CRUD tests
│       ├── role_api_test.rs           # Role CRUD tests
│       ├── permission_api_test.rs     # Permission CRUD tests
│       ├── session_api_test.rs        # Session CRUD tests
│       ├── user_role_api_test.rs      # UserRole (RBAC) tests
│       ├── user_permission_api_test.rs# UserPermission tests
│       ├── role_permission_api_test.rs# RolePermission tests
│       ├── audit_log_api_test.rs      # AuditLog tests
│       ├── mfa_device_api_test.rs     # MFA Device tests
│       ├── password_reset_token_api_test.rs
│       ├── user_settings_api_test.rs  # User preferences tests
│       └── system_settings_api_test.rs# System config tests
│
└── results/                           # Test output (gitignored)
    └── *.json                         # Test result files
```

## Test Categories

| Category | Test Suites | Description |
|----------|-------------|-------------|
| **Api** | users, roles, permissions, sessions | Core entity CRUD |
| **Rbac** | user-roles, user-permissions, role-permissions | Authorization junction tables |
| **Security** | audit-logs, mfa-devices, password-reset-tokens | Security features |
| **Settings** | user-settings, system-settings | Configuration management |

## Creating a New Test Suite

### Step 1: Create Data Generator

Create a new file in `integration/tests/` (e.g., `my_entity_api_test.rs`):

```rust
//! MyEntity API Integration Tests

use async_trait::async_trait;
use serde_json::{json, Value};

use crate::integration::framework::{Test, TestError, TestResult};
use crate::integration::helpers::CommonUtils;
use super::crud_test_base::{CrudTestConfig, GenericCrudTest, TestDataGenerator};

// ============================================================
// Data Generator - Defines test payloads for your entity
// ============================================================

pub struct MyEntityDataGenerator;

impl TestDataGenerator for MyEntityDataGenerator {
    /// Generate valid create payload
    fn generate_create_payload(&self, utils: &CommonUtils) -> Value {
        json!({
            "name": utils.generate_id("myentity"),
            "description": "Test entity",
            "status": "ACTIVE"
        })
    }

    /// Generate valid update payload (full replacement)
    fn generate_update_payload(&self, utils: &CommonUtils) -> Value {
        json!({
            "name": utils.generate_id("updated"),
            "description": "Updated entity",
            "status": "INACTIVE"
        })
    }

    /// Generate partial update payload (PATCH)
    fn generate_patch_payload(&self, _utils: &CommonUtils) -> Value {
        json!({
            "status": "ARCHIVED"
        })
    }

    /// Generate invalid payload (should trigger validation error)
    fn generate_invalid_payload(&self) -> Value {
        json!({
            "name": "",  // Empty - should fail validation
            "status": "INVALID_STATUS"
        })
    }
}
```

### Step 2: Create Test Suite Struct

```rust
// ============================================================
// Test Suite
// ============================================================

pub struct MyEntityApiTest {
    crud_test: GenericCrudTest<MyEntityDataGenerator>,
}

impl MyEntityApiTest {
    pub fn new() -> Self {
        let config = CrudTestConfig::new("/api/v1/my-entities", "MyEntity")
            .with_required_fields(vec!["name", "status"])
            .with_unique_fields(vec!["name"]);
            // Optional configuration:
            // .without_soft_delete()  // If entity uses hard delete
            // .without_bulk()         // If bulk operations not supported
            // .without_upsert()       // If upsert not supported

        Self {
            crud_test: GenericCrudTest::new(config, MyEntityDataGenerator),
        }
    }

    pub fn with_auth(mut self, token: &str) -> Self {
        self.crud_test = self.crud_test.with_auth(token);
        self
    }
}

impl Default for MyEntityApiTest {
    fn default() -> Self {
        Self::new()
    }
}
```

### Step 3: Implement Test Trait

```rust
#[async_trait]
impl Test for MyEntityApiTest {
    fn name(&self) -> &str {
        "my_entity_api_test"
    }

    async fn setup(&mut self) -> Result<(), TestError> {
        // Optional: Setup test prerequisites
        Ok(())
    }

    async fn run_tests(&mut self) -> Vec<TestResult> {
        // Run all generic CRUD tests
        let suite = self.crud_test.run_all_tests().await;
        suite.results
    }

    async fn teardown(&mut self) -> Result<(), TestError> {
        // Optional: Cleanup test data
        Ok(())
    }
}
```

### Step 4: Register in mod.rs

Add to `integration/tests/mod.rs`:

```rust
pub mod my_entity_api_test;
pub use my_entity_api_test::MyEntityApiTest;
```

### Step 5: Register in config.rs

Add to `integration/config.rs` in `TestRegistry::with_defaults()`:

```rust
registry.register(
    "sapiens.api.my_entities",
    TestRegistration {
        config: TestConfig::new_api("MyEntity", "my_entity_api_test", "MyEntity CRUD API tests"),
        factory: || Box::new(MyEntityApiTest::new()),
    },
);
```

## Generic CRUD Tests

The `GenericCrudTest` automatically runs 14 test cases:

| # | Test | HTTP Method | Endpoint | Expected |
|---|------|-------------|----------|----------|
| 1 | List | GET | `/collection` | 200 |
| 2 | Pagination | GET | `/collection?page=1&per_page=5` | 200 |
| 3 | Create | POST | `/collection` | 201/200 |
| 4 | Create Invalid | POST | `/collection` | 400/422 |
| 5 | Get By ID | GET | `/collection/:id` | 200 |
| 6 | Update (PUT) | PUT | `/collection/:id` | 200 |
| 7 | Patch | PATCH | `/collection/:id` | 200 |
| 8 | Delete | DELETE | `/collection/:id` | 200/204 |
| 9 | List Trash | GET | `/collection/trash` | 200 |
| 10 | Restore | POST | `/collection/:id/restore` | 200 |
| 11 | Get Not Found | GET | `/collection/:fake-id` | 404 |
| 12 | Bulk Create | POST | `/collection/bulk` | 201/200 |
| 13 | Upsert | POST | `/collection/upsert` | 201/200 |
| 14 | Empty Trash | DELETE | `/collection/empty` | 200/204 |

## Custom Tests

For entity-specific tests beyond CRUD, add custom methods:

```rust
impl MyEntityApiTest {
    /// Test custom business logic endpoint
    async fn test_activate_entity(&self, id: &str) -> TestResult {
        let test_name = "MyEntity - Activate";
        let start = std::time::Instant::now();

        let endpoint = format!("/api/v1/my-entities/{}/activate", id);

        match self.crud_test.api_test.post(&endpoint, &json!({}), None).await {
            Ok(response) => {
                if response.status_code == 200 {
                    TestResult::success(test_name, "Entity activated")
                        .with_duration(start.elapsed().as_secs_f64())
                } else {
                    TestResult::failure(test_name, format!("Expected 200, got {}", response.status_code))
                        .with_duration(start.elapsed().as_secs_f64())
                }
            }
            Err(e) => TestResult::failure(test_name, format!("Request failed: {}", e))
        }
    }
}
```

Then include in `run_tests()`:

```rust
async fn run_tests(&mut self) -> Vec<TestResult> {
    let mut results = self.crud_test.run_all_tests().await.results;

    // Add custom tests
    if let Some(id) = self.crud_test.created_ids().first() {
        results.push(self.test_activate_entity(id).await);
    }

    results
}
```

## Test Configuration Options

### CrudTestConfig

```rust
CrudTestConfig::new("/api/v1/entities", "Entity")
    .with_required_fields(vec!["field1", "field2"])  // Required for validation tests
    .with_unique_fields(vec!["email"])               // Unique constraint fields
    .without_soft_delete()                           // Skip trash/restore tests
    .without_bulk()                                  // Skip bulk create tests
    .without_upsert()                                // Skip upsert tests
```

### TestConfig Types

```rust
TestConfig::new_api("Entity", "module_name", "Description")
    .with_type(TestType::Api)       // Default
    .with_type(TestType::Rbac)      // For authorization tests
    .with_type(TestType::Security)  // For security feature tests
    .with_type(TestType::Settings)  // For configuration tests
    .with_expected_endpoints(11)    // Default is 11 (Backbone standard)
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `API_BASE_URL` | `http://127.0.0.1:3000` | API server URL |
| `JWT_SECRET` | `test-secret-key-for-integration-tests` | JWT signing secret |

## Test Output

Tests output results in multiple formats:

### Console Output

```
══════════════════════════════════════════════════════════
Test Suite: role_api_test
══════════════════════════════════════════════════════════
  ✓ Create Role - Success (0.009s): Role created successfully
  ✓ List Roles (0.000s): Roles listed successfully
  ✓ Create Role - Duplicate Name (0.001s): Duplicate role names are allowed
──────────────────────────────────────────────────────────
Results: 3 passed, 0 failed, 3 total (100.0%)
Duration: 0.010s
══════════════════════════════════════════════════════════
```

### JSON Output

Test results are saved to `tests/results/`:

```json
{
  "suite_name": "Role CRUD Tests",
  "results": [
    {
      "test_name": "Role - Create",
      "success": true,
      "details": "Entity created successfully",
      "duration_seconds": 0.009,
      "input": { "endpoint": "/api/v1/roles", "method": "POST", "body": {...} },
      "output": { "status_code": 201, "body": {...} }
    }
  ],
  "total_duration_seconds": 0.010,
  "started_at": "2024-01-01T00:00:00Z",
  "completed_at": "2024-01-01T00:00:00Z"
}
```

## Test Runner Commands

```bash
# Run all tests (requires --ignored flag since they need running server)
cargo test --package backbone-sapiens --test run_integration_tests -- --ignored --nocapture

# Run by test type
cargo test --package backbone-sapiens --test run_integration_tests run_api_tests_only -- --ignored --nocapture

# Run specific entity tests
cargo test --package backbone-sapiens --test run_integration_tests run_user_api_tests -- --ignored --nocapture
cargo test --package backbone-sapiens --test run_integration_tests run_role_api_tests -- --ignored --nocapture

# Run unit tests (no server required)
cargo test --package backbone-sapiens --test integration_tests

# Run framework tests
cargo test --package backbone-sapiens --test run_integration_tests framework_tests
```

## Troubleshooting

### Tests return 404

The API endpoint doesn't exist yet. Implement the endpoint in the Sapiens module.

### Tests return 500

Server-side error. Check the API server logs for details.

### Tests return 422

Validation error. Update the `generate_create_payload()` to match the API's expected schema.

### Connection refused

The API server isn't running. Start it with:
```bash
cd apps/backbone && cargo run --bin backbone
```

### Tests hang

Check if the server is responding:
```bash
curl http://127.0.0.1:3000/health
```

## Best Practices

1. **Use descriptive test names** - The test name appears in output and helps debugging
2. **Include input/output in results** - Use `.with_input()` and `.with_output()` for traceability
3. **Clean up test data** - Delete created entities in teardown to avoid pollution
4. **Use unique identifiers** - Use `utils.generate_id()` to avoid conflicts
5. **Handle expected failures** - Some tests (like validation) expect failure responses
6. **Test edge cases** - Include tests for invalid data, missing fields, duplicates

## Coverage Summary

| Entity | Status | Tests |
|--------|--------|-------|
| User | ✅ Implemented | 8 tests |
| Role | ✅ Implemented | 3 tests |
| Permission | ✅ Implemented | 14 CRUD tests |
| Session | ✅ Implemented | 14 CRUD tests |
| UserRole | ✅ Implemented | 14 CRUD tests |
| UserPermission | ✅ Implemented | 14 CRUD tests |
| RolePermission | ✅ Implemented | 14 CRUD tests |
| AuditLog | ✅ Implemented | 14 CRUD tests |
| MfaDevice | ✅ Implemented | 14 CRUD tests |
| PasswordResetToken | ✅ Implemented | 14 CRUD tests |
| UserSettings | ✅ Implemented | 14 CRUD tests |
| SystemSettings | ✅ Implemented | 14 CRUD tests |

**Total: 12 test suites, ~150 test cases**
