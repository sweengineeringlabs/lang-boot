# rustboot-error

| | |
|------|------|
| **WHAT** | Error handling utilities for Rust applications |
| **WHY** | Reusable error conversion, context, and mapping patterns |
| **HOW** | Extension traits and macros (zero runtime cost) |

## The Architecture/Infrastructure Split

| Concern | Location | Example |
|---------|----------|---------|
| Error type definitions | Domain crates | `enum ProviderError { ... }` |
| Error conversion utilities | rustboot-error | `ResultExt::map_err_to()` |
| Error context helpers | rustboot-error | `with_context!()` |
| Retryable error patterns | rustboot-error | `RetryableError` trait |
| HTTP status mapping | rustboot-error | `HttpStatusError` trait |
| From implementations | Application code | Uses `impl_error_from!` macro |

## Features

### ResultExt - Error Mapping

```rust
use rustboot_error::ResultExt;

fn load_config() -> Result<Config, AppError> {
    std::fs::read_to_string("config.toml")
        .map_err_to(|e| AppError::Io(e.to_string()))?;
    // ...
}
```

### OptionExt - None to Error

```rust
use rustboot_error::OptionExt;

fn get_user(id: u64) -> Result<User, AppError> {
    users.get(&id)
        .ok_or_err(AppError::NotFound(id))
}
```

### with_context! - Add Context

```rust
use rustboot_error::with_context;

fn process_file(path: &str) -> Result<(), String> {
    with_context!(
        std::fs::read_to_string(path),
        format!("failed to read {}", path)
    )
}
```

### impl_error_from! - Generate From Impls

```rust
use rustboot_error::impl_error_from;

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(String),
}

impl_error_from!(std::io::Error => MyError::Io);
```

### RetryableError - Retry Patterns

```rust
use rustboot_error::RetryableError;

#[derive(Debug)]
enum ApiError {
    RateLimited { retry_after_ms: Option<u64> },
    ServerError(String),
    InvalidRequest(String),
}

impl RetryableError for ApiError {
    fn is_retryable(&self) -> bool {
        matches!(self, ApiError::RateLimited { .. } | ApiError::ServerError(_))
    }

    fn retry_after_ms(&self) -> Option<u64> {
        match self {
            ApiError::RateLimited { retry_after_ms } => *retry_after_ms,
            ApiError::ServerError(_) => Some(1000),
            _ => None,
        }
    }
}
```

### HttpStatusError - HTTP Response Mapping

```rust
use rustboot_error::{HttpStatusError, HttpStatusCategory};

impl HttpStatusError for ApiError {
    fn from_http_status(status: u16, body: &str, retry_after_ms: Option<u64>) -> Self {
        match HttpStatusCategory::from_status(status) {
            HttpStatusCategory::RateLimited => ApiError::RateLimited { retry_after_ms },
            HttpStatusCategory::ServerError => ApiError::ServerError(body.to_string()),
            _ => ApiError::InvalidRequest(body.to_string()),
        }
    }
}
```

### Convenience Macros

```rust
use rustboot_error::{impl_from_io_error, impl_error_from_many, define_result};

// Single From implementation
impl_from_io_error!(MyError::Io);

// Multiple From implementations
impl_error_from_many!(
    std::io::Error => MyError::Io,
    std::num::ParseIntError => MyError::Parse
);

// Result type alias
define_result!(MyError);
```

## API Reference

### Traits

| Trait | Purpose |
|-------|---------|
| `ErrorExt` | Convert any error to string |
| `ResultExt<T, E>` | Map Result errors with custom functions |
| `OptionExt<T>` | Convert Option to Result |
| `RetryableError` | Query if error is retryable with optional delay |
| `HttpStatusError` | Create errors from HTTP status codes |

### Types

| Type | Purpose |
|------|---------|
| `HttpStatusCategory` | Categorize HTTP status codes (RateLimited, ServerError, etc.) |

### Macros

| Macro | Purpose |
|-------|---------|
| `impl_error_from!` | Generate single From implementation |
| `impl_from_io_error!` | Generate From<std::io::Error> |
| `impl_from_serde_json_error!` | Generate From<serde_json::Error> |
| `impl_from_serde_yaml_error!` | Generate From<serde_yaml::Error> |
| `impl_error_from_many!` | Generate multiple From implementations |
| `define_result!` | Create Result type alias |
| `with_context!` | Wrap errors with context message |

### Functions

| Function | Purpose |
|----------|---------|
| `err_to_string(E)` | Convert any Display error to String |
| `io_err_to_string(io::Error)` | Format IO error with prefix |

## When to Use

**Use rustboot-error when:**
- Converting between error types
- Adding context to errors
- Implementing From for error enums
- Mapping Option to Result
- Implementing retryable error patterns
- Mapping HTTP status codes to errors

**Don't use for:**
- Defining error enums (do that in your domain crate)
- Logging errors (use rustboot-observability)
- Executing retries (use rustboot-resilience)
