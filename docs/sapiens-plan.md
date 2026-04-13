# Sapiens Module Enhancement Plan

> **Module**: Sapiens (Identity & Access Management)
> **Version**: 2.0
> **Date**: 2025-01-01
> **Status**: Planning

---

## Executive Summary

This plan outlines the enhancement of the Sapiens IAM module to support advanced authentication, authorization, and compliance features. The module currently provides core user management, authentication, and RBAC functionality. The proposed enhancements add enterprise-grade features including GDPR compliance, SAML/LDAP integration, and advanced security monitoring.

**Current Status**: Module compiles successfully with 33+ entities implemented.
**Goal**: Add 12 new entities and enhance existing functionality.

---

## Priority Matrix

| Feature | Priority | Complexity | Value | Status |
|---------|----------|------------|-------|--------|
| Password History | HIGH | Low | High | 🆕 New |
| Session Limits | HIGH | Low | High | 🆕 New |
| Temporary Permissions | HIGH | Medium | High | 🆕 New |
| User Impersonation | HIGH | Medium | High | 🆕 New |
| Data Export (GDPR) | HIGH | Medium | Critical | 🆕 New |
| Resource Permissions | MEDIUM | Medium | High | 🆕 New |
| Security Events | MEDIUM | Low | High | 🆕 New |
| Permission Cache | MEDIUM | Low | Medium | 🆕 New |
| User Anonymization | MEDIUM | High | Critical | 🆕 New |
| Notification Preferences | LOW | Low | Medium | 🆕 New |
| SAML 2.0 Support | LOW | High | High | 🆕 New |
| LDAP/AD Integration | LOW | High | High | 🆕 New |

---

## Phase 1: Regeneration & Critical Fixes (Week 1)

### 1.1 Regenerate from Schema

**Objective**: Ensure all backbone framework updates are reflected in generated code.

**Tasks**:
- [ ] Run `backbone schema validate sapiens`
- [ ] Fix any validation errors
- [ ] Run `backbone schema generate --target all sapiens`
- [ ] Run `cargo check -p backbone-sapiens`
- [ ] Verify all entities compile without errors

**Acceptance Criteria**:
- All schema validation passes
- Generated code compiles with zero errors
- All existing tests pass

### 1.2 Fix Known Issues

**Email Validation** (CRITICAL):
- [ ] Verify email validation works in User creation
- [ ] Test: `invalid-email` returns 400
- [ ] Test: `test@` returns 400
- [ ] Test: `@domain.com` returns 400
- [ ] Test: `valid@example.com` returns 201

**ID Persistence** (CRITICAL):
- [ ] Verify created users can be retrieved by ID
- [ ] Test: Create user, get by ID returns 200
- [ ] Check database transaction handling
- [ ] Verify UUID generation consistency

**Role Uniqueness** (HIGH):
- [ ] Add unique constraint to roles.name in migration
- [ ] Test: Second role with same name returns 409
- [ ] Update schema to include unique constraint

---

## Phase 2: Core Security Enhancements (Week 2)

### 2.1 Password History Entity

**Entity**: `PasswordHistory`

**Schema File**: `schema/models/password_history.model.yaml`

**Fields**:
- `id: uuid` (primary key)
- `user_id: uuid` (foreign key to User)
- `password_hash: string` (Argon2id)
- `set_at: timestamp`
- `expires_at: timestamp` (1 year)

**Business Rules**:
- Keep last 5 passwords minimum
- Check history before password change
- Expire entries after 1 year

**Implementation Tasks**:
- [ ] Create entity schema
- [ ] Add migration
- [ ] Generate code
- [ ] Add to PasswordPolicyService
- [ ] Write tests

### 2.2 Session Limits Entity

**Entity**: `SessionLimit`

**Schema File**: `schema/models/session_limit.model.yaml`

**Fields**:
- `id: uuid` (primary key)
- `user_id: uuid` (unique, foreign key)
- `max_sessions: integer` (default: 5)
- `max_sessions_per_device: integer` (optional)
- `enforce_limit: boolean` (default: true)
- `current_session_count: integer` (computed)

**Business Rules**:
- Default 5 concurrent sessions
- Oldest sessions revoked when limit exceeded
- Current session never revoked

**Implementation Tasks**:
- [ ] Create entity schema
- [ ] Add migration
- [ ] Generate code
- [ ] Add session limit check to SessionService
- [ ] Add auto-revoke logic
- [ ] Write tests

### 2.3 Temporary Permission Entity

**Entity**: `TemporaryPermission`

**Schema File**: `schema/models/temporary_permission.model.yaml`

**Fields**:
- `id: uuid` (primary key)
- `user_id: uuid`
- `permission_id: uuid`
- `granted_by: uuid`
- `granted_at: timestamp`
- `expires_at: timestamp`
- `revoked_at: timestamp` (nullable)
- `reason: string` (nullable)

**Business Rules**:
- Max 30 days duration
- User notified 24h before expiry
- Can be revoked early

**Implementation Tasks**:
- [ ] Create entity schema
- [ ] Add migration
- [ ] Generate code
- [ ] Add to AuthorizationService
- [ ] Implement expiry check
- [ ] Write tests

---

## Phase 3: Advanced Features (Week 3)

### 3.1 User Impersonation

**Entity**: `ImpersonationSession`

**Schema File**: `schema/models/impersonation_session.model.yaml`

**Fields**:
- `id: uuid` (primary key)
- `admin_id: uuid`
- `target_user_id: uuid`
- `session_id: uuid` (links to Session)
- `started_at: timestamp`
- `ended_at: timestamp` (nullable)
- `max_duration_minutes: integer` (default: 60)
- `reason: string`
- `actions_performed: integer`

**API Endpoints**:
- `POST /api/v1/admin/impersonate/{user_id}` - Start impersonation
- `DELETE /api/v1/admin/impersonate/{id}` - End impersonation

**Implementation Tasks**:
- [ ] Create entity schema
- [ ] Add migration
- [ ] Generate code
- [ ] Create ImpersonationService
- [ ] Add API handlers
- [ ] Add audit logging
- [ ] Write tests

### 3.2 Data Export (GDPR)

**Entity**: `DataExport`

**Schema File**: `schema/models/data_export.model.yaml`

**Fields**:
- `id: uuid` (primary key)
- `user_id: uuid`
- `requested_by: uuid`
- `requested_at: timestamp`
- `status: enum` (pending, processing, completed, failed)
- `file_path: string` (nullable)
- `file_url: url` (nullable)
- `expires_at: timestamp` (7 days)
- `format: enum` (json, csv, pdf)

**API Endpoints**:
- `POST /api/v1/users/{id}/export` - Request export
- `GET /api/v1/users/{id}/exports/{id}` - Get status
- `GET /api/v1/users/{id}/exports/{id}/download` - Download file

**Implementation Tasks**:
- [ ] Create entity schema
- [ ] Add migration
- [ ] Generate code
- [ ] Create DataExportService
- [ ] Implement file generation
- [ ] Add file cleanup job
- [ ] Write tests

### 3.3 Resource Permissions

**Entity**: `ResourcePermission`

**Schema File**: `schema/models/resource_permission.model.yaml`

**Fields**:
- `id: uuid` (primary key)
- `permission_id: uuid`
- `resource_type: string`
- `resource_id: string`
- `granted_to_user_id: uuid` (nullable)
- `granted_to_role_id: uuid` (nullable)
- `granted_by: uuid`
- `expires_at: timestamp` (nullable)

**API Endpoints**:
- `POST /api/v1/permissions/resource` - Grant resource permission
- `GET /api/v1/permissions/effective` - Get all permissions

**Implementation Tasks**:
- [ ] Create entity schema
- [ ] Add migration
- [ ] Generate code
- [ ] Enhance AuthorizationService
- [ ] Add resource-level checks
- [ ] Write tests

---

## Phase 4: Security & Monitoring (Week 4)

### 4.1 Security Event Entity

**Entity**: `SecurityEvent`

**Schema File**: `schema/models/security_event.model.yaml`

**Fields**:
- `id: uuid` (primary key)
- `user_id: uuid` (nullable)
- `event_type: enum` (login_failed, account_locked, etc.)
- `severity: enum` (low, medium, high, critical)
- `source_ip: ip` (nullable)
- `details: json` (nullable)
- `is_resolved: boolean`
- `resolved_at: timestamp` (nullable)

**API Endpoints**:
- `GET /api/v1/security/events` - List events
- `PUT /api/v1/security/events/{id}/resolve` - Resolve event

**Implementation Tasks**:
- [ ] Create entity schema
- [ ] Add migration
- [ ] Generate code
- [ ] Create SecurityMonitoringService
- [ ] Add event publishing
- [ ] Add alerting for critical events
- [ ] Write tests

### 4.2 Permission Cache Entity

**Entity**: `PermissionCache`

**Schema File**: `schema/models/permission_cache.model.yaml`

**Fields**:
- `id: uuid` (primary key)
- `user_id: uuid` (unique)
- `permission_ids: json` (array of UUIDs)
- `role_ids: json` (array of UUIDs)
- `computed_at: timestamp`
- `expires_at: timestamp` (5 minutes)
- `version: integer`

**Implementation Tasks**:
- [ ] Create entity schema
- [ ] Add migration
- [ ] Generate code
- [ ] Add caching to AuthorizationService
- [ ] Implement cache invalidation
- [ ] Write tests

---

## Phase 5: GDPR & Compliance (Week 5)

### 5.1 User Anonymization

**Entity**: `AnonymizationRecord`

**Schema File**: `schema/models/anonymization_record.model.yaml`

**Fields**:
- `id: uuid` (primary key)
- `user_id: uuid`
- `original_email: string`
- `original_username: string`
- `anonymized_by: uuid`
- `anonymized_at: timestamp`
- `reason: string`
- `method: enum` (full, partial, pseudonymization)

**API Endpoints**:
- `POST /api/v1/users/{id}/anonymize` - Anonymize user

**Implementation Tasks**:
- [ ] Create entity schema
- [ ] Add migration
- [ ] Generate code
- [ ] Create AnonymizationService
- [ ] Implement anonymization logic
- [ ] Add admin approval workflow
- [ ] Write tests

---

## Phase 6: Enterprise Integration (Week 6+)

### 6.1 SAML 2.0 Support

**Entities**: `SAMLProvider`, `UserSAMLLink`

**Schema Files**:
- `schema/models/saml_provider.model.yaml`
- `schema/models/user_saml_link.model.yaml`

**Implementation Tasks**:
- [ ] Create entity schemas
- [ ] Add migrations
- [ ] Generate code
- [ ] Add SAML dependency (rust-saml)
- [ ] Implement SAML flow
- [ ] Create SAML service
- [ ] Add metadata endpoint
- [ ] Write tests

### 6.2 LDAP/AD Integration

**Entity**: `LDAPDirectory`

**Schema File**: `schema/models/ldap_directory.model.yaml`

**Implementation Tasks**:
- [ ] Create entity schema
- [ ] Add migration
- [ ] Generate code
- [ ] Add LDAP dependency (ldap3)
- [ ] Implement LDAP sync
- [ ] Create sync job
- [ ] Write tests

---

## New Enums to Add

| Enum Name | Values |
|-----------|--------|
| `DataExportStatus` | pending, processing, completed, failed, expired |
| `DataExportFormat` | json, csv, pdf, xml |
| `AnonymizationMethod` | full, partial, pseudonymization |
| `SecurityEventType` | login_failed, login_suspicious, password_reused, account_locked, permission_escalation, impersonation_started, impersonation_ended, mfa_enabled, mfa_disabled, data_exported, data_anonymized, rate_limit_exceeded |
| `SecurityEventSeverity` | low, medium, high, critical |
| `DigestFrequency` | immediate, hourly, daily, weekly |

---

## New State Machines

### ImpersonationSession State Machine

**States**: `active`, `ended`, `expired`

**Transitions**:
- `active` -> `ended` (admin terminates)
- `active` -> `expired` (time limit exceeded)

### DataExport State Machine

**States**: `pending`, `processing`, `completed`, `failed`, `expired`

**Transitions**:
- `pending` -> `processing` (job starts)
- `processing` -> `completed` (success)
- `processing` -> `failed` (error)
- `completed` -> `expired` (7 days passed)

---

## New Workflows

### Workflow: PasswordChangeWithHistory

**Trigger**: User changes password

**Steps**:
1. Validate current password
2. Check password history
3. Validate new password strength
4. Hash new password
5. Store in password history
6. Update user password
7. Invalidate all sessions
8. Send confirmation notification

### Workflow: UserImpersonation

**Trigger**: Admin starts impersonation

**Steps**:
1. Validate admin permission
2. Check target user is not admin
3. Create impersonation session
4. Log security event
5. Start impersonation timer
6. Monitor for expiry
7. End impersonation (manual or timeout)
8. Log summary of actions

### Workflow: DataExportGeneration

**Trigger**: User or admin requests export

**Steps**:
1. Validate request and permissions
2. Create DataExport record
3. Collect user data from all entities
4. Generate export file (JSON/CSV/PDF)
5. Store file with expiry
6. Send download link
7. Schedule file cleanup

---

## New Events

| Event Name | Publisher | Trigger |
|------------|-----------|---------|
| `PasswordChangedEvent` | User | Password updated |
| `ImpersonationStartedEvent` | ImpersonationSession | Admin starts impersonation |
| `ImpersonationEndedEvent` | ImpersonationSession | Impersonation ends |
| `SecurityEventCreatedEvent` | SecurityEvent | Security event occurs |
| `DataExportRequestedEvent` | DataExport | Export requested |
| `DataExportCompletedEvent` | DataExport | Export ready |
| `UserAnonymizedEvent` | User | User anonymized |
| `TemporaryPermissionGrantedEvent` | TemporaryPermission | Temp permission created |
| `TemporaryPermissionExpiredEvent` | TemporaryPermission | Temp permission expires |

---

## Migration Strategy

### Step 1: Schema Conversion
```bash
# For each new entity, create schema file
# Example: schema/models/password_history.model.yaml

# Then validate
backbone schema validate sapiens

# Generate code
backbone schema generate --target all sapiens

# Verify compilation
cargo check -p backbone-sapiens
```

### Step 2: Database Migration
```bash
# Create new migration
sqlx migrate add add_password_history

# Run migration
sqlx migrate run --source libs/modules/sapiens/migrations
```

### Step 3: Service Implementation
- Add new services to `src/domain/services/`
- Implement business logic
- Add to `src/lib.rs` exports

### Step 4: API Integration
- Add HTTP handlers to `src/presentation/http/handlers/`
- Register routes in `src/routes/`
- Update OpenAPI specs

---

## Testing Strategy

### Unit Tests
- [ ] Entity validation tests
- [ ] Business rule tests
- [ ] Service logic tests
- [ ] Permission evaluation tests

### Integration Tests
- [ ] API endpoint tests
- [ ] Database operations tests
- [ ] Workflow execution tests
- [ ] Event publishing tests

### E2E Tests
- [ ] Complete user registration flow
- [ ] Password change with history check
- [ ] Session limit enforcement
- [ ] Impersonation flow
- [ ] Data export request and download

---

## Performance Considerations

| Feature | Performance Target | Optimization Strategy |
|---------|-------------------|---------------------|
| Permission Cache | < 10ms check | In-memory cache with 5min TTL |
| Password History | < 100ms check | Index on user_id, limit to last 5 |
| Session Limits | < 50ms check | Computed field, indexed query |
| Security Events | < 100ms insert | Batch inserts, async processing |
| Data Export | Variable | Async job generation |

---

## Security Considerations

### Data Protection
- Password hashes: Never expose via API
- Security events: Encrypt sensitive details
- Data exports: Temporary storage with signed URLs

### Access Control
- Impersonation: Requires admin + justification
- Anonymization: Requires elevated permission
- Data export: Owner or admin with justification

### Audit Trail
- All security events logged
- Impersonation fully tracked
- Data exports recorded with requestor

---

## Rollback Plan

If issues arise during implementation:

1. **Schema Issues**: Revert migration, drop new tables
2. **Compilation Errors**: Fix generated code, regenerate
3. **Test Failures**: Fix business logic, re-run tests
4. **Performance Issues**: Add indexes, optimize queries

---

## Success Criteria

### Phase 1 Success
- [ ] Zero compilation errors
- [ ] All existing tests pass
- [ ] Email validation working
- [ ] ID persistence verified
- [ ] Role uniqueness enforced

### Phase 2 Success
- [ ] Password history prevents reuse
- [ ] Session limits enforced
- [ ] Temporary permissions expire correctly

### Phase 3 Success
- [ ] Impersonation works end-to-end
- [ ] Data export generates valid files
- [ ] Resource permissions evaluated correctly

### Overall Success
- [ ] All 12 new entities implemented
- [ ] All new API endpoints functional
- [ ] Test coverage > 80%
- [ ] Performance targets met
- [ ] Documentation complete

---

## Open Questions

1. **SAML Library Choice**: Which Rust SAML library to use? (Options: `rust-saml`, custom implementation)
2. **LDAP Library Choice**: Which Rust LDAP library to use? (Options: `ldap3`, `tokio-ldap`)
3. **File Storage**: Where to store export files? (Options: S3, local, Azure Blob)
4. **Async Jobs**: How to handle async jobs? (Options: Background workers, external queue)
5. **Notification Service**: Integrate with existing NotificationService or separate?

---

## Dependencies

### New Crate Dependencies

```toml
# SAML Support (Phase 6)
# rust-saml = "0.5"  # or alternative

# LDAP Support (Phase 6)
ldap3 = { version = "0.11", optional = true }

# Enhanced crypto
argon2 = "0.5"  # Already present

# File generation
rust-xlsxwriter = { version = "0.57", optional = true }  # For Excel exports
pdf = { version = "0.8", optional = true }  # For PDF exports
```

---

## Documentation Requirements

- [ ] Update API documentation for new endpoints
- [ ] Add entity relationship diagram
- [ ] Create sequence diagrams for workflows
- [ ] Update README with new features
- [ ] Add deployment guide for enterprise features

---

## Timeline

| Phase | Duration | Target Date |
|-------|----------|-------------|
| Phase 1: Regeneration & Fixes | 1 week | Week 1 |
| Phase 2: Core Security | 1 week | Week 2 |
| Phase 3: Advanced Features | 1 week | Week 3 |
| Phase 4: Security & Monitoring | 1 week | Week 4 |
| Phase 5: GDPR & Compliance | 1 week | Week 5 |
| Phase 6: Enterprise Integration | 2 weeks | Week 6-7 |
| **Total** | **7 weeks** | **Week 7** |

---

## Next Steps

1. Review and approve this plan
2. Prioritize phases based on business needs
3. Set up development branch
4. Begin Phase 1: Regeneration & Fixes
