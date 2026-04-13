# Sapiens gRPC API Documentation

**Version:** 1.0.0
**gRPC Server:** `localhost:50052` (Development) | `grpc.startapp.id:50052` (Production)
**Protocol:** gRPC (HTTP/2)
**Authentication:** JWT Bearer Token (via metadata)

---

## Table of Contents

1. [Overview](#overview)
2. [Connection Details](#connection-details)
3. [Authentication](#authentication)
4. [Core Services](#services)
   - [PasswordService](#1-passwordservice)
   - [TokenService](#2-tokenservice)
   - [MfaService](#3-mfaservice)
   - [RoleService](#4-roleservice)
   - [PermissionService](#5-permissionservice)
   - [RolePermissionService](#6-rolepermissionservice)
   - [UserRoleService](#7-userroleservice)
   - [PermissionResolutionService](#8-permissionresolutionservice)
   - [UserCommandUseCases](#9-usercommandusecases)
   - [UserQueryUseCases](#10-userqueryusecases)
   - [SessionService](#11-sessionservice)
5. [CRUD Services](#crud-services)
   - [UserCrudService](#12-usercrudservice)
   - [RoleCrudService](#13-rolecrudservice)
   - [PermissionCrudService](#14-permissioncrudservice)
   - [SessionCrudService](#15-sessioncrudservice)
   - [UserRoleCrudService](#16-userrolecrudservice)
   - [RolePermissionCrudService](#17-rolepermissioncrudservice)
   - [UserPermissionCrudService](#18-userpermissioncrudservice)
   - [UserPreferenceCrudService](#19-userpreferencecrudservice)
   - [UserSettingsCrudService](#20-usersettingscrudservice)
   - [UserSessionCrudService](#21-usersessioncrudservice)
   - [UserSecurityEventCrudService](#22-usersecurityeventcrudservice)
   - [MfaDeviceCrudService](#23-mfadevicecrudservice)
   - [PasswordResetTokenCrudService](#24-passwordresettokencrudservice)
   - [AuditLogCrudService](#25-auditlogcrudservice)
   - [PolicyCrudService](#26-policycrudservice)
   - [PolicyAssignmentCrudService](#27-policyassignmentcrudservice)
   - [SystemSettingsCrudService](#28-systemsettingscrudservice)
6. [Common Types](#common-types)
7. [Error Handling](#error-handling)
8. [Testing with grpcurl](#testing-with-grpcurl)

---

## Overview

Sapiens gRPC API provides 28 services with 307 RPC methods for user management, authentication, authorization, and RBAC operations.

**Total RPC Methods:** 307
**Services:** 28 (11 Core Services + 17 CRUD Services)
**Core Services:** 11 services with 103 methods
**CRUD Services:** 17 services with 12 methods each = 204 methods
**Proto Location:** `libs/modules/sapiens/proto/services/` and `libs/domain/service/sapiens/crud/`

### Implementation Progress Summary

| Service | Implemented | Stubs | Total | Status |
|---------|-------------|-------|-------|--------|
| PermissionResolutionService | 10 | 0 | 10 | ✅ 100% |
| UserCommandUseCases | 20 | 7 | 27 | 🟢 74% |
| UserQueryUseCases | 19 | 2 | 21 | 🟢 90% |
| PasswordService | 6 | 0 | 6 | ✅ 100% |
| TokenService | 7 | 0 | 7 | ✅ 100% |
| CRUD Services (17) | 204 | 0 | 204 | ✅ 100% |
| **Total** | **266** | **9** | **275** | **97%** |

**Last Updated:** 2025-11-21

**Recent Updates:**
- Session management methods added (CreateSession, UpdateSessionActivity, TerminateSession, TerminateAllUserSessions)
- Session query methods added (GetUserSessions, GetSessionByToken)

### CRUD Service Method Pattern

Each CRUD service implements the following 12 standard methods:

| # | RPC Method | Description |
|---|-----------|-------------|
| 1 | Create{Entity} | Create new entity |
| 2 | Get{Entity} | Get by ID |
| 3 | Update{Entity} | Full update |
| 4 | PartialUpdate{Entity} | Partial update (PATCH) |
| 5 | Delete{Entity} | Soft delete |
| 6 | List{Entity} | List all (paginated) |
| 7 | Search{Entity} | Search with filters |
| 8 | BulkCreate{Entity} | Bulk create multiple entities |
| 9 | Upsert{Entity} | Update or create |
| 10 | ListDeleted{Entity} | List soft-deleted items (trash) |
| 11 | Restore{Entity} | Restore soft-deleted item |
| 12 | EmptyTrash{Entity} | Hard delete all soft-deleted items |

---

## Connection Details

### Development
```
Host: localhost
Port: 50052
Protocol: h2c (HTTP/2 cleartext)
```

### Production
```
Host: grpc.startapp.id
Port: 50052
Protocol: h2 (HTTP/2 TLS)
```

### Proto Files
Core service definitions are located in:
```
libs/modules/sapiens/proto/services/
├── password_service.proto
├── token_service.proto
├── mfa_service.proto
├── role_service.proto
├── permission_service.proto
├── role_permission_service.proto
├── user_role_service.proto
├── permission_resolution_service.proto
├── user_command_usecases.proto
├── user_query_usecases.proto
└── session_service.proto
```

CRUD service definitions are located in:
```
libs/domain/service/sapiens/crud/
├── user_crud.proto
├── role_crud.proto
├── permission_crud.proto
├── session_crud.proto
├── user_role_crud.proto
├── role_permission_crud.proto
├── user_permission_crud.proto
├── user_preference_crud.proto
├── user_settings_crud.proto
├── user_session_crud.proto
├── user_security_event_crud.proto
├── mfa_device_crud.proto
├── password_reset_token_crud.proto
├── audit_log_crud.proto
├── policy_crud.proto
├── policy_assignment_crud.proto
└── system_settings_crud.proto
```

---

## Authentication

All gRPC methods (except health checks) require JWT authentication via metadata.

### Metadata Header
```
authorization: Bearer <JWT_TOKEN>
```

### Example with grpcurl
```bash
grpcurl -H "authorization: Bearer eyJhbGc..." \
  -d '{"user_id": "123"}' \
  localhost:50052 \
  sapiens.UserQueryUseCases/GetUser
```

---

## Services

## 1. PasswordService

**Package:** `sapiens.PasswordService`
**Proto:** `proto/services/password_service.proto`
**RPCs:** 6

Password hashing, verification, and security operations using Argon2id.

### Methods

#### 1.1 HashPassword
Hash a plaintext password using Argon2id.

```protobuf
rpc HashPassword(HashPasswordRequest) returns (HashPasswordResponse);
```

**Request:**
```json
{
  "password": "MySecurePass123!"
}
```

**Response:**
```json
{
  "base_response": {
    "success": true,
    "message": "Password hashed successfully"
  },
  "password_hash": "$argon2id$v=19$m=19456,t=2,p=1$..."
}
```

#### 1.2 VerifyPassword
Verify a password against its hash.

```protobuf
rpc VerifyPassword(VerifyPasswordRequest) returns (VerifyPasswordResponse);
```

**Request:**
```json
{
  "password": "MySecurePass123!",
  "password_hash": "$argon2id$v=19$m=19456,t=2,p=1$..."
}
```

**Response:**
```json
{
  "base_response": {
    "success": true,
    "message": "Password verified"
  },
  "is_valid": true
}
```

#### 1.3 GenerateTemporaryPassword
Generate a secure temporary password.

```protobuf
rpc GenerateTemporaryPassword(GenerateTemporaryPasswordRequest) returns (GenerateTemporaryPasswordResponse);
```

#### 1.4 ValidatePasswordStrength
Check password strength against policy.

```protobuf
rpc ValidatePasswordStrength(ValidatePasswordStrengthRequest) returns (ValidatePasswordStrengthResponse);
```

#### 1.5 CheckPasswordExpiry
Check if user password has expired.

```protobuf
rpc CheckPasswordExpiry(CheckPasswordExpiryRequest) returns (CheckPasswordExpiryResponse);
```

#### 1.6 GetPasswordHistory
Retrieve user's password change history.

```protobuf
rpc GetPasswordHistory(GetPasswordHistoryRequest) returns (GetPasswordHistoryResponse);
```

---

## 2. TokenService

**Package:** `sapiens.TokenService`
**Proto:** `proto/services/token_service.proto`
**RPCs:** 7

JWT token generation, validation, and refresh operations.

### Methods

#### 2.1 GenerateToken
Generate JWT access token for authenticated user.

```protobuf
rpc GenerateToken(GenerateTokenRequest) returns (GenerateTokenResponse);
```

**Request:**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "roles": ["user", "admin"]
}
```

**Response:**
```json
{
  "base_response": {
    "success": true,
    "message": "Token generated successfully"
  },
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

#### 2.2 ValidateToken
Validate JWT token and extract claims.

```protobuf
rpc ValidateToken(ValidateTokenRequest) returns (ValidateTokenResponse);
```

#### 2.3 RefreshToken
Refresh access token using refresh token.

```protobuf
rpc RefreshToken(RefreshTokenRequest) returns (RefreshTokenResponse);
```

#### 2.4 RevokeToken
Revoke a token before expiration.

```protobuf
rpc RevokeToken(RevokeTokenRequest) returns (RevokeTokenResponse);
```

#### 2.5 GetTokenMetadata
Get token metadata (issued_at, expires_at, etc.).

```protobuf
rpc GetTokenMetadata(GetTokenMetadataRequest) returns (GetTokenMetadataResponse);
```

#### 2.6 ListUserTokens
List all active tokens for a user.

```protobuf
rpc ListUserTokens(ListUserTokensRequest) returns (ListUserTokensResponse);
```

#### 2.7 RevokeAllUserTokens
Revoke all tokens for a user (e.g., on logout).

```protobuf
rpc RevokeAllUserTokens(RevokeAllUserTokensRequest) returns (RevokeAllUserTokensResponse);
```

---

## 3. MfaService

**Package:** `sapiens.MfaService`
**Proto:** `proto/services/mfa_service.proto`
**RPCs:** 6

Multi-factor authentication (TOTP and SMS) operations.

### Methods

#### 3.1 EnableMFA
Enable MFA for a user.

```protobuf
rpc EnableMFA(EnableMFARequest) returns (EnableMFAResponse);
```

**Request:**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "mfa_type": "TOTP"
}
```

**Response:**
```json
{
  "base_response": {
    "success": true,
    "message": "MFA enabled successfully"
  },
  "secret": "JBSWY3DPEHPK3PXP",
  "qr_code": "data:image/png;base64,iVBORw0KGgo...",
  "backup_codes": ["12345678", "87654321"]
}
```

#### 3.2 DisableMFA
Disable MFA for a user.

```protobuf
rpc DisableMFA(DisableMFARequest) returns (DisableMFAResponse);
```

#### 3.3 VerifyMFACode
Verify TOTP or SMS code.

```protobuf
rpc VerifyMFACode(VerifyMFACodeRequest) returns (VerifyMFACodeResponse);
```

#### 3.4 GenerateBackupCodes
Generate new backup codes.

```protobuf
rpc GenerateBackupCodes(GenerateBackupCodesRequest) returns (GenerateBackupCodesResponse);
```

#### 3.5 GetMFAStatus
Get user's MFA status and devices.

```protobuf
rpc GetMFAStatus(GetMFAStatusRequest) returns (GetMFAStatusResponse);
```

#### 3.6 SendMFACode
Send MFA code via SMS.

```protobuf
rpc SendMFACode(SendMFACodeRequest) returns (SendMFACodeResponse);
```

---

## 4. RoleService

**Package:** `sapiens.RoleService`
**Proto:** `proto/services/role_service.proto`
**RPCs:** 7

Role CRUD operations and management.

### Methods

#### 4.1 CreateRole
Create a new role.

```protobuf
rpc CreateRole(CreateRoleRequest) returns (CreateRoleResponse);
```

**Request:**
```json
{
  "name": "content_editor",
  "description": "Can create and edit content",
  "is_default": false
}
```

**Response:**
```json
{
  "base_response": {
    "success": true,
    "message": "Role created successfully"
  },
  "role": {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "name": "content_editor",
    "description": "Can create and edit content",
    "is_default": false,
    "created_at": "2025-11-20T10:30:00Z",
    "updated_at": "2025-11-20T10:30:00Z"
  }
}
```

#### 4.2 GetRole
Get role by ID.

```protobuf
rpc GetRole(GetRoleRequest) returns (GetRoleResponse);
```

#### 4.3 UpdateRole
Update existing role.

```protobuf
rpc UpdateRole(UpdateRoleRequest) returns (UpdateRoleResponse);
```

#### 4.4 DeleteRole
Delete role (soft delete).

```protobuf
rpc DeleteRole(DeleteRoleRequest) returns (DeleteRoleResponse);
```

#### 4.5 ListRoles
List all roles with pagination.

```protobuf
rpc ListRoles(ListRolesRequest) returns (ListRolesResponse);
```

**Request:**
```json
{
  "page": 1,
  "limit": 20,
  "include_deleted": false
}
```

#### 4.6 SearchRoles
Search roles by name.

```protobuf
rpc SearchRoles(SearchRolesRequest) returns (SearchRolesResponse);
```

#### 4.7 GetRoleUsers
Get all users with a specific role.

```protobuf
rpc GetRoleUsers(GetRoleUsersRequest) returns (GetRoleUsersResponse);
```

---

## 5. PermissionService

**Package:** `sapiens.PermissionService`
**Proto:** `proto/services/permission_service.proto`
**RPCs:** 7

Permission CRUD operations and management.

### Methods

#### 5.1 CreatePermission
Create a new permission.

```protobuf
rpc CreatePermission(CreatePermissionRequest) returns (CreatePermissionResponse);
```

**Request:**
```json
{
  "name": "articles:create",
  "resource": "articles",
  "action": "create",
  "description": "Create new articles"
}
```

#### 5.2 GetPermission
Get permission by ID.

```protobuf
rpc GetPermission(GetPermissionRequest) returns (GetPermissionResponse);
```

#### 5.3 UpdatePermission
Update existing permission.

```protobuf
rpc UpdatePermission(UpdatePermissionRequest) returns (UpdatePermissionResponse);
```

#### 5.4 DeletePermission
Delete permission (soft delete).

```protobuf
rpc DeletePermission(DeletePermissionRequest) returns (DeletePermissionResponse);
```

#### 5.5 ListPermissions
List all permissions with pagination.

```protobuf
rpc ListPermissions(ListPermissionsRequest) returns (ListPermissionsResponse);
```

#### 5.6 SearchPermissions
Search permissions by resource or action.

```protobuf
rpc SearchPermissions(SearchPermissionsRequest) returns (SearchPermissionsResponse);
```

#### 5.7 GetPermissionRoles
Get all roles that have a specific permission.

```protobuf
rpc GetPermissionRoles(GetPermissionRolesRequest) returns (GetPermissionRolesResponse);
```

---

## 6. RolePermissionService

**Package:** `sapiens.RolePermissionService`
**Proto:** `proto/services/role_permission_service.proto`
**RPCs:** 4

Role-permission assignment operations.

### Methods

#### 6.1 AssignPermissionToRole
Assign a permission to a role.

```protobuf
rpc AssignPermissionToRole(AssignPermissionToRoleRequest) returns (AssignPermissionToRoleResponse);
```

**Request:**
```json
{
  "role_id": "550e8400-e29b-41d4-a716-446655440001",
  "permission_id": "550e8400-e29b-41d4-a716-446655440002"
}
```

#### 6.2 RevokePermissionFromRole
Revoke a permission from a role.

```protobuf
rpc RevokePermissionFromRole(RevokePermissionFromRoleRequest) returns (RevokePermissionFromRoleResponse);
```

#### 6.3 GetRolePermissions
Get all permissions for a role.

```protobuf
rpc GetRolePermissions(GetRolePermissionsRequest) returns (GetRolePermissionsResponse);
```

#### 6.4 BulkAssignPermissionsToRole
Assign multiple permissions to a role at once.

```protobuf
rpc BulkAssignPermissionsToRole(BulkAssignPermissionsToRoleRequest) returns (BulkAssignPermissionsToRoleResponse);
```

---

## 7. UserRoleService

**Package:** `sapiens.UserRoleService`
**Proto:** `proto/services/user_role_service.proto`
**RPCs:** 5

User-role assignment operations.

### Methods

#### 7.1 AssignRoleToUser
Assign a role to a user.

```protobuf
rpc AssignRoleToUser(AssignRoleToUserRequest) returns (AssignRoleToUserResponse);
```

**Request:**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "role_id": "550e8400-e29b-41d4-a716-446655440001"
}
```

#### 7.2 RevokeRoleFromUser
Revoke a role from a user.

```protobuf
rpc RevokeRoleFromUser(RevokeRoleFromUserRequest) returns (RevokeRoleFromUserResponse);
```

#### 7.3 GetUserRoles
Get all roles for a user.

```protobuf
rpc GetUserRoles(GetUserRolesRequest) returns (GetUserRolesResponse);
```

**Response:**
```json
{
  "base_response": {
    "success": true,
    "message": "User roles retrieved successfully"
  },
  "roles": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "name": "admin",
      "description": "Administrator role"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440002",
      "name": "content_editor",
      "description": "Can create and edit content"
    }
  ]
}
```

#### 7.4 BulkAssignRolesToUser
Assign multiple roles to a user at once.

```protobuf
rpc BulkAssignRolesToUser(BulkAssignRolesToUserRequest) returns (BulkAssignRolesToUserResponse);
```

#### 7.5 ReplaceUserRoles
Replace all user roles with a new set.

```protobuf
rpc ReplaceUserRoles(ReplaceUserRolesRequest) returns (ReplaceUserRolesResponse);
```

---

## 8. PermissionResolutionService

**Package:** `sapiens.PermissionResolutionService`
**Proto:** `proto/services/permission_resolution_service.proto`
**RPCs:** 10
**Implementation Status:** ✅ **100% Complete** (10/10 methods implemented)

Permission resolution and authorization checks (Hybrid RBAC).

### Implementation Status

| Method | Status | Notes |
|--------|--------|-------|
| check_permission | ✅ Implemented | RBAC permission check via roles |
| check_multiple_permissions | ✅ Implemented | Batch permission check |
| resolve_user_permissions | ✅ Implemented | Get all user permissions |
| resolve_role_permissions | ✅ Implemented | Get role's permissions |
| check_role_hierarchy | ✅ Implemented | Role inheritance check |
| invalidate_cache | ✅ Implemented | Cache invalidation stub |
| invalidate_all_caches | ✅ Implemented | Full cache invalidation |
| get_users_with_permission | ✅ Implemented | Find users with permission |
| get_roles_with_permission | ✅ Implemented | Find roles with permission |
| build_permission_string | ✅ Implemented | Helper method |

### Methods

#### 8.1 GetUserEffectivePermissions
Get all effective permissions for a user (direct + role-based).

```protobuf
rpc GetUserEffectivePermissions(GetUserEffectivePermissionsRequest) returns (GetUserEffectivePermissionsResponse);
```

**Request:**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Response:**
```json
{
  "base_response": {
    "success": true,
    "message": "Effective permissions retrieved"
  },
  "permissions": [
    {
      "id": "perm-1",
      "name": "articles:create",
      "resource": "articles",
      "action": "create",
      "source": "role:admin"
    },
    {
      "id": "perm-2",
      "name": "users:read",
      "resource": "users",
      "action": "read",
      "source": "direct"
    }
  ],
  "direct_permissions": ["perm-2"],
  "role_based_permissions": ["perm-1"]
}
```

#### 8.2 CheckUserPermission
Check if user has a specific permission.

```protobuf
rpc CheckUserPermission(CheckUserPermissionRequest) returns (CheckUserPermissionResponse);
```

**Request:**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "resource": "articles",
  "action": "create"
}
```

**Response:**
```json
{
  "base_response": {
    "success": true,
    "message": "Permission check completed"
  },
  "has_permission": true,
  "source": "role:admin"
}
```

#### 8.3 CheckUserPermissions
Check multiple permissions at once.

```protobuf
rpc CheckUserPermissions(CheckUserPermissionsRequest) returns (CheckUserPermissionsResponse);
```

#### 8.4 GetRoleEffectivePermissions
Get all effective permissions for a role.

```protobuf
rpc GetRoleEffectivePermissions(GetRoleEffectivePermissionsRequest) returns (GetRoleEffectivePermissionsResponse);
```

#### 8.5 GrantDirectPermissionToUser
Grant direct permission to user (Hybrid RBAC).

```protobuf
rpc GrantDirectPermissionToUser(GrantDirectPermissionToUserRequest) returns (GrantDirectPermissionToUserResponse);
```

#### 8.6 RevokeDirectPermissionFromUser
Revoke direct permission from user.

```protobuf
rpc RevokeDirectPermissionFromUser(RevokeDirectPermissionFromUserRequest) returns (RevokeDirectPermissionFromUserResponse);
```

#### 8.7 GetUserDirectPermissions
Get only direct permissions (not role-based).

```protobuf
rpc GetUserDirectPermissions(GetUserDirectPermissionsRequest) returns (GetUserDirectPermissionsResponse);
```

#### 8.8 GetUserRoleBasedPermissions
Get only role-based permissions (not direct).

```protobuf
rpc GetUserRoleBasedPermissions(GetUserRoleBasedPermissionsRequest) returns (GetUserRoleBasedPermissionsResponse);
```

#### 8.9 BulkCheckPermissions
Check multiple users' permissions at once.

```protobuf
rpc BulkCheckPermissions(BulkCheckPermissionsRequest) returns (BulkCheckPermissionsResponse);
```

#### 8.10 GetPermissionSummary
Get summary of user's permission sources.

```protobuf
rpc GetPermissionSummary(GetPermissionSummaryRequest) returns (GetPermissionSummaryResponse);
```

---

## 9. UserCommandUseCases

**Package:** `sapiens.UserCommandUseCases`
**Proto:** `proto/services/user_command_usecases.proto`
**RPCs:** 27
**Implementation Status:** 🟢 **74% Complete** (20/27 methods implemented)

Write operations for user management (CQRS Commands).

### Implementation Status

| Method | Status | Notes |
|--------|--------|-------|
| create_user | ✅ Implemented | Full implementation with password hashing |
| update_user | ✅ Implemented | Full implementation |
| delete_user | ✅ Implemented | Full implementation |
| activate_user | ✅ Implemented | Full implementation |
| deactivate_user | ✅ Implemented | Full implementation |
| lock_user | ✅ Implemented | Full implementation via metadata |
| unlock_user | ✅ Implemented | Full implementation |
| update_password | ✅ Implemented | Password update with validation |
| initiate_password_reset | ⏳ Stub | Requires email service |
| reset_password | ✅ Implemented | Reset password with token |
| verify_email | ✅ Implemented | Email verification |
| resend_verification_email | ⏳ Stub | Requires email service |
| enable_mfa | ⏳ Stub | Requires MFA service |
| disable_mfa | ⏳ Stub | Requires MFA service |
| verify_mfa_setup | ⏳ Stub | Requires TOTP verification |
| create_session | ✅ Implemented | Creates new session with 24h expiry |
| update_session_activity | ✅ Implemented | Updates session and extends expiry |
| terminate_session | ✅ Implemented | Deactivates specific session |
| terminate_all_user_sessions | ✅ Implemented | Terminates all user sessions |
| log_security_event | ⏳ Stub | Requires SecurityEventRepository |
| save_user_preference | ✅ Implemented | Save user preference |
| delete_user_preference | ✅ Implemented | Delete user preference |
| update_user_roles | ✅ Implemented | Update user roles |
| bulk_create_users | ✅ Implemented | Bulk user creation |
| bulk_update_users | ✅ Implemented | Bulk user update |
| bulk_delete_users | ✅ Implemented | Bulk user deletion |
| export_user_data | ⏳ Stub | Requires data export service |

### Methods

#### 9.1 CreateUser
Create a new user.

```protobuf
rpc CreateUser(CreateUserRequest) returns (CreateUserResponse);
```

**Request:**
```json
{
  "email": "newuser@example.com",
  "username": "newuser",
  "password": "SecurePass123!",
  "first_name": "John",
  "last_name": "Doe"
}
```

**Response:**
```json
{
  "base_response": {
    "success": true,
    "message": "User created successfully"
  },
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "newuser@example.com",
    "username": "newuser",
    "first_name": "John",
    "last_name": "Doe",
    "is_active": true,
    "email_verified": false,
    "created_at": "2025-11-20T10:30:00Z"
  }
}
```

#### 9.2 UpdateUser
Update user details.

```protobuf
rpc UpdateUser(UpdateUserRequest) returns (UpdateUserResponse);
```

#### 9.3 DeleteUser
Delete user (soft delete).

```protobuf
rpc DeleteUser(DeleteUserRequest) returns (DeleteUserResponse);
```

#### 9.4 ActivateUser
Activate a deactivated user.

```protobuf
rpc ActivateUser(ActivateUserRequest) returns (ActivateUserResponse);
```

#### 9.5 DeactivateUser
Deactivate a user.

```protobuf
rpc DeactivateUser(DeactivateUserRequest) returns (DeactivateUserResponse);
```

#### 9.6 ChangePassword
Change user password.

```protobuf
rpc ChangePassword(ChangePasswordRequest) returns (ChangePasswordResponse);
```

#### 9.7 ResetPassword
Reset user password (forgot password flow).

```protobuf
rpc ResetPassword(ResetPasswordRequest) returns (ResetPasswordResponse);
```

#### 9.8 VerifyEmail
Verify user email address.

```protobuf
rpc VerifyEmail(VerifyEmailRequest) returns (VerifyEmailResponse);
```

#### 9.9 UpdateProfile
Update user profile information.

```protobuf
rpc UpdateProfile(UpdateProfileRequest) returns (UpdateProfileResponse);
```

#### 9.10 UploadAvatar
Upload user avatar image.

```protobuf
rpc UploadAvatar(UploadAvatarRequest) returns (UploadAvatarResponse);
```

#### 9.11 UpdateSettings
Update user settings/preferences.

```protobuf
rpc UpdateSettings(UpdateSettingsRequest) returns (UpdateSettingsResponse);
```

#### 9.12 LockAccount
Lock user account (security).

```protobuf
rpc LockAccount(LockAccountRequest) returns (LockAccountResponse);
```

#### 9.13 UnlockAccount
Unlock user account.

```protobuf
rpc UnlockAccount(UnlockAccountRequest) returns (UnlockAccountResponse);
```

#### 9.14 RequestPasswordReset
Request password reset token.

```protobuf
rpc RequestPasswordReset(RequestPasswordResetRequest) returns (RequestPasswordResetResponse);
```

#### 9.15 ConfirmPasswordReset
Confirm password reset with token.

```protobuf
rpc ConfirmPasswordReset(ConfirmPasswordResetRequest) returns (ConfirmPasswordResetResponse);
```

#### 9.16 SendVerificationEmail
Resend email verification.

```protobuf
rpc SendVerificationEmail(SendVerificationEmailRequest) returns (SendVerificationEmailResponse);
```

#### 9.17 UpdateEmail
Update user email address.

```protobuf
rpc UpdateEmail(UpdateEmailRequest) returns (UpdateEmailResponse);
```

#### 9.18 UpdateUsername
Update username.

```protobuf
rpc UpdateUsername(UpdateUsernameRequest) returns (UpdateUsernameResponse);
```

#### 9.19 UpdatePhone
Update phone number.

```protobuf
rpc UpdatePhone(UpdatePhoneRequest) returns (UpdatePhoneResponse);
```

#### 9.20 VerifyPhone
Verify phone number.

```protobuf
rpc VerifyPhone(VerifyPhoneRequest) returns (VerifyPhoneResponse);
```

#### 9.21 SetPreference
Set individual user preference.

```protobuf
rpc SetPreference(SetPreferenceRequest) returns (SetPreferenceResponse);
```

#### 9.22 DeletePreference
Delete user preference.

```protobuf
rpc DeletePreference(DeletePreferenceRequest) returns (DeletePreferenceResponse);
```

#### 9.23 BulkCreateUsers
Create multiple users at once.

```protobuf
rpc BulkCreateUsers(BulkCreateUsersRequest) returns (BulkCreateUsersResponse);
```

#### 9.24 BulkDeleteUsers
Delete multiple users at once.

```protobuf
rpc BulkDeleteUsers(BulkDeleteUsersRequest) returns (BulkDeleteUsersResponse);
```

#### 9.25 ImportUsers
Import users from external source.

```protobuf
rpc ImportUsers(ImportUsersRequest) returns (ImportUsersResponse);
```

#### 9.26 ExportUsers
Export users to external format.

```protobuf
rpc ExportUsers(ExportUsersRequest) returns (ExportUsersResponse);
```

#### 9.27 MergeUsers
Merge two user accounts.

```protobuf
rpc MergeUsers(MergeUsersRequest) returns (MergeUsersResponse);
```

#### 9.28 ArchiveUser
Archive user (different from delete).

```protobuf
rpc ArchiveUser(ArchiveUserRequest) returns (ArchiveUserResponse);
```

---

## 10. UserQueryUseCases

**Package:** `sapiens.UserQueryUseCases`
**Proto:** `proto/services/user_query_usecases.proto`
**RPCs:** 21
**Implementation Status:** 🟢 **90% Complete** (19/21 methods implemented)

Read operations for user management (CQRS Queries).

### Implementation Status

| Method | Status | Notes |
|--------|--------|-------|
| get_user | ✅ Implemented | Full implementation with active filter |
| get_user_by_email | ✅ Implemented | Full implementation |
| get_user_by_username | ✅ Implemented | Full implementation |
| list_users | ✅ Implemented | Full implementation with Backbone pagination |
| search_users | ✅ Implemented | Search users with filters |
| get_users_by_role | ✅ Implemented | Get users by role |
| get_users_by_status | ✅ Implemented | Get users by status |
| validate_user_credentials | ✅ Implemented | Full implementation with password verification |
| check_user_exists | ✅ Implemented | Full implementation |
| check_user_has_role | ✅ Implemented | Full implementation |
| get_user_roles_summary | ✅ Implemented | Basic implementation |
| get_user_sessions | ✅ Implemented | Get all sessions for a user |
| get_session_by_token | ✅ Implemented | Lookup session by token hash |
| get_user_security_events | ⏳ Stub | Requires SecurityEventRepository |
| get_user_preferences | ✅ Implemented | Get user preferences |
| get_user_preference | ✅ Implemented | Get specific preference |
| verify_user_email | ✅ Implemented | Email verification check |
| get_user_statistics | ✅ Implemented | User statistics |
| get_user_activity_summary | ⏳ Stub | Requires activity tracking |
| check_user_permissions | ✅ Implemented | Permission check |
| get_user_effective_permissions | ✅ Implemented | Get effective permissions |

### Methods

#### 10.1 GetUser
Get user by ID.

```protobuf
rpc GetUser(GetUserRequest) returns (GetUserResponse);
```

**Request:**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "include_inactive": false
}
```

**Response:**
```json
{
  "base_response": {
    "success": true,
    "message": "User retrieved successfully"
  },
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "username": "johndoe",
    "first_name": "John",
    "last_name": "Doe",
    "is_active": true,
    "email_verified": true,
    "created_at": "2025-01-15T10:30:00Z",
    "updated_at": "2025-11-20T10:30:00Z"
  }
}
```

#### 10.2 GetUserByEmail
Get user by email address.

```protobuf
rpc GetUserByEmail(GetUserByEmailRequest) returns (GetUserByEmailResponse);
```

#### 10.3 GetUserByUsername
Get user by username.

```protobuf
rpc GetUserByUsername(GetUserByUsernameRequest) returns (GetUserByUsernameResponse);
```

#### 10.4 ListUsers
List all users with pagination.

```protobuf
rpc ListUsers(ListUsersRequest) returns (ListUsersResponse);
```

**Request:**
```json
{
  "page": 1,
  "limit": 20,
  "include_inactive": false,
  "sort_by": "created_at",
  "sort_order": "desc"
}
```

#### 10.5 SearchUsers
Search users by keyword.

```protobuf
rpc SearchUsers(SearchUsersRequest) returns (SearchUsersResponse);
```

#### 10.6 GetUsersByRole
Get all users with specific role.

```protobuf
rpc GetUsersByRole(GetUsersByRoleRequest) returns (GetUsersByRoleResponse);
```

#### 10.7 GetUsersByStatus
Get users by status (active/inactive).

```protobuf
rpc GetUsersByStatus(GetUsersByStatusRequest) returns (GetUsersByStatusResponse);
```

#### 10.8 GetUserSessions
Get all active sessions for a user.

```protobuf
rpc GetUserSessions(GetUserSessionsRequest) returns (GetUserSessionsResponse);
```

#### 10.9 GetSessionByToken
Get session details by token.

```protobuf
rpc GetSessionByToken(GetSessionByTokenRequest) returns (GetSessionByTokenResponse);
```

#### 10.10 GetUserSecurityEvents
Get security events for a user (audit log).

```protobuf
rpc GetUserSecurityEvents(GetUserSecurityEventsRequest) returns (GetUserSecurityEventsResponse);
```

#### 10.11 ValidateUserCredentials
Validate username/email and password.

```protobuf
rpc ValidateUserCredentials(ValidateUserCredentialsRequest) returns (ValidateUserCredentialsResponse);
```

**Request:**
```json
{
  "email_or_username": "johndoe",
  "password": "SecurePass123!"
}
```

**Response:**
```json
{
  "base_response": {
    "success": true,
    "message": "Credentials valid"
  },
  "is_valid": true,
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "requires_mfa": true
}
```

#### 10.12 GetUserPreferences
Get all user preferences.

```protobuf
rpc GetUserPreferences(GetUserPreferencesRequest) returns (GetUserPreferencesResponse);
```

#### 10.13 GetUserPreference
Get specific user preference by key.

```protobuf
rpc GetUserPreference(GetUserPreferenceRequest) returns (GetUserPreferenceResponse);
```

#### 10.14 VerifyUserEmail
Check if email is verified.

```protobuf
rpc VerifyUserEmail(VerifyUserEmailRequest) returns (VerifyUserEmailResponse);
```

#### 10.15 CheckUserExists
Check if user exists by email or username.

```protobuf
rpc CheckUserExists(CheckUserExistsRequest) returns (CheckUserExistsResponse);
```

#### 10.16 GetUserStatistics
Get user statistics (login count, last login, etc.).

```protobuf
rpc GetUserStatistics(GetUserStatisticsRequest) returns (GetUserStatisticsResponse);
```

#### 10.17 GetUserActivitySummary
Get user activity summary (recent actions).

```protobuf
rpc GetUserActivitySummary(GetUserActivitySummaryRequest) returns (GetUserActivitySummaryResponse);
```

#### 10.18 CheckUserPermissions
Check if user has specific permissions (delegates to PermissionResolutionService).

```protobuf
rpc CheckUserPermissions(CheckUserPermissionsRequest) returns (CheckUserPermissionsResponse);
```

---

## 11. SessionService

**Package:** `sapiens.SessionService`
**Proto:** `proto/services/session_service.proto`
**RPCs:** 5

Session management operations.

### Methods

#### 11.1 CreateSession
Create a new user session.

```protobuf
rpc CreateSession(CreateSessionRequest) returns (CreateSessionResponse);
```

**Request:**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "ip_address": "192.168.1.100",
  "user_agent": "Mozilla/5.0...",
  "expires_in_seconds": 3600
}
```

**Response:**
```json
{
  "base_response": {
    "success": true,
    "message": "Session created"
  },
  "session": {
    "id": "550e8400-e29b-41d4-a716-446655440003",
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_at": "2025-11-20T11:30:00Z",
    "created_at": "2025-11-20T10:30:00Z"
  }
}
```

#### 11.2 GetSession
Get session by ID.

```protobuf
rpc GetSession(GetSessionRequest) returns (GetSessionResponse);
```

#### 11.3 ValidateSession
Validate if session is still active.

```protobuf
rpc ValidateSession(ValidateSessionRequest) returns (ValidateSessionResponse);
```

#### 11.4 RevokeSession
Revoke/invalidate a session.

```protobuf
rpc RevokeSession(RevokeSessionRequest) returns (RevokeSessionResponse);
```

#### 11.5 RevokeAllUserSessions
Revoke all sessions for a user (logout from all devices).

```protobuf
rpc RevokeAllUserSessions(RevokeAllUserSessionsRequest) returns (RevokeAllUserSessionsResponse);
```

---

## CRUD Services

The following services provide standardized CRUD operations with full lifecycle management. Each CRUD service implements 12 methods following a consistent pattern for entity management including soft delete and trash functionality.

**Package:** `sapiens.crud`
**Proto Location:** `libs/domain/service/sapiens/crud/`
**Total CRUD Services:** 17
**Methods per Service:** 12
**Total CRUD Methods:** 204

### Standard CRUD Methods (12 per service)

| # | Method | Description | HTTP Equivalent |
|---|--------|-------------|-----------------|
| 1 | Create{Entity} | Create new entity | POST |
| 2 | Get{Entity} | Get entity by ID | GET |
| 3 | Update{Entity} | Full update (replace all fields) | PUT |
| 4 | PartialUpdate{Entity} | Partial update (only provided fields) | PATCH |
| 5 | Delete{Entity} | Soft delete (move to trash) | DELETE |
| 6 | List{Entity} | List all entities (paginated) | GET (list) |
| 7 | Search{Entity} | Search with filters | GET (search) |
| 8 | BulkCreate{Entity} | Create multiple entities at once | POST (bulk) |
| 9 | Upsert{Entity} | Update if exists, create if not | PUT (upsert) |
| 10 | ListDeleted{Entity} | List soft-deleted items (trash) | GET (trash) |
| 11 | Restore{Entity} | Restore soft-deleted item | POST (restore) |
| 12 | EmptyTrash{Entity} | Hard delete all soft-deleted items | DELETE (trash) |

---

## 12. UserCrudService

**Package:** `sapiens.crud.UserCrudService`
**Proto:** `libs/domain/service/sapiens/crud/user_crud.proto`
**RPCs:** 12

Standard CRUD operations for User entity with full lifecycle management.

### Methods

#### 12.1 CreateUser
Create a new user.

```protobuf
rpc CreateUser(CreateUserRequest) returns (CreateUserResponse);
```

**Request:**
```json
{
  "email": "user@example.com",
  "username": "johndoe",
  "full_name": "John Doe",
  "password_hash": "$argon2id$v=19$m=19456,t=2,p=1$...",
  "roles": ["user"],
  "is_active": true,
  "is_verified": false,
  "metadata": "{}"
}
```

**Response:**
```json
{
  "entity": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "username": "johndoe",
    "full_name": "John Doe",
    "roles": ["user"],
    "is_active": true,
    "is_verified": false,
    "created_at": "2025-11-20T10:30:00Z",
    "updated_at": "2025-11-20T10:30:00Z"
  }
}
```

#### 12.2 GetUser
Get user by ID.

```protobuf
rpc GetUser(GetUserRequest) returns (GetUserResponse);
```

#### 12.3 UpdateUser
Full update - replace all user fields.

```protobuf
rpc UpdateUser(UpdateUserRequest) returns (UpdateUserResponse);
```

#### 12.4 PartialUpdateUser
Partial update - only update provided fields.

```protobuf
rpc PartialUpdateUser(PartialUpdateUserRequest) returns (PartialUpdateUserResponse);
```

**Request:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "fields": {
    "full_name": "John D. Doe",
    "is_verified": true
  }
}
```

#### 12.5 DeleteUser
Soft delete user (move to trash).

```protobuf
rpc DeleteUser(DeleteUserRequest) returns (DeleteUserResponse);
```

#### 12.6 ListUser
List users with pagination.

```protobuf
rpc ListUser(ListUserRequest) returns (ListUserResponse);
```

**Request:**
```json
{
  "limit": 20,
  "offset": 0,
  "sort_by": "created_at",
  "sort_order": "desc"
}
```

#### 12.7 SearchUser
Search users by query and filters.

```protobuf
rpc SearchUser(SearchUserRequest) returns (SearchUserResponse);
```

**Request:**
```json
{
  "query": "john",
  "filters": {"is_active": "true"},
  "limit": 20,
  "offset": 0
}
```

#### 12.8 BulkCreateUser
Create multiple users at once.

```protobuf
rpc BulkCreateUser(BulkCreateUserRequest) returns (BulkCreateUserResponse);
```

**Request:**
```json
{
  "entities": [
    {"email": "user1@example.com", "username": "user1", "full_name": "User One"},
    {"email": "user2@example.com", "username": "user2", "full_name": "User Two"}
  ]
}
```

#### 12.9 UpsertUser
Update user if exists, create if not.

```protobuf
rpc UpsertUser(UpsertUserRequest) returns (UpsertUserResponse);
```

**Request:**
```json
{
  "entity": {
    "email": "user@example.com",
    "username": "johndoe",
    "full_name": "John Doe"
  },
  "match_field": "email"
}
```

#### 12.10 ListDeletedUser
List soft-deleted users (trash).

```protobuf
rpc ListDeletedUser(ListDeletedUserRequest) returns (ListDeletedUserResponse);
```

**Request:**
```json
{
  "limit": 20,
  "offset": 0
}
```

#### 12.11 RestoreUser
Restore a soft-deleted user.

```protobuf
rpc RestoreUser(RestoreUserRequest) returns (RestoreUserResponse);
```

**Request:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000"
}
```

#### 12.12 EmptyTrashUser
Hard delete all soft-deleted users.

```protobuf
rpc EmptyTrashUser(EmptyTrashUserRequest) returns (EmptyTrashUserResponse);
```

**Request:**
```json
{
  "confirm": true
}
```

**grpcurl Examples:**
```bash
# Create user
grpcurl -plaintext -d '{"email": "test@example.com", "username": "testuser", "full_name": "Test User", "password_hash": "hash", "roles": ["user"], "is_active": true}' localhost:50052 sapiens.crud.UserCrudService/CreateUser

# Get user
grpcurl -plaintext -d '{"id": "550e8400-e29b-41d4-a716-446655440000"}' localhost:50052 sapiens.crud.UserCrudService/GetUser

# Partial update user
grpcurl -plaintext -d '{"id": "550e8400-e29b-41d4-a716-446655440000", "fields": {"full_name": "Updated Name"}}' localhost:50052 sapiens.crud.UserCrudService/PartialUpdateUser

# List users
grpcurl -plaintext -d '{"limit": 10, "offset": 0}' localhost:50052 sapiens.crud.UserCrudService/ListUser

# Search users
grpcurl -plaintext -d '{"query": "john", "limit": 10}' localhost:50052 sapiens.crud.UserCrudService/SearchUser

# Bulk create users
grpcurl -plaintext -d '{"entities": [{"email": "u1@test.com", "username": "u1"}, {"email": "u2@test.com", "username": "u2"}]}' localhost:50052 sapiens.crud.UserCrudService/BulkCreateUser

# Upsert user
grpcurl -plaintext -d '{"entity": {"email": "test@example.com", "username": "testuser"}, "match_field": "email"}' localhost:50052 sapiens.crud.UserCrudService/UpsertUser

# Delete user (soft delete)
grpcurl -plaintext -d '{"id": "550e8400-e29b-41d4-a716-446655440000"}' localhost:50052 sapiens.crud.UserCrudService/DeleteUser

# List deleted users (trash)
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.UserCrudService/ListDeletedUser

# Restore user
grpcurl -plaintext -d '{"id": "550e8400-e29b-41d4-a716-446655440000"}' localhost:50052 sapiens.crud.UserCrudService/RestoreUser

# Empty trash
grpcurl -plaintext -d '{"confirm": true}' localhost:50052 sapiens.crud.UserCrudService/EmptyTrashUser
```

---

## 13. RoleCrudService

**Package:** `sapiens.crud.RoleCrudService`
**Proto:** `libs/domain/service/sapiens/crud/role_crud.proto`
**RPCs:** 12

Standard CRUD operations for Role entity.

### Methods

#### 13.1 CreateRole
```protobuf
rpc CreateRole(CreateRoleRequest) returns (CreateRoleResponse);
```

#### 13.2 GetRole
```protobuf
rpc GetRole(GetRoleRequest) returns (GetRoleResponse);
```

#### 13.3 UpdateRole
```protobuf
rpc UpdateRole(UpdateRoleRequest) returns (UpdateRoleResponse);
```

#### 13.4 PartialUpdateRole
```protobuf
rpc PartialUpdateRole(PartialUpdateRoleRequest) returns (PartialUpdateRoleResponse);
```

#### 13.5 DeleteRole
```protobuf
rpc DeleteRole(DeleteRoleRequest) returns (DeleteRoleResponse);
```

#### 13.6 ListRole
```protobuf
rpc ListRole(ListRoleRequest) returns (ListRoleResponse);
```

#### 13.7 SearchRole
```protobuf
rpc SearchRole(SearchRoleRequest) returns (SearchRoleResponse);
```

#### 13.8 BulkCreateRole
```protobuf
rpc BulkCreateRole(BulkCreateRoleRequest) returns (BulkCreateRoleResponse);
```

#### 13.9 UpsertRole
```protobuf
rpc UpsertRole(UpsertRoleRequest) returns (UpsertRoleResponse);
```

#### 13.10 ListDeletedRole
```protobuf
rpc ListDeletedRole(ListDeletedRoleRequest) returns (ListDeletedRoleResponse);
```

#### 13.11 RestoreRole
```protobuf
rpc RestoreRole(RestoreRoleRequest) returns (RestoreRoleResponse);
```

#### 13.12 EmptyTrashRole
```protobuf
rpc EmptyTrashRole(EmptyTrashRoleRequest) returns (EmptyTrashRoleResponse);
```

**grpcurl Examples:**
```bash
# Create role
grpcurl -plaintext -d '{"name": "editor", "description": "Content editor role"}' localhost:50052 sapiens.crud.RoleCrudService/CreateRole

# Partial update role
grpcurl -plaintext -d '{"id": "role-id", "fields": {"description": "Updated description"}}' localhost:50052 sapiens.crud.RoleCrudService/PartialUpdateRole

# List roles
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.RoleCrudService/ListRole

# Bulk create roles
grpcurl -plaintext -d '{"entities": [{"name": "viewer"}, {"name": "moderator"}]}' localhost:50052 sapiens.crud.RoleCrudService/BulkCreateRole

# Upsert role
grpcurl -plaintext -d '{"entity": {"name": "admin", "description": "Administrator"}, "match_field": "name"}' localhost:50052 sapiens.crud.RoleCrudService/UpsertRole

# List deleted roles
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.RoleCrudService/ListDeletedRole

# Restore role
grpcurl -plaintext -d '{"id": "role-id"}' localhost:50052 sapiens.crud.RoleCrudService/RestoreRole

# Empty trash
grpcurl -plaintext -d '{"confirm": true}' localhost:50052 sapiens.crud.RoleCrudService/EmptyTrashRole
```

---

## 14. PermissionCrudService

**Package:** `sapiens.crud.PermissionCrudService`
**Proto:** `libs/domain/service/sapiens/crud/permission_crud.proto`
**RPCs:** 12

Standard CRUD operations for Permission entity.

### Methods

#### 14.1 CreatePermission
```protobuf
rpc CreatePermission(CreatePermissionRequest) returns (CreatePermissionResponse);
```

#### 14.2 GetPermission
```protobuf
rpc GetPermission(GetPermissionRequest) returns (GetPermissionResponse);
```

#### 14.3 UpdatePermission
```protobuf
rpc UpdatePermission(UpdatePermissionRequest) returns (UpdatePermissionResponse);
```

#### 14.4 PartialUpdatePermission
```protobuf
rpc PartialUpdatePermission(PartialUpdatePermissionRequest) returns (PartialUpdatePermissionResponse);
```

#### 14.5 DeletePermission
```protobuf
rpc DeletePermission(DeletePermissionRequest) returns (DeletePermissionResponse);
```

#### 14.6 ListPermission
```protobuf
rpc ListPermission(ListPermissionRequest) returns (ListPermissionResponse);
```

#### 14.7 SearchPermission
```protobuf
rpc SearchPermission(SearchPermissionRequest) returns (SearchPermissionResponse);
```

#### 14.8 BulkCreatePermission
```protobuf
rpc BulkCreatePermission(BulkCreatePermissionRequest) returns (BulkCreatePermissionResponse);
```

#### 14.9 UpsertPermission
```protobuf
rpc UpsertPermission(UpsertPermissionRequest) returns (UpsertPermissionResponse);
```

#### 14.10 ListDeletedPermission
```protobuf
rpc ListDeletedPermission(ListDeletedPermissionRequest) returns (ListDeletedPermissionResponse);
```

#### 14.11 RestorePermission
```protobuf
rpc RestorePermission(RestorePermissionRequest) returns (RestorePermissionResponse);
```

#### 14.12 EmptyTrashPermission
```protobuf
rpc EmptyTrashPermission(EmptyTrashPermissionRequest) returns (EmptyTrashPermissionResponse);
```

**grpcurl Examples:**
```bash
# Create permission
grpcurl -plaintext -d '{"name": "articles:read", "resource": "articles", "action": "read"}' localhost:50052 sapiens.crud.PermissionCrudService/CreatePermission

# Bulk create permissions
grpcurl -plaintext -d '{"entities": [{"name": "articles:create", "resource": "articles", "action": "create"}, {"name": "articles:delete", "resource": "articles", "action": "delete"}]}' localhost:50052 sapiens.crud.PermissionCrudService/BulkCreatePermission

# List permissions
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.PermissionCrudService/ListPermission

# List deleted permissions
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.PermissionCrudService/ListDeletedPermission

# Restore permission
grpcurl -plaintext -d '{"id": "perm-id"}' localhost:50052 sapiens.crud.PermissionCrudService/RestorePermission
```

---

## 15. SessionCrudService

**Package:** `sapiens.crud.SessionCrudService`
**Proto:** `libs/domain/service/sapiens/crud/session_crud.proto`
**RPCs:** 12

Standard CRUD operations for Session entity.

### Methods

#### 15.1 CreateSession
```protobuf
rpc CreateSession(CreateSessionRequest) returns (CreateSessionResponse);
```

#### 15.2 GetSession
```protobuf
rpc GetSession(GetSessionRequest) returns (GetSessionResponse);
```

#### 15.3 UpdateSession
```protobuf
rpc UpdateSession(UpdateSessionRequest) returns (UpdateSessionResponse);
```

#### 15.4 PartialUpdateSession
```protobuf
rpc PartialUpdateSession(PartialUpdateSessionRequest) returns (PartialUpdateSessionResponse);
```

#### 15.5 DeleteSession
```protobuf
rpc DeleteSession(DeleteSessionRequest) returns (DeleteSessionResponse);
```

#### 15.6 ListSession
```protobuf
rpc ListSession(ListSessionRequest) returns (ListSessionResponse);
```

#### 15.7 SearchSession
```protobuf
rpc SearchSession(SearchSessionRequest) returns (SearchSessionResponse);
```

#### 15.8 BulkCreateSession
```protobuf
rpc BulkCreateSession(BulkCreateSessionRequest) returns (BulkCreateSessionResponse);
```

#### 15.9 UpsertSession
```protobuf
rpc UpsertSession(UpsertSessionRequest) returns (UpsertSessionResponse);
```

#### 15.10 ListDeletedSession
```protobuf
rpc ListDeletedSession(ListDeletedSessionRequest) returns (ListDeletedSessionResponse);
```

#### 15.11 RestoreSession
```protobuf
rpc RestoreSession(RestoreSessionRequest) returns (RestoreSessionResponse);
```

#### 15.12 EmptyTrashSession
```protobuf
rpc EmptyTrashSession(EmptyTrashSessionRequest) returns (EmptyTrashSessionResponse);
```

**grpcurl Examples:**
```bash
# Get session
grpcurl -plaintext -d '{"id": "session-id"}' localhost:50052 sapiens.crud.SessionCrudService/GetSession

# List sessions
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.SessionCrudService/ListSession

# Search sessions by user
grpcurl -plaintext -d '{"query": "user-id", "filters": {"is_active": "true"}}' localhost:50052 sapiens.crud.SessionCrudService/SearchSession

# List deleted sessions
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.SessionCrudService/ListDeletedSession
```

---

## 16. UserRoleCrudService

**Package:** `sapiens.crud.UserRoleCrudService`
**Proto:** `libs/domain/service/sapiens/crud/user_role_crud.proto`
**RPCs:** 12

Standard CRUD operations for UserRole entity (user-role assignments).

### Methods

#### 16.1 CreateUserRole
```protobuf
rpc CreateUserRole(CreateUserRoleRequest) returns (CreateUserRoleResponse);
```

#### 16.2 GetUserRole
```protobuf
rpc GetUserRole(GetUserRoleRequest) returns (GetUserRoleResponse);
```

#### 16.3 UpdateUserRole
```protobuf
rpc UpdateUserRole(UpdateUserRoleRequest) returns (UpdateUserRoleResponse);
```

#### 16.4 PartialUpdateUserRole
```protobuf
rpc PartialUpdateUserRole(PartialUpdateUserRoleRequest) returns (PartialUpdateUserRoleResponse);
```

#### 16.5 DeleteUserRole
```protobuf
rpc DeleteUserRole(DeleteUserRoleRequest) returns (DeleteUserRoleResponse);
```

#### 16.6 ListUserRole
```protobuf
rpc ListUserRole(ListUserRoleRequest) returns (ListUserRoleResponse);
```

#### 16.7 SearchUserRole
```protobuf
rpc SearchUserRole(SearchUserRoleRequest) returns (SearchUserRoleResponse);
```

#### 16.8 BulkCreateUserRole
```protobuf
rpc BulkCreateUserRole(BulkCreateUserRoleRequest) returns (BulkCreateUserRoleResponse);
```

#### 16.9 UpsertUserRole
```protobuf
rpc UpsertUserRole(UpsertUserRoleRequest) returns (UpsertUserRoleResponse);
```

#### 16.10 ListDeletedUserRole
```protobuf
rpc ListDeletedUserRole(ListDeletedUserRoleRequest) returns (ListDeletedUserRoleResponse);
```

#### 16.11 RestoreUserRole
```protobuf
rpc RestoreUserRole(RestoreUserRoleRequest) returns (RestoreUserRoleResponse);
```

#### 16.12 EmptyTrashUserRole
```protobuf
rpc EmptyTrashUserRole(EmptyTrashUserRoleRequest) returns (EmptyTrashUserRoleResponse);
```

**grpcurl Examples:**
```bash
# Assign role to user
grpcurl -plaintext -d '{"user_id": "user-id", "role_id": "role-id"}' localhost:50052 sapiens.crud.UserRoleCrudService/CreateUserRole

# Bulk assign roles
grpcurl -plaintext -d '{"entities": [{"user_id": "u1", "role_id": "r1"}, {"user_id": "u1", "role_id": "r2"}]}' localhost:50052 sapiens.crud.UserRoleCrudService/BulkCreateUserRole

# List user roles
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.UserRoleCrudService/ListUserRole

# Search by user
grpcurl -plaintext -d '{"query": "user-id", "limit": 10}' localhost:50052 sapiens.crud.UserRoleCrudService/SearchUserRole
```

---

## 17. RolePermissionCrudService

**Package:** `sapiens.crud.RolePermissionCrudService`
**Proto:** `libs/domain/service/sapiens/crud/role_permission_crud.proto`
**RPCs:** 12

Standard CRUD operations for RolePermission entity (role-permission assignments).

### Methods

#### 17.1 CreateRolePermission
```protobuf
rpc CreateRolePermission(CreateRolePermissionRequest) returns (CreateRolePermissionResponse);
```

#### 17.2 GetRolePermission
```protobuf
rpc GetRolePermission(GetRolePermissionRequest) returns (GetRolePermissionResponse);
```

#### 17.3 UpdateRolePermission
```protobuf
rpc UpdateRolePermission(UpdateRolePermissionRequest) returns (UpdateRolePermissionResponse);
```

#### 17.4 PartialUpdateRolePermission
```protobuf
rpc PartialUpdateRolePermission(PartialUpdateRolePermissionRequest) returns (PartialUpdateRolePermissionResponse);
```

#### 17.5 DeleteRolePermission
```protobuf
rpc DeleteRolePermission(DeleteRolePermissionRequest) returns (DeleteRolePermissionResponse);
```

#### 17.6 ListRolePermission
```protobuf
rpc ListRolePermission(ListRolePermissionRequest) returns (ListRolePermissionResponse);
```

#### 17.7 SearchRolePermission
```protobuf
rpc SearchRolePermission(SearchRolePermissionRequest) returns (SearchRolePermissionResponse);
```

#### 17.8 BulkCreateRolePermission
```protobuf
rpc BulkCreateRolePermission(BulkCreateRolePermissionRequest) returns (BulkCreateRolePermissionResponse);
```

#### 17.9 UpsertRolePermission
```protobuf
rpc UpsertRolePermission(UpsertRolePermissionRequest) returns (UpsertRolePermissionResponse);
```

#### 17.10 ListDeletedRolePermission
```protobuf
rpc ListDeletedRolePermission(ListDeletedRolePermissionRequest) returns (ListDeletedRolePermissionResponse);
```

#### 17.11 RestoreRolePermission
```protobuf
rpc RestoreRolePermission(RestoreRolePermissionRequest) returns (RestoreRolePermissionResponse);
```

#### 17.12 EmptyTrashRolePermission
```protobuf
rpc EmptyTrashRolePermission(EmptyTrashRolePermissionRequest) returns (EmptyTrashRolePermissionResponse);
```

**grpcurl Examples:**
```bash
# Assign permission to role
grpcurl -plaintext -d '{"role_id": "role-id", "permission_id": "perm-id"}' localhost:50052 sapiens.crud.RolePermissionCrudService/CreateRolePermission

# Bulk assign permissions
grpcurl -plaintext -d '{"entities": [{"role_id": "r1", "permission_id": "p1"}, {"role_id": "r1", "permission_id": "p2"}]}' localhost:50052 sapiens.crud.RolePermissionCrudService/BulkCreateRolePermission

# List role permissions
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.RolePermissionCrudService/ListRolePermission
```

---

## 18. UserPermissionCrudService

**Package:** `sapiens.crud.UserPermissionCrudService`
**Proto:** `libs/domain/service/sapiens/crud/user_permission_crud.proto`
**RPCs:** 12

Standard CRUD operations for UserPermission entity (direct user-permission assignments).

### Methods

#### 18.1 CreateUserPermission
```protobuf
rpc CreateUserPermission(CreateUserPermissionRequest) returns (CreateUserPermissionResponse);
```

#### 18.2 GetUserPermission
```protobuf
rpc GetUserPermission(GetUserPermissionRequest) returns (GetUserPermissionResponse);
```

#### 18.3 UpdateUserPermission
```protobuf
rpc UpdateUserPermission(UpdateUserPermissionRequest) returns (UpdateUserPermissionResponse);
```

#### 18.4 PartialUpdateUserPermission
```protobuf
rpc PartialUpdateUserPermission(PartialUpdateUserPermissionRequest) returns (PartialUpdateUserPermissionResponse);
```

#### 18.5 DeleteUserPermission
```protobuf
rpc DeleteUserPermission(DeleteUserPermissionRequest) returns (DeleteUserPermissionResponse);
```

#### 18.6 ListUserPermission
```protobuf
rpc ListUserPermission(ListUserPermissionRequest) returns (ListUserPermissionResponse);
```

#### 18.7 SearchUserPermission
```protobuf
rpc SearchUserPermission(SearchUserPermissionRequest) returns (SearchUserPermissionResponse);
```

#### 18.8 BulkCreateUserPermission
```protobuf
rpc BulkCreateUserPermission(BulkCreateUserPermissionRequest) returns (BulkCreateUserPermissionResponse);
```

#### 18.9 UpsertUserPermission
```protobuf
rpc UpsertUserPermission(UpsertUserPermissionRequest) returns (UpsertUserPermissionResponse);
```

#### 18.10 ListDeletedUserPermission
```protobuf
rpc ListDeletedUserPermission(ListDeletedUserPermissionRequest) returns (ListDeletedUserPermissionResponse);
```

#### 18.11 RestoreUserPermission
```protobuf
rpc RestoreUserPermission(RestoreUserPermissionRequest) returns (RestoreUserPermissionResponse);
```

#### 18.12 EmptyTrashUserPermission
```protobuf
rpc EmptyTrashUserPermission(EmptyTrashUserPermissionRequest) returns (EmptyTrashUserPermissionResponse);
```

**grpcurl Examples:**
```bash
# Grant direct permission to user
grpcurl -plaintext -d '{"user_id": "user-id", "permission_id": "perm-id"}' localhost:50052 sapiens.crud.UserPermissionCrudService/CreateUserPermission

# List user permissions
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.UserPermissionCrudService/ListUserPermission

# List deleted user permissions
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.UserPermissionCrudService/ListDeletedUserPermission
```

---

## 19. UserPreferenceCrudService

**Package:** `sapiens.crud.UserPreferenceCrudService`
**Proto:** `libs/domain/service/sapiens/crud/user_preference_crud.proto`
**RPCs:** 12

Standard CRUD operations for UserPreference entity.

### Methods

#### 19.1 CreateUserPreference
```protobuf
rpc CreateUserPreference(CreateUserPreferenceRequest) returns (CreateUserPreferenceResponse);
```

#### 19.2 GetUserPreference
```protobuf
rpc GetUserPreference(GetUserPreferenceRequest) returns (GetUserPreferenceResponse);
```

#### 19.3 UpdateUserPreference
```protobuf
rpc UpdateUserPreference(UpdateUserPreferenceRequest) returns (UpdateUserPreferenceResponse);
```

#### 19.4 PartialUpdateUserPreference
```protobuf
rpc PartialUpdateUserPreference(PartialUpdateUserPreferenceRequest) returns (PartialUpdateUserPreferenceResponse);
```

#### 19.5 DeleteUserPreference
```protobuf
rpc DeleteUserPreference(DeleteUserPreferenceRequest) returns (DeleteUserPreferenceResponse);
```

#### 19.6 ListUserPreference
```protobuf
rpc ListUserPreference(ListUserPreferenceRequest) returns (ListUserPreferenceResponse);
```

#### 19.7 SearchUserPreference
```protobuf
rpc SearchUserPreference(SearchUserPreferenceRequest) returns (SearchUserPreferenceResponse);
```

#### 19.8 BulkCreateUserPreference
```protobuf
rpc BulkCreateUserPreference(BulkCreateUserPreferenceRequest) returns (BulkCreateUserPreferenceResponse);
```

#### 19.9 UpsertUserPreference
```protobuf
rpc UpsertUserPreference(UpsertUserPreferenceRequest) returns (UpsertUserPreferenceResponse);
```

#### 19.10 ListDeletedUserPreference
```protobuf
rpc ListDeletedUserPreference(ListDeletedUserPreferenceRequest) returns (ListDeletedUserPreferenceResponse);
```

#### 19.11 RestoreUserPreference
```protobuf
rpc RestoreUserPreference(RestoreUserPreferenceRequest) returns (RestoreUserPreferenceResponse);
```

#### 19.12 EmptyTrashUserPreference
```protobuf
rpc EmptyTrashUserPreference(EmptyTrashUserPreferenceRequest) returns (EmptyTrashUserPreferenceResponse);
```

**grpcurl Examples:**
```bash
# Create preference
grpcurl -plaintext -d '{"user_id": "user-id", "key": "theme", "value": "dark"}' localhost:50052 sapiens.crud.UserPreferenceCrudService/CreateUserPreference

# Upsert preference
grpcurl -plaintext -d '{"entity": {"user_id": "user-id", "key": "language", "value": "en"}, "match_field": "key"}' localhost:50052 sapiens.crud.UserPreferenceCrudService/UpsertUserPreference

# List user preferences
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.UserPreferenceCrudService/ListUserPreference
```

---

## 20. UserSettingsCrudService

**Package:** `sapiens.crud.UserSettingsCrudService`
**Proto:** `libs/domain/service/sapiens/crud/user_settings_crud.proto`
**RPCs:** 12

Standard CRUD operations for UserSettings entity.

### Methods

#### 20.1 CreateUserSettings
```protobuf
rpc CreateUserSettings(CreateUserSettingsRequest) returns (CreateUserSettingsResponse);
```

#### 20.2 GetUserSettings
```protobuf
rpc GetUserSettings(GetUserSettingsRequest) returns (GetUserSettingsResponse);
```

#### 20.3 UpdateUserSettings
```protobuf
rpc UpdateUserSettings(UpdateUserSettingsRequest) returns (UpdateUserSettingsResponse);
```

#### 20.4 PartialUpdateUserSettings
```protobuf
rpc PartialUpdateUserSettings(PartialUpdateUserSettingsRequest) returns (PartialUpdateUserSettingsResponse);
```

#### 20.5 DeleteUserSettings
```protobuf
rpc DeleteUserSettings(DeleteUserSettingsRequest) returns (DeleteUserSettingsResponse);
```

#### 20.6 ListUserSettings
```protobuf
rpc ListUserSettings(ListUserSettingsRequest) returns (ListUserSettingsResponse);
```

#### 20.7 SearchUserSettings
```protobuf
rpc SearchUserSettings(SearchUserSettingsRequest) returns (SearchUserSettingsResponse);
```

#### 20.8 BulkCreateUserSettings
```protobuf
rpc BulkCreateUserSettings(BulkCreateUserSettingsRequest) returns (BulkCreateUserSettingsResponse);
```

#### 20.9 UpsertUserSettings
```protobuf
rpc UpsertUserSettings(UpsertUserSettingsRequest) returns (UpsertUserSettingsResponse);
```

#### 20.10 ListDeletedUserSettings
```protobuf
rpc ListDeletedUserSettings(ListDeletedUserSettingsRequest) returns (ListDeletedUserSettingsResponse);
```

#### 20.11 RestoreUserSettings
```protobuf
rpc RestoreUserSettings(RestoreUserSettingsRequest) returns (RestoreUserSettingsResponse);
```

#### 20.12 EmptyTrashUserSettings
```protobuf
rpc EmptyTrashUserSettings(EmptyTrashUserSettingsRequest) returns (EmptyTrashUserSettingsResponse);
```

**grpcurl Examples:**
```bash
# Get user settings
grpcurl -plaintext -d '{"id": "settings-id"}' localhost:50052 sapiens.crud.UserSettingsCrudService/GetUserSettings

# Partial update settings
grpcurl -plaintext -d '{"id": "settings-id", "fields": {"notifications_enabled": true}}' localhost:50052 sapiens.crud.UserSettingsCrudService/PartialUpdateUserSettings

# List user settings
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.UserSettingsCrudService/ListUserSettings
```

---

## 21. UserSessionCrudService

**Package:** `sapiens.crud.UserSessionCrudService`
**Proto:** `libs/domain/service/sapiens/crud/user_session_crud.proto`
**RPCs:** 12

Standard CRUD operations for UserSession entity (extended session management).

### Methods

#### 21.1 CreateUserSession
```protobuf
rpc CreateUserSession(CreateUserSessionRequest) returns (CreateUserSessionResponse);
```

#### 21.2 GetUserSession
```protobuf
rpc GetUserSession(GetUserSessionRequest) returns (GetUserSessionResponse);
```

#### 21.3 UpdateUserSession
```protobuf
rpc UpdateUserSession(UpdateUserSessionRequest) returns (UpdateUserSessionResponse);
```

#### 21.4 PartialUpdateUserSession
```protobuf
rpc PartialUpdateUserSession(PartialUpdateUserSessionRequest) returns (PartialUpdateUserSessionResponse);
```

#### 21.5 DeleteUserSession
```protobuf
rpc DeleteUserSession(DeleteUserSessionRequest) returns (DeleteUserSessionResponse);
```

#### 21.6 ListUserSession
```protobuf
rpc ListUserSession(ListUserSessionRequest) returns (ListUserSessionResponse);
```

#### 21.7 SearchUserSession
```protobuf
rpc SearchUserSession(SearchUserSessionRequest) returns (SearchUserSessionResponse);
```

#### 21.8 BulkCreateUserSession
```protobuf
rpc BulkCreateUserSession(BulkCreateUserSessionRequest) returns (BulkCreateUserSessionResponse);
```

#### 21.9 UpsertUserSession
```protobuf
rpc UpsertUserSession(UpsertUserSessionRequest) returns (UpsertUserSessionResponse);
```

#### 21.10 ListDeletedUserSession
```protobuf
rpc ListDeletedUserSession(ListDeletedUserSessionRequest) returns (ListDeletedUserSessionResponse);
```

#### 21.11 RestoreUserSession
```protobuf
rpc RestoreUserSession(RestoreUserSessionRequest) returns (RestoreUserSessionResponse);
```

#### 21.12 EmptyTrashUserSession
```protobuf
rpc EmptyTrashUserSession(EmptyTrashUserSessionRequest) returns (EmptyTrashUserSessionResponse);
```

**grpcurl Examples:**
```bash
# Get user session
grpcurl -plaintext -d '{"id": "session-id"}' localhost:50052 sapiens.crud.UserSessionCrudService/GetUserSession

# List user sessions
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.UserSessionCrudService/ListUserSession

# Search sessions by IP
grpcurl -plaintext -d '{"query": "192.168", "limit": 10}' localhost:50052 sapiens.crud.UserSessionCrudService/SearchUserSession
```

---

## 22. UserSecurityEventCrudService

**Package:** `sapiens.crud.UserSecurityEventCrudService`
**Proto:** `libs/domain/service/sapiens/crud/user_security_event_crud.proto`
**RPCs:** 12

Standard CRUD operations for UserSecurityEvent entity (security audit events).

### Methods

#### 22.1 CreateUserSecurityEvent
```protobuf
rpc CreateUserSecurityEvent(CreateUserSecurityEventRequest) returns (CreateUserSecurityEventResponse);
```

#### 22.2 GetUserSecurityEvent
```protobuf
rpc GetUserSecurityEvent(GetUserSecurityEventRequest) returns (GetUserSecurityEventResponse);
```

#### 22.3 UpdateUserSecurityEvent
```protobuf
rpc UpdateUserSecurityEvent(UpdateUserSecurityEventRequest) returns (UpdateUserSecurityEventResponse);
```

#### 22.4 PartialUpdateUserSecurityEvent
```protobuf
rpc PartialUpdateUserSecurityEvent(PartialUpdateUserSecurityEventRequest) returns (PartialUpdateUserSecurityEventResponse);
```

#### 22.5 DeleteUserSecurityEvent
```protobuf
rpc DeleteUserSecurityEvent(DeleteUserSecurityEventRequest) returns (DeleteUserSecurityEventResponse);
```

#### 22.6 ListUserSecurityEvent
```protobuf
rpc ListUserSecurityEvent(ListUserSecurityEventRequest) returns (ListUserSecurityEventResponse);
```

#### 22.7 SearchUserSecurityEvent
```protobuf
rpc SearchUserSecurityEvent(SearchUserSecurityEventRequest) returns (SearchUserSecurityEventResponse);
```

#### 22.8 BulkCreateUserSecurityEvent
```protobuf
rpc BulkCreateUserSecurityEvent(BulkCreateUserSecurityEventRequest) returns (BulkCreateUserSecurityEventResponse);
```

#### 22.9 UpsertUserSecurityEvent
```protobuf
rpc UpsertUserSecurityEvent(UpsertUserSecurityEventRequest) returns (UpsertUserSecurityEventResponse);
```

#### 22.10 ListDeletedUserSecurityEvent
```protobuf
rpc ListDeletedUserSecurityEvent(ListDeletedUserSecurityEventRequest) returns (ListDeletedUserSecurityEventResponse);
```

#### 22.11 RestoreUserSecurityEvent
```protobuf
rpc RestoreUserSecurityEvent(RestoreUserSecurityEventRequest) returns (RestoreUserSecurityEventResponse);
```

#### 22.12 EmptyTrashUserSecurityEvent
```protobuf
rpc EmptyTrashUserSecurityEvent(EmptyTrashUserSecurityEventRequest) returns (EmptyTrashUserSecurityEventResponse);
```

**grpcurl Examples:**
```bash
# Create security event
grpcurl -plaintext -d '{"user_id": "user-id", "event_type": "LOGIN_SUCCESS", "ip_address": "192.168.1.1"}' localhost:50052 sapiens.crud.UserSecurityEventCrudService/CreateUserSecurityEvent

# Search security events
grpcurl -plaintext -d '{"query": "LOGIN", "limit": 10}' localhost:50052 sapiens.crud.UserSecurityEventCrudService/SearchUserSecurityEvent

# List events by type
grpcurl -plaintext -d '{"filters": {"event_type": "LOGIN_FAILED"}, "limit": 10}' localhost:50052 sapiens.crud.UserSecurityEventCrudService/SearchUserSecurityEvent
```

---

## 23. MfaDeviceCrudService

**Package:** `sapiens.crud.MfaDeviceCrudService`
**Proto:** `libs/domain/service/sapiens/crud/mfa_device_crud.proto`
**RPCs:** 12

Standard CRUD operations for MfaDevice entity (MFA device management).

### Methods

#### 23.1 CreateMfaDevice
```protobuf
rpc CreateMfaDevice(CreateMfaDeviceRequest) returns (CreateMfaDeviceResponse);
```

#### 23.2 GetMfaDevice
```protobuf
rpc GetMfaDevice(GetMfaDeviceRequest) returns (GetMfaDeviceResponse);
```

#### 23.3 UpdateMfaDevice
```protobuf
rpc UpdateMfaDevice(UpdateMfaDeviceRequest) returns (UpdateMfaDeviceResponse);
```

#### 23.4 PartialUpdateMfaDevice
```protobuf
rpc PartialUpdateMfaDevice(PartialUpdateMfaDeviceRequest) returns (PartialUpdateMfaDeviceResponse);
```

#### 23.5 DeleteMfaDevice
```protobuf
rpc DeleteMfaDevice(DeleteMfaDeviceRequest) returns (DeleteMfaDeviceResponse);
```

#### 23.6 ListMfaDevice
```protobuf
rpc ListMfaDevice(ListMfaDeviceRequest) returns (ListMfaDeviceResponse);
```

#### 23.7 SearchMfaDevice
```protobuf
rpc SearchMfaDevice(SearchMfaDeviceRequest) returns (SearchMfaDeviceResponse);
```

#### 23.8 BulkCreateMfaDevice
```protobuf
rpc BulkCreateMfaDevice(BulkCreateMfaDeviceRequest) returns (BulkCreateMfaDeviceResponse);
```

#### 23.9 UpsertMfaDevice
```protobuf
rpc UpsertMfaDevice(UpsertMfaDeviceRequest) returns (UpsertMfaDeviceResponse);
```

#### 23.10 ListDeletedMfaDevice
```protobuf
rpc ListDeletedMfaDevice(ListDeletedMfaDeviceRequest) returns (ListDeletedMfaDeviceResponse);
```

#### 23.11 RestoreMfaDevice
```protobuf
rpc RestoreMfaDevice(RestoreMfaDeviceRequest) returns (RestoreMfaDeviceResponse);
```

#### 23.12 EmptyTrashMfaDevice
```protobuf
rpc EmptyTrashMfaDevice(EmptyTrashMfaDeviceRequest) returns (EmptyTrashMfaDeviceResponse);
```

**grpcurl Examples:**
```bash
# Create MFA device
grpcurl -plaintext -d '{"user_id": "user-id", "device_type": "TOTP", "device_name": "Google Authenticator"}' localhost:50052 sapiens.crud.MfaDeviceCrudService/CreateMfaDevice

# List MFA devices
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.MfaDeviceCrudService/ListMfaDevice

# Search by device type
grpcurl -plaintext -d '{"query": "TOTP", "limit": 10}' localhost:50052 sapiens.crud.MfaDeviceCrudService/SearchMfaDevice
```

---

## 24. PasswordResetTokenCrudService

**Package:** `sapiens.crud.PasswordResetTokenCrudService`
**Proto:** `libs/domain/service/sapiens/crud/password_reset_token_crud.proto`
**RPCs:** 12

Standard CRUD operations for PasswordResetToken entity.

### Methods

#### 24.1 CreatePasswordResetToken
```protobuf
rpc CreatePasswordResetToken(CreatePasswordResetTokenRequest) returns (CreatePasswordResetTokenResponse);
```

#### 24.2 GetPasswordResetToken
```protobuf
rpc GetPasswordResetToken(GetPasswordResetTokenRequest) returns (GetPasswordResetTokenResponse);
```

#### 24.3 UpdatePasswordResetToken
```protobuf
rpc UpdatePasswordResetToken(UpdatePasswordResetTokenRequest) returns (UpdatePasswordResetTokenResponse);
```

#### 24.4 PartialUpdatePasswordResetToken
```protobuf
rpc PartialUpdatePasswordResetToken(PartialUpdatePasswordResetTokenRequest) returns (PartialUpdatePasswordResetTokenResponse);
```

#### 24.5 DeletePasswordResetToken
```protobuf
rpc DeletePasswordResetToken(DeletePasswordResetTokenRequest) returns (DeletePasswordResetTokenResponse);
```

#### 24.6 ListPasswordResetToken
```protobuf
rpc ListPasswordResetToken(ListPasswordResetTokenRequest) returns (ListPasswordResetTokenResponse);
```

#### 24.7 SearchPasswordResetToken
```protobuf
rpc SearchPasswordResetToken(SearchPasswordResetTokenRequest) returns (SearchPasswordResetTokenResponse);
```

#### 24.8 BulkCreatePasswordResetToken
```protobuf
rpc BulkCreatePasswordResetToken(BulkCreatePasswordResetTokenRequest) returns (BulkCreatePasswordResetTokenResponse);
```

#### 24.9 UpsertPasswordResetToken
```protobuf
rpc UpsertPasswordResetToken(UpsertPasswordResetTokenRequest) returns (UpsertPasswordResetTokenResponse);
```

#### 24.10 ListDeletedPasswordResetToken
```protobuf
rpc ListDeletedPasswordResetToken(ListDeletedPasswordResetTokenRequest) returns (ListDeletedPasswordResetTokenResponse);
```

#### 24.11 RestorePasswordResetToken
```protobuf
rpc RestorePasswordResetToken(RestorePasswordResetTokenRequest) returns (RestorePasswordResetTokenResponse);
```

#### 24.12 EmptyTrashPasswordResetToken
```protobuf
rpc EmptyTrashPasswordResetToken(EmptyTrashPasswordResetTokenRequest) returns (EmptyTrashPasswordResetTokenResponse);
```

**grpcurl Examples:**
```bash
# Create password reset token
grpcurl -plaintext -d '{"user_id": "user-id", "token": "reset-token", "expires_at": "2025-11-21T10:30:00Z"}' localhost:50052 sapiens.crud.PasswordResetTokenCrudService/CreatePasswordResetToken

# Get password reset token
grpcurl -plaintext -d '{"id": "token-id"}' localhost:50052 sapiens.crud.PasswordResetTokenCrudService/GetPasswordResetToken

# List tokens
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.PasswordResetTokenCrudService/ListPasswordResetToken
```

---

## 25. AuditLogCrudService

**Package:** `sapiens.crud.AuditLogCrudService`
**Proto:** `libs/domain/service/sapiens/crud/audit_log_crud.proto`
**RPCs:** 12

Standard CRUD operations for AuditLog entity (system-wide audit logging).

### Methods

#### 25.1 CreateAuditLog
```protobuf
rpc CreateAuditLog(CreateAuditLogRequest) returns (CreateAuditLogResponse);
```

#### 25.2 GetAuditLog
```protobuf
rpc GetAuditLog(GetAuditLogRequest) returns (GetAuditLogResponse);
```

#### 25.3 UpdateAuditLog
```protobuf
rpc UpdateAuditLog(UpdateAuditLogRequest) returns (UpdateAuditLogResponse);
```

#### 25.4 PartialUpdateAuditLog
```protobuf
rpc PartialUpdateAuditLog(PartialUpdateAuditLogRequest) returns (PartialUpdateAuditLogResponse);
```

#### 25.5 DeleteAuditLog
```protobuf
rpc DeleteAuditLog(DeleteAuditLogRequest) returns (DeleteAuditLogResponse);
```

#### 25.6 ListAuditLog
```protobuf
rpc ListAuditLog(ListAuditLogRequest) returns (ListAuditLogResponse);
```

#### 25.7 SearchAuditLog
```protobuf
rpc SearchAuditLog(SearchAuditLogRequest) returns (SearchAuditLogResponse);
```

#### 25.8 BulkCreateAuditLog
```protobuf
rpc BulkCreateAuditLog(BulkCreateAuditLogRequest) returns (BulkCreateAuditLogResponse);
```

#### 25.9 UpsertAuditLog
```protobuf
rpc UpsertAuditLog(UpsertAuditLogRequest) returns (UpsertAuditLogResponse);
```

#### 25.10 ListDeletedAuditLog
```protobuf
rpc ListDeletedAuditLog(ListDeletedAuditLogRequest) returns (ListDeletedAuditLogResponse);
```

#### 25.11 RestoreAuditLog
```protobuf
rpc RestoreAuditLog(RestoreAuditLogRequest) returns (RestoreAuditLogResponse);
```

#### 25.12 EmptyTrashAuditLog
```protobuf
rpc EmptyTrashAuditLog(EmptyTrashAuditLogRequest) returns (EmptyTrashAuditLogResponse);
```

**grpcurl Examples:**
```bash
# Create audit log
grpcurl -plaintext -d '{"user_id": "user-id", "action": "USER_CREATED", "resource": "users", "resource_id": "new-user-id"}' localhost:50052 sapiens.crud.AuditLogCrudService/CreateAuditLog

# Search audit logs
grpcurl -plaintext -d '{"query": "USER_CREATED", "limit": 10}' localhost:50052 sapiens.crud.AuditLogCrudService/SearchAuditLog

# List audit logs by resource
grpcurl -plaintext -d '{"filters": {"resource": "users"}, "limit": 10}' localhost:50052 sapiens.crud.AuditLogCrudService/SearchAuditLog
```

---

## 26. PolicyCrudService

**Package:** `sapiens.crud.PolicyCrudService`
**Proto:** `libs/domain/service/sapiens/crud/policy_crud.proto`
**RPCs:** 12

Standard CRUD operations for Policy entity (access control policies).

### Methods

#### 26.1 CreatePolicy
```protobuf
rpc CreatePolicy(CreatePolicyRequest) returns (CreatePolicyResponse);
```

#### 26.2 GetPolicy
```protobuf
rpc GetPolicy(GetPolicyRequest) returns (GetPolicyResponse);
```

#### 26.3 UpdatePolicy
```protobuf
rpc UpdatePolicy(UpdatePolicyRequest) returns (UpdatePolicyResponse);
```

#### 26.4 PartialUpdatePolicy
```protobuf
rpc PartialUpdatePolicy(PartialUpdatePolicyRequest) returns (PartialUpdatePolicyResponse);
```

#### 26.5 DeletePolicy
```protobuf
rpc DeletePolicy(DeletePolicyRequest) returns (DeletePolicyResponse);
```

#### 26.6 ListPolicy
```protobuf
rpc ListPolicy(ListPolicyRequest) returns (ListPolicyResponse);
```

#### 26.7 SearchPolicy
```protobuf
rpc SearchPolicy(SearchPolicyRequest) returns (SearchPolicyResponse);
```

#### 26.8 BulkCreatePolicy
```protobuf
rpc BulkCreatePolicy(BulkCreatePolicyRequest) returns (BulkCreatePolicyResponse);
```

#### 26.9 UpsertPolicy
```protobuf
rpc UpsertPolicy(UpsertPolicyRequest) returns (UpsertPolicyResponse);
```

#### 26.10 ListDeletedPolicy
```protobuf
rpc ListDeletedPolicy(ListDeletedPolicyRequest) returns (ListDeletedPolicyResponse);
```

#### 26.11 RestorePolicy
```protobuf
rpc RestorePolicy(RestorePolicyRequest) returns (RestorePolicyResponse);
```

#### 26.12 EmptyTrashPolicy
```protobuf
rpc EmptyTrashPolicy(EmptyTrashPolicyRequest) returns (EmptyTrashPolicyResponse);
```

**grpcurl Examples:**
```bash
# Create policy
grpcurl -plaintext -d '{"name": "admin-access", "description": "Full admin access policy", "rules": "{}"}' localhost:50052 sapiens.crud.PolicyCrudService/CreatePolicy

# List policies
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.PolicyCrudService/ListPolicy

# Upsert policy
grpcurl -plaintext -d '{"entity": {"name": "admin-access", "rules": "{}"}, "match_field": "name"}' localhost:50052 sapiens.crud.PolicyCrudService/UpsertPolicy
```

---

## 27. PolicyAssignmentCrudService

**Package:** `sapiens.crud.PolicyAssignmentCrudService`
**Proto:** `libs/domain/service/sapiens/crud/policy_assignment_crud.proto`
**RPCs:** 12

Standard CRUD operations for PolicyAssignment entity (policy-to-user/role assignments).

### Methods

#### 27.1 CreatePolicyAssignment
```protobuf
rpc CreatePolicyAssignment(CreatePolicyAssignmentRequest) returns (CreatePolicyAssignmentResponse);
```

#### 27.2 GetPolicyAssignment
```protobuf
rpc GetPolicyAssignment(GetPolicyAssignmentRequest) returns (GetPolicyAssignmentResponse);
```

#### 27.3 UpdatePolicyAssignment
```protobuf
rpc UpdatePolicyAssignment(UpdatePolicyAssignmentRequest) returns (UpdatePolicyAssignmentResponse);
```

#### 27.4 PartialUpdatePolicyAssignment
```protobuf
rpc PartialUpdatePolicyAssignment(PartialUpdatePolicyAssignmentRequest) returns (PartialUpdatePolicyAssignmentResponse);
```

#### 27.5 DeletePolicyAssignment
```protobuf
rpc DeletePolicyAssignment(DeletePolicyAssignmentRequest) returns (DeletePolicyAssignmentResponse);
```

#### 27.6 ListPolicyAssignment
```protobuf
rpc ListPolicyAssignment(ListPolicyAssignmentRequest) returns (ListPolicyAssignmentResponse);
```

#### 27.7 SearchPolicyAssignment
```protobuf
rpc SearchPolicyAssignment(SearchPolicyAssignmentRequest) returns (SearchPolicyAssignmentResponse);
```

#### 27.8 BulkCreatePolicyAssignment
```protobuf
rpc BulkCreatePolicyAssignment(BulkCreatePolicyAssignmentRequest) returns (BulkCreatePolicyAssignmentResponse);
```

#### 27.9 UpsertPolicyAssignment
```protobuf
rpc UpsertPolicyAssignment(UpsertPolicyAssignmentRequest) returns (UpsertPolicyAssignmentResponse);
```

#### 27.10 ListDeletedPolicyAssignment
```protobuf
rpc ListDeletedPolicyAssignment(ListDeletedPolicyAssignmentRequest) returns (ListDeletedPolicyAssignmentResponse);
```

#### 27.11 RestorePolicyAssignment
```protobuf
rpc RestorePolicyAssignment(RestorePolicyAssignmentRequest) returns (RestorePolicyAssignmentResponse);
```

#### 27.12 EmptyTrashPolicyAssignment
```protobuf
rpc EmptyTrashPolicyAssignment(EmptyTrashPolicyAssignmentRequest) returns (EmptyTrashPolicyAssignmentResponse);
```

**grpcurl Examples:**
```bash
# Assign policy to user
grpcurl -plaintext -d '{"policy_id": "policy-id", "assignee_type": "user", "assignee_id": "user-id"}' localhost:50052 sapiens.crud.PolicyAssignmentCrudService/CreatePolicyAssignment

# List policy assignments
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.PolicyAssignmentCrudService/ListPolicyAssignment

# Bulk assign policies
grpcurl -plaintext -d '{"entities": [{"policy_id": "p1", "assignee_type": "role", "assignee_id": "admin"}, {"policy_id": "p2", "assignee_type": "user", "assignee_id": "u1"}]}' localhost:50052 sapiens.crud.PolicyAssignmentCrudService/BulkCreatePolicyAssignment
```

---

## 28. SystemSettingsCrudService

**Package:** `sapiens.crud.SystemSettingsCrudService`
**Proto:** `libs/domain/service/sapiens/crud/system_settings_crud.proto`
**RPCs:** 12

Standard CRUD operations for SystemSettings entity (global system configuration).

### Methods

#### 28.1 CreateSystemSettings
```protobuf
rpc CreateSystemSettings(CreateSystemSettingsRequest) returns (CreateSystemSettingsResponse);
```

#### 28.2 GetSystemSettings
```protobuf
rpc GetSystemSettings(GetSystemSettingsRequest) returns (GetSystemSettingsResponse);
```

#### 28.3 UpdateSystemSettings
```protobuf
rpc UpdateSystemSettings(UpdateSystemSettingsRequest) returns (UpdateSystemSettingsResponse);
```

#### 28.4 PartialUpdateSystemSettings
```protobuf
rpc PartialUpdateSystemSettings(PartialUpdateSystemSettingsRequest) returns (PartialUpdateSystemSettingsResponse);
```

#### 28.5 DeleteSystemSettings
```protobuf
rpc DeleteSystemSettings(DeleteSystemSettingsRequest) returns (DeleteSystemSettingsResponse);
```

#### 28.6 ListSystemSettings
```protobuf
rpc ListSystemSettings(ListSystemSettingsRequest) returns (ListSystemSettingsResponse);
```

#### 28.7 SearchSystemSettings
```protobuf
rpc SearchSystemSettings(SearchSystemSettingsRequest) returns (SearchSystemSettingsResponse);
```

#### 28.8 BulkCreateSystemSettings
```protobuf
rpc BulkCreateSystemSettings(BulkCreateSystemSettingsRequest) returns (BulkCreateSystemSettingsResponse);
```

#### 28.9 UpsertSystemSettings
```protobuf
rpc UpsertSystemSettings(UpsertSystemSettingsRequest) returns (UpsertSystemSettingsResponse);
```

#### 28.10 ListDeletedSystemSettings
```protobuf
rpc ListDeletedSystemSettings(ListDeletedSystemSettingsRequest) returns (ListDeletedSystemSettingsResponse);
```

#### 28.11 RestoreSystemSettings
```protobuf
rpc RestoreSystemSettings(RestoreSystemSettingsRequest) returns (RestoreSystemSettingsResponse);
```

#### 28.12 EmptyTrashSystemSettings
```protobuf
rpc EmptyTrashSystemSettings(EmptyTrashSystemSettingsRequest) returns (EmptyTrashSystemSettingsResponse);
```

**grpcurl Examples:**
```bash
# Create system setting
grpcurl -plaintext -d '{"key": "max_login_attempts", "value": "5", "description": "Maximum login attempts before lockout"}' localhost:50052 sapiens.crud.SystemSettingsCrudService/CreateSystemSettings

# Get system settings
grpcurl -plaintext -d '{"id": "setting-id"}' localhost:50052 sapiens.crud.SystemSettingsCrudService/GetSystemSettings

# Upsert system setting
grpcurl -plaintext -d '{"entity": {"key": "session_timeout", "value": "3600"}, "match_field": "key"}' localhost:50052 sapiens.crud.SystemSettingsCrudService/UpsertSystemSettings

# List system settings
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.SystemSettingsCrudService/ListSystemSettings

# Search system settings
grpcurl -plaintext -d '{"query": "timeout", "limit": 10}' localhost:50052 sapiens.crud.SystemSettingsCrudService/SearchSystemSettings

# List deleted settings
grpcurl -plaintext -d '{"limit": 10}' localhost:50052 sapiens.crud.SystemSettingsCrudService/ListDeletedSystemSettings

# Restore setting
grpcurl -plaintext -d '{"id": "setting-id"}' localhost:50052 sapiens.crud.SystemSettingsCrudService/RestoreSystemSettings

# Empty trash
grpcurl -plaintext -d '{"confirm": true}' localhost:50052 sapiens.crud.SystemSettingsCrudService/EmptyTrashSystemSettings
```

---

## Common Types

### BaseResponse
Standard response wrapper for all RPCs.

```protobuf
message BaseResponse {
  bool success = 1;
  string message = 2;
  optional string error_code = 3;
  repeated string errors = 4;
}
```

### PaginationInfo
Pagination metadata for list responses.

```protobuf
message PaginationInfo {
  uint32 page = 1;
  uint32 limit = 2;
  uint64 total = 3;
  bool has_next = 4;
  bool has_prev = 5;
}
```

### User
User entity structure.

```protobuf
message User {
  string id = 1;
  string email = 2;
  string username = 3;
  string first_name = 4;
  string last_name = 5;
  bool is_active = 6;
  bool email_verified = 7;
  optional string phone = 8;
  optional string avatar_url = 9;
  google.protobuf.Timestamp created_at = 10;
  google.protobuf.Timestamp updated_at = 11;
  optional google.protobuf.Timestamp deleted_at = 12;
}
```

### Role
Role entity structure.

```protobuf
message Role {
  string id = 1;
  string name = 2;
  optional string description = 3;
  bool is_default = 4;
  google.protobuf.Timestamp created_at = 5;
  google.protobuf.Timestamp updated_at = 6;
  optional google.protobuf.Timestamp deleted_at = 7;
}
```

### Permission
Permission entity structure.

```protobuf
message Permission {
  string id = 1;
  string name = 2;
  string resource = 3;
  string action = 4;
  optional string description = 5;
  google.protobuf.Timestamp created_at = 6;
  google.protobuf.Timestamp updated_at = 7;
  optional google.protobuf.Timestamp deleted_at = 8;
}
```

---

## Error Handling

All RPCs return `BaseResponse` with error details.

### Error Response Example
```json
{
  "success": false,
  "message": "User not found",
  "error_code": "USER_NOT_FOUND",
  "errors": ["User with ID 550e8400-e29b-41d4-a716-446655440000 does not exist"]
}
```

### Common Error Codes
- `INVALID_REQUEST` - Invalid request parameters
- `USER_NOT_FOUND` - User does not exist
- `ROLE_NOT_FOUND` - Role does not exist
- `PERMISSION_NOT_FOUND` - Permission does not exist
- `UNAUTHORIZED` - Authentication required
- `FORBIDDEN` - Insufficient permissions
- `DUPLICATE_ENTRY` - Resource already exists (e.g., email/username)
- `INTERNAL_ERROR` - Server error
- `ENTITY_DELETED` - Entity is soft-deleted (in trash)
- `TRASH_EMPTY` - No items in trash to delete

---

## Testing with grpcurl

### Installation
```bash
# macOS
brew install grpcurl

# Linux
go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest
```

### List all services
```bash
grpcurl -plaintext localhost:50052 list
```

**Output:**
```
grpc.reflection.v1alpha.ServerReflection
sapiens.MfaService
sapiens.PasswordService
sapiens.PermissionResolutionService
sapiens.PermissionService
sapiens.RolePermissionService
sapiens.RoleService
sapiens.SessionService
sapiens.TokenService
sapiens.UserCommandUseCases
sapiens.UserQueryUseCases
sapiens.UserRoleService
sapiens.crud.AuditLogCrudService
sapiens.crud.MfaDeviceCrudService
sapiens.crud.PasswordResetTokenCrudService
sapiens.crud.PermissionCrudService
sapiens.crud.PolicyAssignmentCrudService
sapiens.crud.PolicyCrudService
sapiens.crud.RoleCrudService
sapiens.crud.RolePermissionCrudService
sapiens.crud.SessionCrudService
sapiens.crud.SystemSettingsCrudService
sapiens.crud.UserCrudService
sapiens.crud.UserPermissionCrudService
sapiens.crud.UserPreferenceCrudService
sapiens.crud.UserRoleCrudService
sapiens.crud.UserSecurityEventCrudService
sapiens.crud.UserSessionCrudService
sapiens.crud.UserSettingsCrudService
```

### List methods for a service
```bash
grpcurl -plaintext localhost:50052 list sapiens.UserQueryUseCases
```

### List all CRUD methods for a service
```bash
grpcurl -plaintext localhost:50052 list sapiens.crud.UserCrudService
```

**Output:**
```
sapiens.crud.UserCrudService.BulkCreateUser
sapiens.crud.UserCrudService.CreateUser
sapiens.crud.UserCrudService.DeleteUser
sapiens.crud.UserCrudService.EmptyTrashUser
sapiens.crud.UserCrudService.GetUser
sapiens.crud.UserCrudService.ListDeletedUser
sapiens.crud.UserCrudService.ListUser
sapiens.crud.UserCrudService.PartialUpdateUser
sapiens.crud.UserCrudService.RestoreUser
sapiens.crud.UserCrudService.SearchUser
sapiens.crud.UserCrudService.UpdateUser
sapiens.crud.UserCrudService.UpsertUser
```

### Describe a method
```bash
grpcurl -plaintext localhost:50052 describe sapiens.UserQueryUseCases.GetUser
```

### Call GetUser (with authentication)
```bash
# First, authenticate to get token
TOKEN=$(grpcurl -plaintext \
  -d '{"email_or_username": "root", "password": "PiQS5SVL012D"}' \
  localhost:50052 \
  sapiens.UserQueryUseCases/ValidateUserCredentials | jq -r '.user_id')

# Use token to call GetUser
grpcurl -plaintext \
  -H "authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"user_id": "'$TOKEN'"}' \
  localhost:50052 \
  sapiens.UserQueryUseCases/GetUser
```

### Call CreateUser
```bash
grpcurl -plaintext \
  -H "authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "email": "testuser@example.com",
    "username": "testuser",
    "password": "SecurePass123!",
    "first_name": "Test",
    "last_name": "User"
  }' \
  localhost:50052 \
  sapiens.UserCommandUseCases/CreateUser
```

### Call ListUsers with pagination
```bash
grpcurl -plaintext \
  -H "authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"page": 1, "limit": 10, "include_inactive": false}' \
  localhost:50052 \
  sapiens.UserQueryUseCases/ListUsers
```

### Call CheckUserPermission
```bash
grpcurl -plaintext \
  -H "authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "resource": "articles",
    "action": "create"
  }' \
  localhost:50052 \
  sapiens.PermissionResolutionService/CheckUserPermission
```

---

## API Statistics Summary

| Category | Count |
|----------|-------|
| **Total Services** | 28 |
| **Core Services** | 11 |
| **CRUD Services** | 17 |
| **Core Service Methods** | 103 |
| **CRUD Service Methods** | 204 (17 x 12) |
| **Total RPC Methods** | 307 |

---

## Additional Resources

- **Proto Files:** `libs/modules/sapiens/proto/services/`
- **OpenAPI (REST):** `docs/openapi/rest-api.yaml`
- **Requirements:** `docs/domain.md`, `docs/brd.md`
- **Verification Report:** `REQUIREMENTS_VERIFICATION.md`
- **Project README:** `README.md`

---

**Last Updated:** 2025-11-21
**API Version:** 1.0.0
**gRPC Server:** Tonic (Rust)
