# Sapiens API Documentation

**Version:** 2.0.0 | **Last Updated:** 2025-11-22 | **Status:** ✅ Production Ready

This directory contains comprehensive API documentation for the Sapiens RBAC service.

---

## 🆕 What's New in v2.0

**Updated User Schema** (2025-11-22):
- ✅ Separate name fields: `first_name`, `middle_name`, `last_name`
- ✅ Computed `full_name` field (backward compatible)
- ✅ Updated OpenAPI specification
- ✅ New Postman v2 collections with comprehensive test scenarios
- ✅ Complete migration guide

**See:** [API_SCHEMA_UPDATE_v2.md](API_SCHEMA_UPDATE_v2.md) for details.

---

## 📁 Contents

### 1. [openapi.yaml](openapi.yaml)
**OpenAPI 3.0.3 Specification for REST API**

- **Format:** YAML
- **Endpoints:** 25 REST endpoints
- **Specification:** OpenAPI 3.0.3 (formerly Swagger)
- **Use with:** Swagger UI, Redoc, Postman, Stoplight

**What it covers:**
- Role CRUD operations (7 endpoints)
- Permission CRUD operations (7 endpoints)
- Role-Permission assignments (4 endpoints)
- User-Role assignments (3 endpoints)
- User-Permission grants (4 endpoints - Hybrid RBAC)

### 2. [grpc-endpoints.md](grpc-endpoints.md)
**gRPC API Documentation (Markdown)**

- **Format:** Markdown
- **Services:** 28 gRPC services (11 Core + 17 CRUD)
- **Methods:** 275 RPC methods (97% implemented)
- **Use with:** grpcurl, BloomRPC, Postman (gRPC support)

**Core Services (11 services, 103 RPCs):**
- PasswordService (6 RPCs) ✅ 100%
- TokenService (7 RPCs) ✅ 100%
- MfaService (6 RPCs)
- RoleService (7 RPCs)
- PermissionService (7 RPCs)
- RolePermissionService (4 RPCs)
- UserRoleService (5 RPCs)
- PermissionResolutionService (10 RPCs) ✅ 100%
- UserCommandUseCases (27 RPCs) 🟢 74% (20/27 implemented)
- UserQueryUseCases (21 RPCs) 🟢 90% (19/21 implemented)
- SessionService (5 RPCs)

**Implementation Progress (Core Domain Services):**
| Service | Implemented | Total | Status |
|---------|-------------|-------|--------|
| PermissionResolutionService | 10 | 10 | ✅ 100% |
| UserCommandUseCases | 20 | 27 | 🟢 74% |
| UserQueryUseCases | 19 | 21 | 🟢 90% |
| **Total Core Domain** | **49** | **58** | **84%** |

**CRUD Services (17 services, 204 RPCs - 12 methods each):**
- UserCrudService, RoleCrudService, PermissionCrudService
- SessionCrudService, AuditLogCrudService, PolicyCrudService
- PolicyAssignmentCrudService, MfaDeviceCrudService
- PasswordResetTokenCrudService, SystemSettingsCrudService
- UserRoleCrudService, UserPermissionCrudService
- RolePermissionCrudService, UserSessionCrudService
- UserPreferenceCrudService, UserSettingsCrudService
- UserSecurityEventCrudService

**Each CRUD service provides 12 standard methods:**
- Create, Get, Update, Delete, List, Search
- PartialUpdate (PATCH), BulkCreate, Upsert
- ListDeleted (trash), Restore, EmptyTrash

---

## 🚀 Quick Start

### Viewing REST API Documentation

#### Option 1: Swagger UI (Recommended)
```bash
# Install Swagger UI CLI
npm install -g swagger-ui-cli

# Serve the OpenAPI spec
swagger-ui serve openapi.yaml

# Open browser at http://localhost:8080
```

#### Option 2: Redoc (Clean UI)
```bash
# Install Redoc CLI
npm install -g redoc-cli

# Serve the documentation
redoc-cli serve openapi.yaml

# Open browser at http://localhost:8080
```

#### Option 3: Online Viewers
Visit [Swagger Editor](https://editor.swagger.io/) and paste the contents of `openapi.yaml`.

#### Option 4: Postman Collections (Recommended for Testing)

**v2.0 Collections (Updated Schema):**
- [sapiens-rest-api-v2.postman_collection.json](sapiens-rest-api-v2.postman_collection.json) - REST API with comprehensive scenarios
- [sapiens-grpc-api-v2.postman_collection.json](sapiens-grpc-api-v2.postman_collection.json) - gRPC API with test examples

**Import to Postman:**
1. Open Postman
2. Click **Import** → **Upload Files**
3. Select the v2 collection
4. Set collection variables (`baseUrl`, etc.)
5. Run test scenarios

**Test Scenarios Included:**
- Scenario 1: Complete User Lifecycle (Create → Login → Update → Delete)
- Scenario 2: Name Variations (Single name, First+Last, All three names)
- Scenario 3: Data Migration Verification

**See:** [API_SCHEMA_UPDATE_v2.md](API_SCHEMA_UPDATE_v2.md) for detailed testing guide.

#### Option 5: VS Code
Install the **OpenAPI (Swagger) Editor** extension and open `openapi.yaml`.

### Testing REST API

#### Using curl
```bash
# Login to get JWT token
curl -X POST http://localhost:3003/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email_or_username": "root",
    "password": "PiQS5SVL012D"
  }'

# Use token for authenticated requests
TOKEN="your_jwt_token_here"

# List roles
curl -X GET http://localhost:3003/api/v1/roles \
  -H "Authorization: Bearer $TOKEN"

# Create role
curl -X POST http://localhost:3003/api/v1/roles \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "content_editor",
    "description": "Can create and edit content"
  }'
```

#### Using Postman
1. Import `openapi.yaml` into Postman
2. Set environment variable `baseUrl` = `http://localhost:3003/api/v1`
3. Authenticate via `/auth/login` endpoint
4. Save JWT token as environment variable
5. Test other endpoints using the token

---

## 🔌 gRPC API Usage

### Prerequisites
Install gRPC tools:

```bash
# grpcurl (CLI tool)
brew install grpcurl  # macOS
go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest  # Linux/Windows

# BloomRPC (GUI tool)
# Download from https://github.com/bloomrpc/bloomrpc/releases

# Postman (has gRPC support)
# Download from https://www.postman.com/downloads/
```

### Using grpcurl

#### List all services
```bash
grpcurl -plaintext localhost:50052 list
```

#### List methods for a service
```bash
grpcurl -plaintext localhost:50052 list sapiens.UserQueryUseCases
```

#### Describe a method
```bash
grpcurl -plaintext localhost:50052 describe sapiens.UserQueryUseCases.GetUser
```

#### Call a method
```bash
# Authenticate first
grpcurl -plaintext \
  -d '{"email_or_username": "root", "password": "PiQS5SVL012D"}' \
  localhost:50052 \
  sapiens.UserQueryUseCases/ValidateUserCredentials

# Use JWT token for authenticated requests
grpcurl -plaintext \
  -H "authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"user_id": "550e8400-e29b-41d4-a716-446655440000"}' \
  localhost:50052 \
  sapiens.UserQueryUseCases/GetUser
```

### Using BloomRPC

1. **Launch BloomRPC**
2. **Import Proto Files:**
   - Click "+" button
   - Navigate to `libs/modules/sapiens/proto/services/`
   - Select all `.proto` files
3. **Set Server Address:** `localhost:50052`
4. **Select Service and Method** from sidebar
5. **Add Metadata** (for authentication):
   ```
   authorization: Bearer YOUR_JWT_TOKEN
   ```
6. **Edit Request JSON** and click "Play" button

### Using Postman (gRPC)

1. **Create New gRPC Request**
2. **Enter Server URL:** `localhost:50052`
3. **Import Proto Files:**
   - Method definition → Use server reflection OR
   - Import proto files from `libs/modules/sapiens/proto/services/`
4. **Select Service and Method**
5. **Add Metadata:**
   ```
   authorization: Bearer YOUR_JWT_TOKEN
   ```
6. **Edit Request Body** and click "Invoke"

---

## 🔐 Authentication

Both REST and gRPC APIs use **JWT Bearer Token** authentication.

### Getting a Token

#### REST API
```bash
curl -X POST http://localhost:3003/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email_or_username": "root",
    "password": "PiQS5SVL012D"
  }'
```

**Response:**
```json
{
  "success": true,
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

#### gRPC API
```bash
grpcurl -plaintext \
  -d '{"email_or_username": "root", "password": "PiQS5SVL012D"}' \
  localhost:50052 \
  sapiens.UserQueryUseCases/ValidateUserCredentials
```

### Using the Token

#### REST API
Add `Authorization` header:
```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

#### gRPC API
Add `authorization` metadata:
```
authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

---

## 🌐 Server Endpoints

### Development

**REST API:**
```
http://localhost:3003/api/v1
```

**gRPC API:**
```
localhost:50052 (h2c - HTTP/2 cleartext)
```

### Production

**REST API:**
```
https://api.startapp.id/sapiens/api/v1
```

**gRPC API:**
```
grpc.startapp.id:50052 (h2 - HTTP/2 TLS)
```

---

## 📖 API Comparison: REST vs gRPC

| Feature | REST API | gRPC API |
|---------|----------|----------|
| **Protocol** | HTTP/1.1 | HTTP/2 |
| **Format** | JSON | Protocol Buffers |
| **Endpoints** | 25+ (expandable per entity) | 275 methods (28 services) |
| **Use Case** | Public APIs, Web browsers | Internal microservices, high performance |
| **Documentation** | OpenAPI 3.0 (YAML) | Proto files + Markdown |
| **Tools** | Swagger UI, Postman, curl | grpcurl, BloomRPC, Postman |
| **Authentication** | JWT Bearer (Header) | JWT Bearer (Metadata) |
| **Performance** | Good | Excellent (binary protocol) |

### When to Use REST
- ✅ Public-facing APIs
- ✅ Web browser access
- ✅ Simple CRUD operations
- ✅ Wide tooling support
- ✅ Human-readable JSON

### When to Use gRPC
- ✅ Internal microservice communication
- ✅ High-performance requirements
- ✅ Type-safe contracts (proto files)
- ✅ Streaming support
- ✅ Language-agnostic client generation

---

## 🧪 Default Credentials (Development)

**Root User:**
- Username: `root`
- Email: `root@startapp.id`
- Password: `PiQS5SVL012D`
- Roles: `admin`

**⚠️ WARNING:** Change these credentials in production!

---

## 📚 Additional Documentation

- **Main README:** `../../README.md`
- **Requirements:** `../domain.md`, `../brd.md`
- **Verification Report:** `../../REQUIREMENTS_VERIFICATION.md`
- **RBAC REST API Examples:** `../RBAC_REST_API.md`
- **Proto Files:** `../../proto/services/`

---

## 🛠️ Code Generation

### Generate API Clients

#### REST API (from OpenAPI spec)

**JavaScript/TypeScript:**
```bash
npm install -g @openapitools/openapi-generator-cli

openapi-generator-cli generate \
  -i openapi.yaml \
  -g typescript-axios \
  -o ./generated/rest-client
```

**Python:**
```bash
openapi-generator-cli generate \
  -i openapi.yaml \
  -g python \
  -o ./generated/rest-client
```

**Go:**
```bash
openapi-generator-cli generate \
  -i openapi.yaml \
  -g go \
  -o ./generated/rest-client
```

#### gRPC API (from proto files)

**JavaScript/TypeScript:**
```bash
npm install -g grpc-tools

grpc_tools_node_protoc \
  --js_out=import_style=commonjs:./generated \
  --grpc_out=grpc_js:./generated \
  --proto_path=../../proto/services \
  ../../proto/services/*.proto
```

**Python:**
```bash
pip install grpcio-tools

python -m grpc_tools.protoc \
  --python_out=./generated \
  --grpc_python_out=./generated \
  --proto_path=../../proto/services \
  ../../proto/services/*.proto
```

**Go:**
```bash
protoc \
  --go_out=./generated \
  --go-grpc_out=./generated \
  --proto_path=../../proto/services \
  ../../proto/services/*.proto
```

**Rust:**
```bash
# Already configured in build.rs
cargo build
```

---

## 🐛 Troubleshooting

### REST API Issues

**Problem:** 401 Unauthorized
- **Solution:** Ensure JWT token is included in `Authorization: Bearer <token>` header
- **Check:** Token expiration (default: 1 hour)

**Problem:** 404 Not Found
- **Solution:** Verify base URL is `http://localhost:3003/api/v1`
- **Check:** Service is running on port 3003

**Problem:** CORS errors (browser)
- **Solution:** CORS is configured for development origins
- **Check:** Request from allowed origin or update CORS config

### gRPC API Issues

**Problem:** Connection refused
- **Solution:** Ensure gRPC server is running on port 50052
- **Check:** `docker-compose ps sapiens-service`

**Problem:** Method not found
- **Solution:** Verify service and method names are correct (case-sensitive)
- **Check:** Use `grpcurl -plaintext localhost:50052 list` to see available services

**Problem:** gRPC reflection not available
- **Solution:** Reflection is enabled by default in development
- **Check:** Server logs for reflection service registration

---

## 📝 Examples

### Complete REST API Flow

```bash
# 1. Login
TOKEN=$(curl -s -X POST http://localhost:3003/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email_or_username": "root", "password": "PiQS5SVL012D"}' \
  | jq -r '.access_token')

# 2. Create role
ROLE_ID=$(curl -s -X POST http://localhost:3003/api/v1/roles \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "editor", "description": "Content editor"}' \
  | jq -r '.role.id')

# 3. Create permission
PERM_ID=$(curl -s -X POST http://localhost:3003/api/v1/permissions \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "articles:create", "resource": "articles", "action": "create"}' \
  | jq -r '.permission.id')

# 4. Assign permission to role
curl -X POST "http://localhost:3003/api/v1/roles/$ROLE_ID/permissions" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"permission_id\": \"$PERM_ID\"}"

# 5. List role permissions
curl -X GET "http://localhost:3003/api/v1/roles/$ROLE_ID/permissions" \
  -H "Authorization: Bearer $TOKEN"
```

### Complete gRPC API Flow

```bash
# 1. Authenticate
grpcurl -plaintext \
  -d '{"email_or_username": "root", "password": "PiQS5SVL012D"}' \
  localhost:50052 \
  sapiens.UserQueryUseCases/ValidateUserCredentials

# 2. Create role
grpcurl -plaintext \
  -H "authorization: Bearer YOUR_TOKEN" \
  -d '{"name": "editor", "description": "Content editor"}' \
  localhost:50052 \
  sapiens.RoleService/CreateRole

# 3. Create permission
grpcurl -plaintext \
  -H "authorization: Bearer YOUR_TOKEN" \
  -d '{"name": "articles:create", "resource": "articles", "action": "create"}' \
  localhost:50052 \
  sapiens.PermissionService/CreatePermission

# 4. Assign permission to role
grpcurl -plaintext \
  -H "authorization: Bearer YOUR_TOKEN" \
  -d '{"role_id": "ROLE_ID", "permission_id": "PERM_ID"}' \
  localhost:50052 \
  sapiens.RolePermissionService/AssignPermissionToRole

# 5. Get role permissions
grpcurl -plaintext \
  -H "authorization: Bearer YOUR_TOKEN" \
  -d '{"role_id": "ROLE_ID"}' \
  localhost:50052 \
  sapiens.RolePermissionService/GetRolePermissions
```

---

## 🔗 Useful Links

- **OpenAPI Specification:** https://spec.openapis.org/oas/v3.0.3
- **gRPC Documentation:** https://grpc.io/docs/
- **Protocol Buffers:** https://protobuf.dev/
- **grpcurl GitHub:** https://github.com/fullstorydev/grpcurl
- **BloomRPC:** https://github.com/bloomrpc/bloomrpc
- **Swagger UI:** https://swagger.io/tools/swagger-ui/
- **Redoc:** https://github.com/Redocly/redoc
- **Postman:** https://www.postman.com/

---

**Last Updated:** 2025-11-21
**API Version:** 1.1.0
**Maintained by:** Sapiens Development Team
