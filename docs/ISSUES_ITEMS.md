# Sapiens Module - Issues & Lessons Learned

This document captures the key issues encountered during the Sapiens module development and the lessons learned to improve future development workflows.

> **Note:** These issues informed the design of the enhanced Backbone Schema system.
> See [Schema Architecture Documentation](/docs/schema/ARCHITECTURE.md) for how the schema
> strategy prevents these issues through code generation.

---

## 1. Database Schema Mismatches

### Problem
Rust enum values didn't match PostgreSQL enum values defined in migrations.

### Examples
```rust
// Rust had:
pub enum AnalyticsEventType {
    UserLogin, UserLogout, UserRegistration, PasswordChange...
}

// Database had:
CREATE TYPE analytics_event_type AS ENUM (
    'page_view', 'click', 'form_submit', 'login', 'logout'...
);
```

### Affected Files
- `analytics_event_type.rs` - Complete mismatch of enum variants
- `notification_type.rs` - Different variant names
- `workflow_type.rs` - Domain-focused vs generic values
- `workflow_status.rs` - `Running` vs `Active`
- `o_auth_provider_type.rs` - Missing `Twitter`, `Linkedin`, `Discord`
- `notification_channel.rs` - Missing `Webhook`

### Root Cause
- Schema generator and entity generator not synchronized
- No validation between migration files and Rust enum definitions
- Manual creation of entities without checking database schema

### Prevention
- [ ] Add schema validation tool that compares Rust enums with PostgreSQL enums
- [ ] Generate Rust enums directly from database schema
- [ ] Add CI check that validates enum consistency
- [ ] Create single source of truth for enum definitions

---

## 2. SQLx Type Annotations

### Problem
Rust enums using `#[sqlx(type_name = "text")]` instead of actual PostgreSQL enum type names.

### Example
```rust
// Wrong:
#[sqlx(type_name = "text")]
pub enum AnalyticsEventType { ... }

// Correct:
#[sqlx(type_name = "analytics_event_type", rename_all = "snake_case")]
pub enum AnalyticsEventType { ... }
```

### Affected Files
All enum files in `domain/entity/`:
- `analytics_event_type.rs`
- `notification_type.rs`
- `notification_channel.rs`
- `notification_priority.rs`
- `o_auth_provider_type.rs`
- `workflow_type.rs`
- `workflow_status.rs`

### Root Cause
- Code generator not aware of PostgreSQL custom enum types
- Default to `text` type for string-like enums

### Prevention
- [ ] Update schema generator to detect PostgreSQL enum types
- [ ] Generate correct `sqlx(type_name)` annotations automatically
- [ ] Add compile-time check for sqlx type annotations

---

## 3. Column Name Mismatches (Serde/SQLx Rename)

### Problem
Rust field names differ from database column names, causing insert/select failures.

### Example
```rust
// Database column: "type"
// Rust field: "notification_type"

// Solution:
#[serde(rename = "type")]  // For JSON serialization (ORM uses JSON)
#[sqlx(rename = "type")]   // For SQLx queries
pub notification_type: NotificationType,
```

### Affected Files
- `notification.rs` - `notification_type` vs `type` column

### Root Cause
- ORM uses `serde_json` to serialize entity before INSERT
- Field name in JSON must match database column name
- `#[sqlx(rename)]` only affects SQLx direct queries, not JSON-based ORM

### Prevention
- [ ] Use consistent naming: field name = column name
- [ ] If rename needed, always add BOTH `#[serde(rename)]` AND `#[sqlx(rename)]`
- [ ] Document this pattern in coding guidelines
- [ ] Add linter rule to detect mismatched column names

---

## 4. Metadata Check Constraints

### Problem
Database has CHECK constraints requiring specific JSON fields in `metadata` column.

### Example
```sql
CHECK (metadata ? 'created_at' AND metadata ? 'updated_at')
```

### Affected Tables
- `analytics_events`
- `oauth_providers`
- `notifications`
- `workflows`
- (Likely all tables with `metadata` JSONB column)

### Root Cause
- Test data didn't include required `updated_at` field
- No documentation of metadata requirements
- Constraint defined in migration but not enforced in application layer

### Prevention
- [ ] Document metadata field requirements per entity
- [ ] Add application-level validation for metadata fields
- [ ] Create helper function that ensures required metadata fields
- [ ] Generate test data builders that include all required fields

---

## 5. Repository Partial Update Type Casting

### Problem
`partial_update` method binds parameters as text, but database expects specific types (UUID, timestamp, enum).

### Errors
```
operator does not exist: uuid = text
column "read_at" is of type timestamp with time zone but expression is of type text
column "status" is of type workflow_status but expression is of type text
```

### Solution
```rust
// UUID casting for WHERE clause
WHERE id = ${}::uuid

// Timestamp casting
if timestamp_fields.contains(field_name) {
    format!("{} = ${}::timestamptz", field_name, param_idx)
}

// Enum casting
if let Some(enum_type) = enum_casts.get(field_name) {
    format!("{} = ${}::{}", field_name, param_idx, enum_type)
}
```

### Affected Files
All 38 repository files in `infrastructure/persistence/`

### Root Cause
- Dynamic query building loses type information
- JSON values bound as strings by default
- No type metadata available at runtime

### Prevention
- [ ] Create centralized partial_update implementation with type registry
- [ ] Define field type metadata per entity (timestamp fields, enum fields, uuid fields)
- [ ] Generate partial_update with correct casts from schema
- [ ] Consider using sqlx compile-time checked queries where possible

---

## 6. Route Registration Missing

### Problem
Routes created but not merged into module's router.

### Example
```rust
// workflow_handler.rs creates routes
pub fn create_workflow_routes(...) -> Router

// But lib.rs didn't include:
.merge(create_workflow_routes(...))
```

### Affected Routes
- `workflows` - Routes defined but not registered

### Root Cause
- Manual addition of new entities without updating module registration
- No automated check for orphaned route definitions

### Prevention
- [ ] Add test that verifies all defined routes are registered
- [ ] Generate route registration automatically from handler definitions
- [ ] Create route inventory that must match registered routes

---

## 7. Test Data Completeness

### Problem
Integration tests sending partial data to PUT endpoints that require complete DTOs.

### Example
```rust
// Wrong - partial update to PUT endpoint
let update_request = json!({
    "title": "Updated Title"
});

// Correct - complete DTO
let update_request = UpdateWorkflowRequest {
    workflow_type: "sequential".to_string(),
    status: "active".to_string(),
    title: "Updated Title".to_string(),
    // ... all required fields
};
```

### Affected Test Files
- `analytics_event_api_test.rs`
- `notification_api_test.rs`
- `oauth_provider_api_test.rs`
- `workflow_api_test.rs`

### Root Cause
- PUT endpoints require full entity replacement
- Tests assumed PATCH-like behavior for PUT
- No validation that test DTOs match handler DTOs

### Prevention
- [ ] Use PATCH for partial updates, PUT for full replacement
- [ ] Generate test DTOs from handler DTOs to ensure field consistency
- [ ] Add test helpers that create complete update requests from existing entities
- [ ] Document API design: PUT = full replace, PATCH = partial update

---

## 8. Trait Bound Conflicts

### Problem
Repository interfaces defined as traits in domain layer, but infrastructure implementations don't implement them.

### Example
```rust
// Domain layer expects:
trait MFADeviceRepository {
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<MFADevice>>;
}

// Infrastructure provides:
impl MFADeviceRepository {
    pub async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<MFADevice>>;
}
// Note: Different argument type (Uuid vs &str)
```

### Root Cause
- Domain traits and infrastructure implementations evolved separately
- No compile-time enforcement that implementations match trait signatures
- Method signature changes not propagated

### Prevention
- [ ] Make infrastructure types explicitly implement domain traits
- [ ] Add trait implementation tests
- [ ] Use `impl Trait for Type` syntax to catch mismatches at compile time

---

## 9. Dependency Injection Patterns

### Problem
Services expect `Box<dyn Trait>` but receive `Arc<dyn Trait>`, or vice versa.

### Root Cause
- Inconsistent DI patterns across codebase
- Some services use `Box`, others use `Arc`
- No standard for when to use which

### Prevention
- [ ] Standardize on single pattern (recommend `Arc<dyn Trait>` for shared ownership)
- [ ] Document DI patterns in coding guidelines
- [ ] Create type aliases for common patterns

---

## 10. Method Signature Mismatches

### Problem
Methods called with wrong number or type of arguments.

### Examples
```rust
// Expected: find_by_id(&str)
// Called:   find_by_id(&Uuid)

// Expected: new(pool, repo1, repo2, repo3)
// Called:   new(pool, repo1, repo2)
```

### Root Cause
- Constructor signatures changed without updating callers
- No IDE/compiler feedback when using dynamic dispatch
- Traits defined separately from implementations

### Prevention
- [ ] Keep trait definitions and implementations in sync
- [ ] Use concrete types during development, abstract later
- [ ] Add integration tests that exercise full dependency chains

---

## 11. Missing Entity Methods

### Problem
Entity structs missing common methods expected by services.

### Examples
```rust
// Expected: entity.created_at()
// Actual: entity.metadata.get("created_at")

// Expected: entity.id()
// Actual: entity.id (direct field access)
```

### Root Cause
- Inconsistent access patterns (methods vs fields)
- Some entities use accessors, others use public fields
- No standard entity interface

### Prevention
- [ ] Define standard entity trait with common methods
- [ ] Generate accessor methods for all entities
- [ ] Standardize on either accessor methods or public fields

---

## 12. Random Generation Errors

### Problem
`sample_iter` method issues when generating random data for tests/seeds.

### Root Cause
- Using deprecated or incorrect random generation APIs
- Version mismatches in rand crate

### Prevention
- [ ] Standardize on specific rand crate version
- [ ] Create test data factories with known-good patterns
- [ ] Document random generation patterns

---

## 13. Struct Initialization Issues (E0063, E0560)

### Problem
Missing required fields or extra unknown fields when initializing structs.

### Error Codes
- `E0063` - missing field in struct initialization
- `E0560` - unknown field in struct initialization

### Root Cause
- Struct definitions changed but callers not updated
- Copy-paste errors when adding new entity types
- Generated code out of sync with manual code

### Prevention
- [ ] Use builder patterns for complex structs
- [ ] Add Default implementations where appropriate
- [ ] Keep struct definitions stable, add new fields as Option<T>

---

## Summary: Key Success Factors

1. **Root Cause Approach**: Fix underlying issues rather than workarounds
2. **Systematic Pattern Identification**: Target frequent error patterns first
3. **Database-First Verification**: Always check actual database schema
4. **Complete Data Testing**: Ensure test data matches all constraints
5. **Type Casting Awareness**: PostgreSQL requires explicit type casts

---

## Action Items Priority

### High Priority (Prevents Future Errors)
- [ ] Schema validation tool (enum consistency)
- [ ] Metadata field validation helpers
- [ ] Standardized partial_update with type registry
- [ ] Route registration verification test

### Medium Priority (Improves Developer Experience)
- [ ] Generate test DTOs from handler DTOs
- [ ] Standardize DI patterns (Arc vs Box)
- [ ] Document API design conventions

### Low Priority (Nice to Have)
- [ ] Linter rules for naming conventions
- [ ] Entity accessor method standardization
- [ ] Compile-time query checking expansion

---

## Quick Reference: Common Fixes

### Enum Mismatch
```rust
// Check migration: CREATE TYPE my_enum AS ENUM ('value1', 'value2')
#[sqlx(type_name = "my_enum", rename_all = "snake_case")]
pub enum MyEnum { Value1, Value2 }
```

### Column Rename
```rust
#[serde(rename = "db_column")]
#[sqlx(rename = "db_column")]
pub rust_field: Type,
```

### Partial Update Casting
```rust
// UUID: WHERE id = $N::uuid
// Timestamp: field = $N::timestamptz
// Enum: field = $N::enum_type_name
```

### Metadata Requirements
```rust
metadata: json!({
    "created_at": now.to_rfc3339(),
    "updated_at": now.to_rfc3339(),
    // other fields...
})
```

---

## Biggest Lesson: The Integration Gap

The **single biggest issue** was the gap between:
1. **Schema definitions** (migrations)
2. **Code generation** (entities, repositories)
3. **Runtime behavior** (tests, actual usage)

Each layer was developed somewhat independently, leading to mismatches that only surfaced at runtime. The solution is to create tighter coupling between these layers through:

- **Schema-driven development**: Generate code from schema, not the other way around
- **Compile-time validation**: Catch mismatches before runtime
- **Integration tests as contracts**: Tests that verify the full stack works together

This is why 111 integration tests are valuable - they catch issues that unit tests miss.
