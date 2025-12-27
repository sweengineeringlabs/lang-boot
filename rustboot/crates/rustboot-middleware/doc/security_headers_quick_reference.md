# Security Headers - Quick Reference

## Quick Start

```rust
use rustboot_middleware::SecurityHeadersMiddleware;

// Use secure defaults
let middleware = SecurityHeadersMiddleware::default();
```

## Common Configurations

### Production (Strict)

```rust
use rustboot_middleware::security::{SecurityHeadersConfig, SecurityHeadersMiddleware};

let config = SecurityHeadersConfig::default();
let middleware = SecurityHeadersMiddleware::new(config);
```

**Sets:**
- CSP: `default-src 'self'`
- HSTS: 1 year + subdomains + preload
- X-Frame-Options: `DENY`
- X-Content-Type-Options: `nosniff`
- X-XSS-Protection: `1; mode=block`
- Referrer-Policy: `strict-origin-when-cross-origin`
- Permissions-Policy: Restrictive

### Development (Permissive)

```rust
let middleware = SecurityHeadersMiddleware::permissive();
```

**Sets:**
- CSP: Allows inline scripts/styles
- No HSTS (allows HTTP)
- X-Frame-Options: `SAMEORIGIN`
- Other headers: Safe defaults

### Web Application

```rust
let config = SecurityHeadersConfig::new()
    .with_csp("default-src 'self'; script-src 'self' https://cdn.example.com")
    .with_hsts(31536000, true, true)
    .with_frame_options("DENY")
    .with_content_type_options("nosniff")
    .with_referrer_policy("strict-origin-when-cross-origin");
```

### API Endpoint

```rust
let config = SecurityHeadersConfig::new()
    .with_hsts(31536000, true, true)
    .with_content_type_options("nosniff")
    .with_frame_options("DENY");
```

## Header Cheat Sheet

| Header | Purpose | Common Values |
|--------|---------|---------------|
| Content-Security-Policy | Control resource loading | `default-src 'self'` |
| Strict-Transport-Security | Force HTTPS | `max-age=31536000; includeSubDomains; preload` |
| X-Frame-Options | Prevent clickjacking | `DENY`, `SAMEORIGIN` |
| X-Content-Type-Options | Prevent MIME sniffing | `nosniff` |
| X-XSS-Protection | XSS filter (legacy) | `1; mode=block` |
| Referrer-Policy | Control referrer info | `strict-origin-when-cross-origin` |
| Permissions-Policy | Control browser features | `geolocation=(), microphone=()` |

## CSP Directives Quick Reference

```rust
// Strict (recommended starting point)
.with_csp("default-src 'self'")

// Common additions
.with_csp("
    default-src 'self';
    script-src 'self' https://cdn.example.com;
    style-src 'self' 'unsafe-inline';
    img-src 'self' data: https:;
    font-src 'self' https://fonts.gstatic.com;
    connect-src 'self' https://api.example.com;
    frame-ancestors 'none';
    base-uri 'self';
    form-action 'self'
")

// Development (permissive)
.with_csp("default-src 'self' 'unsafe-inline' 'unsafe-eval'")
```

## HSTS Configuration

```rust
// 1 year, basic
.with_hsts(31536000, false, false)

// 1 year, with subdomains
.with_hsts(31536000, true, false)

// 1 year, with subdomains and preload
.with_hsts(31536000, true, true)

// 2 years (recommended for production)
.with_hsts(63072000, true, true)

// 5 minutes (for testing)
.with_hsts(300, false, false)

// Disable HSTS
.without_hsts()
```

## Referrer Policy Values

```rust
.with_referrer_policy("no-referrer")                      // Never send
.with_referrer_policy("no-referrer-when-downgrade")       // Don't send on HTTPS→HTTP
.with_referrer_policy("origin")                           // Send origin only
.with_referrer_policy("origin-when-cross-origin")         // Full for same-origin
.with_referrer_policy("same-origin")                      // Only same-origin
.with_referrer_policy("strict-origin")                    // Origin, no downgrade
.with_referrer_policy("strict-origin-when-cross-origin")  // Recommended default
.with_referrer_policy("unsafe-url")                       // Always send full URL
```

## Disabling Headers

```rust
SecurityHeadersConfig::default()
    .without_csp()              // Remove CSP
    .without_hsts()             // Remove HSTS
    .without_frame_options()    // Remove X-Frame-Options
    .without_content_type_options()  // Remove X-Content-Type-Options
    .without_xss_protection()   // Remove X-XSS-Protection
    .without_referrer_policy()  // Remove Referrer-Policy
    .without_permissions_policy()  // Remove Permissions-Policy
```

## Environment-Based Configuration

```rust
use std::env;

fn get_security_middleware() -> SecurityHeadersMiddleware {
    match env::var("ENVIRONMENT").as_deref() {
        Ok("production") => SecurityHeadersMiddleware::secure(),
        Ok("staging") => {
            let config = SecurityHeadersConfig::default()
                .with_hsts(86400, false, false); // 1 day for testing
            SecurityHeadersMiddleware::new(config)
        }
        _ => SecurityHeadersMiddleware::permissive(),
    }
}
```

## Common Use Cases

### Static Website

```rust
SecurityHeadersConfig::new()
    .with_csp("default-src 'none'; img-src 'self'; style-src 'self'; script-src 'self'")
    .with_hsts(31536000, true, true)
    .with_frame_options("DENY")
    .with_content_type_options("nosniff")
    .with_referrer_policy("no-referrer")
```

### SPA (Single Page Application)

```rust
SecurityHeadersConfig::new()
    .with_csp("default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; connect-src 'self' https://api.example.com")
    .with_hsts(31536000, true, true)
    .with_frame_options("DENY")
    .with_content_type_options("nosniff")
```

### WordPress/CMS

```rust
SecurityHeadersConfig::new()
    .with_csp("default-src 'self' 'unsafe-inline' 'unsafe-eval'")
    .with_hsts(31536000, true, false)
    .with_frame_options("SAMEORIGIN")
    .with_content_type_options("nosniff")
```

### REST API

```rust
SecurityHeadersConfig::new()
    .with_hsts(31536000, true, true)
    .with_content_type_options("nosniff")
    .with_frame_options("DENY")
    .with_referrer_policy("no-referrer")
```

## Testing Headers

```rust
#[tokio::test]
async fn test_security_headers() {
    use rustboot_middleware::{Pipeline, SecurityHeadersMiddleware};
    use rustboot_middleware::security::HasHeaders;

    #[derive(Debug, Clone)]
    struct Response {
        headers: Vec<(String, String)>,
    }

    impl HasHeaders for Response {
        fn add_header(&mut self, name: String, value: String) {
            self.headers.push((name, value));
        }
    }

    let middleware = SecurityHeadersMiddleware::default();
    let pipeline = Pipeline::new().with_middleware(middleware);

    let response = Response { headers: vec![] };
    let result = pipeline.execute(response).await.unwrap();

    assert!(result.headers.iter().any(|(n, _)| n == "Content-Security-Policy"));
}
```

## Troubleshooting

### CSP is breaking my site

1. Start with permissive: `default-src 'self' 'unsafe-inline' 'unsafe-eval'`
2. Check browser console for violations
3. Gradually tighten by removing `'unsafe-*'` directives
4. Add specific sources as needed

### HSTS preventing HTTP access

HSTS is working as intended. Options:
1. Clear browser HSTS cache
2. Use shorter max-age during testing
3. Only enable HSTS in production

### Headers not appearing

1. Ensure your type implements `HasHeaders`
2. Verify middleware is in the pipeline
3. Check that headers aren't being overwritten later

## Security Checklist

- [ ] Use HTTPS before enabling HSTS
- [ ] Start with strict CSP, relax as needed
- [ ] Test HSTS with short max-age first
- [ ] Set X-Frame-Options to DENY unless iframes needed
- [ ] Always set X-Content-Type-Options to nosniff
- [ ] Use strict-origin-when-cross-origin for Referrer-Policy
- [ ] Disable unused browser features in Permissions-Policy
- [ ] Consider disabling X-XSS-Protection if you have strong CSP
- [ ] Test in staging before deploying to production
- [ ] Monitor CSP violation reports

## Resources

- [OWASP Secure Headers](https://owasp.org/www-project-secure-headers/)
- [MDN HTTP Headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers)
- [CSP Reference](https://content-security-policy.com/)
- [Security Headers Checker](https://securityheaders.com/)
