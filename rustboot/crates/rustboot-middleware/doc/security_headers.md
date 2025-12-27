# Security Headers Middleware

The Security Headers middleware provides a robust solution for adding security-related HTTP headers to your responses. This helps protect your application against common web vulnerabilities.

## Overview

Security headers are HTTP response headers that instruct browsers to enable built-in security mechanisms. The `SecurityHeadersMiddleware` makes it easy to configure and apply these headers consistently across your application.

## Supported Headers

### 1. Content-Security-Policy (CSP)

**Purpose**: Controls which resources (scripts, styles, images, etc.) can be loaded by the browser.

**Protection**: Prevents Cross-Site Scripting (XSS) and other code injection attacks.

**Example**:
```rust
use rustboot_middleware::security::SecurityHeadersConfig;

let config = SecurityHeadersConfig::new()
    .with_csp("default-src 'self'; script-src 'self' https://cdn.example.com");
```

**Common Values**:
- `default-src 'self'` - Only allow resources from same origin
- `script-src 'self' 'unsafe-inline'` - Allow inline scripts (not recommended)
- `img-src * data:` - Allow images from any source

### 2. Strict-Transport-Security (HSTS)

**Purpose**: Forces browsers to use HTTPS connections.

**Protection**: Prevents downgrade attacks and cookie hijacking.

**Example**:
```rust
// 1 year HSTS with subdomains and preload
let config = SecurityHeadersConfig::new()
    .with_hsts(31536000, true, true);
```

**Parameters**:
- `max_age`: Duration in seconds (typically 31536000 = 1 year)
- `include_subdomains`: Apply to all subdomains
- `preload`: Submit to browser preload lists

**Important**: Only enable HSTS after confirming HTTPS works correctly!

### 3. X-Frame-Options

**Purpose**: Controls whether the page can be embedded in frames/iframes.

**Protection**: Prevents clickjacking attacks.

**Example**:
```rust
let config = SecurityHeadersConfig::new()
    .with_frame_options("DENY");
```

**Values**:
- `DENY` - Never allow framing (most secure)
- `SAMEORIGIN` - Allow framing from same origin
- `ALLOW-FROM uri` - Allow from specific URI (deprecated)

### 4. X-Content-Type-Options

**Purpose**: Prevents MIME type sniffing.

**Protection**: Prevents browsers from interpreting files as a different MIME type than declared.

**Example**:
```rust
let config = SecurityHeadersConfig::new()
    .with_content_type_options("nosniff");
```

**Value**: Always set to `nosniff`

### 5. X-XSS-Protection

**Purpose**: Enables browser's built-in XSS filter (legacy).

**Protection**: Detects and blocks reflected XSS attacks in older browsers.

**Example**:
```rust
let config = SecurityHeadersConfig::new()
    .with_xss_protection("1; mode=block");
```

**Note**: This is a legacy header. Modern browsers rely on CSP instead. Can be disabled if you have a strong CSP.

**Values**:
- `1; mode=block` - Enable and block on detection
- `0` - Disable (use with modern CSP)

### 6. Referrer-Policy

**Purpose**: Controls how much referrer information is sent with requests.

**Protection**: Prevents leaking sensitive information in URLs.

**Example**:
```rust
let config = SecurityHeadersConfig::new()
    .with_referrer_policy("strict-origin-when-cross-origin");
```

**Common Values**:
- `no-referrer` - Never send referrer
- `strict-origin-when-cross-origin` - Balanced privacy/functionality
- `same-origin` - Only send for same-origin requests
- `unsafe-url` - Always send full URL (not recommended)

### 7. Permissions-Policy

**Purpose**: Controls which browser features and APIs can be used.

**Protection**: Prevents malicious code from accessing sensitive APIs.

**Example**:
```rust
let config = SecurityHeadersConfig::new()
    .with_permissions_policy("geolocation=(), microphone=(), camera=()");
```

**Common Restrictions**:
- `geolocation=()` - Disable geolocation
- `microphone=()` - Disable microphone access
- `camera=()` - Disable camera access
- `payment=()` - Disable payment API

## Usage Examples

### Basic Usage with Default Configuration

```rust
use rustboot_middleware::{Pipeline, SecurityHeadersMiddleware};

let middleware = SecurityHeadersMiddleware::default();
let pipeline = Pipeline::new().with_middleware(middleware);

let response = create_response();
let result = pipeline.execute(response).await?;
```

### Custom Configuration

```rust
use rustboot_middleware::security::{SecurityHeadersConfig, SecurityHeadersMiddleware};

let config = SecurityHeadersConfig::new()
    .with_csp("default-src 'self'; script-src 'self' https://cdn.example.com")
    .with_hsts(31536000, true, true)
    .with_frame_options("SAMEORIGIN")
    .with_referrer_policy("strict-origin-when-cross-origin");

let middleware = SecurityHeadersMiddleware::new(config);
```

### Development vs Production

```rust
// Development: Permissive configuration
#[cfg(debug_assertions)]
let middleware = SecurityHeadersMiddleware::permissive();

// Production: Secure configuration
#[cfg(not(debug_assertions))]
let middleware = SecurityHeadersMiddleware::secure();
```

### Selective Headers

```rust
// Enable only specific headers
let config = SecurityHeadersConfig::new()
    .with_hsts(31536000, true, true)
    .with_content_type_options("nosniff")
    .with_frame_options("DENY");

// Disable specific headers
let config = SecurityHeadersConfig::default()
    .without_xss_protection()
    .without_permissions_policy();
```

## Configuration Presets

### 1. Default (Secure)

```rust
let middleware = SecurityHeadersMiddleware::default();
// or
let middleware = SecurityHeadersMiddleware::secure();
```

Includes:
- CSP: `default-src 'self'`
- HSTS: 1 year with subdomains and preload
- X-Frame-Options: `DENY`
- X-Content-Type-Options: `nosniff`
- X-XSS-Protection: `1; mode=block`
- Referrer-Policy: `strict-origin-when-cross-origin`
- Permissions-Policy: Restrictive defaults

### 2. Permissive (Development)

```rust
let middleware = SecurityHeadersMiddleware::permissive();
```

Includes:
- CSP: `default-src 'self' 'unsafe-inline' 'unsafe-eval'`
- No HSTS (to allow HTTP in development)
- X-Frame-Options: `SAMEORIGIN`
- X-Content-Type-Options: `nosniff`
- X-XSS-Protection: `1; mode=block`
- Referrer-Policy: `no-referrer-when-downgrade`
- No Permissions-Policy

### 3. Empty (Build Your Own)

```rust
let config = SecurityHeadersConfig::new();
// Add only the headers you need
```

## Integration with Your Application

### Implementing HasHeaders Trait

To use the security headers middleware, your context type must implement the `HasHeaders` trait:

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

### Combining with Other Middleware

```rust
use rustboot_middleware::{
    Pipeline,
    SecurityHeadersMiddleware,
    TimingMiddleware,
    LoggingMiddleware,
};

let pipeline = Pipeline::new()
    .with_middleware(LoggingMiddleware::new("app"))
    .with_middleware(TimingMiddleware::new("app"))
    .with_middleware(SecurityHeadersMiddleware::default());

let result = pipeline.execute(context).await?;
```

## Security Best Practices

### 1. Start Strict, Then Relax

Begin with the most restrictive configuration and gradually add exceptions as needed:

```rust
// Start with this
let config = SecurityHeadersConfig::new()
    .with_csp("default-src 'none'");

// Then add what you need
let config = config
    .with_csp("default-src 'self'; script-src 'self' https://trusted-cdn.com");
```

### 2. Test Before Deploying HSTS

HSTS is hard to undo once deployed. Test thoroughly:

```rust
// Development/Staging: Short max-age
let config = SecurityHeadersConfig::new()
    .with_hsts(300, false, false); // 5 minutes

// Production: Long max-age
let config = SecurityHeadersConfig::new()
    .with_hsts(31536000, true, true); // 1 year
```

### 3. Use Environment-Based Configuration

```rust
fn get_security_config() -> SecurityHeadersConfig {
    match std::env::var("ENVIRONMENT").as_deref() {
        Ok("production") => SecurityHeadersConfig::default(),
        Ok("staging") => SecurityHeadersConfig::default()
            .with_hsts(86400, false, false), // 1 day
        _ => SecurityHeadersConfig::permissive(),
    }
}
```

### 4. Monitor CSP Violations

In production, use CSP report-uri to monitor violations:

```rust
let config = SecurityHeadersConfig::new()
    .with_csp("default-src 'self'; report-uri /csp-violation-report");
```

### 5. Regular Security Audits

Periodically review and update your security headers:
- Check for new security features
- Remove deprecated headers
- Tighten policies as your app matures

## Testing

The middleware includes comprehensive tests. Run them with:

```bash
cargo test --lib security::
```

Example test:

```rust
#[tokio::test]
async fn test_custom_security_headers() {
    let config = SecurityHeadersConfig::new()
        .with_csp("default-src 'self'")
        .with_hsts(31536000, true, true);

    let middleware = SecurityHeadersMiddleware::new(config);
    let pipeline = Pipeline::new().with_middleware(middleware);

    let ctx = TestContext::new();
    let result = pipeline.execute(ctx).await;

    assert!(result.is_ok());
    let ctx = result.unwrap();
    assert!(ctx.has_header("Content-Security-Policy"));
    assert!(ctx.has_header("Strict-Transport-Security"));
}
```

## Common Pitfalls

### 1. CSP Too Permissive

**Bad**:
```rust
.with_csp("default-src * 'unsafe-inline' 'unsafe-eval'")
```

**Good**:
```rust
.with_csp("default-src 'self'; script-src 'self' https://trusted-cdn.com")
```

### 2. HSTS Without HTTPS

Don't enable HSTS until HTTPS is fully working and tested.

### 3. Forgetting Subdomains

If you have subdomains, remember to configure HSTS appropriately:

```rust
.with_hsts(31536000, true, false) // Include subdomains
```

### 4. Overly Restrictive in Development

Use the permissive preset for development:

```rust
#[cfg(debug_assertions)]
let middleware = SecurityHeadersMiddleware::permissive();
```

## Further Reading

- [OWASP Secure Headers Project](https://owasp.org/www-project-secure-headers/)
- [MDN Web Security Headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers#security)
- [Content Security Policy Reference](https://content-security-policy.com/)
- [HSTS Preload List](https://hstspreload.org/)

## Version History

- v0.1.0: Initial implementation with all major security headers
