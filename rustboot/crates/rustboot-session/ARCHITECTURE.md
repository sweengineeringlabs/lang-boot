# rustboot-session Architecture

## Overview

The `rustboot-session` crate provides a comprehensive session management abstraction with multiple storage backends. It follows a trait-based design pattern that allows easy extension and integration with different storage systems.

## Core Components

### 1. Session ID (`SessionId`)

**Location**: `src/session_id.rs`

A type-safe wrapper around UUID v4 session identifiers.

**Features**:
- UUID v4 generation for cryptographically secure session IDs
- Validation on creation
- String conversion utilities

**Usage**:
```rust
let id = SessionId::generate();
let id_from_string = SessionId::from_string("uuid-here")?;
```

### 2. Session Data (`SessionData`)

**Location**: `src/session_data.rs`

A container for session values with metadata tracking.

**Features**:
- Type-safe key-value storage using serde
- Automatic timestamp tracking (created_at, last_accessed)
- TTL-based expiration
- JSON serialization/deserialization
- Touch functionality for activity tracking

**Storage**:
- Values stored as `serde_json::Value`
- Supports any type implementing `Serialize`/`Deserialize`
- Metadata stored as UNIX timestamps (seconds)

**Usage**:
```rust
let mut data = SessionData::new();
data.set("user_id", 42u64)?;
data.set("cart", vec![1, 2, 3])?;

let user_id: u64 = data.get("user_id")?.unwrap();
```

### 3. Session Store Trait (`SessionStore`)

**Location**: `src/store.rs`

The core abstraction that defines how sessions are persisted.

**Methods**:
- `load(&self, session_id: &SessionId) -> SessionResult<Option<SessionData>>`
- `save(&self, session_id: &SessionId, data: SessionData) -> SessionResult<()>`
- `delete(&self, session_id: &SessionId) -> SessionResult<()>`
- `exists(&self, session_id: &SessionId) -> SessionResult<bool>`
- `cleanup_expired(&self) -> SessionResult<usize>`
- `count(&self) -> SessionResult<usize>`
- `clear(&self) -> SessionResult<()>`

**Design Pattern**: Async trait-based abstraction allowing multiple implementations

### 4. Storage Implementations

#### Memory Store (`MemorySessionStore`)

**Location**: `src/memory_store.rs`

**Implementation**:
- Uses `Arc<RwLock<HashMap<SessionId, SessionData>>>`
- Thread-safe with tokio's async RwLock
- Automatic expiration checking on load
- Manual cleanup via `cleanup_expired()`

**Use Cases**:
- Development environments
- Single-server deployments
- Testing
- Small-scale applications

**Limitations**:
- Sessions lost on restart
- Not suitable for distributed systems
- Memory usage grows with session count

#### Redis Store (`RedisSessionStore`)

**Location**: `src/redis_store.rs`

**Implementation**:
- Uses Redis connection manager for connection pooling
- Leverages Redis TTL for automatic expiration
- Key prefix for namespace isolation
- Connection-based transactions

**Use Cases**:
- Production environments
- Distributed deployments
- High-traffic applications
- Microservices architectures

**Features**:
- Automatic TTL management
- Persistent across restarts
- Horizontal scaling support
- Minimal cleanup needed (Redis handles TTL)

#### Database Store (`DatabaseSessionStore`)

**Location**: `src/database_store.rs`

**Implementation**:
- Generic trait-based design (`SessionDatabase`)
- Works with any database implementing the trait
- Manual schema management
- Query-based operations

**Use Cases**:
- Applications already using a database
- Need for complex queries on session data
- Audit trail requirements
- Existing infrastructure constraints

**Schema**:
```sql
CREATE TABLE sessions (
    id VARCHAR(255) PRIMARY KEY,
    data TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    last_accessed BIGINT NOT NULL,
    expires_at BIGINT
);
```

### 5. Session Manager (`SessionManager`)

**Location**: `src/manager.rs`

High-level session lifecycle management.

**Features**:
- Session creation with automatic ID generation
- Load, save, update, and delete operations
- Session regeneration for security
- Automatic cleanup task with configurable interval
- Session touching for activity tracking

**Design**:
- Wraps a `SessionStore` implementation
- Manages cleanup background task
- Provides high-level API for common operations

**Usage**:
```rust
let manager = SessionManager::new(store, config);

// Start automatic cleanup
manager.start_cleanup().await;

// Create session
let (id, data) = manager.create().await?;

// Update session
manager.update(&id, |data| {
    data.set("key", "value")?;
    Ok(())
}).await?;

// Regenerate (after login)
let new_id = manager.regenerate(&old_id).await?;
```

### 6. Session Middleware (`SessionMiddleware`)

**Location**: `src/middleware.rs`

Integration layer for web frameworks.

**Components**:
- `SessionContext`: Request-scoped session container
- `SessionMiddleware`: Manager wrapper with middleware utilities

**Features**:
- Lazy session loading
- Automatic save on modification
- Cookie extraction utilities
- Request/response integration helpers

**Design Pattern**: Middleware adapter pattern

**Usage**:
```rust
let middleware = SessionMiddleware::new(manager);

// In request handler:
let mut context = middleware.load_or_create(Some(&session_id)).await?;
context.set("user_id", 42u64)?;
let final_id = middleware.save_if_modified(context).await?;
```

### 7. Session Configuration (`SessionConfig`)

**Location**: `src/store.rs`

Configuration for session behavior and cookie settings.

**Fields**:
- `default_ttl`: Default session expiration time
- `cookie_name`: Session cookie name
- `cookie_path`: Cookie path
- `cookie_domain`: Cookie domain
- `cookie_secure`: HTTPS-only flag
- `cookie_http_only`: JavaScript access prevention
- `cookie_same_site`: CSRF protection
- `cleanup_interval`: Automatic cleanup frequency

**Builder Pattern**:
```rust
let config = SessionConfig::new()
    .with_ttl(Duration::from_secs(3600))
    .with_cookie_name("my_session")
    .with_cookie_secure(true)
    .with_cleanup_interval(Duration::from_secs(1800));
```

## Data Flow

### Session Creation Flow
1. User request arrives without session cookie
2. Middleware calls `load_or_create(None)`
3. Manager generates new `SessionId`
4. Creates empty `SessionData` with TTL
5. Saves to store
6. Returns `SessionContext` to handler
7. Handler modifies session
8. Middleware saves if modified
9. Cookie with session ID sent to client

### Session Load Flow
1. User request arrives with session cookie
2. Middleware extracts session ID
3. Manager loads from store
4. Store checks expiration
5. Returns `SessionData` if valid, `None` if expired/missing
6. Context created and passed to handler

### Cleanup Flow
1. Cleanup task wakes up (based on interval)
2. Calls `cleanup_expired()` on store
3. Store iterates sessions, removes expired ones
4. Returns count of removed sessions
5. Task sleeps until next interval

## Security Considerations

### Session Fixation Prevention
- Use `regenerate()` after authentication state changes
- Generates new session ID while preserving data
- Deletes old session

### Cookie Security
- `HttpOnly` flag prevents JavaScript access
- `Secure` flag ensures HTTPS-only transmission
- `SameSite` attribute protects against CSRF
- Configurable domain and path for isolation

### Session Expiration
- TTL-based expiration at data level
- Automatic cleanup removes expired sessions
- Touch mechanism extends session lifetime on activity

## Performance Characteristics

### Memory Store
- **Read**: O(1) with RwLock contention
- **Write**: O(1) with RwLock contention
- **Cleanup**: O(n) where n = total sessions
- **Memory**: O(n * session_size)

### Redis Store
- **Read**: O(1) network latency + Redis GET
- **Write**: O(1) network latency + Redis SET
- **Cleanup**: O(1) (Redis handles TTL)
- **Memory**: Offloaded to Redis

### Database Store
- **Read**: O(1) query time + network latency
- **Write**: O(1) query time + network latency
- **Cleanup**: O(n) table scan
- **Memory**: Offloaded to database

## Extension Points

### Custom Storage Backend
Implement the `SessionStore` trait:

```rust
struct MyCustomStore;

#[async_trait]
impl SessionStore for MyCustomStore {
    async fn load(&self, id: &SessionId) -> SessionResult<Option<SessionData>> {
        // Custom implementation
    }

    // ... other methods
}
```

### Custom Database Implementation
Implement the `SessionDatabase` trait for your database:

```rust
struct MyDatabase;

#[async_trait]
impl SessionDatabase for MyDatabase {
    async fn load_session(&self, table: &str, id: &SessionId)
        -> SessionResult<Option<SessionData>> {
        // Database-specific query
    }

    // ... other methods
}
```

## Testing Strategy

### Unit Tests
- Each module has comprehensive unit tests
- Mock implementations for database store
- Concurrent access testing for thread safety

### Integration Tests
- End-to-end session lifecycle tests
- Expiration and cleanup verification
- Middleware integration patterns
- Complex data type serialization

### Example Tests
- Basic usage patterns
- Automatic cleanup behavior
- Middleware request handling

## Future Enhancements

Potential areas for expansion:
- **Cookie Store**: Client-side encrypted sessions
- **Distributed Locking**: For race condition prevention
- **Session Sharding**: For horizontal scaling
- **Encryption**: At-rest encryption for sensitive data
- **Compression**: For large session data
- **Monitoring**: Metrics and observability hooks
- **Rate Limiting**: Per-session rate limits
- **Garbage Collection**: Advanced cleanup strategies
