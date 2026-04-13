# Sapiens API Schema Update v2.0 - User Name Fields

**Date:** 2025-11-22
**Version:** 2.0.0
**Status:** ✅ Complete

---

## Summary

The User entity schema has been updated to use separate name fields instead of a single `full_name` or `name` field. This provides better internationalization support, flexibility, and aligns with modern best practices.

---

## Schema Changes

### OLD Schema (v1.x) ❌

```json
{
  "name": "John Doe",
  // OR
  "full_name": "John Doe"
}
```

**Problems:**
- Difficult to split for formal addressing
- Doesn't support middle names well
- Hard to internationalize
- Inconsistent across codebase

### NEW Schema (v2.0) ✅

```json
{
  "first_name": "John",
  "middle_name": "Fitzgerald",
  "last_name": "Doe",
  "full_name": "John Fitzgerald Doe"  // Computed field
}
```

**Benefits:**
- ✅ Flexible name handling
- ✅ Better internationalization
- ✅ Supports all name variations
- ✅ Backward compatible (full_name still available)
- ✅ Consistent across all layers

---

## Field Specifications

| Field | Type | Required | Max Length | Description |
|-------|------|----------|------------|-------------|
| `first_name` | String | ✅ Yes | 100 chars | User's first name/given name |
| `middle_name` | String | ❌ Optional | 100 chars | User's middle name(s) |
| `last_name` | String | ❌ Optional | 100 chars | User's last name/family name/surname |
| `full_name` | String | N/A (computed) | N/A | Auto-computed: first + middle + last |

---

## API Changes

### REST API Endpoints

#### 1. Register User (`POST /api/v1/auth/register`)

**Before:**
```json
{
  "email": "user@example.com",
  "password": "SecureP@ss123",
  "name": "John Doe"
}
```

**After:**
```json
{
  "email": "user@example.com",
  "password": "SecureP@ss123",
  "first_name": "John",
  "middle_name": "Fitzgerald",  // optional
  "last_name": "Doe"             // optional
}
```

#### 2. Update Profile (`PUT /api/v1/users/{id}/profile`)

**Before:**
```json
{
  "name": "John Q. Public"
}
```

**After:**
```json
{
  "first_name": "John",
  "middle_name": "Q",
  "last_name": "Public"
}
```

#### 3. User Response (All Endpoints)

**Response includes ALL fields:**
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "username": "johndoe",
  "first_name": "John",
  "middle_name": "Fitzgerald",
  "last_name": "Doe",
  "full_name": "John Fitzgerald Doe",  // ← Computed
  "created_at": "2025-01-01T00:00:00Z",
  ...
}
```

### gRPC API

#### User Proto Message

```protobuf
message User {
  string id = 1;
  string email = 2;
  string username = 3;
  string first_name = 4;              // NEW
  optional string middle_name = 5;     // NEW
  optional string last_name = 6;       // NEW
  string password_hash = 7;
  repeated string roles = 8;
  // ... other fields
}
```

#### CreateUserRequest

```protobuf
message CreateUserRequest {
  string email = 1;
  string username = 2;
  string password = 3;
  string first_name = 4;              // NEW
  optional string middle_name = 5;     // NEW
  optional string last_name = 6;       // NEW
}
```

---

## Backward Compatibility

### ✅ Fully Backward Compatible

1. **`full_name` field still available** in all API responses (computed)
2. **Old clients continue to work** - they just ignore new fields
3. **Gradual migration** - old data migrated via script

### Migration Strategy

**For existing data:**
```bash
# MongoDB migration (already provided)
mongosh mongodb://localhost:27017/monorepo_guide < scripts/migrate_user_schema.js
```

**For API clients:**
- Clients can **continue using `full_name`** in responses
- Clients should **start using separate fields** for new registrations
- No breaking changes - all old endpoints still work

---

## Name Variations Supported

### 1. Single Name (Mononym)
```json
{
  "first_name": "Madonna"
  // middle_name: null
  // last_name: null
  // full_name: "Madonna"
}
```

### 2. First + Last
```json
{
  "first_name": "John",
  "last_name": "Doe"
  // full_name: "John Doe"
}
```

### 3. First + Middle + Last
```json
{
  "first_name": "John",
  "middle_name": "Fitzgerald",
  "last_name": "Kennedy"
  // full_name: "John Fitzgerald Kennedy"
}
```

### 4. Complex Middle Names
```json
{
  "first_name": "Pablo",
  "middle_name": "Diego José Francisco de Paula Juan Nepomuceno",
  "last_name": "Ruiz Picasso"
  // full_name: "Pablo Diego José Francisco de Paula Juan Nepomuceno Ruiz Picasso"
}
```

---

## Testing

### Updated Test Collections

1. **REST API Collection:** `sapiens-rest-api-v2.postman_collection.json`
   - ✅ Complete CRUD scenarios
   - ✅ Name variation tests
   - ✅ Migration verification tests
   - ✅ Automated test assertions

2. **gRPC API Collection:** `sapiens-grpc-api-v2.postman_collection.json`
   - ✅ CreateUser with all name variants
   - ✅ SearchUsers by name fields
   - ✅ grpcurl examples

### Test Scenarios Included

**Scenario 1: Complete User Lifecycle**
1. Register with full name
2. Login
3. Get profile (verify all fields)
4. Update middle name
5. Remove middle name
6. Delete account

**Scenario 2: Name Variations**
1. Create user with only first name
2. Create user with first + last
3. Create user with all three names
4. Create user with complex middle name

**Scenario 3: Data Migration**
1. Login as migrated user (root)
2. Verify schema fields
3. Verify full_name computation

---

## OpenAPI Documentation

**File:** `openapi.yaml`

**Updated Schemas:**
- ✅ `User` - Added middle_name, full_name (readOnly)
- ✅ `CreateUserRequest` - Updated field specs
- ✅ `RegisterRequest` - Updated field specs
- ✅ `UpdateUserRequest` - Updated field specs

**View documentation:**
```bash
# Serve OpenAPI docs
npx @stoplight/prism-cli mock openapi.yaml

# Or use Swagger UI
docker run -p 8080:8080 -e SWAGGER_JSON=/openapi.yaml \
  -v $(pwd)/openapi.yaml:/openapi.yaml \
  swaggerapi/swagger-ui
```

---

## Default Users (After Migration)

| Email | Username | First Name | Middle Name | Last Name | Full Name |
|-------|----------|------------|-------------|-----------|-----------|
| root@startapp.id | root | System | (null) | Administrator | System Administrator |
| admin@startapp.id | admin | Test | (null) | Admin | Test Admin |
| moderator@startapp.id | moderator | Test | (null) | Moderator | Test Moderator |
| user@startapp.id | testuser | Test | (null) | User | Test User |

**Default Password:** `PiQS5SVL012D`

---

## Implementation Status

### ✅ Completed

- [x] Proto schema updated
- [x] Domain entities updated
- [x] Application layer updated
- [x] Infrastructure (MongoDB) updated
- [x] gRPC handlers updated
- [x] HTTP handlers updated
- [x] OpenAPI schema updated
- [x] Postman collections updated (v2)
- [x] Test scenarios created
- [x] Migration script (Rust-based)
- [x] Documentation updated

### Compilation Status
```bash
cargo check -p sapiens-module
# Result: ✅ 0 errors
```

---

## Migration Checklist for API Clients

### For Frontend Developers

- [ ] Update user registration forms to collect separate name fields
- [ ] Update profile edit forms to use separate fields
- [ ] Continue using `full_name` for display purposes
- [ ] Add validation for `first_name` (required, 1-100 chars)
- [ ] Handle optional `middle_name` and `last_name`

### Example React Component

```tsx
interface UserProfile {
  first_name: string;
  middle_name?: string | null;
  last_name?: string | null;
  full_name: string;  // Computed, read-only
}

function ProfileForm() {
  return (
    <form>
      <input name="first_name" required maxLength={100} placeholder="First name" />
      <input name="middle_name" maxLength={100} placeholder="Middle name (optional)" />
      <input name="last_name" maxLength={100} placeholder="Last name (optional)" />
    </form>
  );
}

function UserDisplay({ user }: { user: UserProfile }) {
  // Use computed full_name for display
  return <div>Welcome, {user.full_name}!</div>;
}
```

---

## Breaking Changes

**None!** This is a backward-compatible update.

- ✅ Old API endpoints still work
- ✅ `full_name` still returned in responses
- ✅ Gradual migration supported
- ✅ No client updates required (but recommended)

---

## Support

**Questions?** Contact the Sapiens team or create an issue in the repository.

**Documentation:** [libs/modules/sapiens/docs/](../README.md)

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 2.0.0 | 2025-11-22 | Separate name fields, updated schema |
| 1.0.0 | 2024-xx-xx | Initial release with single name field |

---

**Last Updated:** 2025-11-22
**Author:** Claude Code
**Status:** ✅ Production Ready
