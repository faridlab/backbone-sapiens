# Sapiens Module Specification

> **Purpose**: Complete specification for the Sapiens IAM (Identity and Access Management) module.
> This document captures all business requirements for Backbone schema generation.

---

## 1. Module Overview

### 1.1 Basic Information

| Field | Value |
|-------|-------|
| **Module Name** | `sapiens` |
| **Display Name** | Identity & Access Management |
| **Description** | Enterprise-grade authentication, authorization, user management, MFA, session management, audit logging, notifications, and workflow orchestration |
| **Business Domain** | Security / Identity Management |
| **Owner/Team** | Platform Security Team |

### 1.2 Module Dependencies

| Module | Relationship | Description |
|--------|--------------|-------------|
| `backbone-core` | Uses | Core framework traits and utilities |
| `backbone-messaging` | Uses | Event publishing and domain events |
| `corpus` | Provides | Organization context for OrganizationUser/Role/Permission |

### 1.3 Business Objectives

1. **Secure Authentication**: Provide enterprise-grade user authentication with password policies, MFA, and OAuth integration
2. **Fine-Grained Authorization**: Implement hybrid RBAC/ABAC for flexible permission management
3. **Session Management**: Track and manage user sessions across devices with security controls
4. **Compliance & Audit**: Maintain immutable audit logs for security compliance
5. **User Lifecycle**: Manage complete user lifecycle from registration to deactivation
6. **Multi-Factor Authentication**: Support TOTP, SMS, email, and hardware key MFA methods
7. **Workflow Automation**: Orchestrate complex identity workflows (onboarding, password reset, etc.)
8. **Notifications**: Multi-channel notification delivery (email, SMS, push, in-app)

---

## 2. Use Cases

### UC-001: User Registration
```
Actor: Guest User
Description: New user creates an account with email verification
Preconditions: User has valid email address
Postconditions: User account created with pending_verification status

Main Flow:
1. User submits registration form (username, email, password)
2. System validates password strength (12+ chars, mixed case, numbers, symbols)
3. System checks email/username uniqueness
4. System creates User with pending_verification status
5. System creates Profile and UserSettings
6. System sends verification email
7. User clicks verification link
8. System updates status to active

Alternative Flows:
- Duplicate email: Return error EMAIL_ALREADY_EXISTS
- Weak password: Return error WEAK_PASSWORD
- Verification timeout: Token expires after 24 hours

Business Rules:
- Username: 3-50 characters, alphanumeric with underscore/hyphen
- Password: minimum 12 characters with uppercase, lowercase, digit, special char
- Email verified before account activation

Related Entities: User, Profile, UserSettings, EmailVerificationToken
```

### UC-002: User Login
```
Actor: Registered User
Description: User authenticates with credentials and receives session token
Preconditions: User has active account
Postconditions: Session created, JWT token issued

Main Flow:
1. User submits username/email and password
2. System validates credentials
3. System checks account not locked
4. System checks MFA requirement
5. If MFA enabled, prompt for second factor
6. System creates Session with JWT
7. System updates last_login timestamp

Alternative Flows:
- Invalid credentials: Increment failed_login_attempts
- Account locked: Return error ACCOUNT_LOCKED (15 min lockout after 5 failures)
- MFA required: Redirect to MFA verification

Business Rules:
- Maximum 5 failed attempts before 15-minute lockout
- Session expires after 24 hours
- Only active users can login

Related Entities: User, Session, MFADevice, AuditLog
```

### UC-003: Multi-Factor Authentication Setup
```
Actor: Authenticated User
Description: User enrolls MFA device for enhanced security
Preconditions: User is logged in
Postconditions: MFA device verified and active

Main Flow:
1. User selects MFA type (TOTP, SMS, Email)
2. System generates secret/sends code
3. User verifies with OTP
4. System marks device as active
5. System updates UserSettings.mfa_enabled

Alternative Flows:
- Invalid OTP: Allow retry, max 3 attempts
- TOTP already exists: Error MULTIPLE_TOTP_DEVICES
- Max devices reached: Error MAX_DEVICES_EXCEEDED (limit: 10)

Business Rules:
- Only one active TOTP device per user
- Maximum 10 MFA devices per user
- Device requires verification before use

Related Entities: User, MFADevice, MFASession, UserSettings
```

### UC-004: Password Reset
```
Actor: User (forgot password)
Description: User resets password via email verification
Preconditions: User has valid email on file
Postconditions: Password updated, all sessions invalidated

Main Flow:
1. User requests password reset with email
2. System creates PasswordResetToken (1 hour expiry)
3. System sends reset email
4. User clicks reset link with token
5. User enters new password
6. System validates password strength
7. System updates password hash
8. System invalidates all existing sessions
9. System sends confirmation email

Alternative Flows:
- Invalid/expired token: Error INVALID_TOKEN
- Same password as before: Error PASSWORD_REUSED
- Email not found: Silently succeed (security)

Business Rules:
- Token expires after 1 hour
- Token single-use only
- All sessions invalidated on password change

Related Entities: User, PasswordResetToken, Session, AuditLog
```

### UC-005: Role Assignment
```
Actor: Admin
Description: Admin assigns role to user
Preconditions: Admin has assign_role permission
Postconditions: User has new role with associated permissions

Main Flow:
1. Admin selects user and role
2. System validates admin permission
3. System creates UserRole assignment
4. System logs role assignment
5. System sends notification to user

Alternative Flows:
- Role already assigned: Error ROLE_ALREADY_ASSIGNED
- Invalid role: Error ROLE_NOT_FOUND

Business Rules:
- Admin cannot modify own super_admin role
- Assignment recorded in audit log

Related Entities: User, Role, UserRole, AuditLog
```

### UC-006: Session Management
```
Actor: User/Admin
Description: View and revoke active sessions
Preconditions: User authenticated or admin access
Postconditions: Session(s) revoked if requested

Main Flow:
1. User views list of active sessions
2. System shows device type, IP, last activity
3. User selects session to revoke
4. System marks session as revoked
5. System logs session revocation

Business Rules:
- Users can only see/revoke own sessions
- Admins can revoke any session
- Revoked sessions cannot be modified

Related Entities: Session, User, AuditLog
```

### UC-007: Bulk User Import
```
Actor: Admin
Description: Import users from CSV/Excel file
Preconditions: Admin has bulk_import permission
Postconditions: Users created, results tracked

Main Flow:
1. Admin uploads file with user data
2. System creates BulkOperation record
3. System processes records asynchronously
4. For each record: validate, create user, log result
5. System updates progress percentage
6. System generates results file
7. Admin downloads results

Business Rules:
- Maximum file size: 10MB
- Supported formats: CSV, XLSX
- Duplicate emails skipped with warning
- Failed records tracked in BulkOperationResult

Related Entities: User, BulkOperation, BulkOperationResult
```

### UC-008: OAuth Login
```
Actor: User
Description: Login via OAuth provider (Google, GitHub, etc.)
Preconditions: OAuth provider configured
Postconditions: User authenticated, session created

Main Flow:
1. User clicks OAuth provider button
2. System redirects to provider authorization URL
3. User authorizes application
4. Provider redirects back with code
5. System exchanges code for tokens
6. System fetches user info from provider
7. If user exists: link account, create session
8. If new user: create account, skip email verification

Business Rules:
- OAuth email auto-verified
- User can link multiple OAuth providers
- Primary OAuth provider can be designated

Related Entities: User, OAuthProvider, UserOAuthLink, Session
```

### UC-009: User Impersonation
```
Actor: Admin
Description: Admin impersonates another user for support/troubleshooting
Preconditions: Admin has impersonate permission
Postconditions: Admin session acts as target user, audit log created

Main Flow:
1. Admin initiates impersonation for target user
2. System validates admin permission
3. System creates impersonation session linked to admin
4. System records audit log with impersonation details
5. Admin can perform actions as target user
6. Admin ends impersonation or session expires

Business Rules:
- All actions during impersonation are logged
- Impersonation sessions have max 1 hour duration
- Cannot impersonate other admins
- Impersonation requires explicit confirmation

Related Entities: User, Session, AuditLog, UserPermission
```

### UC-010: Password Change with History
```
Actor: Authenticated User
Description: User changes password with history validation
Preconditions: User is logged in
Postconditions: Password updated, previous passwords hashed

Main Flow:
1. User submits current password and new password
2. System validates current password
3. System validates new password strength
4. System checks password history (last N passwords)
5. System hashes new password with Argon2id
6. System stores hash in password history
7. System invalidates all existing sessions
8. System sends confirmation notification

Alternative Flows:
- Current password invalid: Error INVALID_CURRENT_PASSWORD
- New password in history: Error PASSWORD_REUSED
- New password too weak: Error WEAK_PASSWORD

Business Rules:
- Password cannot match last 5 passwords
- Password history retained for 1 year
- All sessions invalidated on password change

Related Entities: User, PasswordHistory, Session, AuditLog
```

### UC-011: Session Management with Limits
```
Actor: User
Description: User views and manages active sessions with concurrent limits
Preconditions: User is logged in
Postconditions: Sessions listed, excess sessions revoked

Main Flow:
1. User views list of active sessions
2. System shows device, IP, location, last activity
3. System flags sessions exceeding concurrent limit
4. User can revoke specific or all other sessions
5. System updates session status

Business Rules:
- Max 5 concurrent sessions per user (configurable)
- Oldest sessions automatically revoked when limit exceeded
- Current session cannot be revoked
- Session locations shown via IP geolocation

Related Entities: Session, User, DeviceTrust, SystemSettings
```

### UC-012: User Data Export (GDPR)
```
Actor: User or Admin
Description: Export all user data in machine-readable format
Preconditions: User authenticated or admin request
Postconditions: Data export file generated, download link provided

Main Flow:
1. User or admin requests data export
2. System validates request (admin can export any user)
3. System collects all user data:
   - User profile and settings
   - Account activity logs
   - Login history
   - Associated entities (roles, permissions)
4. System generates JSON/PDF export file
5. System stores file temporarily (7 days)
6. System sends download link via email

Business Rules:
- Export includes all personal data per GDPR
- Data retained for 7 days then auto-deleted
- Admin exports require justification
- Export logged in audit trail

Related Entities: User, Profile, Session, AuditLog, Notification
```

### UC-013: User Anonymization (Right to be Forgotten)
```
Actor: Admin
Description: Anonymize user data while preserving referential integrity
Preconditions: Admin has delete_user permission
Postconditions: User data anonymized, account deactivated

Main Flow:
1. Admin requests user anonymization
2. System validates admin permission
3. System anonymizes personal data:
   - Email: anonymized@deleted.local
   - Username: deleted_{uuid}
   - Profile: All fields set to null
   - Password: Random hash
4. System preserves relationships for audit
5. System deactivates account
6. System creates anonymization record

Business Rules:
- Cannot anonymize admins (demote first)
- Audit logs preserve original data
- External references maintained via UUID
- Anonymization is irreversible

Related Entities: User, Profile, AuditLog, AnonymizationRecord
```

### UC-014: Resource-Based Permission Check
```
Actor: System
Description: Check if user has permission on specific resource instance
Preconditions: User authenticated, resource exists
Postconditions: Access granted or denied

Main Flow:
1. User attempts action on resource
2. System loads user's direct and role permissions
3. System evaluates resource-specific permissions
4. System checks ownership and delegation
5. System grants or denies access

Business Rules:
- Resource owner always has full access
- Delegated permissions checked before role permissions
- Wildcard permissions (*) match all resources
- Deny overrides allow

Related Entities: User, Permission, Role, ResourcePermission, UserRole
```

### UC-015: Temporary Access Grant
```
Actor: Admin
Description: Grant temporary time-bound access to user
Preconditions: Admin has grant_permission
Postconditions: User has access for specified duration

Main Flow:
1. Admin selects user and permission
2. Admin specifies time period
3. System creates TemporaryPermission grant
4. User receives notification of access grant
5. System monitors expiration
6. Expired grants automatically revoked

Business Rules:
- Max duration: 30 days (configurable)
- Requires justification in audit log
- User notified before expiration (24h)
- Revocation possible before expiry

Related Entities: User, Permission, TemporaryPermission, AuditLog, Notification
```

---

## 3. Entities (Data Models)

### 3.1 Entity List Summary

| Entity Name | Description | Type | Parent Entity | Status |
|-------------|-------------|------|---------------|--------|
| User | Core user identity and authentication | aggregate_root | N/A | ✅ Implemented |
| Profile | User profile information | entity | User | ✅ Implemented |
| Session | JWT session management | entity | User | ✅ Implemented |
| Role | Permission grouping | aggregate_root | N/A | ✅ Implemented |
| Permission | Fine-grained access control | entity | N/A | ✅ Implemented |
| UserRole | User-to-role assignment | entity | User, Role | ✅ Implemented |
| RolePermission | Role-to-permission mapping | entity | Role, Permission | ✅ Implemented |
| UserPermission | Direct permission grants | entity | User, Permission | ✅ Implemented |
| UserSettings | User preferences | entity | User | ✅ Implemented |
| MFADevice | Multi-factor auth devices | entity | User | ✅ Implemented |
| MFASession | MFA verification sessions | entity | User, MFADevice | ✅ Implemented |
| MFABackupCode | MFA backup codes | entity | MFADevice | ✅ Implemented |
| AuditLog | Immutable audit trail | entity | User, Session | ✅ Implemented |
| PasswordResetToken | Password reset tokens | entity | User | ✅ Implemented |
| EmailVerificationToken | Email verification | entity | User | ✅ Implemented |
| OAuthProvider | OAuth provider config | entity | N/A | ✅ Implemented |
| UserOAuthLink | User OAuth links | entity | User, OAuthProvider | ✅ Implemented |
| Notification | User notifications | entity | User | ✅ Implemented |
| NotificationTemplate | Notification templates | entity | N/A | ✅ Implemented |
| NotificationLog | Notification delivery log | entity | Notification | ✅ Implemented |
| Workflow | Workflow instances | aggregate_root | User | ✅ Implemented |
| WorkflowDefinition | Workflow templates | entity | N/A | ✅ Implemented |
| WorkflowStep | Workflow steps | entity | Workflow | ✅ Implemented |
| WorkflowExecution | Workflow executions | entity | Workflow | ✅ Implemented |
| WorkflowAction | Workflow actions | entity | WorkflowDefinition | ✅ Implemented |
| WorkflowActionExecution | Action executions | entity | WorkflowExecution | ✅ Implemented |
| BulkOperation | Bulk operation jobs | aggregate_root | User | ✅ Implemented |
| BulkOperationResult | Individual results | entity | BulkOperation | ✅ Implemented |
| DeviceTrust | Device trust management | entity | User | ✅ Implemented |
| SystemSettings | Global configuration | entity | N/A | ✅ Implemented |
| BackupCode | General backup codes | entity | User | ✅ Implemented |
| AnalyticsEvent | User behavior tracking | entity | User | ✅ Implemented |
| AnalyticsMetric | Aggregated metrics | entity | N/A | ✅ Implemented |
| AnalyticsReport | Generated reports | entity | N/A | ✅ Implemented |
| **PasswordHistory** | Track previous password hashes | entity | User | 🆕 New |
| **ResourcePermission** | Resource-level permissions | entity | Permission | 🆕 New |
| **TemporaryPermission** | Time-bound access grants | entity | User, Permission | 🆕 New |
| **SessionLimit** | Concurrent session limits | entity | User | 🆕 New |
| **DataExport** | GDPR data export requests | entity | User | 🆕 New |
| **AnonymizationRecord** | User anonymization records | entity | User | 🆕 New |
| **ImpersonationSession** | Admin impersonation sessions | entity | Session | 🆕 New |
| **SecurityEvent** | Security-related events | entity | User | 🆕 New |
| **PermissionCache** | Cached effective permissions | entity | User | 🆕 New |
| **NotificationPreference** | Per-user notification settings | entity | User | 🆕 New |
| **SAMLProvider** | SAML 2.0 enterprise SSO | entity | N/A | 🆕 New |
| **UserSAMLLink** | User SAML account links | entity | User, SAMLProvider | 🆕 New |
| **LDAPDirectory** | LDAP/AD directory config | entity | N/A | 🆕 New |

### 3.2 Entity Definitions

#### Entity: `User`

**Description**: Core aggregate root for user identity, authentication credentials, and account status.

**Type**: aggregate_root

**Table Name**: `users`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `username` | string | Yes | Yes | - | Unique username (3-50 chars, alphanumeric + _-) |
| `email` | email | Yes | Yes | - | Unique email, normalized to lowercase |
| `password_hash` | string | Yes | No | - | Argon2id hash, never exposed in API |
| `status` | enum:UserStatus | Yes | No | pending_verification | Account status |
| `email_verified` | boolean | Yes | No | false | Email verification flag |
| `failed_login_attempts` | integer | Yes | No | 0 | Login failure counter |
| `locked_until` | timestamp | No | No | null | Account unlock time |
| `last_login` | timestamp | No | No | null | Last successful login |
| `metadata` | Metadata | Yes | No | {} | Audit fields (created_at, updated_at, deleted_at) |

##### Relationships

| Relationship | Target Entity | Type | Field | Description |
|--------------|---------------|------|-------|-------------|
| roles | Role | many_to_many | user_roles | User's assigned roles |
| permissions | UserPermission | has_many | user_id | Direct permission grants |
| user_settings | UserSettings | has_one | user_id | User preferences |
| sessions | Session | has_many | user_id | Active sessions |
| mfa_devices | MFADevice | has_many | user_id | MFA devices |
| audit_logs | AuditLog | has_many | user_id | User's audit trail |
| password_resets | PasswordResetToken | has_many | user_id | Password reset tokens |

##### Indexes

| Index Name | Fields | Unique | Description |
|------------|--------|--------|-------------|
| `idx_users_email` | `email` | Yes | Email lookup |
| `idx_users_username` | `username` | Yes | Username lookup |
| `idx_users_status` | `status` | No | Status filtering |
| `idx_users_email_verified` | `email_verified` | No | Verification status |

##### Validation Rules

| Field | Rule | Message |
|-------|------|---------|
| `username` | `min_length:3, max_length:50, pattern:^[a-zA-Z0-9_-]+$` | "Username must be 3-50 characters, alphanumeric with underscore/hyphen" |
| `email` | `email, lowercase` | "Valid email required" |
| `password_hash` | `min_length:60` | "Invalid password hash" |

##### Business Rules

1. Email must be verified before account becomes active
2. Account locked for 15 minutes after 5 failed login attempts
3. Password change invalidates all existing sessions
4. Username cannot be changed after creation (immutable)

##### Computed Fields

| Field Name | Type | Computation | Description |
|------------|------|-------------|-------------|
| `full_name` | string | `profile.first_name + ' ' + profile.last_name` | Full name from profile |
| `display_name` | string | `username ?? email.split('@')[0]` | Display name with fallback |
| `is_admin` | boolean | `roles.any(name == 'ADMIN')` | Admin role check |
| `is_verified` | boolean | `email_verified && status == 'active'` | Fully verified check |
| `is_locked` | boolean | `locked_until != null && locked_until > now()` | Lock status |
| `active_session_count` | integer | `sessions.count(is_active && !expired)` | Active sessions |
| `mfa_enabled` | boolean | `user_settings.mfa_enabled` | MFA enabled check |

---

#### Entity: `Profile`

**Description**: User profile information (one-to-one with User).

**Type**: entity

**Table Name**: `profiles`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `user_id` | uuid | Yes | Yes | - | Primary key, references User.id |
| `first_name` | string | Yes | No | - | First name (max 100) |
| `middle_name` | string | No | No | null | Middle name (max 100) |
| `last_name` | string | No | No | null | Last name (max 100) |
| `dob` | date | No | No | null | Date of birth |
| `pob` | string | No | No | null | Place of birth |
| `gender` | enum:Gender | No | No | null | Gender |
| `phone_number` | phone | No | No | null | E.164 format |
| `profile_picture_url` | url | No | No | null | Profile image URL |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

---

#### Entity: `Role`

**Description**: Named collection of permissions for RBAC.

**Type**: aggregate_root

**Table Name**: `roles`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `name` | string | Yes | Yes | - | Unique role name, UPPERCASE (e.g., ADMIN, EDITOR) |
| `description` | string | No | No | null | Role description (max 1000) |
| `is_default` | boolean | Yes | No | false | Auto-assigned to new users |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Only one role can have is_default = true
2. Role name must be uppercase

---

#### Entity: `Permission`

**Description**: Granular action definition for fine-grained access control.

**Type**: entity

**Table Name**: `permissions`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `name` | string | Yes | Yes | - | Format: action:resource (e.g., read:users) |
| `description` | string | No | No | null | Human-readable description |
| `resource` | string | Yes | No | - | Resource name (users, roles, *) |
| `action` | string | Yes | No | - | Action verb (read, write, export) |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Permission name format: `{action}:{resource}`
2. Hierarchy: admin:* > admin:{module} > admin:{resource} > write:{resource} > {action}:{resource}

---

#### Entity: `Session`

**Description**: JWT session with device tracking.

**Type**: entity

**Table Name**: `sessions`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Session ID (JWT jti claim) |
| `user_id` | uuid | Yes | No | - | References User.id |
| `token_hash` | string | Yes | Yes | - | SHA-256 hash of JWT |
| `expires_at` | timestamp | Yes | No | - | JWT expiry time (24h default) |
| `last_activity` | timestamp | No | No | null | Last API call timestamp |
| `ip_address` | ip | No | No | null | Client IP at creation |
| `user_agent` | string | No | No | null | Browser/client info |
| `device_type` | enum:DeviceType | Yes | No | unknown | Device category |
| `is_active` | boolean | Yes | No | true | Revocation flag |
| `revoked_at` | timestamp | No | No | null | When revoked |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Computed Fields

| Field Name | Type | Computation | Description |
|------------|------|-------------|-------------|
| `is_expired` | boolean | `expires_at <= now()` | Expiry check |
| `is_valid` | boolean | `is_active && !is_expired` | Validity check |
| `time_until_expiry` | duration | `expires_at - now()` | Time remaining |

---

#### Entity: `MFADevice`

**Description**: Multi-factor authentication device management.

**Type**: entity

**Table Name**: `mfa_devices`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | Yes | No | - | References User.id |
| `device_type` | enum:MFADeviceType | Yes | No | - | Type (totp, sms, email, etc.) |
| `device_name` | string | No | No | null | User-friendly name |
| `secret` | string | No | No | null | Encrypted secret (sensitive) |
| `totp_secret` | string | No | No | null | TOTP secret (sensitive) |
| `phone_number` | string | No | No | null | For SMS MFA |
| `email_address` | string | No | No | null | For email MFA |
| `is_active` | boolean | Yes | No | false | Active status |
| `is_primary` | boolean | Yes | No | false | Primary device flag |
| `verified_at` | timestamp | No | No | null | First verification time |
| `last_used_at` | timestamp | No | No | null | Last usage time |
| `status` | enum:MFADeviceStatus | Yes | No | active | Device status |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Maximum 10 MFA devices per user
2. Only one active TOTP device allowed
3. Device requires verification before use

---

#### Entity: `AuditLog`

**Description**: Immutable record of all user events for compliance.

**Type**: entity (append-only)

**Table Name**: `audit_logs`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | No | No | null | User who performed action |
| `action` | string | Yes | No | - | Event type (login_success, user_created) |
| `details` | json | No | No | null | Event-specific payload |
| `severity` | enum:AuditLogSeverity | Yes | No | info | Severity level |
| `ip_address` | ip | No | No | null | Client IP |
| `user_agent` | string | No | No | null | Browser info |
| `session_id` | uuid | No | No | null | Associated session |
| `resource_type` | string | No | No | null | Affected resource type |
| `resource_id` | string | No | No | null | Affected resource ID |
| `timestamp` | timestamp | Yes | No | now() | Event timestamp |
| `archived` | boolean | Yes | No | false | Archive status |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. **NO updates allowed** - immutable
2. **NO deletes allowed** - append-only
3. Archive after 90 days, retention 365 days

---

#### Entity: `Notification`

**Description**: Multi-channel user notifications.

**Type**: entity

**Table Name**: `notifications`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | Yes | No | - | Recipient user |
| `type` | enum:NotificationType | Yes | No | - | Notification type |
| `channel` | enum:NotificationChannel | Yes | No | - | Delivery channel |
| `title` | string | Yes | No | - | Notification title |
| `message` | string | Yes | No | - | Message content |
| `data` | json | No | No | null | Additional data |
| `is_read` | boolean | Yes | No | false | Read status |
| `read_at` | timestamp | No | No | null | When read |
| `sent_at` | timestamp | No | No | null | When sent |
| `delivered_at` | timestamp | No | No | null | When delivered |
| `priority` | enum:NotificationPriority | Yes | No | normal | Priority level |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

---

#### Entity: `Workflow`

**Description**: Business workflow orchestration.

**Type**: aggregate_root

**Table Name**: `workflows`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Workflow ID |
| `workflow_type` | enum:WorkflowType | Yes | No | - | Workflow type |
| `status` | enum:WorkflowStatus | Yes | No | pending | Current status |
| `initiator_id` | uuid | Yes | No | - | User who initiated |
| `target_user_id` | uuid | No | No | null | Target user |
| `title` | string | Yes | No | - | Workflow title |
| `context` | json | No | No | null | Workflow context |
| `current_step` | integer | Yes | No | 0 | Current step number |
| `total_steps` | integer | Yes | No | 0 | Total steps |
| `progress_percentage` | float | Yes | No | 0.0 | Progress (0-100) |
| `retry_count` | integer | Yes | No | 0 | Retry attempts |
| `max_retries` | integer | Yes | No | 3 | Max retries |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

---

#### Entity: `BulkOperation`

**Description**: Bulk operation job tracking.

**Type**: aggregate_root

**Table Name**: `bulk_operations`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Job ID |
| `operation_type` | enum:BulkOperationType | Yes | No | - | Operation type |
| `status` | enum:BulkOperationStatus | Yes | No | pending | Status |
| `created_by` | uuid | Yes | No | - | Initiator |
| `total_records` | integer | Yes | No | - | Total records |
| `processed_records` | integer | Yes | No | 0 | Processed count |
| `successful_records` | integer | Yes | No | 0 | Success count |
| `failed_records` | integer | Yes | No | 0 | Failure count |
| `progress_percentage` | float | Yes | No | 0.0 | Progress |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

---

#### Entity: `PasswordHistory` 🆕

**Description**: Track user password history to prevent reuse.

**Type**: entity

**Table Name**: `password_history`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | Yes | No | - | References User.id |
| `password_hash` | string | Yes | No | - | Argon2id hash |
| `set_at` | timestamp | Yes | No | now() | When password was set |
| `expires_at` | timestamp | No | No | null | When history entry expires (1 year) |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Keep last 5 passwords minimum
2. Entries expire after 1 year
3. Hashes never exposed via API

##### Indexes

| Index Name | Fields | Unique | Description |
|------------|--------|--------|-------------|
| `idx_password_history_user` | `user_id, set_at DESC` | No | User's password history |
| `idx_password_history_expires` | `expires_at` | No | Cleanup expired entries |

---

#### Entity: `ResourcePermission` 🆕

**Description**: Resource-level permission grants for fine-grained access control.

**Type**: entity

**Table Name**: `resource_permissions`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `permission_id` | uuid | Yes | No | - | References Permission.id |
| `resource_type` | string | Yes | No | - | Type of resource (e.g., "document", "project") |
| `resource_id` | string | Yes | No | - | ID of the resource instance |
| `granted_to_user_id` | uuid | No | No | null | Granted to specific user |
| `granted_to_role_id` | uuid | No | No | null | Granted to role members |
| `granted_by` | uuid | Yes | No | - | Who granted this permission |
| `granted_at` | timestamp | Yes | No | now() | Grant timestamp |
| `expires_at` | timestamp | No | No | null | Optional expiration |
| `reason` | string | No | No | null | Justification for grant |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Either user_id or role_id must be specified
2. Resource owner always has full access
3. Expired grants are automatically ignored

##### Indexes

| Index Name | Fields | Unique | Description |
|------------|--------|--------|-------------|
| `idx_resource_permission_lookup` | `resource_type, resource_id` | No | Find resource permissions |
| `idx_resource_permission_user` | `granted_to_user_id` | No | User's resource permissions |
| `idx_resource_permission_role` | `granted_to_role_id` | No | Role-based resource permissions |
| `idx_resource_permission_expires` | `expires_at` | No | Cleanup expired grants |

---

#### Entity: `TemporaryPermission` 🆕

**Description**: Time-bound permission grants with automatic expiration.

**Type**: entity

**Table Name**: `temporary_permissions`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | Yes | No | - | Recipient user |
| `permission_id` | uuid | Yes | No | - | Granted permission |
| `granted_by` | uuid | Yes | No | - | Admin who granted |
| `granted_at` | timestamp | Yes | No | now() | Grant timestamp |
| `expires_at` | timestamp | Yes | No | - | Expiration timestamp |
| `revoked_at` | timestamp | No | No | null | If revoked early |
| `revoked_by` | uuid | No | No | null | Who revoked |
| `reason` | string | No | No | null | Justification |
| `notified_before_expiry` | boolean | Yes | No | false | 24h notice sent |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Max duration: 30 days (configurable)
2. User notified 24h before expiration
3. Can be revoked early

##### Indexes

| Index Name | Fields | Unique | Description |
|------------|--------|--------|-------------|
| `idx_temp_permission_user` | `user_id, expires_at DESC` | No | User's active temp permissions |
| `idx_temp_permission_expires` | `expires_at` | No | Cleanup expired grants |

---

#### Entity: `SessionLimit` 🆕

**Description**: Per-user concurrent session limits and configuration.

**Type**: entity

**Table Name**: `session_limits`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | Yes | Yes | - | References User.id |
| `max_sessions` | integer | Yes | No | 5 | Max concurrent sessions |
| `max_sessions_per_device` | integer | No | No | null | Per-device limit |
| `enforce_limit` | boolean | Yes | No | true | Whether to enforce |
| `current_session_count` | integer | Yes | No | 0 | Computed: active sessions |
| `last_session_revoke_at` | timestamp | No | No | null | Last auto-revoke time |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Default max 5 sessions (configurable via SystemSettings)
2. Oldest sessions revoked when limit exceeded
3. Current session never revoked

---

#### Entity: `DataExport` 🆕

**Description**: GDPR user data export requests and results.

**Type**: entity

**Table Name**: `data_exports`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | Yes | No | - | Subject user |
| `requested_by` | uuid | Yes | No | - | Who requested (may be admin) |
| `requested_at` | timestamp | Yes | No | now() | Request timestamp |
| `status` | enum:DataExportStatus | Yes | No | pending | Export status |
| `file_path` | string | No | No | null | Generated file path |
| `file_url` | url | No | No | null | Download URL |
| `expires_at` | timestamp | Yes | No | - | File deletion time (7 days) |
| `completed_at` | timestamp | No | No | null | Completion timestamp |
| `record_count` | integer | No | No | null | Number of records exported |
| `format` | enum:DataExportFormat | Yes | No | json | Export format |
| `justification` | string | No | No | null | Required for admin requests |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Files auto-deleted after 7 days
2. Admin requests require justification
3. User can request own data without justification

---

#### Entity: `AnonymizationRecord` 🆕

**Description**: Record of user data anonymization (GDPR right to be forgotten).

**Type**: entity

**Table Name**: `anonymization_records`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | Yes | No | - | Anonymized user |
| `original_email` | string | Yes | No | - | Original email (audit only) |
| `original_username` | string | Yes | No | - | Original username (audit only) |
| `anonymized_by` | uuid | Yes | No | - | Admin who performed |
| `anonymized_at` | timestamp | Yes | No | now() | Anonymization timestamp |
| `reason` | string | Yes | No | - | Legal basis/justification |
| `method` | enum:AnonymizationMethod | Yes | No | full | Method used |
| `retention_period` | integer | No | No | null | Days to keep audit data |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Cannot anonymize admins (must demote first)
2. Original data only in audit table
3. Process is irreversible

---

#### Entity: `ImpersonationSession` 🆕

**Description**: Admin impersonation sessions with audit trail.

**Type**: entity

**Table Name**: `impersonation_sessions`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `admin_id` | uuid | Yes | No | - | Admin performing impersonation |
| `target_user_id` | uuid | Yes | No | - | User being impersonated |
| `session_id` | uuid | Yes | No | - | Linked Session.id |
| `started_at` | timestamp | Yes | No | now() | Start time |
| `ended_at` | timestamp | No | No | null | End time |
| `max_duration_minutes` | integer | Yes | No | 60 | Max duration |
| `reason` | string | Yes | No | - | Justification |
| `actions_performed` | integer | Yes | No | 0 | Count of actions |
| `terminated_by` | uuid | No | No | null | Who terminated |
| `termination_reason` | string | No | No | null | Why terminated |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Max 1 hour duration (configurable)
2. All actions logged with impersonation context
3. Cannot impersonate other admins
4. Auto-termination on expiry

##### Computed Fields

| Field Name | Type | Computation | Description |
|------------|------|-------------|-------------|
| `is_active` | boolean | `ended_at IS NULL AND started_at + max_duration > now()` | Whether session is active |
| `duration_minutes` | integer | `COALESCE(ended_at, now()) - started_at` | Minutes elapsed |

---

#### Entity: `SecurityEvent` 🆕

**Description**: Security-related events for monitoring and alerting.

**Type**: entity

**Table Name**: `security_events`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | No | No | null | Affected user |
| `event_type` | enum:SecurityEventType | Yes | No | - | Type of security event |
| `severity` | enum:SecurityEventSeverity | Yes | No | - | Event severity |
| `source_ip` | ip | No | No | null | Source IP address |
| `user_agent` | string | No | No | null | Browser/client info |
| `details` | json | No | No | null | Event details |
| `is_resolved` | boolean | Yes | No | false | Resolution status |
| `resolved_at` | timestamp | No | No | null | Resolution timestamp |
| `resolved_by` | uuid | No | No | null | Who resolved |
| `created_at` | timestamp | Yes | No | now() | Event timestamp |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Critical events trigger immediate alerts
2. Events kept for 1 year minimum
3. High severity events require resolution

---

#### Entity: `PermissionCache` 🆕

**Description**: Cached effective permissions for performance optimization.

**Type**: entity

**Table Name**: `permission_cache`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | Yes | Yes | - | Cached user |
| `permission_ids` | json | Yes | No | - | Array of permission UUIDs |
| `role_ids` | json | Yes | No | - | Array of role UUIDs |
| `computed_at` | timestamp | Yes | No | now() | Cache computation time |
| `expires_at` | timestamp | Yes | No | - | Cache expiry (5 min) |
| `version` | integer | Yes | No | 1 | Incremented on changes |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Cache invalidated on permission/role change
2. Default TTL: 5 minutes (configurable)
3. Lazy recalculation on cache miss

---

#### Entity: `NotificationPreference` 🆕

**Description**: Per-user notification channel and type preferences.

**Type**: entity

**Table Name**: `notification_preferences`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | Yes | Yes | - | Preferences owner |
| `notification_type` | enum:NotificationType | Yes | Yes | - | Notification type |
| `channel_enabled` | boolean | Yes | No | true | Channel enabled flag |
| `channels` | json | Yes | No | {} | Map of channel->enabled |
| `quiet_hours_start` | time | No | No | null | Quiet hours start |
| `quiet_hours_end` | time | No | No | null | Quiet hours end |
| `quiet_timezone` | string | No | No | UTC | Timezone for quiet hours |
| `digest_enabled` | boolean | Yes | No | false | Batch into digest |
| `digest_frequency` | enum:DigestFrequency | No | No | immediate | Digest frequency |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. User-specific override of system defaults
2. Quiet hours suppress non-urgent notifications
3. Digest mode batches notifications

##### Indexes

| Index Name | Fields | Unique | Description |
|------------|--------|--------|-------------|
| `idx_notification_pref_user` | `user_id` | No | User's preferences |
| `idx_notification_pref_type` | `notification_type` | No | Type-specific prefs |

---

#### Entity: `SAMLProvider` 🆕

**Description**: SAML 2.0 enterprise SSO provider configuration.

**Type**: entity

**Table Name**: `saml_providers`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `name` | string | Yes | Yes | - | Provider name |
| `display_name` | string | Yes | No | - | Display name for UI |
| `entity_id` | string | Yes | Yes | - | SAML Entity ID |
| `sso_url` | url | Yes | No | - | IdP SSO URL |
| `slo_url` | url | No | No | null | IdP SLO URL |
| `certificate` | text | Yes | No | - | IdP X.509 certificate |
| `acs_url` | url | Yes | No | - | Assertion Consumer Service URL |
| `sls_url` | url | No | No | null | Single Logout Service URL |
| `name_id_format` | string | Yes | No | urn:oasis:names:tc:SAML:1.1:nameid-format:unspecified | NameID format |
| `attribute_mapping` | json | No | No | {} | Map SAML attrs to user attrs |
| `is_active` | boolean | Yes | No | true | Provider enabled |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Entity ID must be unique
2. Certificate must be valid X.509
3. Attribute mapping: {"email": "email", "firstName": "first_name"}

---

#### Entity: `UserSAMLLink` 🆕

**Description**: Link between user and SAML provider accounts.

**Type**: entity

**Table Name**: `user_saml_links`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | Yes | No | - | References User.id |
| `provider_id` | uuid | Yes | No | - | References SAMLProvider.id |
| `name_id` | string | Yes | Yes | - | SAML NameID (unique per provider) |
| `session_index` | string | No | No | null | Current SAML session |
| `first_login_at` | timestamp | Yes | No | now() | First login via SAML |
| `last_login_at` | timestamp | Yes | No | now() | Last login via SAML |
| `attributes` | json | No | No | {} | SAML attributes from IdP |
| `is_primary` | boolean | Yes | No | false | Primary auth method |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. NameID unique per provider
2. One primary SAML link per user
3. Auto-creates user on first login if configured

---

#### Entity: `LDAPDirectory` 🆕

**Description**: LDAP/Active Directory directory configuration.

**Type**: entity

**Table Name**: `ldap_directories`

##### Fields

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `name` | string | Yes | Yes | - | Directory name |
| `display_name` | string | Yes | No | - | Display name |
| `host` | string | Yes | No | - | LDAP server host |
| `port` | integer | Yes | No | 389 | LDAP port (389 or 636) |
| `use_ssl` | boolean | Yes | No | true | Use LDAPS |
| `use_tls` | boolean | Yes | No | false | StartTLS |
| `bind_dn` | string | Yes | No | - | Bind DN for authentication |
| `bind_password` | string | Yes | No | - | Bind password (encrypted) |
| `search_base` | string | Yes | No | - | Base DN for searches |
| `search_filter` | string | Yes | No | (uid={username}) | User search filter |
| `attribute_mapping` | json | Yes | No | {} | LDAP to user attr mapping |
| `sync_enabled` | boolean | Yes | No | false | Auto-sync users |
| `sync_interval_minutes` | integer | No | No | 60 | Sync interval |
| `last_sync_at` | timestamp | No | No | null | Last successful sync |
| `is_active` | boolean | Yes | No | true | Directory enabled |
| `metadata` | Metadata | Yes | No | {} | Audit fields |

##### Business Rules

1. Password encrypted at rest
2. Default attribute mapping: {"uid": "username", "mail": "email", "cn": "first_name", "sn": "last_name"}
3. Sync creates/updates users automatically

---

## 4. Enums (Value Types)

### Enum: `UserStatus`

**Description**: User account status

| Variant | Value | Description |
|---------|-------|-------------|
| `Active` | `active` | Active user account |
| `Inactive` | `inactive` | Deactivated by user or admin |
| `Suspended` | `suspended` | Temporarily banned |
| `PendingVerification` | `pending_verification` | Awaiting email verification (default) |

### Enum: `Gender`

**Description**: User gender

| Variant | Value | Description |
|---------|-------|-------------|
| `Male` | `male` | Male gender |
| `Female` | `female` | Female gender |

### Enum: `DeviceType`

**Description**: Session device type

| Variant | Value | Description |
|---------|-------|-------------|
| `Web` | `web` | Web browser |
| `Mobile` | `mobile` | Mobile app |
| `Tablet` | `tablet` | Tablet device |
| `Desktop` | `desktop` | Desktop application |
| `Unknown` | `unknown` | Unknown device (default) |

### Enum: `MFADeviceType`

**Description**: MFA device types

| Variant | Value | Description |
|---------|-------|-------------|
| `Totp` | `totp` | Time-based One-Time Password |
| `Sms` | `sms` | SMS-based verification |
| `Email` | `email` | Email-based verification |
| `HardwareKey` | `hardware_key` | Hardware security key |
| `Biometric` | `biometric` | Biometric verification |
| `PushNotification` | `push_notification` | Push notification |

### Enum: `MFADeviceStatus`

**Description**: MFA device status

| Variant | Value | Description |
|---------|-------|-------------|
| `Active` | `active` | Device is active |
| `Inactive` | `inactive` | Device is inactive |
| `Suspended` | `suspended` | Temporarily suspended |
| `Compromised` | `compromised` | Device may be compromised |
| `Expired` | `expired` | Device has expired |
| `Revoked` | `revoked` | Device has been revoked |

### Enum: `EnrollmentMethod`

**Description**: MFA enrollment method

| Variant | Value | Description |
|---------|-------|-------------|
| `SelfService` | `self_service` | User enrolled themselves |
| `AdminEnforced` | `admin_enforced` | Admin enrolled for user |
| `Automatic` | `automatic` | Automatically enrolled |
| `Emergency` | `emergency` | Emergency enrollment |

### Enum: `AuditLogSeverity`

**Description**: Audit log severity levels

| Variant | Value | Description |
|---------|-------|-------------|
| `Info` | `info` | Normal operations (default) |
| `Warning` | `warning` | Potential issues |
| `Error` | `error` | Operation failures |
| `Critical` | `critical` | Security-critical events |

### Enum: `NotificationType`

**Description**: Notification types

| Variant | Value | Description |
|---------|-------|-------------|
| `Welcome` | `welcome` | Welcome message |
| `EmailVerification` | `email_verification` | Email verification |
| `PasswordReset` | `password_reset` | Password reset |
| `SecurityAlert` | `security_alert` | Security alert |
| `AccountSuspended` | `account_suspended` | Account suspended |
| `RoleAssigned` | `role_assigned` | Role assigned |
| `PermissionGranted` | `permission_granted` | Permission granted |
| `MfaEnabled` | `mfa_enabled` | MFA enabled |
| `LoginAlert` | `login_alert` | Login notification |
| `SystemMaintenance` | `system_maintenance` | System maintenance |
| `General` | `general` | General notification |

### Enum: `NotificationChannel`

**Description**: Notification delivery channels

| Variant | Value | Description |
|---------|-------|-------------|
| `InApp` | `in_app` | In-app notification |
| `Email` | `email` | Email notification |
| `Sms` | `sms` | SMS notification |
| `Push` | `push` | Push notification |

### Enum: `NotificationPriority`

**Description**: Notification priority levels

| Variant | Value | Description |
|---------|-------|-------------|
| `Low` | `low` | Low priority |
| `Normal` | `normal` | Normal priority (default) |
| `High` | `high` | High priority |
| `Urgent` | `urgent` | Urgent priority |

### Enum: `Theme`

**Description**: UI theme preference

| Variant | Value | Description |
|---------|-------|-------------|
| `Light` | `light` | Light mode |
| `Dark` | `dark` | Dark mode |
| `System` | `system` | Follow OS preference (default) |

### Enum: `WorkflowType`

**Description**: Workflow types

| Variant | Value | Description |
|---------|-------|-------------|
| `UserRegistration` | `user_registration` | User registration workflow |
| `EmailVerification` | `email_verification` | Email verification |
| `PasswordReset` | `password_reset` | Password reset |
| `AccountSuspension` | `account_suspension` | Account suspension |
| `RoleAssignment` | `role_assignment` | Role assignment |
| `PermissionGrant` | `permission_grant` | Permission grant |
| `MfaSetup` | `mfa_setup` | MFA setup |
| `EmailChange` | `email_change` | Email change |
| `AccountRecovery` | `account_recovery` | Account recovery |
| `BulkUserImport` | `bulk_user_import` | Bulk user import |
| `OauthLinking` | `oauth_linking` | OAuth account linking |
| `DeviceTrust` | `device_trust` | Device trust |
| `SecurityReview` | `security_review` | Security review |
| `ComplianceAudit` | `compliance_audit` | Compliance audit |

### Enum: `WorkflowStatus`

**Description**: Workflow status

| Variant | Value | Description |
|---------|-------|-------------|
| `Pending` | `pending` | Waiting to start (default) |
| `Running` | `running` | Currently executing |
| `Paused` | `paused` | Temporarily paused |
| `Completed` | `completed` | Successfully completed |
| `Failed` | `failed` | Failed with errors |
| `Cancelled` | `cancelled` | Cancelled by user |
| `Expired` | `expired` | Expired without completion |

### Enum: `BulkOperationType`

**Description**: Bulk operation types

| Variant | Value | Description |
|---------|-------|-------------|
| `UserImport` | `user_import` | Import users from file |
| `UserExport` | `user_export` | Export users to file |
| `RoleAssignment` | `role_assignment` | Bulk role assignment |
| `PermissionGrant` | `permission_grant` | Bulk permission grants |
| `UserUpdate` | `user_update` | Bulk profile updates |
| `UserSuspension` | `user_suspension` | Bulk suspension |
| `UserDeletion` | `user_deletion` | Bulk deletion |

### Enum: `BulkOperationStatus`

**Description**: Bulk operation status

| Variant | Value | Description |
|---------|-------|-------------|
| `Pending` | `pending` | Waiting to start |
| `Running` | `running` | Currently processing |
| `Completed` | `completed` | Successfully completed |
| `Failed` | `failed` | Failed with errors |
| `Cancelled` | `cancelled` | Cancelled by user |
| `Paused` | `paused` | Temporarily paused |

### Enum: `OAuthProviderType`

**Description**: OAuth provider types

| Variant | Value | Description |
|---------|-------|-------------|
| `Google` | `google` | Google OAuth 2.0 |
| `Github` | `github` | GitHub OAuth |
| `Microsoft` | `microsoft` | Microsoft Azure AD |
| `Facebook` | `facebook` | Facebook OAuth |
| `Apple` | `apple` | Apple Sign In |

### Enum: `DataExportStatus` 🆕

**Description**: Data export request status

| Variant | Value | Description |
|---------|-------|-------------|
| `Pending` | `pending` | Export queued |
| `Processing` | `processing` | Generating export |
| `Completed` | `completed` | Export ready for download |
| `Failed` | `failed` | Export failed |
| `Expired` | `expired` | Download link expired |

### Enum: `DataExportFormat` 🆕

**Description**: Data export file format

| Variant | Value | Description |
|---------|-------|-------------|
| `Json` | `json` | JSON format |
| `Csv` | `csv` | CSV format |
| `Pdf` | `pdf` | PDF format |
| `Xml` | `xml` | XML format |

### Enum: `AnonymizationMethod` 🆕

**Description**: User data anonymization method

| Variant | Value | Description |
|---------|-------|-------------|
| `Full` | `full` | Complete anonymization |
| `Partial` | `partial` | Partial anonymization (keep some data) |
| `Pseudonymization` | `pseudonymization` | Replace with pseudonyms |

### Enum: `SecurityEventType` 🆕

**Description**: Security event types

| Variant | Value | Description |
|---------|-------|-------------|
| `LoginFailed` | `login_failed` | Failed login attempt |
| `LoginSuspicious` | `login_suspicious` | Suspicious login (new location/device) |
| `PasswordReused` | `password_reused` | Password from history used |
| `AccountLocked` | `account_locked` | Account auto-locked |
| `PermissionEscalation` | `permission_escalation` | Permission elevation |
| `ImpersonationStarted` | `impersonation_started` | Admin started impersonation |
| `ImpersonationEnded` | `impersonation_ended` | Admin ended impersonation |
| `MFAEnabled` | `mfa_enabled` | MFA was enabled |
| `MFADisabled` | `mfa_disabled` | MFA was disabled |
| `DataExported` | `data_exported` | User data exported |
| `DataAnonymized` | `data_anonymized` | User data anonymized |
| `RateLimitExceeded` | `rate_limit_exceeded` | API rate limit exceeded |

### Enum: `SecurityEventSeverity` 🆕

**Description**: Security event severity levels

| Variant | Value | Description |
|---------|-------|-------------|
| `Low` | `low` | Informational |
| `Medium` | `medium` | Warning |
| `High` | `high` | Significant event |
| `Critical` | `critical` | Immediate action required |

### Enum: `DigestFrequency` 🆕

**Description**: Notification digest frequency

| Variant | Value | Description |
|---------|-------|-------------|
| `Immediate` | `immediate` | Send immediately (no digest) |
| `Hourly` | `hourly` | Batch hourly |
| `Daily` | `daily` | Batch daily |
| `Weekly` | `weekly` | Batch weekly |

---

## 5. Entity Lifecycle (State Machines)

### State Machine: `User`

**Description**: User account lifecycle management

**State Field**: `status`

#### States

| State | Description | Entry Actions | Exit Actions |
|-------|-------------|---------------|--------------|
| `pending_verification` | Initial state | Send verification email, log registration | - |
| `active` | Verified user | Send welcome email, set email_verified=true | - |
| `inactive` | Deactivated | Invalidate all sessions, log deactivation | - |
| `suspended` | Banned (final) | Invalidate sessions, send notification, log | - |

#### Transitions

| From State | To State | Trigger/Event | Guard Condition | Actions |
|------------|----------|---------------|-----------------|---------|
| `pending_verification` | `active` | `verify` | email_verified OR verification_code_valid | Send welcome email |
| `pending_verification` | `active` | `activate` | Admin role | - |
| `active` | `inactive` | `deactivate` | Owner or admin | Invalidate sessions |
| `inactive` | `active` | `reactivate` | Owner or admin | - |
| `active`, `inactive` | `suspended` | `suspend` | Admin AND not self | Notify, invalidate sessions |

#### State Diagram

```
[pending_verification] --verify--> [active] --deactivate--> [inactive]
         |                             |                        |
         +---activate------------------>                        |
                                       +<----reactivate---------+
                                       |
                                       +--suspend--> [suspended] (final)
```

---

### State Machine: `Session`

**Description**: JWT session lifecycle

**State Field**: `is_active` (boolean-based)

#### States

| State | Description | Entry Actions | Exit Actions |
|-------|-------------|---------------|--------------|
| `active` | Valid session | Log session creation | - |
| `revoked` | Invalidated (final) | Set revoked_at, log | - |

#### Transitions

| From State | To State | Trigger/Event | Guard Condition | Actions |
|------------|----------|---------------|-----------------|---------|
| `active` | `revoked` | `revoke` | Owner, admin, or system | Set revoked_at |
| `active` | `revoked` | `expire` | expires_at <= now() | System auto-expire |

---

### State Machine: `MFADevice`

**Description**: MFA device lifecycle

**State Field**: `is_active` (composite with verified_at)

#### States

| State | Description | Entry Actions | Exit Actions |
|-------|-------------|---------------|--------------|
| `pending` | Awaiting verification | Log enrollment | - |
| `active` | Verified and active | Set verified_at, send email, log | - |
| `disabled` | Deactivated (final) | Send email, log | - |

#### Transitions

| From State | To State | Trigger/Event | Guard Condition | Actions |
|------------|----------|---------------|-----------------|---------|
| `pending` | `active` | `verify` | OTP valid | Set verified_at |
| `active` | `disabled` | `disable` | Owner or admin | - |
| `pending` | `disabled` | `cancel` | Owner or admin | - |

---

## 6. Workflows (Multi-Step Processes)

### Workflow: `UserOnboarding`

**Description**: Complete user registration with verification

**Trigger**: Event: `UserCreatedEvent`

#### Steps

| Step | Name | Type | Description | On Success | On Failure |
|------|------|------|-------------|------------|------------|
| 1 | `send_verification_email` | action | Send verification email | Step 2 | Retry 3x |
| 2 | `wait_for_verification` | wait | Wait for email click | Step 3 | Timeout 24h |
| 3 | `create_profile` | action | Create user profile | Step 4 | Fail |
| 4 | `create_settings` | action | Create user settings | Step 5 | Fail |
| 5 | `send_welcome_email` | action | Send welcome email | Complete | Log, continue |

### Workflow: `PasswordResetFlow`

**Description**: Password reset with token

**Trigger**: Event: `PasswordResetRequestedEvent`

#### Steps

| Step | Name | Type | Description | On Success | On Failure |
|------|------|------|-------------|------------|------------|
| 1 | `create_reset_token` | action | Generate reset token | Step 2 | Fail |
| 2 | `send_reset_email` | action | Send reset email | Step 3 | Retry 3x |
| 3 | `wait_for_reset` | wait | Wait for password reset | Step 4 | Timeout 1h |
| 4 | `invalidate_sessions` | action | Revoke all sessions | Step 5 | Log, continue |
| 5 | `send_confirmation` | action | Send confirmation email | Complete | Log, continue |

---

## 7. Events (Domain Events)

### Event: `UserCreatedEvent`

**Description**: Published when a new user is registered

**Publisher**: User aggregate

**Trigger**: User entity created

#### Payload

| Field | Type | Description |
|-------|------|-------------|
| `user_id` | uuid | ID of the created user |
| `email` | email | User's email address |
| `username` | string | User's username |
| `occurred_at` | timestamp | Event timestamp |

### Event: `UserLoggedInEvent`

**Description**: Published when user successfully logs in

**Publisher**: AuthenticationService

#### Payload

| Field | Type | Description |
|-------|------|-------------|
| `user_id` | uuid | User ID |
| `session_id` | uuid | New session ID |
| `ip_address` | ip | Client IP |
| `device_type` | DeviceType | Device type |
| `occurred_at` | timestamp | Event timestamp |

### Event: `UserPasswordChangedEvent`

**Description**: Published when user's password is changed

**Publisher**: User aggregate

#### Payload

| Field | Type | Description |
|-------|------|-------------|
| `user_id` | uuid | User ID |
| `reason` | string | Change reason (user_initiated, admin_reset, forgot_password) |
| `occurred_at` | timestamp | Event timestamp |

### Event: `SessionRevokedEvent`

**Description**: Published when a session is revoked

**Publisher**: Session entity

#### Payload

| Field | Type | Description |
|-------|------|-------------|
| `session_id` | uuid | Session ID |
| `user_id` | uuid | User ID |
| `revoked_by` | uuid | Who revoked |
| `reason` | string | Revocation reason |
| `occurred_at` | timestamp | Event timestamp |

### Event: `MFADeviceEnrolledEvent`

**Description**: Published when MFA device is enrolled

**Publisher**: MFADevice entity

#### Payload

| Field | Type | Description |
|-------|------|-------------|
| `user_id` | uuid | User ID |
| `device_type` | MFADeviceType | Device type |
| `device_name` | string | Device name |
| `occurred_at` | timestamp | Event timestamp |

---

## 8. Services (Business Logic)

### Service: `AuthenticationService`

**Description**: User authentication and session management

**Dependencies**: `UserRepository`, `SessionRepository`, `PasswordHasher`, `JwtService`

#### Methods

| Method | Input | Output | Description |
|--------|-------|--------|-------------|
| `login` | `LoginRequest` | `LoginResponse` | Authenticate user, create session |
| `logout` | `SessionId` | `void` | Revoke session |
| `refresh_token` | `RefreshRequest` | `TokenPair` | Refresh JWT tokens |
| `verify_email` | `VerificationToken` | `Result` | Verify email address |

### Service: `AuthorizationService`

**Description**: Permission evaluation and access control

**Dependencies**: `UserRepository`, `RoleRepository`, `PermissionRepository`

#### Methods

| Method | Input | Output | Description |
|--------|-------|--------|-------------|
| `check_permission` | `UserId, Permission` | `bool` | Check if user has permission |
| `get_effective_permissions` | `UserId` | `Vec<Permission>` | Get all user permissions |
| `evaluate_access` | `AccessRequest` | `AccessDecision` | Evaluate ABAC policy |

### Service: `MFAService`

**Description**: Multi-factor authentication orchestration

**Dependencies**: `MFADeviceRepository`, `TotpService`, `SmsService`, `EmailService`

#### Methods

| Method | Input | Output | Description |
|--------|-------|--------|-------------|
| `enroll_device` | `EnrollRequest` | `EnrollResponse` | Enroll new MFA device |
| `verify_otp` | `VerifyRequest` | `VerifyResponse` | Verify one-time password |
| `generate_backup_codes` | `UserId` | `Vec<BackupCode>` | Generate backup codes |

### Service: `WorkflowService`

**Description**: Workflow orchestration and execution

**Dependencies**: `WorkflowRepository`, `WorkflowDefinitionRepository`, `EventBus`

#### Methods

| Method | Input | Output | Description |
|--------|-------|--------|-------------|
| `start_workflow` | `StartRequest` | `Workflow` | Start new workflow |
| `execute_step` | `WorkflowId, StepId` | `StepResult` | Execute workflow step |
| `complete_workflow` | `WorkflowId` | `Result` | Mark workflow complete |

---

## 9. API Requirements

### 9.1 Custom Endpoints

#### Authentication & User Management

| Method | Path | Description | Request Body | Response |
|--------|------|-------------|--------------|----------|
| POST | `/api/v1/auth/login` | User login | `LoginDto` | `TokenPairDto` |
| POST | `/api/v1/auth/logout` | Logout (revoke session) | - | 204 No Content |
| POST | `/api/v1/auth/refresh` | Refresh tokens | `RefreshDto` | `TokenPairDto` |
| POST | `/api/v1/auth/register` | User registration | `RegisterDto` | `UserDto` |
| POST | `/api/v1/auth/verify-email` | Verify email | `VerifyDto` | `Result` |
| POST | `/api/v1/auth/forgot-password` | Request password reset | `ForgotDto` | 202 Accepted |
| POST | `/api/v1/auth/reset-password` | Reset password | `ResetDto` | `Result` |
| POST | `/api/v1/users/{id}/suspend` | Suspend user | `SuspendDto` | `UserDto` |
| POST | `/api/v1/users/{id}/activate` | Activate user | - | `UserDto` |
| POST | `/api/v1/users/{id}/roles` | Assign role | `RoleAssignDto` | `UserRoleDto` |
| DELETE | `/api/v1/users/{id}/roles/{role_id}` | Revoke role | - | 204 No Content |
| POST | `/api/v1/users/{id}/change-password` | Change password 🆕 | `ChangePasswordDto` | `Result` |
| POST | `/api/v1/users/{id}/deactivate` | Deactivate user 🆕 | `ReasonDto` | `UserDto` |

#### MFA Management

| Method | Path | Description | Request Body | Response |
|--------|------|-------------|--------------|----------|
| POST | `/api/v1/mfa/devices` | Enroll MFA device | `EnrollDto` | `MFADeviceDto` |
| POST | `/api/v1/mfa/verify` | Verify MFA | `VerifyMfaDto` | `Result` |
| DELETE | `/api/v1/mfa/devices/{id}` | Remove MFA device 🆕 | - | 204 No Content |
| POST | `/api/v1/mfa/backup-codes` | Generate backup codes 🆕 | - | `BackupCodesDto` |

#### Session & Impersonation 🆕

| Method | Path | Description | Request Body | Response |
|--------|------|-------------|--------------|----------|
| GET | `/api/v1/sessions` | List user's sessions | - | `SessionListDto` |
| DELETE | `/api/v1/sessions/{id}` | Revoke session | - | 204 No Content |
| DELETE | `/api/v1/sessions/revoke-others` | Revoke all other sessions 🆕 | - | 204 No Content |
| POST | `/api/v1/admin/impersonate/{user_id}` | Start impersonation 🆕 | `ReasonDto` | `ImpersonationDto` |
| DELETE | `/api/v1/admin/impersonate/{id}` | End impersonation 🆕 | - | 204 No Content |

#### Permissions & Authorization 🆕

| Method | Path | Description | Request Body | Response |
|--------|------|-------------|--------------|----------|
| POST | `/api/v1/permissions/temporary` | Grant temporary permission 🆕 | `TemporaryPermissionDto` | `PermissionDto` |
| DELETE | `/api/v1/permissions/temporary/{id}` | Revoke temporary permission 🆕 | - | 204 No Content |
| GET | `/api/v1/permissions/effective` | Get effective permissions 🆕 | - | `PermissionListDto` |
| POST | `/api/v1/permissions/resource` | Grant resource permission 🆕 | `ResourcePermissionDto` | `PermissionDto` |

#### Data Export & GDPR 🆕

| Method | Path | Description | Request Body | Response |
|--------|------|-------------|--------------|----------|
| POST | `/api/v1/users/{id}/export` | Request data export 🆕 | `ExportRequestDto` | `DataExportDto` |
| GET | `/api/v1/users/{id}/exports/{id}` | Get export status 🆕 | - | `DataExportDto` |
| GET | `/api/v1/users/{id}/exports/{id}/download` | Download export 🆕 | - | File Download |
| POST | `/api/v1/users/{id}/anonymize` | Anonymize user data 🆕 | `AnonymizeRequestDto` | `AnonymizationDto` |

#### Security & Audit 🆕

| Method | Path | Description | Request Body | Response |
|--------|------|-------------|--------------|----------|
| GET | `/api/v1/security/events` | List security events 🆕 | - | `SecurityEventListDto` |
| GET | `/api/v1/security/events/{id}` | Get security event 🆕 | - | `SecurityEventDto` |
| PUT | `/api/v1/security/events/{id}/resolve` | Resolve security event 🆕 | `ResolutionDto` | `SecurityEventDto` |
| GET | `/api/v1/audit-logs/export` | Export audit logs 🆕 | `ExportQueryDto` | File Download |

#### Notification Preferences 🆕

| Method | Path | Description | Request Body | Response |
|--------|------|-------------|--------------|----------|
| GET | `/api/v1/users/{id}/notification-preferences` | Get preferences 🆕 | - | `NotificationPreferencesDto` |
| PUT | `/api/v1/users/{id}/notification-preferences` | Update preferences 🆕 | `UpdatePreferencesDto` | `NotificationPreferencesDto` |

#### Enterprise SSO 🆕

| Method | Path | Description | Request Body | Response |
|--------|------|-------------|--------------|----------|
| GET | `/api/v1/saml/providers` | List SAML providers 🆕 | - | `SAMLProviderListDto` |
| POST | `/api/v1/saml/providers` | Create SAML provider 🆕 | `SAMLProviderDto` | `SAMLProviderDto` |
| GET | `/api/v1/saml/sso/{id}` | Initiate SAML SSO 🆕 | - | SAML Redirect |
| POST | `/api/v1/saml/acs` | SAML Assertion Consumer 🆕 | SAML Response | Auth Result |
| GET | `/api/v1/ldap/directories` | List LDAP directories 🆕 | - | `LDAPDirectoryListDto` |
| POST | `/api/v1/ldap/directories/{id}/sync` | Trigger LDAP sync 🆕 | - | `SyncResultDto` |

### 9.2 Query Filters

**Entity: `User`**

| Filter | Type | Example | Description |
|--------|------|---------|-------------|
| `status` | enum | `?status=active` | Filter by status |
| `email_verified` | boolean | `?email_verified=true` | Filter by verification |
| `search` | string | `?search=john` | Search username/email |
| `role` | string | `?role=ADMIN` | Filter by role |
| `created_after` | date | `?created_after=2024-01-01` | Creation date filter |

### 9.3 Pagination

| Parameter | Default | Max | Description |
|-----------|---------|-----|-------------|
| `page` | 1 | - | Page number |
| `limit` | 20 | 100 | Items per page |

---

## 10. Authorization & Permissions

### 10.1 Roles

| Role | Description | Permissions |
|------|-------------|-------------|
| `SUPER_ADMIN` | Full system access | All operations |
| `ADMIN` | User management | CRUD users, assign roles, view audit |
| `USER` | Regular user | Read/update own profile, manage own MFA |
| `GUEST` | Unauthenticated | Register, read public data |

### 10.2 Permission Matrix

| Entity | Create | Read | Update | Delete | Special |
|--------|--------|------|--------|--------|---------|
| `User` | admin | admin (all), user (own) | admin (all), user (own) | admin | suspend: admin, change_password: owner |
| `Role` | super_admin | admin | super_admin | super_admin | assign: admin |
| `Permission` | super_admin | admin | super_admin | super_admin | - |
| `Session` | system | owner, admin | - | - | revoke: owner, admin |
| `MFADevice` | owner | owner, admin | owner (name only) | - | disable: owner, admin |
| `AuditLog` | system | admin (all), user (own) | - | - | - |

### 10.3 Row-Level Security

| Entity | Rule | Description |
|--------|------|-------------|
| `User` | `id = $actor.id OR $actor.has_role('ADMIN')` | Users see self, admins see all |
| `Session` | `user_id = $actor.id OR $actor.has_role('ADMIN')` | Users see own sessions |
| `AuditLog` | `user_id = $actor.id OR $actor.has_role('ADMIN')` | Users see own logs |
| `MFADevice` | `user_id = $actor.id OR $actor.has_role('ADMIN')` | Users manage own devices |

---

## 11. Integrations

### 11.1 External APIs

| System | Purpose | Auth Type | Endpoints Used |
|--------|---------|-----------|----------------|
| Google OAuth | Social login | OAuth2 | authorization, token, userinfo |
| GitHub OAuth | Social login | OAuth2 | authorization, token, userinfo |
| Twilio | SMS MFA | API Key | messages/send |
| SendGrid | Email notifications | API Key | mail/send |

### 11.2 Webhooks (Outgoing)

| Event | URL Pattern | Payload | Retry Policy |
|-------|-------------|---------|--------------|
| `UserCreatedEvent` | `{config.webhook_url}` | `UserCreatedPayload` | 3 retries, exponential backoff |
| `SecurityAlertEvent` | `{config.security_webhook}` | `SecurityAlertPayload` | 5 retries, immediate |

---

## 12. Non-Functional Requirements

### 12.1 Performance

| Metric | Requirement | Notes |
|--------|-------------|-------|
| Login response time (P95) | < 200ms | Including password hashing |
| Token validation (P95) | < 10ms | JWT verification |
| Permission check (P95) | < 50ms | With caching |
| Throughput | > 1000 req/s | Authentication endpoints |

### 12.2 Data Retention

| Entity | Retention Period | Archive Strategy |
|--------|------------------|------------------|
| `User` | Indefinite | Soft delete |
| `Session` | 90 days | Hard delete after expiry |
| `AuditLog` | 365 days | Archive to S3 after 90 days |
| `PasswordResetToken` | 24 hours | Hard delete after use/expiry |
| `Notification` | 90 days | Soft delete |

### 12.3 Audit Requirements

| Entity | Audit Level | Fields to Track |
|--------|-------------|-----------------|
| `User` | full | All field changes |
| `Role` | full | All changes |
| `Permission` | full | All changes |
| `UserRole` | full | Assignment/revocation |
| `Session` | basic | Creation/revocation |

---

## 13. Seed Data

### 13.1 Reference Data

**Entity: `Role`**

| id | name | description | is_default |
|----|------|-------------|------------|
| `00000000-0000-0000-0000-000000000001` | `SUPER_ADMIN` | Full system access | false |
| `00000000-0000-0000-0000-000000000002` | `ADMIN` | Administrative access | false |
| `00000000-0000-0000-0000-000000000003` | `USER` | Standard user | true |

**Entity: `Permission`**

| id | name | resource | action |
|----|------|----------|--------|
| `...` | `admin:*` | `*` | `admin` |
| `...` | `read:users` | `users` | `read` |
| `...` | `write:users` | `users` | `write` |
| `...` | `delete:users` | `users` | `delete` |
| `...` | `read:roles` | `roles` | `read` |
| `...` | `assign:roles` | `roles` | `assign` |
| `...` | `read:audit` | `audit_logs` | `read` |

### 13.2 Test Data

- 5 test users (admin, user1-4)
- 3 roles (SUPER_ADMIN, ADMIN, USER)
- 20 permissions (full CRUD for main entities)
- Sample sessions for each user
- Sample MFA devices

---

## 14. Migration Notes

### 14.1 Breaking Changes

| Change | Impact | Migration Path |
|--------|--------|----------------|
| Organization moved to corpus | OrganizationUser references corpus.Organization | Update foreign keys |
| Profile separated from User | Profile fields moved | Migrate profile data |

### 14.2 New Features (v2.0)

| Feature | Description | Status |
|---------|-------------|--------|
| Password History | Track last N passwords to prevent reuse | 🆕 New |
| Resource Permissions | Fine-grained resource-level access control | 🆕 New |
| Temporary Permissions | Time-bound permission grants | 🆕 New |
| Session Limits | Per-user concurrent session limits | 🆕 New |
| Data Export | GDPR data export functionality | 🆕 New |
| User Anonymization | Right to be forgotten implementation | 🆕 New |
| Impersonation | Admin impersonation for support | 🆕 New |
| Security Events | Enhanced security monitoring | 🆕 New |
| Permission Cache | Performance optimization for permission checks | 🆕 New |
| Notification Preferences | Per-user notification settings | 🆕 New |
| SAML 2.0 | Enterprise SSO support | 🆕 New |
| LDAP/AD | Corporate directory integration | 🆕 New |

---

## Appendix A: Glossary

| Term | Definition |
|------|------------|
| MFA | Multi-Factor Authentication - additional verification beyond password |
| TOTP | Time-based One-Time Password - algorithm for generating OTPs |
| RBAC | Role-Based Access Control - permission via role membership |
| ABAC | Attribute-Based Access Control - permission via attribute evaluation |
| JWT | JSON Web Token - stateless authentication token |
| Argon2id | Password hashing algorithm (recommended) |
| SAML | Security Assertion Markup Language - enterprise SSO protocol |
| LDAP | Lightweight Directory Access Protocol - corporate directory protocol |
| GDPR | General Data Protection Regulation - EU data protection law |

---

## Checklist Before Submission

- [x] All entities have complete field definitions
- [x] All relationships are defined with foreign keys
- [x] All enums are listed with variants
- [x] Validation rules are specified
- [x] Use cases cover main business flows
- [x] State machines defined for entities with lifecycle
- [x] Events defined for cross-module communication
- [x] Authorization requirements specified
- [x] Seed data requirements documented

---

## Notes for Schema Conversion

When converting this spec to Backbone schema:

1. **Entities** -> `schema/models/{entity}.model.yaml`
2. **Enums** -> Defined within model files or shared `index.model.yaml`
3. **State Machines** -> `schema/hooks/{entity}.hook.yaml`
4. **Workflows** -> `schema/workflows/{workflow}.workflow.yaml`
5. **API Extensions** -> `schema/openapi/{entity}.openapi.yaml`

Run validation after conversion:
```bash
backbone schema validate sapiens
backbone schema generate --target all sapiens
cargo check -p backbone-sapiens
```
