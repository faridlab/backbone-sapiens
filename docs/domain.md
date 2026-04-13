# Sapiens User Management System - Technical Domain Documentation

**Version**: 2.0
**Date**: 2025-01-19
**Author**: StartApp Engineering Team
**Purpose**: Authoritative technical reference for implementing Sapiens UMS domain layer

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Domain-Driven Design Overview](#2-domain-driven-design-overview)
3. [Entities (Aggregate Roots)](#3-entities-aggregate-roots)
4. [Value Objects](#4-value-objects)
5. [Domain Events](#5-domain-events)
6. [Use Cases (CQRS)](#6-use-cases-cqrs)
7. [Repositories](#7-repositories)
8. [Domain Services](#8-domain-services)
9. [Specifications](#9-specifications)
10. [API Endpoints](#10-api-endpoints)
11. [Database Schema](#11-database-schema)
12. [Proto File Mappings](#12-proto-file-mappings)

---

## 1. Introduction

### 1.1 Purpose

This document provides **complete technical specifications** for the Sapiens User Management System domain layer. It serves as:

- **Proto file design reference** for `libs/domain/entity/sapiens/`
- **Implementation guide** for repositories, use cases, and services
- **Migration blueprint** for MongoDB and PostgreSQL databases
- **API contract** for both Backbone generic CRUD and domain-specific endpoints

### 1.2 Architecture Principles

**Domain-Driven Design (DDD)**:
- Clean separation between domain, application, infrastructure, and presentation layers
- Domain logic lives in `libs/domain/`
- Schema-first approach for all domain definitions
- CQRS for read/write separation

**Two-Layer API Architecture**:
- **Layer 1**: Backbone Generic CRUD (132 auto-generated endpoints)
- **Layer 2**: Domain-Specific Use Cases (custom business logic)

**Database Strategy**:
- **Primary**: MongoDB (document-oriented, flexible schema)
- **Secondary**: PostgreSQL (relational, complex queries)
- Dual migration support for both databases

---

## 2. Domain-Driven Design Overview

### 2.1 Bounded Context

**Sapiens UMS Bounded Context** encompasses:
- User identity and lifecycle management
- Authentication and authorization (RBAC + Hybrid RBAC)
- Audit logging and compliance
- User preferences and settings
- Session management
- Password and MFA management

### 2.2 Ubiquitous Language

| Term | Definition |
|------|------------|
| **User** | An individual with an account in the system |
| **Role** | A named collection of permissions (e.g., admin, editor) |
| **Permission** | A granular action allowed on a resource (e.g., read:users) |
| **Hybrid RBAC** | Role-Based Access Control + Direct User Permissions |
| **Effective Permissions** | Union of role-based permissions and direct user permissions |
| **Soft Delete** | Marking entity as deleted without physical removal (deleted_at timestamp) |
| **Aggregate Root** | Entry point for accessing aggregate entities |
| **Value Object** | Immutable object defined by its attributes (e.g., Email, Password) |
| **Domain Event** | Something that happened in the domain (e.g., UserCreated) |

### 2.3 Aggregates

**Primary Aggregates**:

1. **User Aggregate**
   - Root: User
   - Contains: UserSettings (embedded)
   - References: Roles (via UserRoles), Permissions (via UserPermissions)

2. **Role Aggregate**
   - Root: Role
   - References: Permissions (via RolePermissions)

3. **Session Aggregate**
   - Root: Session
   - References: User

4. **AuditLog Aggregate**
   - Root: AuditLog (append-only, immutable)

---

## 3. Entities (Aggregate Roots)

### 3.1 User Entity

**Proto Location**: `libs/domain/entity/sapiens/user.proto`

**Description**: Core aggregate root for user identity and profile.

**Attributes**:

```protobuf
message User {
  string id = 1;                          // UUID v4
  string username = 2;                    // Unique, 3-50 chars, alphanumeric+_-
  string email = 3;                       // Unique, valid email, lowercase
  string password_hash = 4;               // Argon2id hash, never exposed in API
  string first_name = 5;                  // Max 100 chars
  string last_name = 6;                   // Max 100 chars
  UserStatus status = 7;                  // Enum: active, inactive, suspended
  google.protobuf.Timestamp created_at = 8;
  google.protobuf.Timestamp updated_at = 9;
  google.protobuf.Timestamp last_login = 10;  // Nullable
  string phone_number = 11;               // Nullable, E.164 format
  string profile_picture_url = 12;        // Nullable, max 500 chars
  google.protobuf.Timestamp deleted_at = 13;  // Nullable, soft delete

  // Embedded value objects
  UserSettings settings = 14;             // One-to-one

  // Business logic fields
  int32 failed_login_attempts = 15;       // Counter for account lockout
  google.protobuf.Timestamp locked_until = 16;  // Nullable, account unlock time
  bool email_verified = 17;               // Email verification status
  bool mfa_enabled = 18;                  // MFA enrollment status
}

enum UserStatus {
  USER_STATUS_UNSPECIFIED = 0;
  USER_STATUS_ACTIVE = 1;
  USER_STATUS_INACTIVE = 2;
  USER_STATUS_SUSPENDED = 3;
}
```

**Business Rules**:
- Username: alphanumeric + underscore/hyphen, case-insensitive uniqueness
- Email: Must be valid RFC 5322 format, normalized to lowercase
- Password: Min 12 chars, complexity requirements (uppercase, lowercase, digit, special)
- Status: 'suspended' for temporary bans, 'inactive' for soft deletes
- Account lockout: 5 failed login attempts → lock for 15 minutes
- Email verification: Required before full account activation

**Invariants**:
- Email and username must be unique
- Password hash must never be null or empty
- created_at is immutable
- updated_at automatically updated on any change
- deleted_at is only set on soft delete, never updated

**MongoDB Collection**: `users`
**PostgreSQL Table**: `users`

---

### 3.2 Role Entity

**Proto Location**: `libs/domain/entity/sapiens/role.proto`

**Description**: Named collection of permissions for RBAC.

**Attributes**:

```protobuf
message Role {
  string id = 1;                          // UUID v4
  string name = 2;                        // Unique, max 50 chars, uppercase
  string description = 3;                 // Nullable, max 1000 chars
  bool is_default = 4;                    // Auto-assigned to new users
  google.protobuf.Timestamp created_at = 5;
  google.protobuf.Timestamp updated_at = 6;
  google.protobuf.Timestamp deleted_at = 7;  // Nullable, soft delete
}
```

**Business Rules**:
- Name: Uppercase convention (e.g., ADMIN, EDITOR, USER)
- Uniqueness: Name must be unique across active roles
- Default role: Only one role can have is_default=true
- Deletion: Cannot delete role if assigned to users (check via UserRoles)

**Invariants**:
- Name must not be empty
- created_at is immutable
- Only one role with is_default=true globally

**MongoDB Collection**: `roles`
**PostgreSQL Table**: `roles`

---

### 3.3 Permission Entity

**Proto Location**: `libs/domain/entity/sapiens/permission.proto`

**Description**: Granular action definition for fine-grained access control.

**Attributes**:

```protobuf
message Permission {
  string id = 1;                          // UUID v4 or action:resource format
  string name = 2;                        // Unique, format: action:resource
  string description = 3;                 // Nullable, max 500 chars
  string resource = 4;                    // Nullable, max 50 chars (e.g., users, posts)
  PermissionAction action = 5;            // Enum: read, write, delete, admin
  google.protobuf.Timestamp created_at = 6;
  google.protobuf.Timestamp updated_at = 7;
  google.protobuf.Timestamp deleted_at = 8;  // Nullable, soft delete
}

enum PermissionAction {
  PERMISSION_ACTION_UNSPECIFIED = 0;
  PERMISSION_ACTION_READ = 1;
  PERMISSION_ACTION_WRITE = 2;
  PERMISSION_ACTION_CREATE = 3;
  PERMISSION_ACTION_UPDATE = 4;
  PERMISSION_ACTION_DELETE = 5;
  PERMISSION_ACTION_ADMIN = 6;
}
```

**Business Rules**:
- Name format: `<action>:<resource>` (e.g., read:users, write:posts, admin:system)
- Uniqueness: Name must be unique
- Resource: Category grouping (e.g., users, roles, permissions, posts)
- Action: Standardized CRUD + admin

**Invariants**:
- Name must match pattern `^[a-z_]+:[a-z_]+$`
- Resource and action derived from name for consistency

**MongoDB Collection**: `permissions`
**PostgreSQL Table**: `permissions`

---

### 3.4 UserPermission Entity (Hybrid RBAC)

**Proto Location**: `libs/domain/entity/sapiens/user_permission.proto`

**Description**: Direct permission grants to users (exception-based, temporary, resource-scoped).

**Attributes**:

```protobuf
message UserPermission {
  string id = 1;                          // UUID v4
  string user_id = 2;                     // Foreign key to User
  string permission_id = 3;               // Foreign key to Permission
  google.protobuf.Timestamp granted_at = 4;
  string granted_by = 5;                  // User ID of admin who granted
  google.protobuf.Timestamp expires_at = 6;  // Nullable, for temporary access
  string reason = 7;                      // Mandatory, 10-500 chars, audit trail
  string resource_id = 8;                 // Nullable, scope to specific resource
  string resource_type = 9;               // Nullable, type of resource
  bool is_active = 10;                    // Revocation flag
  google.protobuf.Timestamp revoked_at = 11;  // Nullable
  string revoked_by = 12;                 // Nullable, User ID
  string revoked_reason = 13;             // Nullable, max 500 chars
}
```

**Business Rules**:
- Reason: Mandatory, immutable, 10-500 characters (audit compliance)
- Expiry: Optional, must be in future if provided
- Resource scoping: Optional, for fine-grained control (e.g., access to project-123 only)
- Revocation: Soft delete via is_active=false
- Duplicate prevention: No two active grants for same user-permission pair

**Invariants**:
- reason is immutable after creation
- expires_at must be > granted_at
- revoked_* fields must be null if is_active=true

**MongoDB Collection**: `user_permissions`
**PostgreSQL Table**: `user_permissions`

---

### 3.5 UserRole Entity (Junction)

**Proto Location**: `libs/domain/entity/sapiens/user_role.proto`

**Description**: Many-to-many relationship between Users and Roles.

**Attributes**:

```protobuf
message UserRole {
  string user_id = 1;                     // Foreign key to User
  string role_id = 2;                     // Foreign key to Role
  google.protobuf.Timestamp assigned_at = 3;
  string assigned_by = 4;                 // Nullable, User ID of admin
}
```

**Business Rules**:
- Composite primary key: (user_id, role_id)
- Cascade delete: Removed when user or role is deleted
- Audit trail: assigned_at and assigned_by for tracking

**Invariants**:
- user_id and role_id must reference existing entities
- No duplicate assignments

**MongoDB Collection**: `user_roles`
**PostgreSQL Table**: `user_roles`

---

### 3.6 RolePermission Entity (Junction)

**Proto Location**: `libs/domain/entity/sapiens/role_permission.proto`

**Description**: Many-to-many relationship between Roles and Permissions.

**Attributes**:

```protobuf
message RolePermission {
  string role_id = 1;                     // Foreign key to Role
  string permission_id = 2;               // Foreign key to Permission
  google.protobuf.Timestamp granted_at = 3;
}
```

**Business Rules**:
- Composite primary key: (role_id, permission_id)
- Cascade delete: Removed when role or permission is deleted

**Invariants**:
- role_id and permission_id must reference existing entities
- No duplicate grants

**MongoDB Collection**: `role_permissions`
**PostgreSQL Table**: `role_permissions`

---

### 3.7 UserSettings Entity

**Proto Location**: `libs/domain/entity/sapiens/user_settings.proto`

**Description**: User preferences and configuration (one-to-one with User).

**Attributes**:

```protobuf
message UserSettings {
  string user_id = 1;                     // Primary key, foreign key to User
  string theme = 2;                       // Enum: light, dark, system
  string language = 3;                    // ISO 639-1 code (e.g., en, es)
  string timezone = 4;                    // IANA timezone (e.g., America/New_York)
  bool notifications_enabled = 5;
  bool email_notifications = 6;
  bool sms_notifications = 7;
  google.protobuf.Timestamp updated_at = 8;

  // Additional settings (flexible JSON in implementation)
  map<string, string> custom_settings = 9;
}
```

**Business Rules**:
- One-to-one relationship with User
- Theme: light, dark, or system (follows OS preference)
- Language: ISO 639-1 two-letter codes
- Timezone: IANA timezone database format
- Custom settings: Key-value pairs for extensibility

**Invariants**:
- user_id must reference existing User
- Only one settings record per user

**MongoDB**: Embedded in User document OR separate collection `user_settings`
**PostgreSQL Table**: `user_settings`

---

### 3.8 AuditLog Entity

**Proto Location**: `libs/domain/entity/sapiens/audit_log.proto`

**Description**: Immutable record of all user events for compliance and security.

**Attributes**:

```protobuf
message AuditLog {
  string id = 1;                          // UUID v4
  string user_id = 2;                     // Nullable, for system events
  string action = 3;                      // Event type (e.g., login_success, user_created)
  google.protobuf.Struct details = 4;     // Nullable, event-specific JSON
  string ip_address = 5;                  // Nullable, client IP (IPv4 or IPv6)
  string user_agent = 6;                  // Nullable, browser/client info
  google.protobuf.Timestamp timestamp = 7;
  string session_id = 8;                  // Nullable, associated session
  string resource_type = 9;               // Nullable, type of resource affected
  string resource_id = 10;                // Nullable, ID of affected resource
  AuditLogSeverity severity = 11;         // Enum: info, warning, error, critical
  bool archived = 12;                     // Archive status for log management
  google.protobuf.Timestamp archived_at = 13;  // Nullable
  string archive_location = 14;           // Nullable, S3 path
}

enum AuditLogSeverity {
  AUDIT_LOG_SEVERITY_UNSPECIFIED = 0;
  AUDIT_LOG_SEVERITY_INFO = 1;
  AUDIT_LOG_SEVERITY_WARNING = 2;
  AUDIT_LOG_SEVERITY_ERROR = 3;
  AUDIT_LOG_SEVERITY_CRITICAL = 4;
}
```

**Business Rules**:
- Append-only: No updates or deletes allowed (immutable)
- Retention: Keep in MongoDB for 90 days (hot), archive to S3 for 7 years (cold)
- Severity: Info (normal), Warning (potential issue), Error (failure), Critical (security)
- Archival: Automated daily job exports logs older than 90 days

**Invariants**:
- timestamp is immutable
- action must not be empty
- No updates allowed after creation
- archived and archive_location managed by archival job only

**MongoDB Collection**: `audit_logs`
**PostgreSQL Table**: `audit_logs`

---

### 3.9 Session Entity

**Proto Location**: `libs/domain/entity/sapiens/session.proto`

**Description**: Active user session with JWT token management.

**Attributes**:

```protobuf
message Session {
  string id = 1;                          // UUID v4, also JWT jti claim
  string user_id = 2;                     // Foreign key to User
  string token_hash = 3;                  // SHA-256 hash of JWT (for revocation)
  google.protobuf.Timestamp created_at = 4;
  google.protobuf.Timestamp expires_at = 5;
  google.protobuf.Timestamp last_activity = 6;  // Nullable, last API call
  string ip_address = 7;                  // Client IP at session creation
  string user_agent = 8;                  // Browser/client info
  string device_type = 9;                 // Enum: web, mobile, tablet, desktop
  bool is_active = 10;                    // Revocation flag
  google.protobuf.Timestamp revoked_at = 11;  // Nullable
}
```

**Business Rules**:
- JWT expiry: 24 hours default (configurable)
- Refresh token: Optional, 30 days expiry
- Revocation: Set is_active=false to invalidate token
- Concurrent sessions: Allow multiple active sessions per user
- Activity tracking: Update last_activity on each API call

**Invariants**:
- token_hash must be unique
- expires_at must be > created_at
- revoked_at must be null if is_active=true

**MongoDB Collection**: `sessions`
**PostgreSQL Table**: `sessions`

---

### 3.10 PasswordResetToken Entity

**Proto Location**: `libs/domain/entity/sapiens/password_reset_token.proto`

**Description**: One-time token for password reset flow.

**Attributes**:

```protobuf
message PasswordResetToken {
  string id = 1;                          // UUID v4
  string user_id = 2;                     // Foreign key to User
  string token_hash = 3;                  // SHA-256 hash of token
  google.protobuf.Timestamp created_at = 4;
  google.protobuf.Timestamp expires_at = 5;  // 1 hour expiry
  bool is_used = 6;                       // One-time use flag
  google.protobuf.Timestamp used_at = 7;  // Nullable
  string ip_address = 8;                  // IP that requested reset
}
```

**Business Rules**:
- Expiry: 1 hour from creation
- One-time use: is_used=true after password reset
- Token format: URL-safe random string (32+ chars)
- Invalidation: All previous tokens for user invalidated on creation

**Invariants**:
- token_hash must be unique
- expires_at = created_at + 1 hour
- used_at must be null if is_used=false

**MongoDB Collection**: `password_reset_tokens`
**PostgreSQL Table**: `password_reset_tokens`

---

### 3.11 MFADevice Entity

**Proto Location**: `libs/domain/entity/sapiens/mfa_device.proto`

**Description**: Multi-factor authentication device configuration.

**Attributes**:

```protobuf
message MFADevice {
  string id = 1;                          // UUID v4
  string user_id = 2;                     // Foreign key to User
  MFADeviceType device_type = 3;          // Enum: totp, sms, backup_codes
  string device_name = 4;                 // User-friendly name (e.g., "Google Authenticator")
  string secret = 5;                      // Encrypted TOTP secret or phone number
  repeated string backup_codes = 6;       // Encrypted backup codes
  google.protobuf.Timestamp created_at = 7;
  google.protobuf.Timestamp last_used = 8;  // Nullable
  bool is_active = 9;                     // Device enabled/disabled
  google.protobuf.Timestamp verified_at = 10;  // Nullable, enrollment verification
}

enum MFADeviceType {
  MFA_DEVICE_TYPE_UNSPECIFIED = 0;
  MFA_DEVICE_TYPE_TOTP = 1;               // Time-based OTP (Google Authenticator)
  MFA_DEVICE_TYPE_SMS = 2;                // SMS-based OTP
  MFA_DEVICE_TYPE_BACKUP_CODES = 3;       // One-time backup codes
}
```

**Business Rules**:
- TOTP: 30-second window, 6-digit codes
- SMS: Phone number in E.164 format
- Backup codes: 10 codes generated, each usable once
- Encryption: secret and backup_codes encrypted at rest
- Enrollment: Requires verification (scan QR code, verify first code)

**Invariants**:
- secret must be encrypted before storage
- verified_at must not be null if is_active=true
- One TOTP device active per user at a time

**MongoDB Collection**: `mfa_devices`
**PostgreSQL Table**: `mfa_devices`

---

### 3.12 SystemSettings Entity

**Proto Location**: `libs/domain/entity/sapiens/system_settings.proto`

**Description**: Global system configuration (feature flags, audit settings).

**Attributes**:

```protobuf
message SystemSettings {
  string id = 1;                          // Setting key (e.g., "audit_logging_enabled")
  string key = 2;                         // Same as ID for indexing
  google.protobuf.Value value = 3;        // Flexible value (bool, string, number, object)
  string description = 4;                 // Nullable, max 1000 chars
  google.protobuf.Timestamp updated_at = 5;
  string updated_by = 6;                  // Nullable, User ID
}
```

**Business Rules**:
- Key-value store for system-wide settings
- Examples:
  - `audit_logging_enabled`: true/false
  - `audit_excluded_actions`: {read:users: true, health_check: true}
  - `password_min_length`: 12
- Admin-only access
- Cached in memory for performance

**Invariants**:
- key must be unique
- value type validated per key (schema validation)

**MongoDB Collection**: `system_settings`
**PostgreSQL Table**: `system_settings`

---

## 4. Value Objects

Value objects are immutable objects defined by their attributes, not identity.

### 4.1 Email

**Proto Location**: `libs/domain/value_object/common/email.proto`

```protobuf
message Email {
  string value = 1;                       // RFC 5322 compliant email
  bool verified = 2;                      // Verification status
  string domain = 3;                      // Extracted domain (e.g., example.com)
}
```

**Validations**:
- RFC 5322 regex validation
- Lowercase normalization
- Domain extraction for analytics

---

### 4.2 Password

**Proto Location**: `libs/domain/value_object/sapiens/password.proto`

```protobuf
message Password {
  string value = 1;                       // Plain-text password (never persisted)
}

message PasswordHash {
  string value = 1;                       // Argon2id hash
  PasswordHashParams params = 2;
}

message PasswordHashParams {
  uint32 memory_cost = 1;                 // m=19456
  uint32 time_cost = 2;                   // t=2
  uint32 parallelism = 3;                 // p=1
  uint32 output_length = 4;               // 32 bytes
}
```

**Business Rules**:
- Min 12 characters
- At least one uppercase, lowercase, digit, special char
- No common passwords (check against list)
- Argon2id hashing with params: m=19456, t=2, p=1, output=32

---

### 4.3 PhoneNumber

**Proto Location**: `libs/domain/value_object/common/phone_number.proto`

```protobuf
message PhoneNumber {
  string value = 1;                       // E.164 format (e.g., +14155552671)
  string country_code = 2;                // Extracted country code (+1)
  string national_number = 3;             // National format (415-555-2671)
}
```

**Validations**:
- E.164 format: `^\+?[1-9]\d{1,14}$`
- Country code extraction
- National number formatting

---

### 4.4 Username

**Proto Location**: `libs/domain/value_object/sapiens/username.proto`

```protobuf
message Username {
  string value = 1;                       // 3-50 chars, alphanumeric + _-
}
```

**Validations**:
- Regex: `^[a-zA-Z0-9_-]{3,50}$`
- Case-insensitive uniqueness
- No reserved words (admin, root, system)

---

## 5. Domain Events

**Proto Location**: `libs/domain/event/sapiens/`

### 5.1 User Events

```protobuf
// user_events.proto
message UserCreatedEvent {
  string user_id = 1;
  string email = 2;
  string username = 3;
  google.protobuf.Timestamp created_at = 4;
  string created_by = 5;                  // Nullable, admin or self-registration
}

message UserUpdatedEvent {
  string user_id = 1;
  repeated string updated_fields = 2;
  google.protobuf.Timestamp updated_at = 3;
  string updated_by = 4;
}

message UserDeletedEvent {
  string user_id = 1;
  UserDeletionReason reason = 2;
  google.protobuf.Timestamp deleted_at = 3;
  string deleted_by = 4;
}

enum UserDeletionReason {
  USER_DELETION_REASON_UNSPECIFIED = 0;
  USER_DELETION_REASON_SELF_REQUEST = 1;
  USER_DELETION_REASON_ADMIN_ACTION = 2;
  USER_DELETION_REASON_COMPLIANCE = 3;
  USER_DELETION_REASON_INACTIVITY = 4;
}

message UserPasswordChangedEvent {
  string user_id = 1;
  PasswordChangeReason reason = 2;
  google.protobuf.Timestamp changed_at = 3;
}

enum PasswordChangeReason {
  PASSWORD_CHANGE_REASON_UNSPECIFIED = 0;
  PASSWORD_CHANGE_REASON_USER_REQUEST = 1;
  PASSWORD_CHANGE_REASON_FORGOT_PASSWORD = 2;
  PASSWORD_CHANGE_REASON_ADMIN_RESET = 3;
  PASSWORD_CHANGE_REASON_POLICY_ENFORCEMENT = 4;
}

message UserEmailVerifiedEvent {
  string user_id = 1;
  string email = 2;
  google.protobuf.Timestamp verified_at = 3;
}
```

---

### 5.2 Authentication Events

```protobuf
// auth_events.proto
message UserLoggedInEvent {
  string user_id = 1;
  string session_id = 2;
  string ip_address = 3;
  string user_agent = 4;
  bool mfa_used = 5;
  google.protobuf.Timestamp logged_in_at = 6;
}

message UserLoggedOutEvent {
  string user_id = 1;
  string session_id = 2;
  google.protobuf.Timestamp logged_out_at = 3;
}

message LoginFailedEvent {
  string email_or_username = 1;
  LoginFailureReason reason = 2;
  string ip_address = 3;
  int32 failed_attempts = 4;
  google.protobuf.Timestamp failed_at = 5;
}

enum LoginFailureReason {
  LOGIN_FAILURE_REASON_UNSPECIFIED = 0;
  LOGIN_FAILURE_REASON_INVALID_PASSWORD = 1;
  LOGIN_FAILURE_REASON_USER_NOT_FOUND = 2;
  LOGIN_FAILURE_REASON_ACCOUNT_LOCKED = 3;
  LOGIN_FAILURE_REASON_ACCOUNT_SUSPENDED = 4;
  LOGIN_FAILURE_REASON_EMAIL_NOT_VERIFIED = 5;
  LOGIN_FAILURE_REASON_MFA_REQUIRED = 6;
  LOGIN_FAILURE_REASON_MFA_INVALID = 7;
}
```

---

### 5.3 Permission Events

```protobuf
// permission_events.proto
message RoleAssignedEvent {
  string user_id = 1;
  string role_id = 2;
  string assigned_by = 3;
  google.protobuf.Timestamp assigned_at = 4;
}

message RoleRevokedEvent {
  string user_id = 1;
  string role_id = 2;
  string revoked_by = 3;
  google.protobuf.Timestamp revoked_at = 4;
}

message DirectPermissionGrantedEvent {
  string user_id = 1;
  string permission_id = 2;
  string granted_by = 3;
  google.protobuf.Timestamp granted_at = 4;
  google.protobuf.Timestamp expires_at = 5;  // Nullable
  string reason = 6;
}

message DirectPermissionRevokedEvent {
  string grant_id = 1;
  string revoked_by = 2;
  google.protobuf.Timestamp revoked_at = 3;
  string reason = 4;
}
```

---

## 6. Use Cases (CQRS)

**Proto Location**: `libs/domain/usecase/sapiens/`

### 6.1 Commands (Write Operations)

**File**: `commands.proto`

```protobuf
// User Commands
message CreateUserCommand {
  string email = 1;
  string username = 2;
  string password = 3;
  string first_name = 4;
  string last_name = 5;
  string phone_number = 6;              // Optional
  string profile_picture_url = 7;       // Optional
}

message UpdateUserCommand {
  string user_id = 1;
  string first_name = 2;                // Optional
  string last_name = 3;                 // Optional
  string phone_number = 4;              // Optional
  string profile_picture_url = 5;       // Optional
}

message DeleteUserCommand {
  string user_id = 1;
  UserDeletionReason reason = 2;
}

message ChangePasswordCommand {
  string user_id = 1;
  string old_password = 2;
  string new_password = 3;
}

// Auth Commands
message AuthenticateUserCommand {
  string email_or_username = 1;
  string password = 2;
  string mfa_code = 3;                  // Optional
}

message RevokeSessionCommand {
  string session_id = 1;
}

message InitiatePasswordResetCommand {
  string email = 1;
}

message CompletePasswordResetCommand {
  string token = 1;
  string new_password = 2;
}

// Role Commands
message AssignRoleCommand {
  string user_id = 1;
  string role_id = 2;
  string assigned_by = 3;
}

message RevokeRoleCommand {
  string user_id = 1;
  string role_id = 2;
  string revoked_by = 3;
}

// Permission Commands
message GrantDirectPermissionCommand {
  string user_id = 1;
  string permission_id = 2;
  string granted_by = 3;
  google.protobuf.Timestamp expires_at = 4;  // Optional
  string reason = 5;
  string resource_id = 6;               // Optional
  string resource_type = 7;             // Optional
}

message RevokeDirectPermissionCommand {
  string grant_id = 1;
  string revoked_by = 2;
  string reason = 3;
}

// MFA Commands
message SetupMFACommand {
  string user_id = 1;
  MFADeviceType device_type = 2;
  string device_name = 3;
}

message VerifyMFACommand {
  string user_id = 1;
  string mfa_code = 2;
}
```

---

### 6.2 Queries (Read Operations)

**File**: `queries.proto`

```protobuf
// User Queries
message GetUserByIdQuery {
  string user_id = 1;
}

message GetUserByEmailQuery {
  string email = 1;
}

message GetUserByUsernameQuery {
  string username = 1;
}

message ListUsersQuery {
  UserStatus status = 1;                // Optional filter
  int32 page = 2;
  int32 limit = 3;
  string sort_by = 4;                   // Field name
  SortOrder sort_order = 5;
}

enum SortOrder {
  SORT_ORDER_UNSPECIFIED = 0;
  SORT_ORDER_ASC = 1;
  SORT_ORDER_DESC = 2;
}

// Permission Queries
message GetUserRolesQuery {
  string user_id = 1;
}

message GetUserEffectivePermissionsQuery {
  string user_id = 1;
}

message GetUserDirectPermissionsQuery {
  string user_id = 1;
  bool include_expired = 2;             // Optional, default false
}

// Session Queries
message GetActiveSessionsQuery {
  string user_id = 1;
}

message GetSessionByIdQuery {
  string session_id = 1;
}

// Audit Queries
message GetUserActivityQuery {
  string user_id = 1;
  google.protobuf.Timestamp start_date = 2;
  google.protobuf.Timestamp end_date = 3;
  int32 page = 4;
  int32 limit = 5;
}

message GetFailedLoginAttemptsQuery {
  string email_or_username = 1;
  google.protobuf.Duration time_window = 2;  // e.g., last 1 hour
}
```

---

## 7. Repositories

**Proto Location**: `libs/domain/repository/`

### 7.1 UserRepository

**File**: `user_repository.proto`

```protobuf
service UserRepository {
  rpc Save(SaveUserRequest) returns (SaveUserResponse);
  rpc FindById(FindUserByIdRequest) returns (FindUserByIdResponse);
  rpc FindByEmail(FindUserByEmailRequest) returns (FindUserByEmailResponse);
  rpc FindByUsername(FindUserByUsernameRequest) returns (FindUserByUsernameResponse);
  rpc List(ListUsersRequest) returns (ListUsersResponse);
  rpc Delete(DeleteUserRequest) returns (DeleteUserResponse);
  rpc Restore(RestoreUserRequest) returns (RestoreUserResponse);
  rpc UpdateLastLogin(UpdateLastLoginRequest) returns (UpdateLastLoginResponse);
}

message SaveUserRequest {
  User user = 1;
}

message SaveUserResponse {
  User user = 1;
}

message FindUserByIdRequest {
  string user_id = 1;
}

message FindUserByIdResponse {
  User user = 1;
  bool found = 2;
}

// ... (similar patterns for other methods)
```

---

### 7.2 RoleRepository

**File**: `role_repository.proto`

```protobuf
service RoleRepository {
  rpc Save(SaveRoleRequest) returns (SaveRoleResponse);
  rpc FindById(FindRoleByIdRequest) returns (FindRoleByIdResponse);
  rpc FindByName(FindRoleByNameRequest) returns (FindRoleByNameResponse);
  rpc List(ListRolesRequest) returns (ListRolesResponse);
  rpc Delete(DeleteRoleRequest) returns (DeleteRoleResponse);
  rpc AssignToUser(AssignRoleToUserRequest) returns (AssignRoleToUserResponse);
  rpc RevokeFromUser(RevokeRoleFromUserRequest) returns (RevokeRoleFromUserResponse);
  rpc GetUserRoles(GetUserRolesRequest) returns (GetUserRolesResponse);
}
```

---

### 7.3 PermissionRepository

**File**: `permission_repository.proto`

```protobuf
service PermissionRepository {
  rpc Save(SavePermissionRequest) returns (SavePermissionResponse);
  rpc FindById(FindPermissionByIdRequest) returns (FindPermissionByIdResponse);
  rpc FindByName(FindPermissionByNameRequest) returns (FindPermissionByNameResponse);
  rpc List(ListPermissionsRequest) returns (ListPermissionsResponse);
  rpc GetRolePermissions(GetRolePermissionsRequest) returns (GetRolePermissionsResponse);
  rpc GetUserEffectivePermissions(GetUserEffectivePermissionsRequest) returns (GetUserEffectivePermissionsResponse);
}
```

---

### 7.4 SessionRepository

**File**: `session_repository.proto`

```protobuf
service SessionRepository {
  rpc Create(CreateSessionRequest) returns (CreateSessionResponse);
  rpc FindById(FindSessionByIdRequest) returns (FindSessionByIdResponse);
  rpc FindByTokenHash(FindSessionByTokenHashRequest) returns (FindSessionByTokenHashResponse);
  rpc GetActiveSessions(GetActiveSessionsRequest) returns (GetActiveSessionsResponse);
  rpc Revoke(RevokeSessionRequest) returns (RevokeSessionResponse);
  rpc RevokeAllUserSessions(RevokeAllUserSessionsRequest) returns (RevokeAllUserSessionsResponse);
  rpc DeleteExpired(DeleteExpiredSessionsRequest) returns (DeleteExpiredSessionsResponse);
}
```

---

## 8. Domain Services

**Proto Location**: `libs/domain/service/`

### 8.1 PasswordService

**File**: `password_service.proto`

```protobuf
service PasswordService {
  rpc HashPassword(HashPasswordRequest) returns (HashPasswordResponse);
  rpc VerifyPassword(VerifyPasswordRequest) returns (VerifyPasswordResponse);
  rpc ValidatePasswordStrength(ValidatePasswordStrengthRequest) returns (ValidatePasswordStrengthResponse);
  rpc GenerateRandomPassword(GenerateRandomPasswordRequest) returns (GenerateRandomPasswordResponse);
}

message HashPasswordRequest {
  string password = 1;
}

message HashPasswordResponse {
  string password_hash = 1;
}

message VerifyPasswordRequest {
  string password = 1;
  string password_hash = 2;
}

message VerifyPasswordResponse {
  bool is_valid = 1;
}
```

---

### 8.2 TokenService

**File**: `token_service.proto`

```protobuf
service TokenService {
  rpc GenerateJWT(GenerateJWTRequest) returns (GenerateJWTResponse);
  rpc ValidateJWT(ValidateJWTRequest) returns (ValidateJWTResponse);
  rpc RefreshJWT(RefreshJWTRequest) returns (RefreshJWTResponse);
  rpc RevokeJWT(RevokeJWTRequest) returns (RevokeJWTResponse);
}

message GenerateJWTRequest {
  string user_id = 1;
  repeated string permissions = 2;
  google.protobuf.Duration expiry = 3;
}

message GenerateJWTResponse {
  string token = 1;
  google.protobuf.Timestamp expires_at = 2;
}
```

---

### 8.3 PermissionResolutionService

**File**: `permission_resolution_service.proto`

```protobuf
service PermissionResolutionService {
  rpc GetEffectivePermissions(GetEffectivePermissionsRequest) returns (GetEffectivePermissionsResponse);
  rpc CheckPermission(CheckPermissionRequest) returns (CheckPermissionResponse);
}

message GetEffectivePermissionsRequest {
  string user_id = 1;
}

message GetEffectivePermissionsResponse {
  repeated EffectivePermission permissions = 1;
}

message EffectivePermission {
  Permission permission = 1;
  PermissionSource source = 2;
  repeated string via_roles = 3;          // If source=ROLE
  google.protobuf.Timestamp expires_at = 4;  // If source=DIRECT
  string resource_id = 5;                 // If resource-scoped
}

enum PermissionSource {
  PERMISSION_SOURCE_UNSPECIFIED = 0;
  PERMISSION_SOURCE_ROLE = 1;
  PERMISSION_SOURCE_DIRECT = 2;
}

message CheckPermissionRequest {
  string user_id = 1;
  string permission_name = 2;
  string resource_id = 3;                 // Optional, for resource-scoped check
}

message CheckPermissionResponse {
  bool has_permission = 1;
  PermissionSource source = 2;
}
```

---

## 9. Specifications

**Proto Location**: `libs/domain/specification/specification.proto`

```protobuf
// Business rules that can be reused and composed

message UserIsActiveSpecification {
  bool require_email_verified = 1;
  bool check_account_locked = 2;
}

message UserHasRoleSpecification {
  repeated string required_roles = 1;
}

message UserHasPermissionSpecification {
  string required_permission = 1;
  string resource_id = 2;                 // Optional
}

message PasswordMeetsComplexitySpecification {
  uint32 min_length = 1;
  bool require_uppercase = 2;
  bool require_lowercase = 3;
  bool require_digit = 4;
  bool require_special_char = 5;
  repeated string disallowed_passwords = 6;  // Common password list
}

message SessionIsValidSpecification {
  bool check_expiry = 1;
  bool check_revoked = 2;
}
```

---

## 10. API Endpoints

### 10.1 Layer 1: Backbone Generic CRUD (132 Endpoints)

**All 12 entities get 11 standard endpoints**:

```
GET    /api/v1/{collection}              - List (paginated, filtered, sorted)
POST   /api/v1/{collection}              - Create
GET    /api/v1/{collection}/:id          - Get by ID
PUT    /api/v1/{collection}/:id          - Full update
PATCH  /api/v1/{collection}/:id          - Partial update
DELETE /api/v1/{collection}/:id          - Soft delete
POST   /api/v1/{collection}/bulk         - Bulk create
POST   /api/v1/{collection}/upcreate     - Upsert
GET    /api/v1/{collection}/trash        - List deleted
POST   /api/v1/{collection}/:id/restore  - Restore
DELETE /api/v1/{collection}/empty        - Empty trash (hard delete)
```

**Collections**:
1. `users`
2. `roles`
3. `permissions`
4. `user_permissions`
5. `user_roles`
6. `role_permissions`
7. `user_settings`
8. `audit_logs` (read-only: GET list, GET :id, GET trash only)
9. `sessions`
10. `password_reset_tokens`
11. `mfa_devices`
12. `system_settings`

---

### 10.2 Layer 2: Domain-Specific Endpoints

#### **Authentication & Authorization**

```
POST   /api/v1/auth/register              - User registration (FR-001)
POST   /api/v1/auth/login                 - User login (FR-002)
POST   /api/v1/auth/logout                - User logout (FR-002)
POST   /api/v1/auth/refresh-token         - Refresh JWT (FR-002)
POST   /api/v1/auth/forgot-password       - Initiate password reset (FR-003)
POST   /api/v1/auth/reset-password        - Complete password reset (FR-003)
POST   /api/v1/auth/verify-email          - Verify email address
POST   /api/v1/auth/resend-verification   - Resend verification email
```

#### **MFA Management**

```
POST   /api/v1/auth/mfa/setup             - Setup MFA device (FR-004)
POST   /api/v1/auth/mfa/verify            - Verify MFA code (FR-004)
POST   /api/v1/auth/mfa/disable           - Disable MFA
GET    /api/v1/auth/mfa/devices           - List MFA devices
DELETE /api/v1/auth/mfa/devices/:id       - Remove MFA device
POST   /api/v1/auth/mfa/regenerate-backup - Regenerate backup codes
```

#### **User Profile Management**

```
GET    /api/v1/users/:id/profile          - Get user profile (FR-005)
PATCH  /api/v1/users/:id/profile          - Update user profile (FR-005)
POST   /api/v1/users/:id/change-password  - Change password
POST   /api/v1/users/:id/change-email     - Change email (requires verification)
DELETE /api/v1/users/:id/account          - Delete account (self-service)
```

#### **Role & Permission Management**

```
POST   /api/v1/users/:id/roles            - Assign role to user (FR-006)
DELETE /api/v1/users/:id/roles/:roleId    - Revoke role from user (FR-006)
GET    /api/v1/users/:id/roles            - Get user's roles
POST   /api/v1/users/:id/permissions      - Grant direct permission (FR-006A)
DELETE /api/v1/users/:id/permissions/:grantId  - Revoke direct permission (FR-006A)
GET    /api/v1/users/:id/permissions      - Get direct permissions
GET    /api/v1/users/:id/effective-permissions  - Get effective permissions (role + direct)
```

#### **Admin Role Management**

```
POST   /api/v1/roles                      - Create role
GET    /api/v1/roles/:id                  - Get role details
PUT    /api/v1/roles/:id                  - Update role
DELETE /api/v1/roles/:id                  - Delete role
POST   /api/v1/roles/:id/permissions      - Add permission to role
DELETE /api/v1/roles/:id/permissions/:permId  - Remove permission from role
GET    /api/v1/roles/:id/permissions      - Get role permissions
```

#### **Session Management**

```
GET    /api/v1/users/:id/sessions         - List active sessions
DELETE /api/v1/users/:id/sessions/:sessionId  - Revoke specific session
DELETE /api/v1/users/:id/sessions/all     - Revoke all sessions (logout all devices)
```

#### **Audit & Compliance**

```
GET    /api/v1/users/:id/activity         - Get user activity log
GET    /api/v1/audit/events               - Search audit events (admin)
POST   /api/v1/audit/search-archived      - Search archived logs
GET    /api/v1/audit/settings             - Get audit settings (feature flags)
POST   /api/v1/audit/settings/enable      - Enable audit logging
POST   /api/v1/audit/settings/disable     - Disable audit logging
```

#### **Admin Endpoints**

```
GET    /api/v1/users/list           - Advanced user filtering/search
POST   /api/v1/users/:id/suspend    - Suspend user account
POST   /api/v1/users/:id/unsuspend  - Unsuspend user account
POST   /api/v1/users/:id/reset-password  - Admin password reset
GET    /api/v1/direct-permissions/list    - List all direct permission grants
GET    /api/v1/stats                - System statistics
```

---

## 11. Database Schema

### 11.1 MongoDB Collections

All collections use MongoDB 5.0+ with schema validation.

#### **11.1.1 users Collection**

```javascript
db.createCollection("users", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "username", "email", "password_hash", "first_name", "last_name", "status", "created_at", "updated_at"],
      properties: {
        _id: {
          bsonType: "string",
          pattern: "^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$"
        },
        username: {
          bsonType: "string",
          minLength: 3,
          maxLength: 50,
          pattern: "^[a-zA-Z0-9_-]+$"
        },
        email: {
          bsonType: "string",
          pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
        },
        password_hash: {
          bsonType: "string",
          minLength: 60
        },
        first_name: {
          bsonType: "string",
          maxLength: 100
        },
        last_name: {
          bsonType: "string",
          maxLength: 100
        },
        status: {
          enum: ["active", "inactive", "suspended"]
        },
        created_at: { bsonType: "date" },
        updated_at: { bsonType: "date" },
        last_login: { bsonType: ["date", "null"] },
        phone_number: {
          bsonType: ["string", "null"],
          pattern: "^\\+?[1-9]\\d{1,14}$"
        },
        profile_picture_url: {
          bsonType: ["string", "null"],
          maxLength: 500
        },
        deleted_at: { bsonType: ["date", "null"] },
        failed_login_attempts: {
          bsonType: "int",
          minimum: 0
        },
        locked_until: { bsonType: ["date", "null"] },
        email_verified: { bsonType: "bool" },
        mfa_enabled: { bsonType: "bool" }
      }
    }
  }
});

// Indexes
db.users.createIndex({ email: 1 }, { unique: true });
db.users.createIndex({ username: 1 }, { unique: true });
db.users.createIndex({ status: 1 });
db.users.createIndex({ created_at: -1 });
db.users.createIndex({ deleted_at: 1 });
```

#### **11.1.2 roles Collection**

```javascript
db.createCollection("roles", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "name", "created_at", "updated_at"],
      properties: {
        _id: { bsonType: "string" },
        name: {
          bsonType: "string",
          minLength: 1,
          maxLength: 50
        },
        description: {
          bsonType: ["string", "null"],
          maxLength: 1000
        },
        is_default: { bsonType: "bool" },
        created_at: { bsonType: "date" },
        updated_at: { bsonType: "date" },
        deleted_at: { bsonType: ["date", "null"] }
      }
    }
  }
});

// Indexes
db.roles.createIndex({ name: 1 }, { unique: true });
db.roles.createIndex({ is_default: 1 });
db.roles.createIndex({ deleted_at: 1 });
```

#### **11.1.3 permissions Collection**

```javascript
db.createCollection("permissions", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "name", "created_at", "updated_at"],
      properties: {
        _id: { bsonType: "string" },
        name: {
          bsonType: "string",
          pattern: "^[a-z_]+:[a-z_]+$"
        },
        description: {
          bsonType: ["string", "null"],
          maxLength: 500
        },
        resource: {
          bsonType: ["string", "null"],
          maxLength: 50
        },
        action: {
          enum: ["read", "write", "create", "update", "delete", "admin"]
        },
        created_at: { bsonType: "date" },
        updated_at: { bsonType: "date" },
        deleted_at: { bsonType: ["date", "null"] }
      }
    }
  }
});

// Indexes
db.permissions.createIndex({ name: 1 }, { unique: true });
db.permissions.createIndex({ resource: 1 });
db.permissions.createIndex({ action: 1 });
```

#### **11.1.4 user_permissions Collection**

```javascript
db.createCollection("user_permissions", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "user_id", "permission_id", "granted_at", "granted_by", "reason", "is_active"],
      properties: {
        _id: { bsonType: "string" },
        user_id: { bsonType: "string" },
        permission_id: { bsonType: "string" },
        granted_at: { bsonType: "date" },
        granted_by: { bsonType: "string" },
        expires_at: { bsonType: ["date", "null"] },
        reason: {
          bsonType: "string",
          minLength: 10,
          maxLength: 500
        },
        resource_id: {
          bsonType: ["string", "null"],
          maxLength: 100
        },
        resource_type: {
          bsonType: ["string", "null"],
          maxLength: 50
        },
        is_active: { bsonType: "bool" },
        revoked_at: { bsonType: ["date", "null"] },
        revoked_by: { bsonType: ["string", "null"] },
        revoked_reason: {
          bsonType: ["string", "null"],
          maxLength: 500
        }
      }
    }
  }
});

// Indexes
db.user_permissions.createIndex({ user_id: 1, permission_id: 1 });
db.user_permissions.createIndex({ user_id: 1, is_active: 1 });
db.user_permissions.createIndex({ expires_at: 1 });
db.user_permissions.createIndex({ granted_at: -1 });
db.user_permissions.createIndex({ resource_id: 1, resource_type: 1 });
```

#### **11.1.5 user_roles Collection**

```javascript
db.createCollection("user_roles", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["user_id", "role_id", "assigned_at"],
      properties: {
        user_id: { bsonType: "string" },
        role_id: { bsonType: "string" },
        assigned_at: { bsonType: "date" },
        assigned_by: { bsonType: ["string", "null"] }
      }
    }
  }
});

// Indexes
db.user_roles.createIndex({ user_id: 1, role_id: 1 }, { unique: true });
db.user_roles.createIndex({ role_id: 1 });
db.user_roles.createIndex({ assigned_at: -1 });
```

#### **11.1.6 role_permissions Collection**

```javascript
db.createCollection("role_permissions", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["role_id", "permission_id", "granted_at"],
      properties: {
        role_id: { bsonType: "string" },
        permission_id: { bsonType: "string" },
        granted_at: { bsonType: "date" }
      }
    }
  }
});

// Indexes
db.role_permissions.createIndex({ role_id: 1, permission_id: 1 }, { unique: true });
db.role_permissions.createIndex({ permission_id: 1 });
```

#### **11.1.7 user_settings Collection**

```javascript
db.createCollection("user_settings", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["user_id", "theme", "language", "timezone", "updated_at"],
      properties: {
        user_id: { bsonType: "string" },
        theme: {
          enum: ["light", "dark", "system"]
        },
        language: {
          bsonType: "string",
          pattern: "^[a-z]{2}$"  // ISO 639-1 codes
        },
        timezone: {
          bsonType: "string",
          maxLength: 50  // IANA timezone format
        },
        notifications_enabled: { bsonType: "bool" },
        email_notifications: { bsonType: "bool" },
        sms_notifications: { bsonType: "bool" },
        updated_at: { bsonType: "date" },
        custom_settings: {
          bsonType: ["object", "null"]  // Flexible JSON for additional settings
        }
      }
    }
  }
});

// Indexes
db.user_settings.createIndex({ user_id: 1 }, { unique: true });
```

#### **11.1.8 audit_logs Collection**

```javascript
db.createCollection("audit_logs", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "action", "timestamp", "severity"],
      properties: {
        _id: { bsonType: "string" },
        user_id: { bsonType: ["string", "null"] },
        action: {
          bsonType: "string",
          minLength: 1,
          maxLength: 100
        },
        details: { bsonType: ["object", "null"] },
        ip_address: {
          bsonType: ["string", "null"],
          maxLength: 45  // IPv6 max length
        },
        user_agent: {
          bsonType: ["string", "null"],
          maxLength: 500
        },
        timestamp: { bsonType: "date" },
        session_id: { bsonType: ["string", "null"] },
        resource_type: {
          bsonType: ["string", "null"],
          maxLength: 50
        },
        resource_id: { bsonType: ["string", "null"] },
        severity: {
          enum: ["info", "warning", "error", "critical"]
        },
        archived: { bsonType: "bool" },
        archived_at: { bsonType: ["date", "null"] },
        archive_location: {
          bsonType: ["string", "null"],
          maxLength: 500  // S3 path
        }
      }
    }
  }
});

// Indexes
db.audit_logs.createIndex({ user_id: 1, timestamp: -1 });
db.audit_logs.createIndex({ action: 1, timestamp: -1 });
db.audit_logs.createIndex({ timestamp: -1 });
db.audit_logs.createIndex({ severity: 1, timestamp: -1 });
db.audit_logs.createIndex({ archived: 1, timestamp: 1 });  // For archival job
db.audit_logs.createIndex({ session_id: 1 });
db.audit_logs.createIndex({ resource_type: 1, resource_id: 1 });

// TTL Index: Auto-delete archived logs after 7 years (2555 days)
db.audit_logs.createIndex(
  { archived_at: 1 },
  { expireAfterSeconds: 220752000 }  // 7 years in seconds
);
```

#### **11.1.9 sessions Collection**

```javascript
db.createCollection("sessions", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "user_id", "token_hash", "created_at", "expires_at", "is_active"],
      properties: {
        _id: { bsonType: "string" },  // Also JWT jti claim
        user_id: { bsonType: "string" },
        token_hash: {
          bsonType: "string",
          minLength: 64,
          maxLength: 64  // SHA-256 hash
        },
        created_at: { bsonType: "date" },
        expires_at: { bsonType: "date" },
        last_activity: { bsonType: ["date", "null"] },
        ip_address: {
          bsonType: ["string", "null"],
          maxLength: 45
        },
        user_agent: {
          bsonType: ["string", "null"],
          maxLength: 500
        },
        device_type: {
          enum: ["web", "mobile", "tablet", "desktop", "unknown"]
        },
        is_active: { bsonType: "bool" },
        revoked_at: { bsonType: ["date", "null"] }
      }
    }
  }
});

// Indexes
db.sessions.createIndex({ token_hash: 1 }, { unique: true });
db.sessions.createIndex({ user_id: 1, is_active: 1 });
db.sessions.createIndex({ user_id: 1, created_at: -1 });
db.sessions.createIndex({ expires_at: 1 });

// TTL Index: Auto-delete expired inactive sessions after 30 days
db.sessions.createIndex(
  { expires_at: 1 },
  {
    expireAfterSeconds: 2592000,  // 30 days in seconds
    partialFilterExpression: { is_active: false }
  }
);
```

#### **11.1.10 password_reset_tokens Collection**

```javascript
db.createCollection("password_reset_tokens", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "user_id", "token_hash", "created_at", "expires_at", "is_used"],
      properties: {
        _id: { bsonType: "string" },
        user_id: { bsonType: "string" },
        token_hash: {
          bsonType: "string",
          minLength: 64,
          maxLength: 64  // SHA-256 hash
        },
        created_at: { bsonType: "date" },
        expires_at: { bsonType: "date" },
        is_used: { bsonType: "bool" },
        used_at: { bsonType: ["date", "null"] },
        ip_address: {
          bsonType: ["string", "null"],
          maxLength: 45
        }
      }
    }
  }
});

// Indexes
db.password_reset_tokens.createIndex({ token_hash: 1 }, { unique: true });
db.password_reset_tokens.createIndex({ user_id: 1, created_at: -1 });
db.password_reset_tokens.createIndex({ expires_at: 1 });

// TTL Index: Auto-delete expired tokens after 24 hours
db.password_reset_tokens.createIndex(
  { expires_at: 1 },
  { expireAfterSeconds: 86400 }  // 24 hours in seconds
);
```

#### **11.1.11 mfa_devices Collection**

```javascript
db.createCollection("mfa_devices", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "user_id", "device_type", "device_name", "secret", "created_at", "is_active"],
      properties: {
        _id: { bsonType: "string" },
        user_id: { bsonType: "string" },
        device_type: {
          enum: ["totp", "sms", "backup_codes"]
        },
        device_name: {
          bsonType: "string",
          minLength: 1,
          maxLength: 100
        },
        secret: {
          bsonType: "string",  // Encrypted TOTP secret or phone number
          minLength: 1
        },
        backup_codes: {
          bsonType: ["array", "null"],
          items: {
            bsonType: "string"  // Encrypted backup codes
          }
        },
        created_at: { bsonType: "date" },
        last_used: { bsonType: ["date", "null"] },
        is_active: { bsonType: "bool" },
        verified_at: { bsonType: ["date", "null"] }
      }
    }
  }
});

// Indexes
db.mfa_devices.createIndex({ user_id: 1, device_type: 1 });
db.mfa_devices.createIndex({ user_id: 1, is_active: 1 });
db.mfa_devices.createIndex({ created_at: -1 });
```

#### **11.1.12 system_settings Collection**

```javascript
db.createCollection("system_settings", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "key", "value", "updated_at"],
      properties: {
        _id: { bsonType: "string" },
        key: {
          bsonType: "string",
          minLength: 1,
          maxLength: 100
        },
        value: {
          // Flexible value type (bool, string, number, object)
        },
        description: {
          bsonType: ["string", "null"],
          maxLength: 1000
        },
        updated_at: { bsonType: "date" },
        updated_by: { bsonType: ["string", "null"] }
      }
    }
  }
});

// Indexes
db.system_settings.createIndex({ key: 1 }, { unique: true });
```

---

### 11.2 PostgreSQL Schema

Complete PostgreSQL database schema with constraints, indexes, and foreign keys.

#### **11.2.1 users Table**

```sql
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username VARCHAR(50) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  first_name VARCHAR(100) NOT NULL,
  last_name VARCHAR(100) NOT NULL,
  status VARCHAR(20) NOT NULL CHECK (status IN ('active', 'inactive', 'suspended')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  last_login TIMESTAMPTZ,
  phone_number VARCHAR(20),
  profile_picture_url VARCHAR(500),
  deleted_at TIMESTAMPTZ,
  failed_login_attempts INTEGER NOT NULL DEFAULT 0,
  locked_until TIMESTAMPTZ,
  email_verified BOOLEAN NOT NULL DEFAULT FALSE,
  mfa_enabled BOOLEAN NOT NULL DEFAULT FALSE,

  CONSTRAINT users_username_unique UNIQUE (username),
  CONSTRAINT users_email_unique UNIQUE (email),
  CONSTRAINT users_username_format CHECK (username ~ '^[a-zA-Z0-9_-]{3,50}$'),
  CONSTRAINT users_email_format CHECK (email ~ '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'),
  CONSTRAINT users_failed_attempts_positive CHECK (failed_login_attempts >= 0)
);

-- Indexes
CREATE INDEX idx_users_status ON users(status);
CREATE INDEX idx_users_created_at ON users(created_at DESC);
CREATE INDEX idx_users_deleted_at ON users(deleted_at);
CREATE INDEX idx_users_email_verified ON users(email_verified);

-- Updated_at trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_updated_at
  BEFORE UPDATE ON users
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();
```

#### **11.2.2 roles Table**

```sql
CREATE TABLE roles (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(50) NOT NULL,
  description TEXT,
  is_default BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ,

  CONSTRAINT roles_name_unique UNIQUE (name),
  CONSTRAINT roles_name_uppercase CHECK (name = UPPER(name))
);

-- Indexes
CREATE INDEX idx_roles_is_default ON roles(is_default);
CREATE INDEX idx_roles_deleted_at ON roles(deleted_at);

-- Trigger
CREATE TRIGGER update_roles_updated_at
  BEFORE UPDATE ON roles
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

-- Ensure only one default role
CREATE UNIQUE INDEX idx_roles_single_default
  ON roles(is_default)
  WHERE is_default = TRUE AND deleted_at IS NULL;
```

#### **11.2.3 permissions Table**

```sql
CREATE TABLE permissions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(100) NOT NULL,
  description VARCHAR(500),
  resource VARCHAR(50),
  action VARCHAR(20) NOT NULL CHECK (action IN ('read', 'write', 'create', 'update', 'delete', 'admin')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ,

  CONSTRAINT permissions_name_unique UNIQUE (name),
  CONSTRAINT permissions_name_format CHECK (name ~ '^[a-z_]+:[a-z_]+$')
);

-- Indexes
CREATE INDEX idx_permissions_resource ON permissions(resource);
CREATE INDEX idx_permissions_action ON permissions(action);
CREATE INDEX idx_permissions_deleted_at ON permissions(deleted_at);

-- Trigger
CREATE TRIGGER update_permissions_updated_at
  BEFORE UPDATE ON permissions
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();
```

#### **11.2.4 user_permissions Table**

```sql
CREATE TABLE user_permissions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL,
  permission_id UUID NOT NULL,
  granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  granted_by UUID NOT NULL,
  expires_at TIMESTAMPTZ,
  reason VARCHAR(500) NOT NULL,
  resource_id VARCHAR(100),
  resource_type VARCHAR(50),
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  revoked_at TIMESTAMPTZ,
  revoked_by UUID,
  revoked_reason VARCHAR(500),

  CONSTRAINT user_permissions_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT user_permissions_permission_fk FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE,
  CONSTRAINT user_permissions_granted_by_fk FOREIGN KEY (granted_by) REFERENCES users(id) ON DELETE SET NULL,
  CONSTRAINT user_permissions_revoked_by_fk FOREIGN KEY (revoked_by) REFERENCES users(id) ON DELETE SET NULL,
  CONSTRAINT user_permissions_reason_length CHECK (LENGTH(reason) >= 10),
  CONSTRAINT user_permissions_expires_future CHECK (expires_at IS NULL OR expires_at > granted_at),
  CONSTRAINT user_permissions_revoked_check CHECK (
    (is_active = TRUE AND revoked_at IS NULL AND revoked_by IS NULL AND revoked_reason IS NULL) OR
    (is_active = FALSE AND revoked_at IS NOT NULL)
  )
);

-- Indexes
CREATE INDEX idx_user_permissions_user_id ON user_permissions(user_id, is_active);
CREATE INDEX idx_user_permissions_permission_id ON user_permissions(permission_id);
CREATE INDEX idx_user_permissions_expires_at ON user_permissions(expires_at);
CREATE INDEX idx_user_permissions_granted_at ON user_permissions(granted_at DESC);
CREATE INDEX idx_user_permissions_resource ON user_permissions(resource_id, resource_type);

-- Prevent duplicate active grants
CREATE UNIQUE INDEX idx_user_permissions_unique_active
  ON user_permissions(user_id, permission_id)
  WHERE is_active = TRUE;
```

#### **11.2.5 user_roles Table**

```sql
CREATE TABLE user_roles (
  user_id UUID NOT NULL,
  role_id UUID NOT NULL,
  assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  assigned_by UUID,

  PRIMARY KEY (user_id, role_id),
  CONSTRAINT user_roles_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT user_roles_role_fk FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
  CONSTRAINT user_roles_assigned_by_fk FOREIGN KEY (assigned_by) REFERENCES users(id) ON DELETE SET NULL
);

-- Indexes
CREATE INDEX idx_user_roles_role_id ON user_roles(role_id);
CREATE INDEX idx_user_roles_assigned_at ON user_roles(assigned_at DESC);
```

#### **11.2.6 role_permissions Table**

```sql
CREATE TABLE role_permissions (
  role_id UUID NOT NULL,
  permission_id UUID NOT NULL,
  granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  PRIMARY KEY (role_id, permission_id),
  CONSTRAINT role_permissions_role_fk FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
  CONSTRAINT role_permissions_permission_fk FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
);

-- Indexes
CREATE INDEX idx_role_permissions_permission_id ON role_permissions(permission_id);
```

#### **11.2.7 user_settings Table**

```sql
CREATE TABLE user_settings (
  user_id UUID PRIMARY KEY,
  theme VARCHAR(20) NOT NULL DEFAULT 'system' CHECK (theme IN ('light', 'dark', 'system')),
  language VARCHAR(2) NOT NULL DEFAULT 'en' CHECK (language ~ '^[a-z]{2}$'),
  timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
  notifications_enabled BOOLEAN NOT NULL DEFAULT TRUE,
  email_notifications BOOLEAN NOT NULL DEFAULT TRUE,
  sms_notifications BOOLEAN NOT NULL DEFAULT FALSE,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  custom_settings JSONB,

  CONSTRAINT user_settings_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Trigger
CREATE TRIGGER update_user_settings_updated_at
  BEFORE UPDATE ON user_settings
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();
```

#### **11.2.8 audit_logs Table**

```sql
CREATE TABLE audit_logs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID,
  action VARCHAR(100) NOT NULL,
  details JSONB,
  ip_address INET,
  user_agent VARCHAR(500),
  timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  session_id UUID,
  resource_type VARCHAR(50),
  resource_id VARCHAR(100),
  severity VARCHAR(20) NOT NULL CHECK (severity IN ('info', 'warning', 'error', 'critical')),
  archived BOOLEAN NOT NULL DEFAULT FALSE,
  archived_at TIMESTAMPTZ,
  archive_location VARCHAR(500),

  CONSTRAINT audit_logs_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- Indexes
CREATE INDEX idx_audit_logs_user_timestamp ON audit_logs(user_id, timestamp DESC);
CREATE INDEX idx_audit_logs_action_timestamp ON audit_logs(action, timestamp DESC);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(timestamp DESC);
CREATE INDEX idx_audit_logs_severity_timestamp ON audit_logs(severity, timestamp DESC);
CREATE INDEX idx_audit_logs_archived ON audit_logs(archived, timestamp);
CREATE INDEX idx_audit_logs_session_id ON audit_logs(session_id);
CREATE INDEX idx_audit_logs_resource ON audit_logs(resource_type, resource_id);

-- Partitioning (recommended for large audit logs)
-- CREATE TABLE audit_logs_y2025m01 PARTITION OF audit_logs
--   FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');
```

#### **11.2.9 sessions Table**

```sql
CREATE TABLE sessions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL,
  token_hash CHAR(64) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  expires_at TIMESTAMPTZ NOT NULL,
  last_activity TIMESTAMPTZ,
  ip_address INET,
  user_agent VARCHAR(500),
  device_type VARCHAR(20) NOT NULL DEFAULT 'unknown' CHECK (device_type IN ('web', 'mobile', 'tablet', 'desktop', 'unknown')),
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  revoked_at TIMESTAMPTZ,

  CONSTRAINT sessions_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT sessions_token_hash_unique UNIQUE (token_hash),
  CONSTRAINT sessions_expires_future CHECK (expires_at > created_at),
  CONSTRAINT sessions_revoked_check CHECK (
    (is_active = TRUE AND revoked_at IS NULL) OR
    (is_active = FALSE AND revoked_at IS NOT NULL)
  )
);

-- Indexes
CREATE INDEX idx_sessions_user_active ON sessions(user_id, is_active);
CREATE INDEX idx_sessions_user_created ON sessions(user_id, created_at DESC);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
```

#### **11.2.10 password_reset_tokens Table**

```sql
CREATE TABLE password_reset_tokens (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL,
  token_hash CHAR(64) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  expires_at TIMESTAMPTZ NOT NULL,
  is_used BOOLEAN NOT NULL DEFAULT FALSE,
  used_at TIMESTAMPTZ,
  ip_address INET,

  CONSTRAINT password_reset_tokens_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT password_reset_tokens_token_hash_unique UNIQUE (token_hash),
  CONSTRAINT password_reset_tokens_expires_1hour CHECK (expires_at = created_at + INTERVAL '1 hour'),
  CONSTRAINT password_reset_tokens_used_check CHECK (
    (is_used = FALSE AND used_at IS NULL) OR
    (is_used = TRUE AND used_at IS NOT NULL)
  )
);

-- Indexes
CREATE INDEX idx_password_reset_tokens_user ON password_reset_tokens(user_id, created_at DESC);
CREATE INDEX idx_password_reset_tokens_expires ON password_reset_tokens(expires_at);
```

#### **11.2.11 mfa_devices Table**

```sql
CREATE TABLE mfa_devices (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL,
  device_type VARCHAR(20) NOT NULL CHECK (device_type IN ('totp', 'sms', 'backup_codes')),
  device_name VARCHAR(100) NOT NULL,
  secret TEXT NOT NULL,  -- Encrypted
  backup_codes TEXT[],   -- Encrypted array
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  last_used TIMESTAMPTZ,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  verified_at TIMESTAMPTZ,

  CONSTRAINT mfa_devices_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT mfa_devices_verified_check CHECK (
    (is_active = FALSE) OR
    (is_active = TRUE AND verified_at IS NOT NULL)
  )
);

-- Indexes
CREATE INDEX idx_mfa_devices_user ON mfa_devices(user_id, device_type);
CREATE INDEX idx_mfa_devices_user_active ON mfa_devices(user_id, is_active);
CREATE INDEX idx_mfa_devices_created ON mfa_devices(created_at DESC);

-- Ensure only one active TOTP device per user
CREATE UNIQUE INDEX idx_mfa_devices_one_totp_per_user
  ON mfa_devices(user_id)
  WHERE device_type = 'totp' AND is_active = TRUE;
```

#### **11.2.12 system_settings Table**

```sql
CREATE TABLE system_settings (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  key VARCHAR(100) NOT NULL,
  value JSONB NOT NULL,
  description TEXT,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_by UUID,

  CONSTRAINT system_settings_key_unique UNIQUE (key),
  CONSTRAINT system_settings_updated_by_fk FOREIGN KEY (updated_by) REFERENCES users(id) ON DELETE SET NULL
);

-- Trigger
CREATE TRIGGER update_system_settings_updated_at
  BEFORE UPDATE ON system_settings
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();
```

---

## 12. Proto File Mappings

Complete mapping of domain entities to proto files in `libs/domain/`.

### 12.1 Directory Structure

```
libs/domain/
├── entity/
│   ├── common/                           # Shared entities
│   │   └── common.proto                  # Common enums and types
│   ├── core/                             # Core business entities
│   │   ├── user.proto                    # User aggregate root
│   │   ├── role.proto                    # Role entity
│   │   └── permission.proto              # Permission entity
│   └── sapiens/                          # Sapiens-specific entities
│       ├── user_permission.proto         # Hybrid RBAC direct grants
│       ├── user_role.proto               # User-role junction
│       ├── role_permission.proto         # Role-permission junction
│       ├── user_settings.proto           # User preferences
│       ├── audit_log.proto               # Audit logging
│       ├── session.proto                 # Session management
│       ├── password_reset_token.proto    # Password reset flow
│       ├── mfa_device.proto              # MFA configuration
│       └── system_settings.proto         # System configuration
│
├── value_object/
│   ├── common/
│   │   ├── email.proto                   # Email value object
│   │   ├── phone_number.proto            # Phone number value object
│   │   └── address.proto                 # Address value object (future)
│   └── sapiens/
│       ├── password.proto                # Password and hash value objects
│       └── username.proto                # Username value object
│
├── event/
│   ├── common/
│   │   └── base_event.proto              # Base event definitions
│   └── sapiens/
│       ├── user_events.proto             # User lifecycle events
│       ├── auth_events.proto             # Authentication events
│       └── permission_events.proto       # RBAC events
│
├── specification/
│   └── specification.proto               # Business rules
│
├── repository/
│   ├── user_repository.proto             # User repository interface
│   ├── role_repository.proto             # Role repository interface
│   ├── permission_repository.proto       # Permission repository interface
│   └── session_repository.proto          # Session repository interface
│
├── usecase/
│   └── sapiens/
│       ├── commands.proto                # Write operations (CQRS)
│       └── queries.proto                 # Read operations (CQRS)
│
└── service/
    ├── password_service.proto            # Password hashing/validation
    ├── token_service.proto               # JWT generation/validation
    └── permission_resolution_service.proto  # Permission resolution
```

### 12.2 Proto Generation Configuration

#### **Option 1: buf.yaml (Recommended)**

```yaml
# libs/domain/buf.yaml
version: v1
name: buf.build/startapp/sapiens-domain
breaking:
  use:
    - FILE
lint:
  use:
    - DEFAULT
```

#### **Option 2: buf.gen.yaml**

```yaml
# libs/domain/buf.gen.yaml
version: v1
managed:
  enabled: true
plugins:
  - plugin: buf.build/community/rust-prost:v0.3.0
    out: gen/rust
    opt:
      - compile_well_known_types=true
      - extern_path=.google.protobuf=::prost_types
  - plugin: buf.build/community/rust-tonic:v0.3.0
    out: gen/rust
    opt:
      - compile_well_known_types=true
```

#### **Generation Command**

```bash
# From libs/domain/ directory
buf generate

# Output: libs/domain/gen/rust/
```

### 12.3 Rust Integration

#### **Service build.rs**

```rust
// apps/sapiens/build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Point to centralized proto files
    let proto_paths = [
        "../../libs/domain/entity/core/user.proto",
        "../../libs/domain/entity/core/role.proto",
        "../../libs/domain/entity/core/permission.proto",
        "../../libs/domain/entity/sapiens/user_permission.proto",
        "../../libs/domain/entity/sapiens/user_role.proto",
        "../../libs/domain/entity/sapiens/role_permission.proto",
        "../../libs/domain/entity/sapiens/user_settings.proto",
        "../../libs/domain/entity/sapiens/audit_log.proto",
        "../../libs/domain/entity/sapiens/session.proto",
        "../../libs/domain/entity/sapiens/password_reset_token.proto",
        "../../libs/domain/entity/sapiens/mfa_device.proto",
        "../../libs/domain/entity/sapiens/system_settings.proto",
        "../../libs/domain/value_object/common/email.proto",
        "../../libs/domain/value_object/common/phone_number.proto",
        "../../libs/domain/value_object/sapiens/password.proto",
        "../../libs/domain/value_object/sapiens/username.proto",
        "../../libs/domain/event/sapiens/user_events.proto",
        "../../libs/domain/event/sapiens/auth_events.proto",
        "../../libs/domain/event/sapiens/permission_events.proto",
        "../../libs/domain/usecase/sapiens/commands.proto",
        "../../libs/domain/usecase/sapiens/queries.proto",
        "../../libs/domain/repository/user_repository.proto",
        "../../libs/domain/repository/role_repository.proto",
        "../../libs/domain/repository/permission_repository.proto",
        "../../libs/domain/repository/session_repository.proto",
        "../../libs/domain/service/password_service.proto",
        "../../libs/domain/service/token_service.proto",
        "../../libs/domain/service/permission_resolution_service.proto",
    ];

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/presentation/grpc/generated")
        .compile_protos(&proto_paths, &["../../libs/domain"])?;

    Ok(())
}
```

#### **Using Generated Types**

```rust
// apps/sapiens/src/domain/entities/user.rs
use crate::presentation::grpc::generated::entity::core::User;
use crate::presentation::grpc::generated::entity::core::UserStatus;
use crate::presentation::grpc::generated::value_object::common::Email;
use crate::presentation::grpc::generated::value_object::sapiens::PasswordHash;

impl User {
    pub fn new(username: String, email: Email, password_hash: PasswordHash) -> Self {
        User {
            id: uuid::Uuid::new_v4().to_string(),
            username,
            email: email.value,
            password_hash: password_hash.value,
            status: UserStatus::Active as i32,
            created_at: Some(prost_types::Timestamp::from(std::time::SystemTime::now())),
            // ... other fields
        }
    }
}
```

---

## 13. Implementation Checklist

### Phase 1: Proto Files Creation
- [ ] Create all 12 entity proto files in `libs/domain/entity/`
- [ ] Create all 4 value object proto files in `libs/domain/value_object/`
- [ ] Create all 3 event proto files in `libs/domain/event/`
- [ ] Create specification proto file
- [ ] Create all 4 repository proto files
- [ ] Create commands and queries proto files (CQRS)
- [ ] Create all 3 domain service proto files
- [ ] Configure buf.yaml and buf.gen.yaml
- [ ] Test proto generation with `buf generate`

### Phase 2: MongoDB Setup
- [ ] Create MongoDB initialization script with all 12 collections
- [ ] Add schema validation for each collection
- [ ] Create all indexes (unique, compound, TTL)
- [ ] Insert default data (root user, default role, base permissions)
- [ ] Test MongoDB connection and validation

### Phase 3: PostgreSQL Setup
- [ ] Create PostgreSQL migration scripts (up/down)
- [ ] Create all 12 tables with constraints
- [ ] Create all indexes and foreign keys
- [ ] Create triggers for updated_at columns
- [ ] Insert default data (matching MongoDB)
- [ ] Test PostgreSQL connection and constraints

### Phase 4: Repository Implementation
- [ ] Implement UserRepository (MongoDB + PostgreSQL)
- [ ] Implement RoleRepository (MongoDB + PostgreSQL)
- [ ] Implement PermissionRepository (MongoDB + PostgreSQL)
- [ ] Implement SessionRepository (MongoDB + PostgreSQL)
- [ ] Add repository tests (unit + integration)

### Phase 5: Domain Services
- [ ] Implement PasswordService (Argon2id hashing)
- [ ] Implement TokenService (JWT generation/validation)
- [ ] Implement PermissionResolutionService (effective permissions)
- [ ] Add service tests

### Phase 6: Use Cases (CQRS)
- [ ] Implement all User commands (Create, Update, Delete, ChangePassword)
- [ ] Implement all Auth commands (Login, Logout, PasswordReset, MFA)
- [ ] Implement all Role/Permission commands (Assign, Revoke, Grant)
- [ ] Implement all queries (GetUser, ListUsers, GetEffectivePermissions)
- [ ] Add use case tests

### Phase 7: API Endpoints
- [ ] Implement Layer 1: Backbone generic CRUD (132 endpoints)
- [ ] Implement Layer 2: Domain-specific endpoints
- [ ] Add request/response validation
- [ ] Add API documentation (OpenAPI/Swagger)
- [ ] Add endpoint tests (E2E)

### Phase 8: Audit & Observability
- [ ] Implement audit logging middleware
- [ ] Implement audit log archival job (S3)
- [ ] Implement audit log cleanup job (MongoDB)
- [ ] Implement feature flags (system_settings)
- [ ] Add monitoring and alerting

---

## 14. Conclusion

This technical domain documentation provides **complete specifications** for implementing the Sapiens User Management System. All 12 entities, value objects, domain events, use cases, repositories, and domain services are fully documented with:

- **Proto file definitions** for type-safe domain modeling
- **MongoDB schemas** with validation and indexes
- **PostgreSQL schemas** with constraints and triggers
- **API endpoint specifications** (Backbone + Domain-specific)
- **Implementation checklist** for phased development

**Next Steps**:
1. Review this document with the engineering team
2. Begin Phase 1: Create all proto files in `libs/domain/`
3. Configure buf for proto generation
4. Proceed with repository and service implementation

**Document Version**: 2.0 (Complete)
**Last Updated**: 2025-01-19
**Status**: Ready for implementation

---
