# Security Headers Middleware Examples

## Running the Examples

### Security Headers Example

This comprehensive example demonstrates all features of the security headers middleware:

```bash
cargo run --example security_headers
```

The example shows:
1. Default secure configuration
2. Permissive configuration for development
3. Custom configuration
4. Selective headers
5. API endpoint configuration
6. Combining with other middleware
7. Best practices guide

## Quick Start

### 1. Add to your dependencies

The middleware is part of `rustboot-middleware`:

```toml
[dependencies]
dev-engineeringlabs-rustboot-middleware = "0.1.0"
```

### 2. Implement HasHeaders trait

Your response type needs to implement the `HasHeaders` trait:

```rust
use rustboot_middleware::security::HasHeaders;

struct MyResponse {
    headers: Vec<(String, String)>,
    body: String,
}

impl HasHeaders for MyResponse {
    fn add_header(&mut self, name: String, value: String) {
        self.headers.push((name, value));
    }
}
```

### 3. Create and use the middleware

```rust
use rustboot_middleware::{Pipeline, SecurityHeadersMiddleware};

// Use default secure configuration
let middleware = SecurityHeadersMiddleware::default();
let pipeline = Pipeline::new().with_middleware(middleware);

// Apply to your response
let response = MyResponse::new();
let secured = pipeline.execute(response).await?;
```

## Common Patterns

### Production Configuration

```rust
use rustboot_middleware::security::{SecurityHeadersConfig, SecurityHeadersMiddleware};

let config = SecurityHeadersConfig::default()
    .with_csp("default-src 'self'; script-src 'self' https://cdn.example.com")
    .with_hsts(31536000, true, true);

let middleware = SecurityHeadersMiddleware::new(config);
```

### Development Configuration

```rust
// Permissive for development
let middleware = SecurityHeadersMiddleware::permissive();
```

### Custom Configuration

```rust
let config = SecurityHeadersConfig::new()
    .with_csp("default-src 'self'")
    .with_hsts(31536000, true, true)
    .with_frame_options("DENY")
    .with_referrer_policy("strict-origin-when-cross-origin");

let middleware = SecurityHeadersMiddleware::new(config);
```

## Documentation

For more information, see:

- **Comprehensive Guide**: `../doc/security_headers.md`
- **Quick Reference**: `../doc/security_headers_quick_reference.md`
- **Implementation Details**: `../doc/SECURITY_HEADERS_IMPLEMENTATION.md`

## Testing

Run the tests:

```bash
# All tests
cargo test

# Security tests only
cargo test security::

# Integration tests
cargo test --test security_headers_integration
```

## Support

For issues or questions, please refer to the main Rustboot repository.
