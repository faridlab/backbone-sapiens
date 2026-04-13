# Sapiens Documentation

This directory contains all technical and business documentation for the Sapiens RBAC (Role-Based Access Control) service.

---

## 📚 Documentation Index

### 1. Technical Specifications

#### [domain.md](domain.md) (75 KB)
**Complete Technical Domain Specification**

Defines the entire domain model, data structures, and technical implementation details for the Sapiens RBAC system.

**Contents:**
- **12 Domain Entities**: User, Role, Permission, UserRole, RolePermission, UserPermission, Session, AuditLog, PasswordResetToken, MFADevice, UserSettings, SystemSettings
- **3 Core Domain Services**: PasswordService, TokenService, PermissionResolutionService
- **Database Schemas**:
  - MongoDB collections (11 collections with indexes and validation rules)
  - PostgreSQL schemas (alternative storage option)
- **CQRS Architecture**:
  - Command definitions (write operations)
  - Query definitions (read operations)
  - Event definitions (domain events)
- **Implementation Phases**: 8-phase roadmap from MVP to production
- **Proto File Mappings**: gRPC service definitions

**Key Sections:**
1. Domain Entities with field specifications
2. Value Objects (Email, Phone, Address, Money, Password)
3. Domain Services with algorithms
4. Repository interfaces
5. Use Cases (Commands & Queries)
6. Domain Events
7. Business Rules & Specifications
8. Database schemas with indexes
9. Implementation checklist

**Who should read this:**
- Backend developers implementing features
- System architects
- Database administrators
- Tech leads planning sprints

---

#### [brd.md](brd.md) (188 KB)
**Business Requirements Document**

Comprehensive business requirements, user stories, acceptance criteria, and functional specifications.

**Contents:**
- **7 Functional Requirements**:
  - FR-001: User Account Creation
  - FR-002: Role and Permission Assignment
  - FR-003: Auditing and Logging
  - FR-004: Multi-Factor Authentication (MFA)
  - FR-005: Password Management
  - FR-006: User-Role Assignment
  - FR-006A: Hybrid RBAC (Direct Permissions)
- **Non-Functional Requirements**:
  - Performance (response times, throughput)
  - Security (encryption, authentication, authorization)
  - Scalability (horizontal scaling, load balancing)
  - Usability (API design, error handling)
  - Reliability (uptime, fault tolerance)
- **User Stories & Personas**:
  - System Administrator
  - Application Developer
  - Security Auditor
  - End User
- **Success Criteria & KPIs**:
  - User adoption metrics
  - System performance benchmarks
  - Security compliance measures
- **Two-Layer API Architecture**:
  - REST API (25 endpoints)
  - gRPC API (11 services, 103 methods)
- **Data Models & Relationships**
- **Security Considerations**
- **Audit & Compliance Requirements**

**Key Sections:**
1. Executive Summary
2. Project Scope & Objectives
3. Functional Requirements (detailed)
4. Non-Functional Requirements
5. User Stories with acceptance criteria
6. Data Models & ER Diagrams
7. API Specifications (REST & gRPC)
8. Security & Compliance
9. Testing Strategy
10. Deployment & Maintenance

**Who should read this:**
- Product managers
- Business analysts
- Project stakeholders
- QA engineers
- Compliance officers
- Frontend developers (for API contracts)

---

### 2. API Documentation

#### [openapi/](openapi/) Directory
**Complete REST & gRPC API Documentation**

Industry-standard API documentation for both REST and gRPC protocols.

**Files:**

##### [openapi/openapi.yaml](openapi/openapi.yaml) (36 KB)
**OpenAPI 3.0.3 Specification for REST API**

- **Format**: YAML (OpenAPI 3.0.3 standard)
- **Coverage**: 25 REST endpoints
- **Schemas**: Complete request/response types
- **Authentication**: JWT Bearer token
- **Features**: Rate limiting, pagination, error handling

**Endpoints by Category:**
- **Roles** (7 endpoints): CRUD + search + list users
- **Permissions** (7 endpoints): CRUD + search + list roles
- **Role-Permission** (4 endpoints): Assign, revoke, list, bulk operations
- **User-Role** (3 endpoints): Assign, revoke, list
- **User-Permission** (4 endpoints): Grant, revoke, list, check (Hybrid RBAC)

**How to use:**
```bash
# View with Swagger UI
swagger-ui serve openapi/openapi.yaml

# View with Redoc
redoc-cli serve openapi/openapi.yaml

# Import to Postman
# File → Import → Select openapi.yaml
```

---

##### [openapi/grpc-endpoints.md](openapi/grpc-endpoints.md) (30 KB)
**gRPC API Documentation**

- **Format**: Markdown with JSON examples
- **Coverage**: 11 gRPC services, 103 RPC methods
- **Examples**: Request/response samples for all methods
- **Tools**: grpcurl, BloomRPC, Postman (gRPC)

**Services Documented:**

1. **PasswordService** (6 RPCs)
   - Hash, verify, validate strength, check expiry

2. **TokenService** (7 RPCs)
   - Generate, validate, refresh, revoke tokens

3. **MfaService** (6 RPCs)
   - Enable/disable MFA, verify codes, backup codes

4. **RoleService** (7 RPCs)
   - Role CRUD operations, search, list users

5. **PermissionService** (7 RPCs)
   - Permission CRUD operations, search, list roles

6. **RolePermissionService** (4 RPCs)
   - Assign/revoke permissions to roles

7. **UserRoleService** (5 RPCs)
   - Assign/revoke roles to users

8. **PermissionResolutionService** (10 RPCs)
   - Effective permissions, authorization checks, Hybrid RBAC

9. **UserCommandUseCases** (28 RPCs)
   - All user write operations (CQRS Commands)

10. **UserQueryUseCases** (18 RPCs)
    - All user read operations (CQRS Queries)

11. **SessionService** (5 RPCs)
    - Session management, validation, revocation

**How to use:**
```bash
# List all services
grpcurl -plaintext localhost:50052 list

# Call a method
grpcurl -plaintext \
  -H "authorization: Bearer YOUR_TOKEN" \
  -d '{"user_id": "123"}' \
  localhost:50052 \
  sapiens.UserQueryUseCases/GetUser
```

---

##### [openapi/README.md](openapi/README.md) (12 KB)
**Complete API Usage Guide**

Comprehensive guide for using both REST and gRPC APIs.

**Contents:**
- **Quick Start**: Getting started with APIs
- **Authentication**: How to authenticate (REST & gRPC)
- **Testing**:
  - REST: curl examples, Postman collections
  - gRPC: grpcurl, BloomRPC, Postman
- **Code Generation**:
  - REST clients: TypeScript, Python, Go, Java
  - gRPC clients: TypeScript, Python, Go, Rust
- **Troubleshooting**: Common issues and solutions
- **Comparison**: When to use REST vs gRPC
- **Tools & Resources**: Links to documentation and tools

**Quick Links:**
- View REST docs: [Swagger UI Setup](#viewing-rest-api-documentation)
- Test gRPC API: [grpcurl Examples](#testing-grpc-api)
- Generate clients: [Code Generation](#code-generation)

---

### 3. Additional Resources

#### Other Documentation Files

- **[../README.md](../README.md)**: Main Sapiens service README with quick start guide
- **[../REQUIREMENTS_VERIFICATION.md](../REQUIREMENTS_VERIFICATION.md)**: Verification report confirming 100% implementation of all requirements
- **[../docs/archive/](archive/)**: Historical phase completion documentation

---

## 🎯 Quick Navigation by Role

### For Developers

**Starting a new feature?**
1. Read [domain.md](domain.md) - Understand domain model
2. Check [brd.md](brd.md) - Find relevant functional requirements
3. Reference [openapi/](openapi/) - Understand API contracts

**Integrating with Sapiens?**
1. Start with [openapi/README.md](openapi/README.md) - API usage guide
2. Choose protocol:
   - REST: Use [openapi/openapi.yaml](openapi/openapi.yaml)
   - gRPC: Use [openapi/grpc-endpoints.md](openapi/grpc-endpoints.md)
3. Generate client code (instructions in openapi/README.md)

---

### For Product Managers

**Planning features?**
1. Review [brd.md](brd.md) - Business requirements and user stories
2. Check [domain.md](domain.md) - Technical feasibility and constraints

**Tracking progress?**
1. See [../REQUIREMENTS_VERIFICATION.md](../REQUIREMENTS_VERIFICATION.md) - Implementation status

---

### For QA Engineers

**Writing test cases?**
1. Reference [brd.md](brd.md) - Acceptance criteria for each requirement
2. Use [openapi/](openapi/) - API endpoint testing guide

**API testing?**
1. REST: Import [openapi/openapi.yaml](openapi/openapi.yaml) into Postman
2. gRPC: Follow examples in [openapi/grpc-endpoints.md](openapi/grpc-endpoints.md)

---

### For System Architects

**System design?**
1. Read [domain.md](domain.md) - Complete technical architecture
2. Review [brd.md](brd.md) - Non-functional requirements (performance, scalability, security)

**Database design?**
1. See [domain.md](domain.md) - MongoDB schemas with indexes and validation rules

---

### For Security/Compliance Officers

**Security audit?**
1. Review [brd.md](brd.md) - Security requirements (FR-001, FR-003, FR-004, FR-005)
2. Check [domain.md](domain.md) - Password hashing (Argon2id), JWT tokens, MFA implementation

**Audit logging?**
1. See [brd.md](brd.md) - FR-003: Auditing and Logging requirements
2. Check [domain.md](domain.md) - AuditLog entity schema and retention policies

---

## 📊 Documentation Statistics

| Document | Size | Type | Last Updated |
|----------|------|------|--------------|
| domain.md | 75 KB | Technical Spec | 2025-11-19 |
| brd.md | 188 KB | Business Requirements | 2025-11-19 |
| openapi/openapi.yaml | 36 KB | API Spec (REST) | 2025-11-20 |
| openapi/grpc-endpoints.md | 30 KB | API Docs (gRPC) | 2025-11-20 |
| openapi/README.md | 12 KB | API Guide | 2025-11-20 |
| **Total** | **341 KB** | - | - |

---

## 🔗 Related Documentation

### In Parent Directory ([../](../))
- **README.md**: Service overview, quick start, architecture summary
- **REQUIREMENTS_VERIFICATION.md**: Complete verification of implementation vs requirements
- **PROJECT_STATUS.md**: Current project status, metrics, deployment info

### In Source Code
- **../src/domain/**: Domain entities, services, value objects (Rust implementation)
- **../src/application/**: Use cases, command/query handlers (Rust implementation)
- **../src/infrastructure/**: Repositories, MongoDB, external integrations
- **../src/presentation/**: gRPC/REST handlers, API endpoints
- **../proto/services/**: gRPC service proto definitions

### Archive
- **archive/**: Historical phase completion documentation (10 files, historical reference)

---

## 💡 Documentation Best Practices

### When to Update Documentation

**domain.md** - Update when:
- Adding/modifying domain entities
- Changing database schemas
- Adding new domain services
- Modifying business rules

**brd.md** - Update when:
- Adding new functional requirements
- Changing acceptance criteria
- Adding user stories
- Modifying non-functional requirements

**openapi/** - Update when:
- Adding/modifying REST endpoints
- Adding/modifying gRPC methods
- Changing request/response schemas
- Updating authentication methods

---

## 🚀 Getting Started

### New to Sapiens?

**Recommended Reading Order:**

1. **[../README.md](../README.md)** (5 min) - Quick overview
2. **[brd.md](brd.md)** - Executive Summary section (10 min) - Business context
3. **[openapi/README.md](openapi/README.md)** (15 min) - How to use APIs
4. **[domain.md](domain.md)** - Domain Entities section (20 min) - Core concepts

**Total time: ~50 minutes** to understand Sapiens basics.

### Deep Dive?

After the quick start, read:

1. **[domain.md](domain.md)** - Complete read (1-2 hours) - Full technical understanding
2. **[brd.md](brd.md)** - Functional Requirements (1 hour) - All business rules
3. **[openapi/openapi.yaml](openapi/openapi.yaml)** or **[grpc-endpoints.md](openapi/grpc-endpoints.md)** (30 min) - API details

**Total time: 2.5-3.5 hours** for comprehensive understanding.

---

## 📝 Contributing to Documentation

### Documentation Standards

- **Technical docs**: Use clear, precise language with code examples
- **Business docs**: Use plain language, avoid jargon
- **API docs**: Provide request/response examples
- **Always**: Keep docs in sync with code changes

### File Naming Conventions

- Technical specs: `domain.md`, `architecture.md`
- Business docs: `brd.md`, `requirements.md`
- API docs: `openapi.yaml`, `grpc-endpoints.md`
- Guides: `README.md`, `GUIDE.md`

---

## 🔍 Search Tips

### Finding Information Quickly

**Need to find a specific entity?**
```bash
grep -r "User entity" docs/
grep -r "Role" docs/domain.md
```

**Need to find an API endpoint?**
```bash
grep "POST /roles" docs/openapi/openapi.yaml
grep "CreateRole" docs/openapi/grpc-endpoints.md
```

**Need to find a requirement?**
```bash
grep "FR-001" docs/brd.md
grep "Multi-Factor Authentication" docs/brd.md
```

---

## 📞 Support & Feedback

### Questions?

- **Technical questions**: Check [domain.md](domain.md) or [../README.md](../README.md)
- **API questions**: See [openapi/README.md](openapi/README.md)
- **Business questions**: Review [brd.md](brd.md)

### Found an issue?

- Documentation errors: Create an issue in the project repository
- API bugs: See [../README.md](../README.md) for bug reporting process
- Feature requests: Refer to [brd.md](brd.md) for requirements process

---

## 📅 Version History

| Version | Date | Changes |
|---------|------|---------|
| 3.0 | 2025-11-20 | Added comprehensive OpenAPI documentation (REST & gRPC) |
| 2.0 | 2025-11-19 | Updated domain.md and brd.md with complete specifications |
| 1.0 | 2025-01-15 | Initial documentation structure |

---

**Last Updated:** 2025-11-20
**Maintained by:** Sapiens Development Team
**Total Documentation Size:** 341 KB
