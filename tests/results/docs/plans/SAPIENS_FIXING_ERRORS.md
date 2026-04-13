# SAPIENS API Error Fixing Plan

## Overview

**Analysis Date:** 2025-12-11
**Test Results Location:** `libs/modules/sapiens/tests/results/`
**Critical Issues Identified:** 3 high-priority API failures

This plan addresses critical API errors discovered during E2E testing of the Sapiens module. The test results reveal fundamental issues with email validation, ID persistence, and role uniqueness that prevent basic CRUD operations from working correctly.

---

## Executive Summary

### Current Status
- **31 tests passed** - Basic framework functionality working
- **4 critical API failures** - Core functionality broken
- **Root cause:** Validation and entity persistence issues

### Business Impact
- **CRITICAL:** Users cannot be created with valid emails
- **CRITICAL:** Created users cannot be retrieved (ID lookup failure)
- **HIGH:** Duplicate roles allowed (violates business rules)
- **MEDIUM:** Missing metadata in API responses

---

## Critical Issues Analysis

### Issue #1: Email Validation Not Working (CRITICAL)

**Test Evidence:**
```json
{
  "test_name": "Create User - Invalid Email Format",
  "success": false,
  "details": "Expected HTTP 400, got HTTP 201",
  "input": {
    "email": "invalid-email-format",
    "username": "testuser_invalid_email"
  },
  "output": {
    "status_code": 201,
    "body": {
      "id": "01JA8K6J5K2M8N3P9Q7R6S4T5",
      "email": "invalid-email-format"
    }
  }
}
```

**Problem:** API accepts invalid email addresses (`invalid-email-format`, `test@`, `@domain.com`) instead of returning HTTP 400.

**Root Cause:** Email validation is not implemented or not enforced at the API layer.

**Files to Investigate:**
- `libs/modules/sapiens/src/application/services/user_services.rs`
- `libs/modules/sapiens/src/presentation/http/handlers/user_handlers.rs`
- Proto validation rules in `libs/modules/sapiens/proto/domain/entity/user.proto`

---

### Issue #2: ID Persistence & Lookup Failure (CRITICAL)

**Test Evidence:**
```json
{
  "test_name": "Get User by ID",
  "success": false,
  "details": "Expected HTTP 200, got HTTP 404",
  "input": {
    "user_id": "01JA8K6J5K2M8N3P9Q7R6S4T5"
  },
  "output": {
    "status_code": 404,
    "body": {
      "error": "User with ID '01JA8K6J5K2M8N3P9Q7R6S4T5' not found"
    }
  }
}
```

**Problem:** Users are created successfully and returned with IDs, but subsequent GET requests with those IDs return 404.

**Root Cause:** Either:
1. ID is not properly stored in database
2. ID lookup query has incorrect field mapping
3. Transaction rollback after creation
4. ID format mismatch between creation and retrieval

**Files to Investigate:**
- `libs/modules/sapiens/src/infrastructure/persistence/postgres/user_repository.rs`
- ID generation in `libs/modules/sapiens/src/domain/entity/user.rs`
- Database schema mapping

---

### Issue #3: Role Name Uniqueness Not Enforced (HIGH)

**Test Evidence:**
```json
{
  "test_name": "Create Role - Duplicate Name",
  "success": false,
  "details": "Expected HTTP 409, got HTTP 201",
  "input": {
    "name": "admin",
    "description": "Second admin role"
  },
  "output": {
    "status_code": 201,
    "body": {
      "id": "01JA8K6J5K2M8N3P9Q7R6S4T5",
      "name": "admin"
    }
  }
}
```

**Problem:** Multiple roles with same name `admin` allowed, violating uniqueness constraint.

**Root Cause:** Missing unique constraint on role name field or validation not implemented.

**Files to Investigate:**
- Database migration for roles table
- Role creation validation
- Unique constraint enforcement

---

## Implementation Plan

### Phase 1: Emergency Fixes (Priority: CRITICAL)
**Timeline:** 1-2 days

#### 1.1 Fix Email Validation
**Objective:** Reject invalid email formats with HTTP 400

**Implementation Steps:**

1. **Check Proto Validation Rules:**
   ```protobuf
   // libs/modules/sapiens/proto/domain/entity/user.proto
   message User {
     string email = 1 [
       (buf.validate.field).string.email = true,
       (buf.validate.field).required = true
     ];
   }
   ```

2. **Implement Service Layer Validation:**
   ```rust
   // libs/modules/sapiens/src/application/services/user_services.rs
   impl UserServices {
       pub async fn create_user(&self, command: CreateUserCommand) -> Result<User> {
           // Add email validation
           if !is_valid_email(&command.email) {
               return Err(Error::ValidationError("Invalid email format".to_string()));
           }
           // ... rest of creation logic
       }
   }

   fn is_valid_email(email: &str) -> bool {
       // Use proper email validation regex
       regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
           .unwrap()
           .is_match(email)
   }
   ```

3. **Update HTTP Handler:**
   ```rust
   // libs/modules/sapiens/src/presentation/http/handlers/user_handlers.rs
   pub async fn create_user(
       State(app_state): State<AppState>,
       Json(request): Json<CreateUserRequest>,
   ) -> Result<Json<CreateUserResponse>, AppError> {
       let command = CreateUserRequest::try_into(request)?;

       match app_state.user_service.create_user(command).await {
           Ok(user) => Ok(Json(CreateUserResponse::from(user))),
           Err(Error::ValidationError(msg)) => {
               Err(AppError::ValidationError(msg))
           }
           Err(e) => Err(e.into()),
       }
   }
   ```

**Success Criteria:**
- [ ] `invalid-email-format` returns HTTP 400
- [ ] `test@` returns HTTP 400
- [ ] `@domain.com` returns HTTP 400
- [ ] `valid@example.com` returns HTTP 201

#### 1.2 Fix ID Persistence Issue
**Objective:** Ensure created users can be retrieved by ID

**Implementation Steps:**

1. **Verify ID Generation:**
   ```rust
   // libs/modules/sapiens/src/domain/entity/user.rs
   impl User {
       pub fn new(email: String, username: String) -> Self {
           Self {
               id: Uuid::new_v4().to_string(),  // Ensure consistent generation
               email,
               username,
               metadata: Metadata::new(),
           }
       }
   }
   ```

2. **Check Repository Implementation:**
   ```rust
   // libs/modules/sapiens/src/infrastructure/persistence/postgres/user_repository.rs
   impl UserRepository for PostgresUserRepository {
       async fn save(&self, user: &User) -> Result<()> {
           let query = r#"
               INSERT INTO users (id, email, username, metadata)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (id) DO UPDATE SET
                   email = EXCLUDED.email,
                   username = EXCLUDED.username,
                   metadata = EXCLUDED.metadata
           "#;

           sqlx::query(query)
               .bind(&user.id)
               .bind(&user.email)
               .bind(&user.username)
               .bind(&user.metadata)
               .execute(&self.pool)
               .await?;

           Ok(())
       }

       async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
           let query = r#"
               SELECT id, email, username, metadata
               FROM users
               WHERE id = $1 AND metadata->>'deleted_at' IS NULL
           "#;

           let row = sqlx::query(query)
               .bind(id)
               .fetch_optional(&self.pool)
               .await?;

           // Map row to User entity
           match row {
               Some(row) => {
                   let user = User {
                       id: row.get("id"),
                       email: row.get("email"),
                       username: row.get("username"),
                       metadata: row.get("metadata"),
                   };
                   Ok(Some(user))
               }
               None => Ok(None)
           }
       }
   }
   ```

3. **Add Transaction Logging:**
   ```rust
   // Add logging to verify transactions
   tracing::info!("Creating user with ID: {}", user.id);
   tracing::info!("Saved user to database");
   ```

**Success Criteria:**
- [ ] Created user returns consistent ID
- [ ] GET by ID returns HTTP 200 with correct user
- [ ] Database contains user record after creation

#### 1.3 Fix Role Name Uniqueness
**Objective:** Enforce unique role names

**Implementation Steps:**

1. **Add Database Constraint:**
   ```sql
   -- Migration: Add unique constraint to roles table
   ALTER TABLE roles ADD CONSTRAINT roles_name_unique UNIQUE (name);
   ```

2. **Implement Validation:**
   ```rust
   // libs/modules/sapiens/src/application/services/role_services.rs
   impl RoleServices {
       pub async fn create_role(&self, command: CreateRoleCommand) -> Result<Role> {
           // Check for existing role
           if let Some(_) = self.role_repository.find_by_name(&command.name).await? {
               return Err(Error::Conflict("Role name already exists".to_string()));
           }

           // Create role
           let role = Role::new(command.name, command.description);
           self.role_repository.save(&role).await?;
           Ok(role)
       }
   }
   ```

**Success Criteria:**
- [ ] First `admin` role returns HTTP 201
- [ ] Second `admin` role returns HTTP 409
- [ ] Unique name roles work normally

---

### Phase 2: Data Quality & Response Improvements (Priority: HIGH)
**Timeline:** 2-3 days

#### 2.1 Add Missing Metadata Fields
**Objective:** Include `created_at`, `updated_at`, `version` in all responses

**Implementation:**
```rust
// libs/modules/sapiens/src/presentation/http/handlers/user_handlers.rs
impl From<User> for GetUserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            created_at: user.metadata.created_at,
            updated_at: user.metadata.updated_at,
            version: user.metadata.version,
        }
    }
}
```

#### 2.2 Implement Proper Error Responses
**Objective:** Consistent error format across all endpoints

**Implementation:**
```rust
// Standardized error response
{
    "error": {
        "code": "VALIDATION_ERROR",
        "message": "Invalid email format",
        "details": {
            "field": "email",
            "value": "invalid-email"
        }
    },
    "timestamp": "2025-12-11T10:30:00Z",
    "request_id": "req_123456789"
}
```

---

### Phase 3: Comprehensive Testing (Priority: MEDIUM)
**Timeline:** 1-2 days

#### 3.1 Add Validation Tests
```rust
#[tokio::test]
async fn test_create_user_invalid_email_rejected() {
    let test_cases = vec![
        ("invalid-email-format", "plain text"),
        ("test@", "missing domain"),
        ("@domain.com", "missing local part"),
        ("test@.com", "invalid domain start"),
        ("test@domain", "no TLD"),
    ];

    for (invalid_email, description) in test_cases {
        let response = client
            .post("/api/v1/users")
            .json(&json!({
                "email": invalid_email,
                "username": format!("test_{}", uuid::Uuid::new_v4())
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 400,
                  "Should reject {} ({})", invalid_email, description);
    }
}
```

#### 3.2 Add ID Persistence Tests
```rust
#[tokio::test]
async fn test_user_id_persistence_flow() {
    // Create user
    let create_response = client.post("/api/v1/users")
        .json(&valid_user_request())
        .send()
        .await
        .unwrap();

    assert_eq!(create_response.status(), 201);

    let created_user: GetUserResponse = create_response.json().await.unwrap();
    let user_id = created_user.id;

    // Verify user exists with exact ID
    let get_response = client
        .get(&format!("/api/v1/users/{}", user_id))
        .send()
        .await
        .unwrap();

    assert_eq!(get_response.status(), 200);

    let retrieved_user: GetUserResponse = get_response.json().await.unwrap();
    assert_eq!(retrieved_user.id, user_id);
    assert_eq!(retrieved_user.email, created_user.email);
}
```

---

## Development Environment Setup

### Prerequisites
1. **Database Setup:**
   ```bash
   # Reset test database
   psql -h localhost -U root -d monorepodb -c "TRUNCATE TABLE users, roles CASCADE;"
   ```

2. **Test Environment:**
   ```bash
   # Set test environment
   export DATABASE_URL="postgresql://root:password@localhost:5432/monorepodb"
   export API_BASE_URL="http://127.0.0.1:3003"
   ```

3. **Run Tests:**
   ```bash
   # Run specific failing tests
   cargo test --test run_integration_tests create_user_invalid_email -- --exact --nocapture

   # Run all API tests
   cargo test --test run_integration_tests run_api_tests_only -- --nocapture
   ```

### Debugging Tools
1. **Database Query Inspection:**
   ```sql
   -- Check if users are actually saved
   SELECT id, email, username, metadata->>'created_at' as created_at
   FROM users
   ORDER BY created_at DESC
   LIMIT 5;

   -- Check role names
   SELECT name, COUNT(*) as count
   FROM roles
   GROUP BY name
   HAVING COUNT(*) > 1;
   ```

2. **API Testing:**
   ```bash
   # Test email validation
   curl -X POST http://127.0.0.1:3003/api/v1/users \
     -H "Content-Type: application/json" \
     -d '{"email": "invalid-email", "username": "test"}'

   # Test ID persistence
   USER_ID=$(curl -s -X POST http://127.0.0.1:3003/api/v1/users \
     -H "Content-Type: application/json" \
     -d '{"email": "test@example.com", "username": "test"}' | jq -r '.id')

   curl -X GET http://127.0.0.1:3003/api/v1/users/$USER_ID
   ```

---

## Risk Assessment & Mitigation

### High-Risk Changes
1. **Database Schema Changes:**
   - **Risk:** Breaking existing data
   - **Mitigation:** Create backup, test on staging first

2. **Validation Logic Changes:**
   - **Risk:** Breaking existing integrations
   - **Mitigation:** Version API changes, maintain backward compatibility

### Rollback Plan
1. **Database Rollback:**
   ```sql
   -- Remove unique constraint if issues arise
   ALTER TABLE roles DROP CONSTRAINT IF EXISTS roles_name_unique;
   ```

2. **Code Rollback:**
   ```bash
   # Quick revert to working commit
   git revert HEAD~1
   cargo build --release
   ```

---

## Success Metrics

### Phase 1 Success Criteria
- [ ] Email validation rejects all invalid formats (HTTP 400)
- [ ] Email validation accepts all valid formats (HTTP 201)
- [ ] Created users can be retrieved by ID (HTTP 200)
- [ ] Duplicate role names return HTTP 409
- [ ] All existing passing tests continue to pass

### Phase 2 Success Criteria
- [ ] All responses include metadata fields
- [ ] Error responses follow consistent format
- [ ] Response times under 200ms for single resource operations

### Phase 3 Success Criteria
- [ ] 100% test coverage for validation logic
- [ ] Integration tests pass consistently
- [ ] Performance benchmarks meet targets

---

## Timeline Summary

| Phase | Duration | Target Completion |
|-------|----------|-------------------|
| Phase 1: Emergency Fixes | 1-2 days | 2025-12-13 |
| Phase 2: Quality Improvements | 2-3 days | 2025-12-15 |
| Phase 3: Comprehensive Testing | 1-2 days | 2025-12-17 |
| **Total** | **4-7 days** | **2025-12-17** |

---

## Next Steps

1. **Immediate (Today):** Start with email validation fix
2. **Tomorrow:** Fix ID persistence issue
3. **Day 3:** Implement role uniqueness constraint
4. **Day 4-5:** Add comprehensive test coverage
5. **Day 6-7:** Performance testing and documentation

---

## Files to Monitor

### Critical Files
- `libs/modules/sapiens/src/application/services/user_services.rs`
- `libs/modules/sapiens/src/infrastructure/persistence/postgres/user_repository.rs`
- `libs/modules/sapiens/src/presentation/http/handlers/user_handlers.rs`
- Database migrations in `libs/modules/sapiens/migrations/`

### Test Files
- `libs/modules/sapiens/tests/results/` - Monitor for improvements
- `libs/modules/sapiens/tests/run_integration_tests.rs` - Add new test cases

---

## Notes

- **Testing First:** Always write failing tests before fixing issues
- **Incremental Changes:** Fix one issue at a time, validate with tests
- **Documentation:** Update API documentation as fixes are implemented
- **Communication:** Notify stakeholders of progress and any breaking changes

---

*This plan should be executed immediately due to the critical nature of the API failures. All changes should be tested thoroughly before production deployment.*
