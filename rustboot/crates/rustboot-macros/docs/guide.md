# Rustboot Macros Guide

**Audience**: Application Developers

This guide explains when and how to use Rustboot's procedural macros effectively.

## When to Use Macros

### Decision Tree: Should I Use a Macro?

```
Is it a cross-cutting concern? (logging, retry, auth, caching)
├─ YES → Use attribute macro (#[traced], #[retry], #[cached], #[authorized])
│
└─ NO → Is it repetitive boilerplate?
         ├─ YES → Is it a common pattern?
         │        ├─ Builder pattern → #[derive(Builder)]
         │        ├─ DI registration → #[derive(Injectable)]
         │        ├─ Validation → #[derive(Validate)]
         │        └─ Other → Consider manual impl first
         │
         └─ NO → Is it complex business logic?
                  ├─ YES → Use regular functions (easier to debug)
                  └─ NO → Is it one-off code?
                           ├─ YES → Manual implementation
                           └─ NO → Evaluate case-by-case
```

### Decision Tree: Which Macro Do I Need?

```
What problem are you solving?
│
├─ OBSERVABILITY
│  ├─ Need to trace function calls? → #[traced]
│  ├─ Need to measure duration? → #[timed]
│  └─ Need audit logging? → #[audit]
│
├─ RESILIENCE
│  ├─ External call might fail? → #[retry]
│  ├─ Need circuit breaker? → #[circuit_breaker]
│  └─ Need async timeout? → #[timeout]
│
├─ PERFORMANCE
│  ├─ Expensive computation? → #[cached]
│  └─ Need memoization? → #[memoize]
│
├─ SECURITY
│  ├─ Need role check? → #[authorized(role = "...")]
│  └─ Need permission check? → #[authorized(permission = "...")]
│
├─ CODE GENERATION
│  ├─ Need builder pattern? → #[derive(Builder)]
│  ├─ Need DI registration? → #[derive(Injectable)]
│  ├─ Need input validation? → #[derive(Validate)]
│  ├─ Need event handling? → #[derive(Event)]
│  └─ Need OpenAPI schema? → #[derive(OpenApiSchema)]
│
└─ NONE OF ABOVE → Manual implementation
```

### Quick Reference Tables

**Use Macros For:**

| Pattern | Macro | Benefit |
|---------|-------|---------|
| Boilerplate reduction | `#[derive(Builder)]` | 96% less code |
| Cross-cutting concerns | `#[traced]`, `#[retry]` | Consistent behavior |
| Compile-time validation | `#[derive(Validate)]` | Early error detection |
| DI registration | `#[derive(Injectable)]` | Automatic wiring |

**Don't Use Macros For:**

| Scenario | Why Not | Alternative |
|----------|---------|-------------|
| Complex business logic | Hard to debug | Regular functions |
| One-off code | Overhead not justified | Manual implementation |
| Runtime-dynamic behavior | Macros are compile-time | Traits + generics |
| Simple structs | No benefit | Plain struct |

## Quick Reference

### Derive Macros

```rust
use rustboot_macros::{Injectable, Validate, Builder, Event, OpenApiSchema};

#[derive(Injectable)]          // Auto DI registration
#[derive(Validate)]            // Field validation
#[derive(Builder)]             // Builder pattern
#[derive(Event)]               // Event trait impl
#[derive(OpenApiSchema)]       // OpenAPI schema
struct MyType { ... }
```

### Attribute Macros

```rust
use rustboot_macros::{traced, retry, cached, timed, authorized, timeout};

#[traced]                      // Add tracing span
#[retry(attempts = 3)]         // Retry on failure
#[cached(ttl = 600)]           // Cache results
#[timed]                       // Measure duration
#[authorized(role = "admin")]  // Check authorization
#[timeout(ms = 5000)]          // Async timeout
async fn my_function() { ... }
```

## Detailed Usage

### 1. Dependency Injection (`#[derive(Injectable)]`)

Automatically registers a struct with the DI container.

```rust
use rustboot_macros::Injectable;
use std::sync::Arc;

#[derive(Injectable)]
struct UserService {
    repository: Arc<dyn UserRepository>,
    cache: Arc<dyn Cache>,
}

// Generated code:
// impl UserService {
//     pub fn from_container(c: &Container) -> Result<Self, DiError> { ... }
//     pub fn register(c: &mut Container) -> Result<(), DiError> { ... }
// }

// Usage:
let service = UserService::from_container(&container)?;
```

**Attributes:**
- `#[inject(name = "custom")]` - Custom registration name
- `#[inject(lazy)]` - Lazy initialization

### 2. Validation (`#[derive(Validate)]`)

Generates validation logic from field attributes.

```rust
use rustboot_macros::Validate;

#[derive(Validate)]
struct CreateUser {
    #[validate(length(min = 3, max = 50))]
    username: String,

    #[validate(email)]
    email: String,

    #[validate(range(min = 18, max = 120))]
    age: u8,

    #[validate(regex = r"^\+?[0-9]{10,14}$")]
    phone: Option<String>,

    #[validate(custom = "validate_password")]
    password: String,
}

// Usage:
let user = CreateUser { ... };
user.validate()?;  // Returns Result<(), ValidationError>
```

**Validators:**
| Validator | Usage | Example |
|-----------|-------|---------|
| `length` | String length | `#[validate(length(min = 1, max = 100))]` |
| `range` | Numeric range | `#[validate(range(min = 0, max = 999))]` |
| `email` | Email format | `#[validate(email)]` |
| `url` | URL format | `#[validate(url)]` |
| `regex` | Pattern match | `#[validate(regex = r"^\d+$")]` |
| `custom` | Custom function | `#[validate(custom = "my_validator")]` |

### 3. Builder (`#[derive(Builder)]`)

Two patterns available for builder generation.

#### Standard Builder (Separate Builder Struct)

```rust
use rustboot_macros::Builder;

#[derive(Builder)]
struct HttpRequest {
    method: String,
    url: String,
    #[builder(default)]
    headers: HashMap<String, String>,
    #[builder(default)]
    body: Option<Vec<u8>>,
}

// Usage:
let request = HttpRequest::builder()
    .method("GET".to_string())
    .url("https://api.example.com".to_string())
    .build()?;
```

#### Fluent Builder (with_* Methods on Struct)

Use `#[builder(fluent)]` to generate `with_*` methods directly on the struct:

```rust
use rustboot_macros::Builder;

#[derive(Builder, Default)]
#[builder(fluent)]
struct QueryOptions {
    limit: usize,
    offset: usize,
    timeout: Option<u64>,  // with_timeout takes u64, wraps in Some
}

// Usage:
let opts = QueryOptions::default()
    .with_limit(100)
    .with_offset(50)
    .with_timeout(5000);
```

**Struct-Level Attributes:**
- `#[builder(fluent)]` - Generate `with_*` methods instead of separate builder

**Field-Level Attributes:**
- `#[builder(default)]` - Use Default::default() if not set
- `#[builder(default = "value")]` - Custom default value
- `#[builder(skip)]` - Don't include in builder

**Fluent Pattern Notes:**
- For `Option<T>` fields, the `with_*` method takes `T` and wraps it in `Some`
- Requires the struct to implement `Default` for initialization
- Methods return `Self` for chaining

### 4. Tracing (`#[traced]`)

Adds automatic tracing spans to functions.

```rust
use rustboot_macros::traced;

#[traced]                           // Default: function name as span
async fn process_order(id: u64) -> Result<()> { ... }

#[traced(level = "debug")]          // Custom level
async fn helper() { ... }

#[traced(name = "order.process")]   // Custom span name
async fn process(id: u64) { ... }

#[traced(skip(password))]           // Don't log sensitive params
async fn login(user: &str, password: &str) { ... }
```

**Options:**
| Option | Description | Example |
|--------|-------------|---------|
| `level` | Log level | `level = "info"` |
| `name` | Span name | `name = "my.span"` |
| `skip` | Skip params | `skip(password, token)` |
| `fields` | Extra fields | `fields(user_id = %id)` |

### 5. Retry (`#[retry]`)

Automatically retries failed operations.

```rust
use rustboot_macros::retry;

#[retry]                                    // Default: 3 attempts, exponential backoff
async fn call_api() -> Result<Response> { ... }

#[retry(max_attempts = 5)]                  // Custom attempts
async fn flaky_operation() -> Result<()> { ... }

#[retry(max_attempts = 3, backoff = "linear")]  // Linear backoff
async fn database_query() -> Result<Data> { ... }

#[retry(max_attempts = 3, delay = 100)]     // Fixed delay
async fn retry_fixed() -> Result<()> { ... }

#[retry(when = "is_retryable")]             // Conditional retry
async fn smart_retry() -> Result<(), MyError> { ... }
```

#### RetryableError Integration

Use `retryable = true` to integrate with the `RetryableError` trait:

```rust
use rustboot_macros::retry;
use rustboot_error::RetryableError;

// Error type must implement RetryableError
impl RetryableError for MyError {
    fn is_retryable(&self) -> bool {
        matches!(self, MyError::Timeout | MyError::RateLimited)
    }
    fn retry_after_ms(&self) -> Option<u64> {
        if let MyError::RateLimited { retry_after } = self {
            Some(*retry_after)
        } else {
            None
        }
    }
}

#[retry(max_attempts = 3, retryable = true, name = "api.call")]
async fn call_api() -> Result<Response, MyError> {
    // Only retries if error.is_retryable() returns true
    // Uses error.retry_after_ms() hint if available
    ...
}
```

**Options:**
| Option | Description | Default |
|--------|-------------|---------|
| `max_attempts` | Max retry attempts | 3 |
| `backoff` | Backoff strategy | `"exponential"` |
| `delay` | Initial delay (ms) | 100 |
| `max_delay` | Max delay (ms) | 10000 |
| `jitter` | Add random jitter | false |
| `retryable` | Check `RetryableError::is_retryable()` | false |
| `name` | Operation name for logging | function name |

**Backoff Strategies:**
- `"exponential"` - 100ms, 200ms, 400ms, 800ms...
- `"linear"` - 100ms, 200ms, 300ms, 400ms...
- `"fixed"` - 100ms, 100ms, 100ms...

### 6. Caching (`#[cached]`)

Caches function results.

```rust
use rustboot_macros::cached;

#[cached]                          // Default: cache forever
fn expensive_computation(x: i32) -> i32 { ... }

#[cached(ttl = 600)]               // TTL in seconds
async fn fetch_user(id: u64) -> Result<User> { ... }

#[cached(key = "user:{id}")]       // Custom cache key
async fn get_user(id: u64) -> Result<User> { ... }

#[cached(condition = "result.is_ok()")]  // Only cache success
async fn maybe_cache() -> Result<Data> { ... }
```

**Options:**
| Option | Description | Default |
|--------|-------------|---------|
| `ttl` | Time-to-live (seconds) | forever |
| `key` | Cache key template | auto-generated |
| `condition` | When to cache | always |

### 7. Timing (`#[timed]`)

Measures and logs function duration.

```rust
use rustboot_macros::timed;

#[timed]                           // Log duration
async fn process() { ... }

#[timed(slow_threshold_ms = 100)]  // Warn if > 100ms
async fn should_be_fast() { ... }

#[timed(histogram = "api_latency")] // Record to metrics
async fn api_call() { ... }
```

### 8. Authorization (`#[authorized]`)

Checks authorization before execution.

```rust
use rustboot_macros::authorized;

#[authorized]                       // Requires any authenticated user
async fn user_profile() { ... }

#[authorized(role = "admin")]       // Requires admin role
async fn delete_user(id: u64) { ... }

#[authorized(permission = "users:write")]  // Requires permission
async fn update_user(id: u64) { ... }

#[authorized(role = "admin", permission = "users:delete")]  // Both required
async fn purge_user(id: u64) { ... }
```

### 9. Timeout (`#[timeout]`)

Adds timeout to async functions.

```rust
use rustboot_macros::timeout;

#[timeout(ms = 5000)]              // 5 second timeout
async fn external_call() -> Result<Response> { ... }

#[timeout(ms = 1000, error = "TimeoutError::new()")]
async fn quick_check() -> Result<(), TimeoutError> { ... }
```

## Macro Composition

Macros can be combined. **Order matters** - outermost executes first.

```rust
// Recommended order: auth -> trace -> time -> retry -> cache
#[authorized(role = "user")]   // 1. Check auth first
#[traced(level = "info")]      // 2. Create trace span
#[timed(slow_threshold = 100)] // 3. Measure total time
#[retry(attempts = 3)]         // 4. Retry failures
#[cached(ttl = 600)]           // 5. Cache final result
async fn get_user_data(id: u64) -> Result<UserData> {
    self.repository.fetch(id).await
}
```

### Execution Flow

```
Request arrives
    ↓
[authorized] - Check permissions → Reject if unauthorized
    ↓
[traced] - Start span, log parameters
    ↓
[timed] - Start timer
    ↓
[retry] - Execute with retry logic
    ↓
[cached] - Check cache, execute if miss, store result
    ↓
[timed] - Stop timer, log duration
    ↓
[traced] - End span
    ↓
Response returned
```

## Troubleshooting

### Macro Doesn't Compile

**Symptom:** Compilation error in macro-generated code

**Solutions:**
1. Check attribute syntax matches documentation
2. Ensure required traits are implemented
3. Run `cargo expand` to see generated code:
   ```bash
   cargo install cargo-expand
   cargo expand --lib | grep -A 50 "fn my_function"
   ```

### IDE Shows Errors But Code Compiles

**Symptom:** rust-analyzer shows errors, but `cargo build` succeeds

**Solutions:**
1. Restart rust-analyzer
2. Run `cargo check` to refresh
3. Add explicit type annotations if inference fails

### Unexpected Behavior

**Symptom:** Macro doesn't behave as expected

**Solutions:**
1. Check macro ordering (outermost executes first)
2. Verify attribute parameters
3. Use `cargo expand` to inspect generated code
4. Add manual logging to verify execution flow

### Performance Issues

**Symptom:** Macro overhead affects performance

**Solutions:**
1. Macros have zero runtime overhead (code generation only)
2. Check if nested macros create redundant operations
3. Profile with `#[timed]` to identify bottlenecks

## Best Practices

### DO

- Compose macros for multiple concerns
- Use `#[traced]` on public API boundaries
- Apply `#[retry]` to external service calls
- Cache expensive computations
- Validate all user input with `#[derive(Validate)]`

### DON'T

- Over-use macros for simple code
- Nest too many macros (max 4-5 recommended)
- Use macros for complex business logic
- Skip testing macro-decorated functions
- Ignore macro-generated error messages

## Integration with Rustboot Crates

| Macro | Integrates With |
|-------|-----------------|
| `Injectable` | rustboot-di |
| `Validate` | rustboot-validation |
| `traced` | rustboot-observability |
| `retry`, `circuit_breaker` | rustboot-resilience |
| `cached` | rustboot-cache |
| `authorized` | rustboot-security |
| `timed` | rustboot-observability |

---

**Related Documentation:**
- [Macros Overview](overview.md) - Architecture and implementation details
- [Developer Guide](../../../docs/4-development/developer-guide.md) - Development patterns
- [Backlog](backlog.md) - Planned enhancements

**External Resources:**
- [The Rust Reference: Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)
- [cargo-expand](https://github.com/dtolnay/cargo-expand) - View macro expansions
