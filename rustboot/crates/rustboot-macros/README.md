# Rustboot Macros

Procedural macros for the Rustboot framework providing ergonomic derive and attribute macros.

## Status

✅ **18 Production-Ready Macros** - Complete implementation with comprehensive tests

## Available Macros

### Derive Macros (4)

#### `#[derive(Injectable)]`
Automatic dependency injection registration.

```rust
#[derive(Injectable)]
struct UserService {
    repository: Arc<dyn UserRepository>,
    logger: Arc<Logger>,
}

// Generated: from_container() and register() methods
```

#### `#[derive(Validate)]`
Automatic validation implementation based on field attributes.

```rust
#[derive(Validate)]
struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    name: String,
    
    #[validate(email)]
    email: String,
    
    #[validate(range(min = 18, max = 120))]
    age: u8,
}
```

**Supported validators**: `email`, `length(min, max)`, `range(min, max)`

#### `#[derive(Builder)]` ⭐ NEW
Builder pattern generation with fluent API.

```rust
#[derive(Builder)]
struct Config {
    host: String,
    port: u16,
    database: String,
}

// Usage:
let config = Config::builder()
    .host("localhost".to_string())
    .port(5432)
    .database("mydb".to_string())
    .build()?;
```

#### `#[derive(Event)]` ⭐ NEW
Event trait implementation for messaging systems.

```rust
#[derive(Event)]
struct UserCreated {
    user_id: u64,
    timestamp: DateTime,
}

// Generated: event_type(), event_version(), to_message()
```

### Attribute Macros (14)

#### Caching & Performance

**`#[cached]`** - Method-level result caching with TTL

```rust
#[cached(ttl = 300, capacity = 1000)]
async fn get_user(&self, id: u64) -> Result<User> {
    // Result cached for 5 minutes
}
```

**`#[memoize]`** ⭐ NEW - Permanent memoization for pure functions

```rust
#[memoize]
fn fibonacci(n: u64) -> u64 {
    if n <= 1 { n } else { fibonacci(n-1) +fibonacci(n-2) }
}
```

#### Observability

**`#[traced]`** - Automatic tracing with parameter logging

```rust
#[traced(level = "info")]
async fn process_order(&self, order: Order) -> Result<()> {
    // Logs entry, parameters, duration, exit
}

#[traced(level = "debug", skip = ["password"])]
async fn authenticate(&self, username: &str, password: &str) -> Result<Token> {
    // Sensitive params excluded from logs
}
```

**`#[timed]`** - Performance timing and metrics

```rust
#[timed(name = "user_query", slow_threshold = 100)]
async fn query_users(&self) -> Result<Vec<User>> {
    // Records duration, warns if > 100ms
}
```

**`#[metrics_histogram]`** ⭐ NEW - Histogram metrics recording

```rust
#[metrics_histogram(name = "api_latency", buckets = [10, 50, 100, 500])]
async fn api_call() -> Result<()> {
    // Records to Prometheus histogram
}
```

**`#[audit]`** - Audit logging for sensitive operations

```rust
#[audit(action = "delete_user", severity = "high")]
async fn delete_user(&self, user_id: u64) -> Result<()> {
    // Automatically audited
}
```

#### Resilience

**`#[retry]`** - Advanced retry with backoff strategies ⭐

```rust
#[retry(max_attempts = 3, backoff = "exponential", delay = 100)]
async fn call_external_api(&self) -> Result<Response> {
    // Retried with exponential backoff
}

#[retry(max_attempts = 5, backoff = "fibonacci", jitter = true)]
async fn flaky_operation(&self) -> Result<Data> {
    // Fibonacci backoff with random jitter
}
```

**Backoff strategies**: `fixed`, `exponential`, `fibonacci`

**`#[circuit_breaker]`** - Circuit breaker pattern ⭐

```rust
#[circuit_breaker(failure_threshold = 5, timeout = 60)]
async fn call_external(&self) -> Result<Response> {
    // Opens after 5 failures, retries after 60s
}
```

**`#[timeout]`** ⭐ NEW - Async timeout wrapper

```rust
#[timeout(duration = 5000)]
async fn slow_operation() -> Result<Data> {
    // Times out after 5 seconds
}
```

**`#[rate_limit]`** - Rate limiting

```rust
#[rate_limit(requests = 100, window = 60)]
async fn api_endpoint(&self) -> Result<Response> {
    // 100 requests per minute
}
```

#### Security & Database

**`#[authorized]`** ⭐ NEW - Authorization checks

```rust
#[authorized(role = "admin")]
async fn delete_user(&self, id: u64) -> Result<()> {
    // Only admins can execute
}

#[authorized(permission = "write")]
async fn update_data(&self) -> Result<()> {
    // Permission-based
}

#[authorized(require_all = ["read", "write"])]
async fn admin_action(&self) -> Result<()> {
    // Multiple permissions required
}
```

**`#[transactional]`** ⭐ NEW - Database transaction wrapper

```rust
#[transactional]
async fn create_user(&self, user: User) -> Result<User> {
    // Auto commit on success, rollback on error
}
```

**`#[validate_params]`** - Parameter validation

```rust
#[validate_params]
fn create_user(
    #[validate(length(min = 3))] name: String,
    #[validate(email)] email: String,
) -> Result<User> {
    // Parameters validated before execution
}
```

#### Feature Management

**`#[feature_flag]`** ⭐ NEW - Feature toggle conditional execution

```rust
#[feature_flag(flag = "new_ui")]
fn new_feature() -> Response {
    // Only executes if feature enabled
}
```

## Macro Composition

Combine multiple macros for comprehensive cross-cutting concerns:

```rust
#[traced(level = "info")]           // Tracing
#[timed(slow_threshold = 200)]      // Performance monitoring
#[retry(max_attempts = 3)]          // Resilience
#[cached(ttl = 600)]                // Caching
async fn get_user_profile(&self, id: u64) -> Result<UserProfile> {
    // All concerns handled declaratively
    self.repository.find_user(id).await
}
```

**Enterprise Example**:
```rust
#[authorized(role = "admin")]       // Security
#[transactional]                    // Database safety
#[traced(level = "warn")]           // Audit trail
#[retry(max_attempts = 2)]          // Resilience
async fn delete_user(&self, id: u64) -> Result<()> {
    // Production-ready with 4 macros
}
```

## Implementation Status

| Macro | Status | Category |
|-------|--------|----------|
| `#[derive(Injectable)]` | ✅ Complete | DI |
| `#[derive(Validate)]` | ✅ Complete | Validation |
| `#[derive(Builder)]` | ✅ Complete | Patterns |
| `#[derive(Event)]` | ✅ Complete | Messaging |
| `#[cached]` | ✅ Complete | Performance |
| `#[memoize]` | ✅ Complete | Performance |
| `#[traced]` | ✅ Complete | Observability |
| `#[timed]` | ✅ Complete | Observability |
| `#[metrics_histogram]` | ✅ Complete | Observability |
| `#[audit]` | ✅ Complete | Security |
| `#[retry]` | ✅ Complete | Resilience |
| `#[circuit_breaker]` | ✅ Complete | Resilience |
| `#[timeout]` | ✅ Complete | Resilience |
| `#[rate_limit]` | ✅ Complete | Resilience |
| `#[authorized]` | ✅ Complete | Security |
| `#[transactional]` | ✅ Complete | Database |
| `#[validate_params]` | ✅ Complete | Validation |
| `#[feature_flag]` | ✅ Complete | Features |

**Total**: 18 macros (4 derive + 14 attribute)

## Key Features ⭐

- ✅ **Advanced retry** with exponential, fibonacci, fixed backoff + jitter
- ✅ **Circuit breaker** pattern for fault tolerance
- ✅ **Permanent memoization** for pure functions
- ✅ **Parameter-level tracing** with sensitive data exclusion
- ✅ **Timeout** for async operations
- ✅ **Builder pattern** generation
- ✅ **Event trait** implementation
- ✅ **Authorization** (role & permission-based)
- ✅ **Transactions** (auto commit/rollback)
- ✅ **Feature flags** for conditional execution

## Requirements

Integration with Rustboot crates:
- `rustboot-di` - Dependency injection
- `rustboot-validation` - Validation framework
- `rustboot-cache` - Caching
- `rustboot-observability` - Tracing, metrics, logging
- `rustboot-resilience` - Retry, circuit breaker, timeout
- `rustboot-security` - Authorization, audit
- `rustboot-database` - Transactions
- `rustboot-messaging` - Events
- `rustboot-config` - Feature flags

## Testing

```bash
# Run all tests
cargo test -p rustboot-macros

# Run specific test file
cargo test -p rustboot-macros --test derive_tests
cargo test -p rustboot-macros --test attribute_tests
cargo test -p rustboot-macros --test integration_tests
```

**Test Coverage**: 50+ tests across 3 test files

## Documentation

- [Overview](docs/overview.md) - Comprehensive WHAT-WHY-HOW guide
- [Backlog](docs/backlog.md) - Future enhancements
- [Examples](examples/usage.rs) - Real-world usage patterns

## Framework Comparison

### vs Spring Boot
✅ @Transactional → `#[transactional]`  
✅ @Cacheable → `#[cached]`  
✅ @PreAuthorize → `#[authorized]`  
✅ @Timed → `#[timed]`  
✅ @Async → (Rust native)  
**Plus**: `#[retry]`, `#[circuit_breaker]`, `#[memoize]`, `#[timeout]`

### vs .NET
✅ [Authorize] → `#[authorized]`  
✅ Builder pattern → `#[derive(Builder)]`  
**Plus**: Full resilience suite, advanced retry, memoization

**Rustboot macros match or exceed enterprise framework capabilities!**

## License

MIT OR Apache-2.0 (same as Rustboot framework)

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md)

Areas for contribution:
- Additional validators
- More backoff strategies
- Performance optimizations
- Documentation improvements
