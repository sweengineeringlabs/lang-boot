# 4. Error Handling Strategy

**Status**: Accepted

**Date**: 2025-12-24

**Decision Makers**: Framework Architecture Team

## Context

Error handling is a critical aspect of any framework. Rust's type system forces us to handle errors explicitly, but we have several options for how to represent and propagate errors:

1. **Error Representation**: Should we use custom error types, trait objects, or string messages?
2. **Error Derivation**: Should we hand-write error types or use derive macros?
3. **Error Composition**: How should errors from different layers compose?
4. **User Experience**: What information should errors contain for debugging?
5. **Type Safety**: How much type safety vs ergonomics?

Considerations:
- Framework has multiple layers: HTTP, database, messaging, caching, etc.
- Each layer has domain-specific errors (connection failed, query error, timeout, etc.)
- Users need actionable error messages
- Errors should be compatible with Rust's `?` operator
- Need to balance between rich type information and ergonomics

Available options in the ecosystem:
- `std::error::Error` - Standard trait
- `thiserror` - Derive macro for error types
- `anyhow` - Type-erased error handling
- `eyre` - Enhanced error reporting
- Custom hand-written implementations

## Decision

Rustboot will use **`thiserror` for library error types** with a structured error hierarchy.

Strategy:

1. **Domain-Specific Enums**: Each crate defines its own error enum
2. **thiserror Derives**: Use `#[derive(thiserror::Error)]` for boilerplate
3. **Structured Variants**: Error enums have meaningful variants with context
4. **No anyhow in Public APIs**: anyhow is for applications, not libraries
5. **Error Context**: Include relevant context (what failed, why it failed)
6. **Result Type Aliases**: Define `type FooResult<T> = Result<T, FooError>`
7. **Error Propagation**: Use `#[from]` attribute for automatic conversions

Example pattern:
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("Record not found")]
    NotFound,

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Transaction error: {0}")]
    Transaction(#[from] TransactionError),
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;
```

## Consequences

### Positive

- **Type Safety**: Each error type is distinct and strongly typed
- **Pattern Matching**: Users can match on specific error variants
  ```rust
  match db.query("SELECT ...").await {
      Ok(rows) => { /* ... */ },
      Err(DatabaseError::NotFound) => { /* handle not found */ },
      Err(DatabaseError::Connection(msg)) => { /* handle connection error */ },
      Err(e) => { /* handle other errors */ },
  }
  ```

- **Clear Documentation**: Error variants are self-documenting
- **Minimal Boilerplate**: thiserror generates `Display`, `Error`, and `From` impls
- **Good Error Messages**: `#[error("...")]` provides clear user-facing messages
- **Composability**: Errors convert naturally via `#[from]` attribute
- **Debug Information**: Derive `Debug` for full error details
- **Library Best Practice**: Following Rust ecosystem conventions
- **IDE Support**: Full autocomplete and type hints for error handling
- **Zero Runtime Cost**: All code generated at compile time

### Negative

- **Dependency**: Adds proc-macro dependency (thiserror)
- **Verbosity**: Must define error enum for each crate
- **Error Explosion**: Many error types across the framework
- **Conversion Complexity**: Cross-crate error conversions can be verbose
- **Maintenance**: Must keep error variants up to date
- **Limited Context**: Unlike anyhow, no automatic backtrace or context chain
- **Learning Curve**: Users must understand error types for each crate

### Neutral

- **Compile Time**: Proc macros add to compile time (minimal for thiserror)
- **Error Size**: Enum errors can be larger than trait objects
- **User Choice**: Users can still wrap in anyhow for applications

## Alternatives Considered

### 1. anyhow for Everything

**Approach**: Use `anyhow::Error` as the error type everywhere.

```rust
pub async fn query(&self, sql: &str) -> anyhow::Result<Vec<Row>> {
    // ...
}
```

**Rejected because**:
- **Type Erasure**: Loses specific error type information
- **No Pattern Matching**: Can't match on specific error variants
- **Library Anti-Pattern**: anyhow is designed for applications, not libraries
- **API Instability**: Error types become opaque implementation details
- **Poor User Experience**: Users can't handle specific errors differently
- **Documentation Loss**: No clear documentation of possible errors

**Note**: anyhow is still appropriate for application code using rustboot.

### 2. Hand-Written Error Types

**Approach**: Manually implement `Display`, `Error`, and `From` for each error type.

```rust
#[derive(Debug)]
pub enum DatabaseError {
    Connection(String),
    Query(String),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::Connection(msg) => write!(f, "Connection error: {}", msg),
            DatabaseError::Query(msg) => write!(f, "Query error: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}
```

**Rejected because**:
- **Boilerplate**: Significant repetitive code
- **Error Prone**: Easy to forget implementations or make mistakes
- **Maintenance Burden**: Every error change requires updating multiple impls
- **Inconsistency**: Different developers write errors differently
- **No Benefit**: thiserror provides the same result with less code

### 3. String-Based Errors

**Approach**: Use `String` or `&'static str` for errors.

```rust
pub type DatabaseResult<T> = Result<T, String>;
```

**Rejected because**:
- **No Type Safety**: All errors are just strings
- **No Pattern Matching**: Can't distinguish error types programmatically
- **Poor Ergonomics**: Must parse strings to understand errors
- **No Composition**: Can't nest or convert errors properly
- **Not Idiomatic**: Goes against Rust best practices

### 4. eyre Instead of anyhow

**Approach**: Use `eyre` for enhanced error reporting.

**Rejected because**:
- **Same Issues as anyhow**: Type erasure, not suitable for libraries
- **Larger Dependency**: eyre is larger and more complex
- **Application-Focused**: Designed for end-user error reporting
- **Overkill**: Features (custom handlers, suggestions) not needed in library code

### 5. failure Crate

**Approach**: Use the `failure` crate (predecessor to thiserror/anyhow).

**Rejected because**:
- **Deprecated**: Actively deprecated in favor of thiserror/anyhow
- **Outdated**: Doesn't work well with std::error::Error
- **Ecosystem Shift**: Rust community has moved on

## Error Design Patterns

### Pattern 1: Domain-Specific Errors

Each crate defines errors relevant to its domain:

```rust
// rustboot-http
#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Request error: {0}")]
    Request(String),

    #[error("Request timeout")]
    Timeout,

    #[error("Connection error: {0}")]
    Connection(String),
}

// rustboot-database
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("Record not found")]
    NotFound,
}
```

### Pattern 2: Error Conversion with Context

```rust
#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("Failed to load migration: {0}")]
    LoadError(String),

    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### Pattern 3: Result Type Aliases

```rust
pub type HttpResult<T> = Result<T, HttpError>;
pub type DatabaseResult<T> = Result<T, DatabaseError>;
pub type CacheResult<T> = Result<T, CacheError>;

// Usage
pub async fn fetch(&self) -> HttpResult<Response> { /* ... */ }
```

### Pattern 4: Error Context in Messages

```rust
#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Failed to create connection: {0}")]
    CreateError(String),

    #[error("Pool timeout after {timeout:?} waiting for connection")]
    Timeout { timeout: Duration },

    #[error("Pool is closed")]
    Closed,
}
```

### Pattern 5: Nested Error Chains

```rust
#[derive(Debug, Error)]
pub enum AppError {
    #[error("HTTP error")]
    Http(#[from] HttpError),

    #[error("Database error")]
    Database(#[from] DatabaseError),

    #[error("Cache error")]
    Cache(#[from] CacheError),
}

// Errors automatically convert through the chain
async fn process() -> Result<(), AppError> {
    let response = http_client.get("...").await?; // HttpError -> AppError
    let data = db.query("...").await?; // DatabaseError -> AppError
    cache.set("...", data).await?; // CacheError -> AppError
    Ok(())
}
```

## Error Guidelines

### DO:
- ✅ Use descriptive error messages
- ✅ Include relevant context (what operation failed, why)
- ✅ Use specific error variants for different failure modes
- ✅ Implement `Debug` for all error types
- ✅ Use `#[from]` for automatic conversions
- ✅ Document error conditions in function docs
- ✅ Keep error enums focused and domain-specific

### DON'T:
- ❌ Use generic error messages ("Something went wrong")
- ❌ Swallow errors without handling
- ❌ Use anyhow in public library APIs
- ❌ Create overly granular error variants
- ❌ Mix error types from different domains without wrapping
- ❌ Panic instead of returning errors
- ❌ Use `unwrap()` or `expect()` in library code

## Migration from thiserror 1.x to 2.x

When thiserror 2.0 is released, migration should be straightforward as it maintains backward compatibility. Monitor:
- [thiserror changelog](https://github.com/dtolnay/thiserror/releases)

## Error Documentation Template

```rust
/// Attempts to fetch data from the database.
///
/// # Errors
///
/// This function will return an error if:
/// - The database connection fails ([`DatabaseError::Connection`])
/// - The SQL query is invalid ([`DatabaseError::Query`])
/// - The requested record is not found ([`DatabaseError::NotFound`])
///
/// # Example
///
/// ```
/// match db.query("SELECT * FROM users").await {
///     Ok(rows) => println!("Found {} rows", rows.len()),
///     Err(DatabaseError::NotFound) => println!("No results"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>> {
    // implementation
}
```

## Performance Considerations

- **Error Size**: Enums with many variants can be large. Consider `Box<T>` for large variants:
  ```rust
  #[derive(Debug, Error)]
  pub enum MyError {
      #[error("Large error: {0}")]
      Large(Box<LargeErrorData>),
  }
  ```

- **Error Path**: Error paths are typically cold (not frequently executed), so optimize for the happy path

- **Zero Cost**: thiserror is zero-cost abstraction, generates efficient code

## Testing Error Conditions

```rust
#[tokio::test]
async fn test_error_handling() {
    let db = MockDatabase::new();
    db.expect_query()
        .returning(|_| Err(DatabaseError::Connection("Failed".to_string())));

    let result = db.query("SELECT *").await;

    assert!(matches!(result, Err(DatabaseError::Connection(_))));
}
```

## References

- [thiserror Documentation](https://docs.rs/thiserror/)
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Error Handling in Libraries](https://rust-lang.github.io/api-guidelines/interoperability.html#error-types-are-meaningful-c-good-err)
- [anyhow vs thiserror](https://github.com/dtolnay/anyhow#comparison-with-thiserror)

---

**Related ADRs**:
- [ADR-0003: Trait-Based Abstractions](./0003-trait-based-abstractions.md)
