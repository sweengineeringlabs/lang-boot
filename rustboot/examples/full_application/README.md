# Rustboot Full Application Example - TODO List REST API

A comprehensive example demonstrating the integration of multiple Rustboot crates to build a complete, production-ready REST API.

## Overview

This example implements a TODO list REST API with user authentication, showcasing:

- **Web Server**: Axum-based HTTP server using `rustboot-web`
- **Database**: SQLite database access with migrations using `rustboot-database`
- **Session Management**: In-memory session handling using `rustboot-session`
- **Health Checks**: Liveness and readiness probes using `rustboot-health`
- **Middleware**: CORS, security headers, and request logging using `rustboot-middleware`
- **Validation**: Request validation using `rustboot-validation`
- **Configuration**: Hierarchical configuration using `rustboot-config`

## Features

### Authentication & Authorization
- User registration with validation
- Login/logout with session management
- Session-based authentication
- User-specific todo lists

### TODO Management
- Create, read, update, and delete todos
- Filter todos by user (when authenticated)
- Support for public (unauthenticated) todos
- Full CRUD operations with validation

### Observability
- Structured logging with `tracing`
- Health check endpoints
- Database connectivity checks
- HTTP request tracing

### Security
- CORS configuration
- Security headers middleware
- Input validation
- SQL injection prevention (parameterized queries in production)

## Project Structure

```
examples/full_application/
├── Cargo.toml           # Dependencies and build configuration
├── README.md            # This file
└── src/
    └── main.rs          # Complete application code
```

## Prerequisites

- Rust 1.70 or later
- SQLite 3.x (for database)

## Building

```bash
# From the project root
cd examples/full_application

# Build the application
cargo build

# Or build and run
cargo run --bin todo-api
```

## Running

```bash
# Run with default configuration
cargo run --bin todo-api

# Run with custom log level
RUST_LOG=debug cargo run --bin todo-api

# Run with custom port
SERVER_PORT=8080 cargo run --bin todo-api
```

The server will start on `http://127.0.0.1:3000` by default.

## API Endpoints

### Root
- `GET /` - API information and endpoint listing

### Health Check
- `GET /health` - Health check endpoint (returns JSON health status)

### Authentication
- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - Login and create session
- `POST /api/auth/logout` - Logout and destroy session

### TODO Management
- `GET /api/todos` - List all todos (filtered by user if authenticated)
- `POST /api/todos` - Create a new todo
- `GET /api/todos/:id` - Get a specific todo by ID
- `PUT /api/todos/:id` - Update a todo
- `DELETE /api/todos/:id` - Delete a todo

## Usage Examples

### 1. Check API Information

```bash
curl http://localhost:3000/
```

Response:
```json
{
  "success": true,
  "data": {
    "name": "Rustboot TODO API",
    "version": "1.0.0",
    "endpoints": { ... }
  },
  "message": "Welcome to Rustboot TODO API"
}
```

### 2. Health Check

```bash
curl http://localhost:3000/health
```

Response:
```json
{
  "status": "Healthy",
  "checks": [
    {
      "name": "liveness",
      "status": "Healthy",
      "message": "Always healthy"
    },
    {
      "name": "database",
      "status": "Healthy",
      "message": "Database connection is healthy"
    }
  ],
  "version": "1.0.0"
}
```

### 3. Register a New User

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "SecurePass123"
  }'
```

Response:
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "johndoe",
    "email": "john@example.com",
    "created_at": "2024-01-01T12:00:00Z"
  },
  "message": "User registered successfully"
}
```

### 4. Login

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "password": "SecurePass123"
  }'
```

Response:
```json
{
  "success": true,
  "data": {
    "session_id": "abc123...",
    "user_id": "550e8400-e29b-41d4-a716-446655440000"
  },
  "message": "Login successful"
}
```

### 5. Create a TODO

```bash
# Without authentication (public todo)
curl -X POST http://localhost:3000/api/todos \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Buy groceries",
    "description": "Milk, bread, eggs"
  }'

# With authentication (user-specific todo)
curl -X POST http://localhost:3000/api/todos \
  -H "Content-Type: application/json" \
  -H "Cookie: todo_session=abc123..." \
  -d '{
    "title": "Finish project",
    "description": "Complete the Rustboot example"
  }'
```

Response:
```json
{
  "success": true,
  "data": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "title": "Buy groceries",
    "description": "Milk, bread, eggs",
    "completed": false,
    "created_at": "2024-01-01T12:00:00Z",
    "updated_at": "2024-01-01T12:00:00Z",
    "user_id": null
  },
  "message": "Todo created successfully"
}
```

### 6. List TODOs

```bash
# List all todos
curl http://localhost:3000/api/todos

# List user-specific todos (with session)
curl http://localhost:3000/api/todos \
  -H "Cookie: todo_session=abc123..."
```

Response:
```json
{
  "success": true,
  "data": [
    {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "title": "Buy groceries",
      "description": "Milk, bread, eggs",
      "completed": false,
      "created_at": "2024-01-01T12:00:00Z",
      "updated_at": "2024-01-01T12:00:00Z",
      "user_id": null
    }
  ],
  "message": "Todos retrieved successfully"
}
```

### 7. Get a Specific TODO

```bash
curl http://localhost:3000/api/todos/123e4567-e89b-12d3-a456-426614174000
```

### 8. Update a TODO

```bash
curl -X PUT http://localhost:3000/api/todos/123e4567-e89b-12d3-a456-426614174000 \
  -H "Content-Type: application/json" \
  -d '{
    "completed": true
  }'
```

Response:
```json
{
  "success": true,
  "data": null,
  "message": "Todo updated successfully"
}
```

### 9. Delete a TODO

```bash
curl -X DELETE http://localhost:3000/api/todos/123e4567-e89b-12d3-a456-426614174000
```

Response:
```json
{
  "success": true,
  "data": null,
  "message": "Todo deleted successfully"
}
```

## Rustboot Crates Demonstrated

### 1. rustboot-web
- Axum integration for HTTP server
- Route handlers with extractors
- JSON request/response handling
- Path parameters
- State management

### 2. rustboot-database
- SQLite database driver (SQLx)
- Schema creation and migrations
- CRUD operations
- Connection management
- Query execution

### 3. rustboot-session
- In-memory session storage
- Session creation and management
- Session data serialization
- TTL-based expiration
- Session ID generation

### 4. rustboot-health
- Health check aggregator
- Liveness probes
- Database connectivity checks
- Custom health checks
- JSON health reports

### 5. rustboot-middleware
- CORS middleware (ready to add)
- Security headers middleware (ready to add)
- HTTP logging middleware (ready to add)
- Middleware pipeline
- Request/response transformation

### 6. rustboot-validation
- String validation (length, email, patterns)
- Numeric validation (ranges)
- Custom validation rules
- Fluent validation builders
- Error aggregation

### 7. rustboot-config
- Configuration management
- Default values
- Environment variable support
- Type-safe configuration
- Hierarchical settings

## Configuration

The application uses the following default configuration:

```rust
AppConfig {
    server: ServerConfig {
        host: "127.0.0.1",
        port: 3000,
    },
    database: DatabaseConfig {
        url: "sqlite::memory:",
    },
    session: SessionSettings {
        ttl_seconds: 3600,  // 1 hour
        cookie_name: "todo_session",
    },
}
```

Configuration can be customized by:
1. Modifying the `Default` implementation in `main.rs`
2. Loading from environment variables
3. Loading from configuration files (YAML, TOML, JSON)

## Database Schema

### Users Table
```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL
);
```

### Todos Table
```sql
CREATE TABLE todos (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    completed INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    user_id TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

## Security Considerations

**Note**: This example is for demonstration purposes. For production use, consider:

1. **Password Hashing**: Use proper password hashing (bcrypt, argon2)
2. **SQL Injection**: Use parameterized queries (properly, not string formatting)
3. **HTTPS**: Enable TLS/SSL for production
4. **CORS**: Configure appropriate CORS policies
5. **Rate Limiting**: Add rate limiting middleware
6. **Input Sanitization**: Sanitize all user inputs
7. **Session Security**: Use secure session storage (Redis, database)
8. **Authentication**: Consider JWT or OAuth2 for APIs
9. **Authorization**: Implement proper authorization checks
10. **Security Headers**: Enable all security headers in middleware

## Error Handling

The application implements comprehensive error handling:

- **Database Errors**: Wrapped and logged with context
- **Validation Errors**: Returns 400 Bad Request with details
- **Not Found**: Returns 404 Not Found
- **Unauthorized**: Returns 401 Unauthorized
- **Internal Errors**: Returns 500 Internal Server Error

All errors return a consistent JSON response:
```json
{
  "success": false,
  "data": null,
  "message": "Error description"
}
```

## Testing

```bash
# Run tests (if tests are added)
cargo test

# Run with verbose logging
RUST_LOG=debug cargo run --bin todo-api

# Check for compilation errors
cargo check

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## Extending the Example

This example can be extended with:

1. **Persistent Database**: Switch from SQLite in-memory to file-based or PostgreSQL
2. **Redis Sessions**: Use Redis for distributed session storage
3. **Authentication**: Add JWT tokens, OAuth2, or API keys
4. **Authorization**: Implement role-based access control (RBAC)
5. **Pagination**: Add pagination to list endpoints
6. **Filtering**: Add query parameters for filtering todos
7. **Sorting**: Add sorting options for list endpoints
8. **Search**: Implement full-text search for todos
9. **WebSockets**: Add real-time updates using WebSockets
10. **File Uploads**: Add attachment support for todos
11. **Email**: Send notifications for todo updates
12. **Caching**: Add caching layer with rustboot-cache
13. **Messaging**: Add event-driven architecture with rustboot-messaging
14. **Metrics**: Add Prometheus metrics
15. **OpenAPI**: Generate OpenAPI documentation

## Performance Considerations

- **Connection Pooling**: Database connections are pooled for efficiency
- **Async/Await**: All I/O operations are asynchronous
- **Middleware**: Middleware is applied efficiently with Tower
- **Session Storage**: In-memory sessions are fast but not distributed
- **Database**: SQLite in-memory is fast for demos, use PostgreSQL for production

## Troubleshooting

### Database Connection Errors
- Ensure SQLite is installed
- Check database URL configuration
- Verify file permissions (for file-based SQLite)

### Port Already in Use
- Change the port in configuration
- Stop other services using port 3000
- Use `SERVER_PORT=8080` environment variable

### Session Not Working
- Check cookie headers in requests
- Verify session TTL configuration
- Ensure session store is initialized

### Validation Errors
- Check request body format
- Verify all required fields are present
- Review validation rules in code

## License

This example is part of the Rustboot framework and follows the same license.

## Contributing

Contributions are welcome! Please see the main Rustboot repository for contribution guidelines.

## Additional Resources

- [Rustboot Documentation](https://github.com/phdsystems/rustboot)
- [Axum Documentation](https://docs.rs/axum)
- [SQLx Documentation](https://docs.rs/sqlx)
- [Tokio Documentation](https://docs.rs/tokio)

## Support

For questions and support:
- Open an issue in the Rustboot repository
- Check the documentation
- Review other examples in the `examples/` directory

---

**Happy coding with Rustboot!**
