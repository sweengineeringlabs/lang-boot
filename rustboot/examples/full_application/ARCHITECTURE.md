# Architecture Documentation

## Overview

This document describes the architecture and design decisions for the Rustboot Full Application Example, a TODO List REST API demonstrating integration of multiple Rustboot framework crates.

## System Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Client                               │
│                    (HTTP Requests)                           │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                    Axum Web Server                           │
│                  (rustboot-web)                              │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐    │
│  │            Middleware Layer                          │    │
│  │  - Request Tracing (tower-http)                     │    │
│  │  - CORS (ready to add)                              │    │
│  │  - Security Headers (ready to add)                  │    │
│  │  - HTTP Logging (ready to add)                      │    │
│  └─────────────────────────────────────────────────────┘    │
│                         ▼                                    │
│  ┌─────────────────────────────────────────────────────┐    │
│  │              Route Handlers                          │    │
│  │  - Root (/)                                         │    │
│  │  - Health (/health)                                 │    │
│  │  - Auth (/api/auth/*)                               │    │
│  │  - TODOs (/api/todos/*)                             │    │
│  └─────────────────────────────────────────────────────┘    │
└────────────┬─────────────────────────┬────────────────┬─────┘
             │                         │                │
             ▼                         ▼                ▼
┌────────────────────┐  ┌────────────────────┐  ┌────────────────────┐
│  Session Manager   │  │  Database Layer    │  │  Health Checks     │
│  (rustboot-session)│  │  (rustboot-database│  │  (rustboot-health) │
│                    │  │                    │  │                    │
│  - Memory Store    │  │  - SQLx Driver     │  │  - Liveness        │
│  - TTL Management  │  │  - SQLite          │  │  - Database Check  │
│  - User Sessions   │  │  - Migrations      │  │                    │
└────────────────────┘  └────────────────────┘  └────────────────────┘
                                 │
                                 ▼
                        ┌────────────────────┐
                        │  SQLite Database   │
                        │  - users table     │
                        │  - todos table     │
                        └────────────────────┘
```

## Component Details

### 1. Web Server Layer (rustboot-web + Axum)

**Responsibilities:**
- HTTP request routing
- Request/response serialization
- Handler execution
- State management

**Key Components:**
- `Router`: Axum router with defined routes
- `HandlerContext`: Request context and extractors
- `Json`: JSON serialization/deserialization
- `Path`: URL path parameter extraction
- `State`: Application state sharing

**Design Decisions:**
- Uses Axum for production-grade HTTP server
- Async/await throughout for non-blocking I/O
- Tower layers for middleware composition
- Type-safe extractors for request data

### 2. Database Layer (rustboot-database)

**Responsibilities:**
- Database connection management
- Schema creation and migration
- CRUD operations
- Transaction support

**Key Components:**
- `SqlxDatabase`: SQLx-based database driver
- `Database` trait: Generic database operations
- Schema: Users and TODOs tables

**Schema Design:**

```sql
-- Users table
CREATE TABLE users (
    id TEXT PRIMARY KEY,              -- UUID
    username TEXT NOT NULL UNIQUE,    -- Unique username
    email TEXT NOT NULL UNIQUE,       -- Unique email
    password_hash TEXT NOT NULL,      -- Hashed password
    created_at TEXT NOT NULL          -- ISO 8601 timestamp
);

-- TODOs table
CREATE TABLE todos (
    id TEXT PRIMARY KEY,              -- UUID
    title TEXT NOT NULL,              -- TODO title
    description TEXT,                 -- Optional description
    completed INTEGER NOT NULL DEFAULT 0,  -- Boolean (0/1)
    created_at TEXT NOT NULL,         -- ISO 8601 timestamp
    updated_at TEXT NOT NULL,         -- ISO 8601 timestamp
    user_id TEXT,                     -- Optional foreign key to users
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Indexes for performance
CREATE INDEX idx_todos_user_id ON todos(user_id);
CREATE INDEX idx_todos_completed ON todos(completed);
```

**Design Decisions:**
- SQLite for simplicity (can be swapped for PostgreSQL/MySQL)
- In-memory database for demo (configurable)
- UUIDs for IDs (cross-database compatibility)
- Separate user and public todos

### 3. Session Management (rustboot-session)

**Responsibilities:**
- Session creation and lifecycle
- User authentication state
- Session data storage
- TTL-based expiration

**Key Components:**
- `SessionManager`: Session lifecycle management
- `MemorySessionStore`: In-memory session storage
- `SessionConfig`: Configuration (TTL, cookie settings)
- `SessionId`: Unique session identifiers

**Session Flow:**

```
1. User Login
   └─> Create Session
       └─> Store user_id in session data
           └─> Return session_id to client

2. Authenticated Request
   └─> Extract session_id from cookie
       └─> Load session data
           └─> Retrieve user_id
               └─> Use for authorization

3. Session Expiration
   └─> TTL-based automatic cleanup
       └─> Configurable expiration time
```

**Design Decisions:**
- In-memory storage for demo (Redis/Database for production)
- Cookie-based session transport
- Automatic expiration with TTL
- Session regeneration for security

### 4. Health Checks (rustboot-health)

**Responsibilities:**
- Application health monitoring
- Dependency health checks
- Readiness/liveness probes

**Key Components:**
- `HealthAggregator`: Combines multiple checks
- `AlwaysHealthyCheck`: Basic liveness probe
- `FunctionCheck`: Custom check (database)
- `HealthReport`: JSON health status

**Health Check Types:**

```
Liveness Check:
- Purpose: Is the application running?
- Implementation: AlwaysHealthyCheck
- Response: Always healthy

Database Check:
- Purpose: Can we query the database?
- Implementation: FunctionCheck with SELECT 1
- Response: Healthy if query succeeds

Future Checks:
- Session store connectivity
- External API availability
- Disk space
- Memory usage
```

**Design Decisions:**
- Multiple independent checks
- Async check execution
- JSON response format
- Standard HTTP status codes (200/503)

### 5. Validation Layer (rustboot-validation)

**Responsibilities:**
- Input validation
- Business rule enforcement
- Error aggregation

**Key Components:**
- `StringValidationBuilder`: String validation
- `Validator` trait: Generic validation
- Custom validation rules

**Validation Rules:**

```rust
User Registration:
- Username: 3-50 chars, not empty
- Email: Valid email format, not empty
- Password: Minimum 8 chars, not empty

TODO Creation:
- Title: 1-200 chars, not empty
- Description: Max 1000 chars (optional)

TODO Update:
- At least one field must be provided
- Same validation as creation
```

**Design Decisions:**
- Builder pattern for fluent validation
- Early validation (fail fast)
- Descriptive error messages
- Type-safe validation builders

### 6. Configuration (rustboot-config)

**Responsibilities:**
- Application configuration
- Environment-based settings
- Default values

**Configuration Structure:**

```rust
AppConfig {
    server: {
        host: String,    // Bind address
        port: u16,       // Port number
    },
    database: {
        url: String,     // Database connection string
    },
    session: {
        ttl_seconds: u64,      // Session lifetime
        cookie_name: String,   // Session cookie name
    }
}
```

**Design Decisions:**
- Serde for serialization
- Default implementations
- Environment override capability
- Type-safe configuration structs

### 7. Middleware (rustboot-middleware)

**Current Implementation:**
- Request tracing with tower-http
- Structured logging with tracing

**Ready to Add:**
- CORS middleware
- Security headers
- HTTP logging
- Rate limiting
- Request/response compression

**Design Decisions:**
- Tower middleware architecture
- Composable middleware pipeline
- Zero-cost abstractions
- Framework-agnostic core

## Data Flow

### 1. User Registration Flow

```
Client
  │
  ├─> POST /api/auth/register
  │   Body: { username, email, password }
  │
  ▼
Axum Handler (register_handler)
  │
  ├─> Validate request
  │   └─> StringValidationBuilder
  │
  ├─> Check username uniqueness
  │   └─> Database query
  │
  ├─> Create user
  │   └─> INSERT INTO users
  │
  ▼
Response
  └─> 201 Created
      Body: { success, data: user, message }
```

### 2. Login Flow

```
Client
  │
  ├─> POST /api/auth/login
  │   Body: { username, password }
  │
  ▼
Axum Handler (login_handler)
  │
  ├─> Find user
  │   └─> SELECT FROM users WHERE username = ?
  │
  ├─> Verify password
  │   └─> Compare hashes (simplified in demo)
  │
  ├─> Create session
  │   └─> SessionManager.create()
  │       └─> MemorySessionStore
  │
  ├─> Store user_id in session
  │   └─> SessionManager.update()
  │
  ▼
Response
  └─> 200 OK
      Body: { session_id, user_id }
```

### 3. Authenticated TODO Creation Flow

```
Client
  │
  ├─> POST /api/todos
  │   Cookie: todo_session=<session_id>
  │   Body: { title, description }
  │
  ▼
Axum Handler (create_todo_handler)
  │
  ├─> Extract session from cookie
  │   └─> get_session_user_id()
  │       └─> SessionManager.load()
  │
  ├─> Validate TODO request
  │   └─> validate_create_todo()
  │
  ├─> Create TODO with user_id
  │   └─> INSERT INTO todos
  │       └─> Include user_id from session
  │
  ▼
Response
  └─> 201 Created
      Body: { success, data: todo, message }
```

### 4. Health Check Flow

```
Client
  │
  ├─> GET /health
  │
  ▼
Axum Handler (health_handler)
  │
  ├─> Execute all health checks
  │   ├─> Liveness check (always healthy)
  │   └─> Database check
  │       └─> SELECT 1 query
  │
  ├─> Aggregate results
  │   └─> HealthAggregator.check()
  │
  ▼
Response
  └─> 200 OK (if all healthy)
      or 503 Service Unavailable (if any unhealthy)
      Body: { status, checks[], version, duration_ms }
```

## Error Handling

### Error Types

```rust
ApiError:
- Database(String)      -> 500 Internal Server Error
- NotFound(String)      -> 404 Not Found
- Validation(String)    -> 400 Bad Request
- Session(String)       -> 500 Internal Server Error
- Unauthorized(String)  -> 401 Unauthorized
- Internal(String)      -> 500 Internal Server Error
```

### Error Response Format

```json
{
  "success": false,
  "data": null,
  "message": "Error description"
}
```

### Error Flow

```
Error Occurrence
  │
  ├─> Create ApiError with context
  │
  ├─> IntoResponse trait
  │   └─> Map to HTTP status code
  │   └─> Serialize to JSON
  │
  ├─> Log error (via tracing)
  │
  ▼
HTTP Response
  └─> Appropriate status code
      └─> JSON error body
```

## Security Considerations

### Current Implementation

1. **Input Validation**: All inputs validated before processing
2. **Type Safety**: Rust's type system prevents many errors
3. **Session Management**: UUID-based session IDs
4. **Database**: Async driver with connection pooling

### Production Recommendations

1. **Password Hashing**: Use bcrypt/argon2 (not plain text!)
2. **SQL Injection**: Use parameterized queries (not string formatting)
3. **HTTPS**: Enable TLS/SSL in production
4. **CORS**: Configure appropriate CORS policies
5. **Rate Limiting**: Add rate limiting middleware
6. **Session Storage**: Use Redis or database (not in-memory)
7. **Security Headers**: Enable CSP, HSTS, etc.
8. **Authentication**: Consider JWT or OAuth2
9. **Authorization**: Implement proper access control
10. **Audit Logging**: Log security-relevant events

## Performance Considerations

### Current Design

- **Async I/O**: All operations are non-blocking
- **Connection Pooling**: Database connections are pooled
- **Stateless Handlers**: No shared mutable state
- **Zero-Copy**: Minimal data copying with Arc

### Scalability

```
Vertical Scaling:
- Increase tokio worker threads
- Tune connection pool sizes
- Optimize database queries

Horizontal Scaling:
- Deploy multiple instances
- Use external session store (Redis)
- Load balancer in front
- Distributed database

Caching:
- Add rustboot-cache layer
- Cache frequently accessed data
- Use Redis for distributed cache
```

## Testing Strategy

### Unit Tests
- Validation functions
- Error handling
- Session helpers
- Health check logic

### Integration Tests
- API endpoint tests
- Database operations
- Session management
- Health checks

### End-to-End Tests
- Full user workflows
- Authentication flows
- CRUD operations
- Error scenarios

### Test Script
- `test_api.sh`: Automated API testing
- Covers all major endpoints
- Validates responses

## Deployment

### Local Development

```bash
# In-memory database
cargo run --bin todo-api

# With file-based database
DATABASE_URL=sqlite:todos.db cargo run --bin todo-api
```

### Production Deployment

```bash
# Build release binary
cargo build --release

# Run with production config
DATABASE_URL=postgresql://... \
SESSION_STORE=redis://... \
cargo run --release --bin todo-api
```

### Docker Deployment

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/todo-api /usr/local/bin/
CMD ["todo-api"]
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: todo-api
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: todo-api
        image: todo-api:latest
        ports:
        - containerPort: 3000
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: db-secret
              key: url
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
        readinessProbe:
          httpGet:
            path: /health
            port: 3000
```

## Monitoring and Observability

### Logging

- Structured logging with `tracing`
- Different log levels (trace, debug, info, warn, error)
- Request/response logging
- Performance metrics

### Metrics (Future)

- Request count
- Response times
- Error rates
- Database query times
- Session count
- Active connections

### Tracing (Future)

- Distributed tracing
- Request correlation
- Performance profiling

## Future Enhancements

### Short Term
1. Add CORS middleware configuration
2. Enable security headers
3. Implement proper password hashing
4. Add pagination to list endpoints
5. Add filtering and sorting

### Medium Term
1. Switch to PostgreSQL
2. Add Redis for sessions
3. Implement JWT authentication
4. Add WebSocket support
5. Add file upload capability

### Long Term
1. Microservices architecture
2. Event-driven design with rustboot-messaging
3. GraphQL API
4. Admin dashboard
5. Mobile app support

## References

- [Rustboot Framework](https://github.com/phdsystems/rustboot)
- [Axum Documentation](https://docs.rs/axum)
- [SQLx Documentation](https://docs.rs/sqlx)
- [Tower Documentation](https://docs.rs/tower)
- [Tokio Documentation](https://docs.rs/tokio)

---

This architecture is designed to be:
- **Modular**: Easy to swap components
- **Testable**: Clear separation of concerns
- **Scalable**: Handles growth gracefully
- **Maintainable**: Clean code structure
- **Educational**: Demonstrates best practices
