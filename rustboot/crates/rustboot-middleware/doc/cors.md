# CORS Middleware Documentation

## Overview

The CORS (Cross-Origin Resource Sharing) middleware provides comprehensive support for handling cross-origin HTTP requests in Rustboot applications. It implements the W3C CORS specification and handles both simple requests and preflight requests.

## Features

- **Flexible Origin Configuration**: Support for wildcard, specific origins, and regex pattern matching
- **Method Control**: Configure allowed HTTP methods (GET, POST, PUT, DELETE, etc.)
- **Header Management**: Control both request headers and exposed response headers
- **Credentials Support**: Enable/disable credentials (cookies, authorization headers)
- **Preflight Handling**: Automatic handling of OPTIONS preflight requests
- **Caching**: Configurable max-age for preflight responses

## Quick Start

### Permissive CORS (Allow All Origins)

```rust
use rustboot_middleware::{CorsMiddleware, Pipeline};

let cors = CorsMiddleware::permissive();
let pipeline = Pipeline::new().with_middleware(cors);
```

### Restrictive CORS (Specific Origins)

```rust
use rustboot_middleware::{CorsMiddleware, Pipeline};

let cors = CorsMiddleware::restrictive(vec![
    "https://example.com".to_string(),
    "https://app.example.com".to_string(),
]);
let pipeline = Pipeline::new().with_middleware(cors);
```

## Configuration Options

### Origin Configuration

#### Allow All Origins (Wildcard)

```rust
use rustboot_middleware::{CorsConfig, CorsMiddleware};

let config = CorsConfig::new().allow_all_origins();
let cors = CorsMiddleware::new(config);
```

This sets the `Access-Control-Allow-Origin` header to `*` for all requests.

#### Specific Origins

```rust
use rustboot_middleware::{CorsConfig, CorsMiddleware};

let config = CorsConfig::new().allow_origins(vec![
    "https://example.com".to_string(),
    "https://api.example.com".to_string(),
]);
let cors = CorsMiddleware::new(config);
```

Only requests from the specified origins will be allowed.

#### Regex Pattern Matching

```rust
use rustboot_middleware::{CorsConfig, CorsMiddleware};

let config = CorsConfig::new()
    .allow_origin_regex(r"^https://.*\.example\.com$")
    .expect("Valid regex pattern");
let cors = CorsMiddleware::new(config);
```

Allows any origin matching the regex pattern (e.g., all subdomains of example.com).

### HTTP Methods

```rust
use rustboot_middleware::{CorsConfig, CorsMiddleware};

let config = CorsConfig::new().allow_methods(vec![
    "GET".to_string(),
    "POST".to_string(),
    "PUT".to_string(),
    "DELETE".to_string(),
]);
let cors = CorsMiddleware::new(config);
```

Default allowed methods: GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS

### Request Headers

```rust
use rustboot_middleware::{CorsConfig, CorsMiddleware};

let config = CorsConfig::new().allow_headers(vec![
    "Content-Type".to_string(),
    "Authorization".to_string(),
    "X-Custom-Header".to_string(),
]);
let cors = CorsMiddleware::new(config);
```

Default allowed headers: Content-Type, Authorization

### Exposed Response Headers

```rust
use rustboot_middleware::{CorsConfig, CorsMiddleware};

let config = CorsConfig::new().expose_headers(vec![
    "X-Total-Count".to_string(),
    "X-Request-Id".to_string(),
]);
let cors = CorsMiddleware::new(config);
```

These headers will be exposed to the client via the `Access-Control-Expose-Headers` header.

### Credentials Support

```rust
use rustboot_middleware::{CorsConfig, CorsMiddleware};

let config = CorsConfig::new().allow_credentials(true);
let cors = CorsMiddleware::new(config);
```

When credentials are enabled:
- The `Access-Control-Allow-Credentials: true` header is set
- The specific origin is echoed (wildcard `*` cannot be used with credentials)
- The `Vary: Origin` header is set

### Preflight Cache Control

```rust
use rustboot_middleware::{CorsConfig, CorsMiddleware};
use std::time::Duration;

let config = CorsConfig::new().max_age(Duration::from_secs(86400)); // 24 hours
let cors = CorsMiddleware::new(config);
```

Sets the `Access-Control-Max-Age` header for preflight responses. Default: 1 hour (3600 seconds)

## Complete Example

```rust
use rustboot_middleware::{CorsConfig, CorsMiddleware, Pipeline, HttpContext};
use std::time::Duration;

let config = CorsConfig::new()
    .allow_origins(vec!["https://example.com".to_string()])
    .allow_methods(vec![
        "GET".to_string(),
        "POST".to_string(),
        "PUT".to_string(),
        "DELETE".to_string(),
    ])
    .allow_headers(vec![
        "Content-Type".to_string(),
        "Authorization".to_string(),
        "X-API-Key".to_string(),
    ])
    .expose_headers(vec![
        "X-Total-Count".to_string(),
        "X-RateLimit-Remaining".to_string(),
    ])
    .allow_credentials(true)
    .max_age(Duration::from_secs(7200)); // 2 hours

let cors = CorsMiddleware::new(config);
let pipeline = Pipeline::new().with_middleware(cors);

// Use the pipeline to process requests
let ctx = HttpContext::from_headers(
    "GET".to_string(),
    vec![("Origin".to_string(), "https://example.com".to_string())]
        .into_iter()
        .collect()
);

let result = pipeline.execute(ctx).await?;
```

## Preflight Requests

Preflight requests are automatically detected (OPTIONS method) and handled by the middleware:

1. **Validation**: The middleware validates the requested method and headers
2. **Response Headers**: Sets appropriate CORS headers
3. **Status Code**: Returns 204 No Content
4. **Short-circuit**: Does not call the next middleware in the chain

Example preflight request:
```
OPTIONS /api/users HTTP/1.1
Origin: https://example.com
Access-Control-Request-Method: POST
Access-Control-Request-Headers: Content-Type, Authorization
```

Response:
```
HTTP/1.1 204 No Content
Access-Control-Allow-Origin: https://example.com
Access-Control-Allow-Methods: GET, POST, PUT, DELETE
Access-Control-Allow-Headers: Content-Type, Authorization
Access-Control-Max-Age: 3600
```

## Response Headers

The middleware sets the following response headers:

### Simple Requests
- `Access-Control-Allow-Origin`: The allowed origin or `*`
- `Access-Control-Allow-Credentials`: `true` if credentials are enabled
- `Access-Control-Expose-Headers`: List of exposed headers
- `Vary`: `Origin` (when not using wildcard)

### Preflight Requests
- All simple request headers, plus:
- `Access-Control-Allow-Methods`: List of allowed methods
- `Access-Control-Allow-Headers`: List of allowed headers
- `Access-Control-Max-Age`: Cache duration in seconds

## Error Handling

The middleware returns `MiddlewareError::Rejected` in the following cases:

1. **Origin not allowed**: The request origin doesn't match the configured origins
2. **Method not allowed**: The preflight request method is not in the allowed list
3. **Header not allowed**: A preflight request header is not in the allowed list

Example error:
```rust
match result {
    Ok(ctx) => { /* Request allowed */ },
    Err(MiddlewareError::Rejected(msg)) => {
        // msg will be something like:
        // "Origin 'https://evil.com' is not allowed"
        // "Method 'DELETE' is not allowed"
        // "Header 'X-Custom' is not allowed"
    },
    Err(e) => { /* Other error */ }
}
```

## Security Considerations

### Don't Use Wildcard with Credentials

```rust
// ❌ INSECURE - Don't do this
let config = CorsConfig::new()
    .allow_all_origins()
    .allow_credentials(true);
```

When credentials are enabled, the middleware automatically echoes the specific origin instead of using wildcard.

### Validate Origins Carefully

```rust
// ✅ SECURE - Use specific origins
let config = CorsConfig::new().allow_origins(vec![
    "https://app.example.com".to_string(),
]);

// ✅ SECURE - Use restrictive regex
let config = CorsConfig::new()
    .allow_origin_regex(r"^https://[a-z]+\.example\.com$")
    .unwrap();

// ❌ INSECURE - Too permissive regex
let config = CorsConfig::new()
    .allow_origin_regex(r".*")  // Allows any origin!
    .unwrap();
```

### Limit Exposed Headers

Only expose headers that are necessary for the client:

```rust
// ✅ GOOD - Only expose necessary headers
let config = CorsConfig::new().expose_headers(vec![
    "X-Total-Count".to_string(),
]);

// ❌ BAD - Exposing sensitive headers
let config = CorsConfig::new().expose_headers(vec![
    "Set-Cookie".to_string(),  // Don't expose sensitive headers
]);
```

## Testing

The middleware includes comprehensive tests covering:

- Simple CORS requests
- Preflight requests
- Origin validation (wildcard, specific, regex)
- Method and header validation
- Credentials support
- Header exposure

Run tests:
```bash
cargo test -p dev-engineeringlabs-rustboot-middleware --lib cors
```

Run example:
```bash
cargo run -p dev-engineeringlabs-rustboot-middleware --example cors_example
```

## Integration with Other Middleware

CORS middleware should typically be placed early in the middleware pipeline:

```rust
use rustboot_middleware::{
    CorsMiddleware,
    LoggingMiddleware,
    TimingMiddleware,
    Pipeline
};

let pipeline = Pipeline::new()
    .with_middleware(CorsMiddleware::permissive())  // First
    .with_middleware(LoggingMiddleware::new("app"))
    .with_middleware(TimingMiddleware::new("request"));
```

This ensures CORS headers are set even if later middleware rejects the request.

## Performance Considerations

- **Regex Caching**: Compiled regex patterns are stored in the configuration and reused
- **Lock-Free**: No locks are used for simple request handling
- **Early Returns**: Preflight requests return immediately without calling downstream middleware
- **Header Pooling**: Uses Rust's efficient HashMap for header storage

## Common Patterns

### Development vs Production

```rust
#[cfg(debug_assertions)]
fn cors_config() -> CorsConfig {
    CorsConfig::new().allow_all_origins()  // Permissive for development
}

#[cfg(not(debug_assertions))]
fn cors_config() -> CorsConfig {
    CorsConfig::new()
        .allow_origins(vec!["https://app.example.com".to_string()])
        .allow_credentials(true)  // Restrictive for production
}
```

### Multiple Frontend Domains

```rust
let config = CorsConfig::new().allow_origins(vec![
    "https://example.com".to_string(),
    "https://www.example.com".to_string(),
    "https://app.example.com".to_string(),
    "https://admin.example.com".to_string(),
]);
```

### Dynamic Subdomains

```rust
let config = CorsConfig::new()
    .allow_origin_regex(r"^https://[a-z0-9-]+\.example\.com$")
    .unwrap();
```

## References

- [W3C CORS Specification](https://www.w3.org/TR/cors/)
- [MDN CORS Documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
- [OWASP CORS Security](https://owasp.org/www-community/attacks/CORS_OriginHeaderScrutiny)
