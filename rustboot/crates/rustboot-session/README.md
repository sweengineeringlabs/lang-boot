# rustboot-session

Session management abstraction for Rustboot framework with multiple storage backends.

## Features

- **Flexible Storage Backends**: In-memory, Redis, and database storage
- **Session Lifecycle Management**: Create, load, update, touch, and delete sessions
- **Automatic Expiration**: TTL-based session expiration with automatic cleanup
- **Session Security**: Session ID regeneration, secure cookie settings
- **Middleware Integration**: Easy integration with web frameworks
- **Type-Safe Data Storage**: Serialize/deserialize any type implementing Serde
- **Concurrent Access**: Thread-safe session operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dev-engineeringlabs-rustboot-session = "0.1.0"
```

### Optional Features

```toml
[dependencies]
dev-engineeringlabs-rustboot-session = { version = "0.1.0", features = ["redis-store"] }
```

Available features:
- `memory` (default): In-memory session storage
- `redis-store`: Redis session storage

## Quick Start

### Basic Usage

```rust
use rustboot_session::{SessionManager, MemorySessionStore, SessionConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a session manager
    let store = MemorySessionStore::new();
    let config = SessionConfig::default()
        .with_ttl(Duration::from_secs(3600));
    let manager = SessionManager::new(store, config);

    // Create a new session
    let (session_id, _) = manager.create().await?;

    // Update session data
    manager.update(&session_id, |data| {
        data.set("user_id", 42u64)?;
        data.set("username", "alice".to_string())?;
        Ok(())
    }).await?;

    // Load session
    let session = manager.load(&session_id).await?.unwrap();
    let user_id: u64 = session.get("user_id")?.unwrap();

    // Delete session
    manager.delete(&session_id).await?;

    Ok(())
}
```

### Middleware Integration

```rust
use rustboot_session::{SessionManager, SessionMiddleware, MemorySessionStore};

let store = MemorySessionStore::new();
let manager = SessionManager::with_defaults(store);
let middleware = SessionMiddleware::new(manager);

// In a request handler:
let mut context = middleware.load_or_create(None).await?;
context.set("cart_items", vec![1, 2, 3])?;
middleware.save_if_modified(context).await?;
```

### Redis Storage

```rust
use rustboot_session::{SessionManager, RedisSessionStore};

let store = RedisSessionStore::with_defaults("redis://127.0.0.1:6379").await?;
let manager = SessionManager::with_defaults(store);
```

### Database Storage

```rust
use rustboot_session::{SessionManager, DatabaseSessionStore, SessionDatabase};
use std::sync::Arc;

// Implement SessionDatabase for your database
struct MyDatabase { /* ... */ }

#[async_trait::async_trait]
impl SessionDatabase for MyDatabase {
    // Implement required methods
}

let db = Arc::new(MyDatabase::new());
let store = DatabaseSessionStore::with_defaults(db);
let manager = SessionManager::with_defaults(store);
```

## Configuration

```rust
use rustboot_session::SessionConfig;
use std::time::Duration;

let config = SessionConfig::new()
    .with_ttl(Duration::from_secs(3600))
    .with_cookie_name("my_session")
    .with_cookie_domain("example.com")
    .with_cookie_secure(true)
    .with_cleanup_interval(Duration::from_secs(1800));
```

## Automatic Cleanup

```rust
// Enable automatic cleanup of expired sessions
manager.start_cleanup().await;

// Cleanup runs periodically based on config.cleanup_interval
// ...

// Stop cleanup when shutting down
manager.stop_cleanup().await;
```

## Session Regeneration

Regenerate session IDs after authentication to prevent session fixation attacks:

```rust
// After successful login
let new_session_id = manager.regenerate(&old_session_id).await?;
```

## Storage Backends

### In-Memory Store

Best for development and single-server deployments:

```rust
use rustboot_session::MemorySessionStore;

let store = MemorySessionStore::new();
```

**Pros:**
- Fast
- No external dependencies
- Simple setup

**Cons:**
- Sessions lost on restart
- Not suitable for distributed systems

### Redis Store

Best for production and distributed deployments:

```rust
use rustboot_session::RedisSessionStore;

let store = RedisSessionStore::new("redis://127.0.0.1:6379", "session:").await?;
```

**Pros:**
- Persistent across restarts
- Automatic TTL handling
- Suitable for distributed systems
- High performance

**Cons:**
- Requires Redis server
- Additional infrastructure

### Database Store

Best for applications already using a database:

```rust
use rustboot_session::{DatabaseSessionStore, SessionDatabase};

let store = DatabaseSessionStore::new(database, "sessions");
```

**Pros:**
- Persistent across restarts
- No additional infrastructure if database exists
- Query capabilities

**Cons:**
- Slower than Redis or memory
- Requires database schema

## Examples

See the `examples/` directory for complete examples:

- `basic_usage.rs` - Basic session operations
- `auto_cleanup.rs` - Automatic session cleanup
- `middleware_integration.rs` - Middleware integration patterns

Run examples:

```bash
cargo run --example basic_usage
cargo run --example auto_cleanup
cargo run --example middleware_integration
```

## Testing

Run tests:

```bash
# All tests
cargo test

# With Redis features (requires Redis running)
cargo test --features redis-store
```

## License

MIT
