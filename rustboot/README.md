# Rustboot

**Application framework for Rust** with validation, caching, DI, state machines, and more.

## Features

- 🏷️ **Macros** - Procedural macros for DI, validation, caching, tracing, retry, and more
- 🔍 **Validation** - Type-safe validation with fluent builders
- 💾 **Caching** - TTL-based caching abstraction
- 💉 **DI** - Dependency injection container
- 🔄 **State Machines** - Transitions with guards
- 🌐 **HTTP** - HTTP client abstraction
- 📨 **Messaging** - Pub/sub messaging
- 🗄️ **Database** - CRUD and transaction abstractions
- 🔌 **Middleware** - Pipeline pattern
- 📊 **Observability** - Metrics, logging, tracing
- 🔐 **Security** - Authentication, authorization, secrets, auditing
- 🛡️ **Resilience** - Retry, circuit breaker, timeout patterns
- ⏱️ **Rate Limiting** - Token bucket, leaky bucket, sliding window

## Quick Start

### Using Procedural Macros

```rust
use rustboot_macros::{Injectable, Validate, cached, traced, retry};

// Automatic dependency injection
#[derive(Injectable)]
struct UserService {
    repository: Arc<dyn UserRepository>,
    cache: Arc<Cache>,
}

// Automatic validation
#[derive(Validate)]
struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    username: String,
    
    #[validate(email)]
    email: String,
    
    #[validate(range(min = 18, max = 120))]
    age: u8,
}

impl UserService {
    // Combine multiple cross-cutting concerns
    #[traced(level = "info")]
    #[retry(max_attempts = 3)]
    #[cached(ttl = 600)]
    async fn get_user(&self, id: u64) -> Result<User> {
        self.repository.find_user(id).await
    }
}
```

### Using Core APIs

```rust
use rustboot::prelude::*;

// Validation
let validator = StringValidationBuilder::new("email")
    .not_empty()
    .email()
    .build();

// Caching
let cache = InMemoryCache::new();
cache.set("key", "value")?;

// DI
let container = Container::new();
container.register(MyService::new());

// Resilience
let retry_policy = RetryPolicy::builder()
    .max_attempts(3)
    .build();
retry_policy.execute(|| risky_operation()).await?;
```

## Documentation

See [docs/overview.md](docs/overview.md) for:
- Architecture guides
- Security documentation
- Development guides
- Individual crate documentation
- Backlog and roadmap

## Architecture

Rustboot follows the [SEA (Stratified Encapsulation Architecture)](https://github.com/phdsystems/rustratify) pattern.

## License

MIT
