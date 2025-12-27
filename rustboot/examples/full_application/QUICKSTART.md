# Quick Start Guide

Get the Rustboot TODO API up and running in under 5 minutes!

## Prerequisites

- Rust 1.70 or later
- curl (for testing)
- jq (optional, for pretty JSON output)

## Step 1: Build the Application

```bash
cd examples/full_application
cargo build
```

This will download dependencies and compile the application (first build may take a few minutes).

## Step 2: Run the Server

```bash
cargo run --bin todo-api
```

You should see output like:

```
INFO rustboot_full_application: Starting Rustboot Full Application Example - TODO API
INFO rustboot_full_application: Server configuration: 127.0.0.1:3000
INFO rustboot_full_application: Connecting to database: sqlite::memory:
INFO rustboot_full_application: Setting up database schema...
INFO rustboot_full_application: Database schema setup complete
INFO rustboot_full_application: Initializing session manager
INFO rustboot_full_application: Configuring health checks
INFO rustboot_full_application: Starting server on 127.0.0.1:3000
```

Leave this running in your terminal.

## Step 3: Test the API

Open a new terminal and try these commands:

### Check if the server is running

```bash
curl http://localhost:3000/
```

Expected response:
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

### Check health status

```bash
curl http://localhost:3000/health
```

Expected response:
```json
{
  "status": "Healthy",
  "checks": [
    {
      "name": "liveness",
      "status": "Healthy"
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

### Create a TODO

```bash
curl -X POST http://localhost:3000/api/todos \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My first TODO",
    "description": "This is a test TODO"
  }'
```

Expected response:
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "My first TODO",
    "description": "This is a test TODO",
    "completed": false,
    "created_at": "2024-01-01T12:00:00Z",
    "updated_at": "2024-01-01T12:00:00Z",
    "user_id": null
  },
  "message": "Todo created successfully"
}
```

### List all TODOs

```bash
curl http://localhost:3000/api/todos
```

## Step 4: Run the Full Test Suite

We've provided a comprehensive test script that demonstrates all API features:

```bash
chmod +x test_api.sh
./test_api.sh
```

This script will:
1. Check the root endpoint
2. Check health status
3. Register a new user
4. Login to create a session
5. Create public and user-specific TODOs
6. List TODOs
7. Get a specific TODO
8. Update a TODO
9. Delete a TODO

## Common Commands

### Run with debug logging

```bash
RUST_LOG=debug cargo run --bin todo-api
```

### Run with custom port

```bash
# Edit the default configuration in src/main.rs
# Or use environment variables (if implemented)
```

### Build release version

```bash
cargo build --release
./target/release/todo-api
```

## API Endpoints Summary

### Authentication
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login
- `POST /api/auth/logout` - Logout

### TODOs
- `GET /api/todos` - List all TODOs
- `POST /api/todos` - Create a TODO
- `GET /api/todos/:id` - Get a specific TODO
- `PUT /api/todos/:id` - Update a TODO
- `DELETE /api/todos/:id` - Delete a TODO

### System
- `GET /` - API information
- `GET /health` - Health check

## Example Workflows

### 1. Creating a User Account

```bash
# Register
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "email": "alice@example.com",
    "password": "SecurePass123"
  }'

# Login
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "password": "SecurePass123"
  }'

# Save the session_id from the response for authenticated requests
```

### 2. Managing TODOs

```bash
# Create a TODO
curl -X POST http://localhost:3000/api/todos \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Learn Rustboot",
    "description": "Complete the tutorial"
  }'

# Update it
curl -X PUT http://localhost:3000/api/todos/<TODO_ID> \
  -H "Content-Type: application/json" \
  -d '{
    "completed": true
  }'

# Delete it
curl -X DELETE http://localhost:3000/api/todos/<TODO_ID>
```

### 3. Using Authentication

```bash
# Create a user-specific TODO (requires session)
curl -X POST http://localhost:3000/api/todos \
  -H "Content-Type: application/json" \
  -H "Cookie: todo_session=<SESSION_ID>" \
  -d '{
    "title": "Private TODO",
    "description": "Only visible to my account"
  }'

# List only my TODOs
curl http://localhost:3000/api/todos \
  -H "Cookie: todo_session=<SESSION_ID>"
```

## Troubleshooting

### Port 3000 already in use

If you get an error about port 3000 being in use:

1. Stop the process using port 3000
2. Or change the port in `src/main.rs` (in the `Default` implementation for `AppConfig`)

### Database errors

The application uses an in-memory SQLite database by default. If you see database errors:

1. Check that SQLite is installed
2. Try restarting the application
3. Check the logs for more details

### Compilation errors

If you encounter compilation errors:

1. Ensure you're in the `examples/full_application` directory
2. Run `cargo clean` and try again
3. Check that all dependencies are available

### Request fails with "Connection refused"

Make sure the server is running! Check that you see the startup logs in your terminal.

## What's Next?

Now that you have the basic API running, you can:

1. **Explore the Code**: Check out `src/main.rs` to see how everything works
2. **Read the README**: `README.md` has detailed API documentation
3. **Check the Architecture**: `ARCHITECTURE.md` explains the design
4. **Modify the Code**: Try adding new features!
5. **Use in Production**: See deployment recommendations in `ARCHITECTURE.md`

## Learning Resources

- **Rustboot Crates Used**:
  - `rustboot-web` - Web server and routing
  - `rustboot-database` - Database access
  - `rustboot-session` - Session management
  - `rustboot-health` - Health checks
  - `rustboot-middleware` - Request/response middleware
  - `rustboot-validation` - Input validation
  - `rustboot-config` - Configuration management

- **External Dependencies**:
  - Axum - HTTP server framework
  - SQLx - Database driver
  - Tokio - Async runtime
  - Serde - Serialization

## Getting Help

If you run into issues:

1. Check the logs (use `RUST_LOG=debug` for verbose output)
2. Review the `README.md` for detailed documentation
3. Check the examples in individual rustboot crates
4. Open an issue on the Rustboot GitHub repository

---

**Happy coding with Rustboot!** 🚀
