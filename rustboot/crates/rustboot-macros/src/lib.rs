/*! # Rustboot Macros

Procedural macros for the Rustboot framework providing ergonomic attribute and derive macros.

## Available Macros

### Derive Macros
- `#[derive(Injectable)]` - Automatic dependency injection registration
- `#[derive(Validate)]` - Automatic validation implementation
- `#[derive(Builder)]` - Builder pattern generation
- `#[derive(Event)]` - Event trait implementation
- `#[derive(OpenApiSchema)]` - OpenAPI schema generation

### Attribute Macros
- `#[cached]` - Method-level result caching
- `#[traced]` - Automatic tracing and logging
- `#[retry]` - Automatic retry logic
- `#[timed]` - Performance timing
- `#[transactional]` - Database transaction wrapper
- `#[authorized]` - Authorization checks
- `#[timeout]` - Async timeout
- `#[memoize]` - Permanent memoization
- `#[feature_flag]` - Feature toggle
- `#[metrics_histogram]` - Histogram metrics
- `#[http_request]` - Declarative HTTP API client methods

## Examples

```rust,ignore
use rustboot_macros::{Injectable, Builder, cached, traced, retry, authorized};

#[derive(Injectable)]
struct MyService {
    database: Arc<Database>,
}

#[derive(Builder)]
struct Config {
    host: String,
    port: u16,
}

impl MyService {
    #[authorized(role = "admin")]
    #[transactional]
    #[traced]
    async fn delete_user(&self, id: u64) -> Result<()> {
        // Automatically: authorized, transacted, and traced
    }
}
```
*/

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemFn};

mod derive;
mod attrs;
mod utils;

// ============================================================================
// DERIVE MACROS
// ============================================================================

/// Derive macro for automatic dependency injection registration.
#[proc_macro_derive(Injectable, attributes(inject))]
pub fn derive_injectable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive::injectable::impl_injectable(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derive macro for automatic validation implementation.
#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive::validate::impl_validate(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derive macro for builder pattern generation.
///
/// # Standard Builder Pattern
///
/// By default, generates a separate `XxxBuilder` struct:
///
/// ```rust,ignore
/// #[derive(Builder)]
/// struct UserConfig {
///     name: String,
///     age: u8,
///     #[builder(default = "None")]
///     nickname: Option<String>,
/// }
///
/// let config = UserConfig::builder()
///     .name("Alice".to_string())
///     .age(25)
///     .build()?;
/// ```
///
/// # Fluent Builder Pattern
///
/// Use `#[builder(fluent)]` to generate `with_*` methods directly on the struct:
///
/// ```rust,ignore
/// #[derive(Builder, Default)]
/// #[builder(fluent)]
/// struct QueryOptions {
///     limit: usize,
///     offset: usize,
///     timeout: Option<u64>,
/// }
///
/// let opts = QueryOptions::default()
///     .with_limit(100)
///     .with_offset(50)
///     .with_timeout(5000);  // For Option<T>, takes T and wraps in Some
/// ```
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive::builder::impl_builder(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derive macro for event trait implementation.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Event)]
/// struct UserCreated {
///     user_id: u64,
///     timestamp: DateTime,
/// }
/// ```
#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive::event::impl_event(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derive macro for OpenAPI schema generation.
///
/// Automatically generates OpenAPI schema implementations for structs and enums.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(OpenApiSchema)]
/// struct User {
///     id: u64,
///     name: String,
///     email: String,
/// }
/// ```
#[proc_macro_derive(OpenApiSchema)]
pub fn derive_openapi_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive::openapi_schema::impl_openapi_schema(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

// ============================================================================
// ATTRIBUTE MACROS - CACHING & PERFORMANCE
// ============================================================================

/// Method-level caching attribute.
#[proc_macro_attribute]
pub fn cached(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::cached::impl_cached(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Permanent memoization attribute for pure functions.
///
/// # Example
///
/// ```rust,ignore
/// #[memoize]
/// fn fibonacci(n: u64) -> u64 {
///     if n <= 1 { n } else { fibonacci(n-1) + fibonacci(n-2) }
/// }
/// ```
#[proc_macro_attribute]
pub fn memoize(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::memoize::impl_memoize(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

// ============================================================================
// ATTRIBUTE MACROS - OBSERVABILITY
// ============================================================================

/// Automatic tracing and logging attribute.
#[proc_macro_attribute]
pub fn traced(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::traced::impl_traced(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Performance timing attribute.
#[proc_macro_attribute]
pub fn timed(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::timed::impl_timed(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Metrics histogram recording attribute.
///
/// # Example
///
/// ```rust,ignore
/// #[metrics_histogram(name = "api_latency", buckets = [10, 50, 100, 500])]
/// async fn api_call() -> Result<()> {
///     // Recorded to Prometheus histogram
/// }
/// ```
#[proc_macro_attribute]
pub fn metrics_histogram(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::metrics_histogram::impl_metrics_histogram(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Audit logging attribute for sensitive operations.
#[proc_macro_attribute]
pub fn audit(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::audit::impl_audit(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

// ============================================================================
// ATTRIBUTE MACROS - RESILIENCE
// ============================================================================

/// Automatic retry logic attribute.
#[proc_macro_attribute]
pub fn retry(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::retry::impl_retry(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Circuit breaker attribute for fault tolerance.
#[proc_macro_attribute]
pub fn circuit_breaker(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::circuit_breaker::impl_circuit_breaker(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Timeout attribute for async functions.
///
/// # Example
///
/// ```rust,ignore
/// #[timeout(duration = 5000)]
/// async fn slow_operation() -> Result<Data> {
///     // Times out after 5 seconds
/// }
/// ```
#[proc_macro_attribute]
pub fn timeout(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::timeout::impl_timeout(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Rate limiting attribute.
#[proc_macro_attribute]
pub fn rate_limit(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::rate_limit::impl_rate_limit(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

// ============================================================================
// ATTRIBUTE MACROS - SECURITY & DATABASE
// ============================================================================

/// Authorization check attribute.
///
/// # Example
///
/// ```rust,ignore
/// #[authorized(role = "admin")]
/// async fn delete_user(&self, id: u64) -> Result<()> {
///     // Only admins can execute
/// }
/// ```
#[proc_macro_attribute]
pub fn authorized(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::authorized::impl_authorized(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Database transaction wrapper attribute.
///
/// # Example
///
/// ```rust,ignore
/// #[transactional]
/// async fn create_user(&self, user: User) -> Result<User> {
///     // Automatically wrapped in transaction
///     self.db.insert(user).await
/// }
/// ```
#[proc_macro_attribute]
pub fn transactional(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::transactional::impl_transactional(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Parameter validation attribute.
#[proc_macro_attribute]
pub fn validate_params(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::validate_params::impl_validate_params(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

// ============================================================================
// ATTRIBUTE MACROS - FEATURE MANAGEMENT
// ============================================================================

/// Feature flag attribute for conditional execution.
///
/// # Example
///
/// ```rust,ignore
/// #[feature_flag(flag = "new_ui")]
/// fn new_feature() -> Response {
///     // Only executes if feature enabled
/// }
/// ```
#[proc_macro_attribute]
pub fn feature_flag(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::feature_flag::impl_feature_flag(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

// ============================================================================
// ATTRIBUTE MACROS - HTTP
// ============================================================================

/// HTTP request attribute for declarative API client methods.
///
/// This macro simplifies defining HTTP API client methods by generating
/// the boilerplate for making HTTP requests.
///
/// # Arguments
///
/// - `method` - HTTP method (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
/// - `path` - URL path, can include placeholders like `{id}`
/// - `body` - Whether the last argument is the request body (for POST, PUT, PATCH)
/// - `content_type` - Content type for request body (default: "application/json")
/// - `response_type` - Expected response content type (default: "application/json")
///
/// # Example
///
/// ```rust,ignore
/// use rustboot_macros::http_request;
/// use rustboot_http::{HttpClient, HttpResult, ReqwestClient};
///
/// struct UserApi {
///     base_url: String,
///     client: ReqwestClient,
/// }
///
/// impl UserApi {
///     #[http_request(method = "GET", path = "/users/{id}")]
///     async fn get_user(&self, id: u64) -> HttpResult<User> {}
///
///     #[http_request(method = "POST", path = "/users", body)]
///     async fn create_user(&self, user: CreateUser) -> HttpResult<User> {}
///
///     #[http_request(method = "PUT", path = "/users/{id}", body)]
///     async fn update_user(&self, id: u64, user: UpdateUser) -> HttpResult<User> {}
///
///     #[http_request(method = "DELETE", path = "/users/{id}")]
///     async fn delete_user(&self, id: u64) -> HttpResult<()> {}
/// }
/// ```
#[proc_macro_attribute]
pub fn http_request(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    attrs::http_request::impl_http_request(args, input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

// NOTE: openapi_path macro is not yet implemented and has been removed from public API
// The implementation exists in attrs/openapi_path.rs but is non-functional (marker only).
// This will be re-enabled once proper OpenAPI path registration is implemented.
//
// See: rustboot-openapi/IMPLEMENTATION.md - "Future Enhancements"
//
// /// OpenAPI path documentation attribute.
// ///
// /// Documents API endpoints with OpenAPI metadata for automatic documentation generation.
// ///
// /// # Example
// ///
// /// ```rust,ignore
// /// #[openapi_path(
// ///     method = "GET",
// ///     path = "/users/{id}",
// ///     tag = "users",
// ///     summary = "Get user by ID"
// /// )]
// /// async fn get_user(id: u64) -> Result<User> {
// ///     // Implementation
// /// }
// /// ```
// #[proc_macro_attribute]
// pub fn openapi_path(args: TokenStream, input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as ItemFn);
//     attrs::openapi_path::impl_openapi_path(args, input)
//         .unwrap_or_else(|err| err.to_compile_error())
//         .into()
// }
