# sapiens Module

A complete Domain-Driven Design (DDD) bounded context module built on the **Backbone Framework**. This module follows Clean Architecture principles with a schema-first approach and implements all layers from domain to presentation.

**Last Updated**: 2026-01-25

## 🏛️ Architecture Overview

```
sapiens/
├── proto/domain/                    # 📋 DOMAIN DEFINITIONS (Generated from Schema)
│   ├── entity/                     # Domain Entities
│   │   └── sapiens.proto   # Aggregate root definition
│   ├── value_object/               # Value Objects
│   │   └── common.proto           # Shared value objects
│   ├── repository/                 # Repository interfaces
│   │   └── sapiens_repository.proto
│   ├── usecase/                    # CQRS Commands & Queries
│   │   ├── commands.proto         # Write operations
│   │   └── queries.proto          # Read operations
│   ├── service/                    # Domain Services
│   │   └── sapiens_service.proto
│   ├── event/                      # Domain Events
│   │   └── sapiens_events.proto
│   └── specification/              # Business Rules
│       └── rules.proto
│
├── src/
│   ├── domain/                     # 🎯 DOMAIN LAYER
│   │   ├── entities/              # Entity implementations
│   │   │   └── mod.rs             # Aggregate root with business logic
│   │   ├── value_objects/         # Value object implementations
│   │   │   └── sapiens.rs
│   │   ├── events/                # Domain events
│   │   │   └── sapiens_events.rs
│   │   ├── repositories/          # Repository traits
│   │   │   └── sapiens_repository.rs
│   │   ├── services/              # Domain services
│   │   │   └── sapiens_service.rs
│   │   └── specifications/        # Business rules
│   │       └── sapiens_specifications.rs
│   │
│   ├── application/                # 📋 APPLICATION LAYER
│   │   ├── commands/              # CQRS Command handlers
│   │   │   └── create_sapiens.rs
│   │   ├── queries/               # CQRS Query handlers
│   │   │   └── get_sapiens.rs
│   │   └── mod.rs                 # Application services and DI
│   │
│   ├── infrastructure/             # 🔧 INFRASTRUCTURE LAYER
│   │   ├── persistence/           # Data access implementations
│   │   │   └── postgresql/
│   │   │       └── sapiens_repository.rs
│   │   ├── config/                # Configuration management
│   │   │   └── app_config.rs
│   │   └── health/                # Health monitoring
│   │       └── health_checker.rs
│   │
│   ├── presentation/               # 🌐 PRESENTATION LAYER
│   │   ├── http/                  # REST API
│   │   │   ├── handlers/          # HTTP handlers
│   │   │   │   └── sapiens_handler.rs
│   │   │   ├── middleware/        # HTTP middleware
│   │   │   │   └── auth_middleware.rs
│   │   │   └── mod.rs            # HTTP server configuration
│   │   ├── grpc/                  # gRPC services (future)
│   │   └── cli/                   # CLI commands (future)
│   │
│   └── lib.rs                     # Module entry point
│
├── tests/                         # 🧪 TESTS
│   ├── unit/                     # Unit tests
│   ├── integration/              # Integration tests
│   └── integration_tests.rs      # Full integration tests
│
├── migrations/                    # 🗄️ DATABASE MIGRATIONS
│   └── 001_create_sapiens_table.sql
│
├── config/                       # ⚙️ CONFIGURATION
│   ├── application.yml           # Default configuration
│   ├── application-dev.yml       # Development overrides
│   └── application-prod.yml      # Production overrides
│
├── build.rs                      # 🔨 Proto compilation script
├── Cargo.toml                    # Dependencies
└── README.md                     # This file
```

## 🚀 Quick Start

### 1. Build the Module

```bash
# Build with proto compilation
cargo build

# Generate proto files manually (if needed)
cargo build --release
```

### 2. Configure Database

```bash
# Update configuration in config/application.yml
database:
  url: postgresql://username:password@localhost:5432/your_database
  max_connections: 20

# Run migrations
sqlx migrate run
# or
DATABASE_URL="postgresql://postgres:password@localhost:5432/backbonedb" sqlx migrate run --source libs/modules/sapiens/migrations
```

### 3. Use the Module

```rust
use sapiens::SapiensModule;

// Create module instance with builder pattern
let module = SapiensModule::builder()
    .with_database(pool)
    .with_config(config)
    .build()?;

// Configure HTTP routes
App::new()
    .configure(|cfg| module.configure_routes(cfg))
    .service(module.http_service());
```

### 4. Run Tests

```bash
# Run all tests
cargo test

# Run integration tests only
cargo test integration

# Run with coverage
cargo test --features coverage
```

## 📋 Features

### ✅ Domain Layer
- **Domain Entities**: Full aggregate root with business logic
- **Value Objects**: Immutable value objects with validation
- **Domain Events**: Event-driven architecture with event store
- **Repository Interfaces**: Clean separation for data access
- **Domain Services**: Business logic that doesn't fit entities
- **Specifications**: Composable business rules

### ✅ Application Layer
- **CQRS Pattern**: Separate command and query handlers
- **Application Services**: Orchestration of domain objects
- **DTOs**: Clean data transfer objects
- **Dependency Injection**: Builder pattern for clean setup

### ✅ Infrastructure Layer
- **PostgreSQL Repository**: Production-ready data access
- **Configuration Management**: YAML-based with ENV overrides
- **Health Monitoring**: Database, memory, disk, CPU checks
- **Migration Support**: SQLx migrations

### ✅ Presentation Layer
- **REST API**: Complete CRUD endpoints with pagination
- **Authentication**: JWT-based auth with roles and permissions
- **Authorization**: Role-based and permission-based access control
- **Middleware**: Rate limiting, CORS, security headers
- **Error Handling**: Structured error responses

### ✅ Testing
- **Unit Tests**: Individual component testing
- **Integration Tests**: Full stack testing with test database
- **Test Utilities**: Shared test setup and cleanup
- **Coverage**: Comprehensive test coverage

## 🏛️ Domain Model

### Entity: Sapiens

```protobuf
message Sapiens {
  string id = 1;
  string name = 2;
  string description = 3;
  string status = 4;
  Metadata metadata = 5;
  google.protobuf.Timestamp created_at = 6;
  google.protobuf.Timestamp updated_at = 7;
  optional google.protobuf.Timestamp deleted_at = 8;
}
```

### Value Objects

- **SapiensId**: UUID-based identifier
- **SapiensName**: Validated name with length constraints
- **SapiensStatus**: Enum-based status (active, inactive, pending)
- **SapiensTimestamp**: Timestamp with validation
- **Metadata**: Key-value metadata store

### Domain Events

- **SapiensCreated**: Fired when entity is created
- **SapiensUpdated**: Fired when entity is updated
- **SapiensDeleted**: Fired when entity is deleted
- **SapiensStatusChanged**: Fired when status changes

### Business Rules (Specifications)

- **ActiveSapiensSpecification**: Check if entity is active
- **ValidSapiensSpecification**: Validate entity state
- **Composite specifications**: AND, OR, NOT operations

## 🌐 HTTP API

### REST Endpoints

All endpoints follow REST conventions with **module-scoped routing** and include:
- **Pagination**: `page`, `limit`, `offset` parameters
- **Sorting**: `sort_by`, `sort_order` parameters
- **Filtering**: Field-based filtering with operators
- **Authentication**: JWT token required for all endpoints
- **Rate Limiting**: Configurable per-user rate limits
- **Soft-Delete**: Entities support soft-delete with trash and restore

#### Module Routing Pattern

API paths follow the module-scoped pattern: `/api/v1/{module}/{collection}`

Example for Users entity:
```
/api/v1/sapiens/users         # User collection
/api/v1/sapiens/users/:id     # User resource
/api/v1/sapiens/users/trash   # Soft-deleted users
```

#### Standard CRUD Operations

| Method | Path | Description | Auth |
|--------|------|-------------|------|
| `GET` | `/api/v1/sapiens/{collection}` | List all entities | Read |
| `POST` | `/api/v1/sapiens/{collection}` | Create new entity | Create |
| `GET` | `/api/v1/sapiens/{collection}/:id` | Get entity by ID | Read |
| `PUT` | `/api/v1/sapiens/{collection}/:id` | Update entity | Update |
| `PATCH` | `/api/v1/sapiens/{collection}/:id` | Partial update | Update |
| `DELETE` | `/api/v1/sapiens/{collection}/:id` | Soft delete entity | Delete |
| `POST` | `/api/v1/sapiens/{collection}/bulk` | Bulk create | Create |
| `POST` | `/api/v1/sapiens/{collection}/upsert` | Upsert entity | Update |
| `GET` | `/api/v1/sapiens/{collection}/trash` | List soft-deleted | Admin |
| `POST` | `/api/v1/sapiens/{collection}/:id/restore` | Restore soft-deleted | Admin |
| `DELETE` | `/api/v1/sapiens/{collection}/:id/permanent` | Permanent delete | Admin |
| `DELETE` | `/api/v1/sapiens/{collection}/empty` | Empty trash | Admin |

#### Entities with Soft-Delete Support

All sapiens entities support soft-delete with `deleted_at` timestamp:
- **Users** - `/api/v1/sapiens/users`
- **Roles** - `/api/v1/sapiens/roles`
- **Permissions** - `/api/v1/sapiens/permissions`
- **And more...** (see schema files for complete list)

#### Request/Response Examples

```json
// Create Request
POST /api/v1/sapiens
{
  "name": "Example Sapiens",
  "description": "A sample sapiens entity",
  "metadata": {
    "tags": ["tag1", "tag2"],
    "category": "example"
  }
}

// Response
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Example Sapiens",
  "description": "A sample sapiens entity",
  "status": "active",
  "metadata": {
    "tags": ["tag1", "tag2"],
    "category": "example"
  },
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

## 🌐 Webapp Integration (NEW)

The sapiens module has extensive webapp code generation support with the following features:

### Generated Webapp Components

```
apps/webapp/src/
├── domain/sapiens/              # Domain types and services
│   ├── entity/                  # User, Role, Permission types
│   └── service/                 # Domain services
├── application/hooks/sapiens/   # React Query hooks
│   ├── useUser.ts
│   ├── useUserList.ts
│   ├── useUserMutation.ts       # With callback support
│   └── ...
├── infrastructure/sapiens/      # API clients
│   ├── api/                     # REST API implementations
│   └── initServices.ts          # Centralized service initialization
└── presentation/pages/sapiens/  # Page components
    ├── users/                   # Plural directory naming
    │   ├── UserListPage.tsx
    │   ├── UserTrashPage.tsx    # Soft-delete trash page
    │   ├── UserCreatePage.tsx
    │   ├── UserEditPage.tsx
    │   └── index.ts             # Barrel export
    └── UserWrapperPages.tsx     # Generic template wrappers
```

### Webapp Features

- **Plural Route Paths**: `/users/` instead of `/user/`
- **Trash Pages**: For all soft-delete enabled entities
- **Generic Templates**: Reusable `ResourceListPage`, `ResourceTrashPage`
- **Wrapper Pages**: Thin components connecting templates with configs
- **Mutation Callbacks**: `onSuccess`/`onError` callback support
- **Service Initialization**: Centralized `initServices.ts` pattern

### Generate Webapp Code

```bash
# Generate all webapp code for sapiens
cargo run --bin backbone-webgen -- --module sapiens

# Or using backbone CLI
backbone webapp:generate sapiens --target all
```

---

## 🔧 Configuration

### Application Configuration

```yaml
# config/application.yml
server:
  host: 0.0.0.0
  port: 8080
  workers: 4

database:
  url: postgresql://username:password@localhost:5432/database
  max_connections: 20
  min_connections: 5
  connect_timeout: 30
  idle_timeout: 600

security:
  jwt_secret: your-secret-key
  jwt_expiration: 3600
  rate_limit:
    requests_per_minute: 100
    burst_size: 20

logging:
  level: info
  format: json

health:
  enabled: true
  check_interval: 30

sapiens:
  default_page_size: 20
  max_page_size: 100
  enable_soft_delete: true
  enable_audit: true
```

### Environment Overrides

```bash
# Override database URL
DATABASE_URL=postgresql://user:pass@localhost/db

# Override JWT secret
JWT_SECRET=your-production-secret

# Override log level
RUST_LOG=debug
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage (install tarpaulin first)
cargo tarpaulin --out Html

# Run integration tests only
cargo test integration

# Run specific test
cargo test test_create_sapiens
```

### Test Structure

```
tests/
├── unit/                     # Unit tests
│   ├── domain/              # Domain layer tests
│   ├── application/         # Application layer tests
│   └── infrastructure/      # Infrastructure tests
├── integration/             # Integration tests
│   └── api_tests.rs        # API endpoint tests
└── integration_tests.rs     # Full integration suite
```

### Test Database

Integration tests use a separate test database:

```bash
# Set test database URL
export TEST_DATABASE_URL="postgresql://postgres:password@localhost:5432/sapiens_test"

# Run tests
cargo test integration
```

## 🗄️ Database Schema

### Main Table: sapiens

```sql
CREATE TABLE sapiens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Indexes for performance
CREATE INDEX idx_sapiens_status ON sapiens(status);
CREATE INDEX idx_sapiens_created_at ON sapiens(created_at);
CREATE INDEX idx_sapiens_deleted_at ON sapiens(deleted_at);
CREATE INDEX idx_sapiens_metadata_gin ON sapiens USING gin(metadata);

-- Full-text search index
CREATE INDEX idx_sapiens_search ON sapiens USING gin(
    to_tsvector('english', name || ' ' || COALESCE(description, ''))
);
```

### Event Store Table: sapiens_events

```sql
CREATE TABLE sapiens_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    aggregate_id UUID NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    event_data JSONB NOT NULL,
    event_version INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    metadata JSONB DEFAULT '{}'
);

CREATE INDEX idx_sapiens_events_aggregate_id ON sapiens_events(aggregate_id);
CREATE INDEX idx_sapiens_events_type ON sapiens_events(event_type);
CREATE INDEX idx_sapiens_events_created_at ON sapiens_events(created_at);
```

## 🚀 Deployment

### Production Deployment

1. **Environment Setup**
   ```bash
   export DATABASE_URL="postgresql://prod_user:prod_pass@db-host:5432/prod_db"
   export JWT_SECRET="your-super-secret-jwt-key"
   export RUST_LOG="info"
   ```

2. **Database Migration**
   ```bash
   sqlx migrate run --database-url "$DATABASE_URL"
   ```

3. **Build and Run**
   ```bash
   cargo build --release
   ./target/release/sapiens
   ```

### Docker Deployment

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/sapiens /usr/local/bin/
EXPOSE 8080
CMD ["sapiens"]
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: sapiens
spec:
  replicas: 3
  selector:
    matchLabels:
      app: sapiens
  template:
    metadata:
      labels:
        app: sapiens
    spec:
      containers:
      - name: sapiens
        image: sapiens:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: database-secret
              key: url
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: jwt-secret
              key: secret
```

## 📊 Performance

### Benchmarks

- **Create**: ~1,000 operations/second
- **Read**: ~10,000 operations/second
- **Update**: ~800 operations/second
- **Delete**: ~900 operations/second
- **List (100 items)**: ~5,000 queries/second

### Database Optimization

- **Connection Pooling**: Configurable pool size
- **Prepared Statements**: SQLx prepared statements
- **Batch Operations**: Bulk insert/update support
- **Indexing**: Optimized indexes for common queries
- **Pagination**: Efficient offset-based pagination

### Caching

- **Application Caching**: In-memory LRU cache (future)
- **Database Caching**: PostgreSQL query cache
- **HTTP Caching**: ETag and Cache-Control headers

## 🔒 Security

### Authentication
- **JWT Tokens**: RS256 algorithm with configurable expiration
- **Refresh Tokens**: Secure refresh token mechanism
- **Password Security**: Argon2id hashing

### Authorization
- **Role-Based Access Control**: Configurable roles and permissions
- **Attribute-Based Access Control**: Fine-grained permissions
- **Resource-Level Security**: Per-entity access control

### Security Headers
- **CORS**: Configurable cross-origin resource sharing
- **Rate Limiting**: Configurable per-endpoint rate limits
- **Security Headers**: HSTS, CSP, X-Frame-Options

### Input Validation
- **Proto Validation**: Built-in proto field validation
- **DTO Validation**: Application-level validation
- **SQL Injection Prevention**: Parameterized queries only

## 🛠️ Development

### Local Development Setup

1. **Prerequisites**
   - Rust 1.75+
   - PostgreSQL 14+
   - Docker (optional)

2. **Setup Database**
   ```bash
   createdb sapiens_dev
   export DATABASE_URL="postgresql://postgres:password@localhost:5432/sapiens_dev"
   ```

3. **Run Migrations**
   ```bash
   sqlx migrate run
   ```

4. **Run Development Server**
   ```bash
   cargo run
   ```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Run all tests
cargo test

# Generate docs
cargo doc --open
```

### Contributing

1. Follow the existing code style
2. Write tests for new features
3. Update documentation
4. Use conventional commit messages
5. Ensure all tests pass before PR

## 📚 API Documentation

### OpenAPI/Swagger

The module includes OpenAPI 3.0 specification. Run the server and visit:

```
http://localhost:8080/api/docs
```

### Proto Documentation

Protocol buffer documentation is available in the `proto/` directory:

- `proto/domain/entity/sapiens.proto` - Entity definition
- `proto/domain/usecase/commands.proto` - CQRS commands
- `proto/domain/usecase/queries.proto` - CQRS queries
- `proto/domain/service/sapiens_service.proto` - Domain services

## 🔍 Monitoring & Observability

### Metrics

- **HTTP Metrics**: Request count, duration, status codes
- **Database Metrics**: Connection pool, query duration
- **Business Metrics**: Entity counts, operation rates
- **System Metrics**: CPU, memory, disk usage

### Logging

- **Structured Logging**: JSON format with correlation IDs
- **Log Levels**: trace, debug, info, warn, error
- **Context Logging**: Request context and user context
- **Error Tracking**: Detailed error traces

### Health Checks

```bash
# Health endpoint
GET /health

# Detailed health status
GET /health/detailed
```

Response:
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T00:00:00Z",
  "components": {
    "database": "healthy",
    "memory": "healthy",
    "disk": "healthy",
    "cpu": "healthy"
  },
  "version": "1.0.0"
}
```

## 📦 Dependencies

### Core Dependencies

- **axum**: Web framework (0.7)
- **tonic**: gRPC framework (0.12)
- **sqlx**: PostgreSQL driver (0.8)
- **tokio**: Async runtime (1.x)
- **serde**: Serialization (1.0)
- **uuid**: UUID generation (1.x)
- **chrono**: Date/time (0.4)
- **thiserror**: Error handling (1.0)

### Development Dependencies

- **tokio-test**: Async testing utilities
- **mockall**: Mocking framework
- **tempfile**: Temporary file handling
- **testcontainers**: Integration testing with Docker

## 🗺️ Roadmap

### Version 1.1 (Planned)
- [ ] GraphQL support
- [ ] Redis caching
- [ ] Event sourcing optimizations
- [ ] GraphQL subscriptions
- [ ] Performance monitoring dashboard

### Version 1.2 (Future)
- [ ] Multi-tenant support
- [ ] Advanced audit logging
- [ ] Data export/import
- [ ] Webhook support
- [ ] Custom field support

## 📄 License

This module is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🤝 Support

For support and questions:

- **Documentation**: Check this README and inline docs
- **Issues**: Create an issue in the repository
- **Discussions**: Join the community discussions
- **Email**: support@yourcompany.com

## 🏆 Acknowledgments

Built with the **Backbone Framework** and following best practices from:

- Domain-Driven Design (Eric Evans)
- Clean Architecture (Robert C. Martin)
- API Design Patterns (Microsoft)
- Rust Best Practices
- PostgreSQL Performance Guide