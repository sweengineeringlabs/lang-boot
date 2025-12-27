# Rustboot Security Guide

Developer guide for implementing security in Rustboot applications.

**Audience**: Application Developers, DevOps Engineers

> **Related Documentation**:
> - [Security Overview](../3-design/security-overview.md) - Architecture and capabilities
> - [Security Audit Guide](../3-design/security-audit.md) - Compliance and vulnerabilities

## Overview

This guide shows how to use Rustboot's security features to build secure applications. It covers cryptography, authentication, input validation, rate limiting, and security best practices.

## Security Features by Category

### 🔐 Cryptography - `rustboot-crypto`

Industry-standard cryptographic operations for hashing, signing, and password security.

#### Password Security
```rust
use dev_engineeringlabs_rustboot_crypto::*;

// Hash password for storage
let hash = hash_password("user_password")?;

// Verify on login
verify_password("login_attempt", &hash)?;
```

**Why Bcrypt**: Slow by design to prevent brute force attacks, automatically salted.

#### Message Authentication (HMAC)
```rust
// Sign API requests
let signature = hmac_sha256(secret_key, request_body);

// Verify webhook signatures
if !verify_hmac(incoming_signature, expected_signature) {
    return Err("Invalid signature");
}
```

**Use Cases**: API authentication, webhook verification, message integrity.

#### Data Integrity (SHA256)
```rust
// Verify file integrity
let checksum = sha256(&file_data);
```

**Critical Rules**:
- ❌ Never roll your own crypto
- ❌ Never use MD5 or SHA1 (deprecated, insecure)
- ❌ Never store passwords in plain text
- ✅ Always use industry-standard algorithms

---

### 🔑 Authentication & Authorization - `rustboot-security`

Dedicated crate for identity and access management.

**Core Features**:
- **Authentication** - JWT tokens, sessions, OAuth2, API keys, MFA
- **Authorization** - RBAC, ABAC, permission policies
- **Secrets Management** - AES-256-GCM encryption, vault integration, secret rotation
- **Security Auditing** - Compliance logging (SOC 2, HIPAA, GDPR), audit trails, SIEM

**Status**: Initial structure created, implementation in progress.

**Documentation**: See [rustboot-security/overview.md](../crates/rustboot-security/docs/overview.md) for detailed API documentation and usage examples.

**Backlog**: See [rustboot-security/backlog.md](../crates/rustboot-security/backlog.md) for planned features.

---

### �🛡️ Input Validation - `rustboot-validation`

Prevent injection attacks, malformed data, and invalid inputs.

```rust
use dev_engineeringlabs_rustboot_validation::*;

// Email validation
let validator = StringValidationBuilder::new("email")
    .not_empty()
    .email()
    .build();

validator.validate(user_input)?;

// Prevent SQL injection via parameterized queries + validation
let validator = StringValidationBuilder::new("username")
    .not_empty()
    .max_length(50)
    .matches(r"^[a-zA-Z0-9_]+$") // Alphanumeric only
    .build();
```

**Security Benefits**:
- Prevents XSS attacks (cross-site scripting)
- Prevents SQL injection
- Prevents path traversal
- Validates data types and formats

---

### 🚧 Rate Limiting - `rustboot-ratelimit`

Protect against DoS attacks, brute force attempts, and API abuse.

#### Prevent Brute Force Attacks
```rust
use dev_engineeringlabs_rustboot_ratelimit::*;
use std::time::Duration;

// Login endpoint: 5 attempts per minute per IP
let limiter = TokenBucket::new(5, 5, Duration::from_secs(60));

async fn login_handler(ip: IpAddr) -> Result<Response> {
    match limiter.try_acquire().await {
        Ok(()) => process_login().await,
        Err(_) => {
            log::warn!("Rate limit exceeded for IP: {}", ip);
            Err("Too many login attempts. Try again later.")
        }
    }
}
```

#### API Protection
```rust
// Public API: 100 requests per hour per user
let api_limiter = SlidingWindow::new(100, Duration::from_secs(3600));
```

**Attack Mitigation**:
- DoS (Denial of Service) attacks
- Brute force password attempts
- API scraping/abuse
- Resource exhaustion

---

### 🔒 Secure File Operations - `rustboot-fileio`

Prevent directory traversal attacks and ensure crash-safe writes.

#### Directory Traversal Prevention
```rust
use dev_engineeringlabs_rustboot_fileio::*;

// UNSAFE: User could provide "../../etc/passwd"
let path = Path::new(base_dir).join(user_provided_path); // ❌ DANGEROUS

// SAFE: Validates and prevents traversal
let safe_path = safe_join(base_dir, user_provided_path)?; // ✅ SECURE
```

**Attack Prevention**: Prevents users from accessing files outside allowed directories.

#### Atomic Writes (Data Integrity)
```rust
// Crash-safe configuration writes
write_atomic("config.json", data)?;
```

**Security Benefit**: Prevents partial writes on crash (data corruption).

---

### 🔑 Authentication & Authorization - `rustboot-middleware`

**Current Status**: Planned in backlog

```rust
// Planned features (from backlog):
// - Built-in auth middleware
// - CORS middleware
// - Request validation middleware
```

---

### 🔐 Secret Management - `rustboot-config`

**Current Status**: Planned in backlog

Secure handling of API keys, database credentials, and secrets.

---

## Security Best Practices

### 1. Defense in Depth

Layer multiple security measures:

```rust
// Example: Secure user registration
async fn register_user(input: UserInput) -> Result<User> {
    // Layer 1: Input validation
    let validator = UserInputValidator::new();
    validator.validate(&input)?;
    
    // Layer 2: Rate limiting (prevent spam)
    rate_limiter.try_acquire().await?;
    
    // Layer 3: Password hashing
    let password_hash = hash_password(&input.password)?;
    
    // Layer 4: Safe file operations (if saving to file)
    let safe_path = safe_join(user_dir, &input.username)?;
    
    // Layer 5: Atomic write
    write_atomic(&safe_path, &user_data)?;
    
    Ok(user)
}
```

### 2. Fail Securely

```rust
// ❌ BAD: Reveals information
match authenticate(user, pass) {
    Err(AuthError::InvalidUser) => "User not found",
    Err(AuthError::InvalidPassword) => "Wrong password",
}

// ✅ GOOD: Generic error (prevents user enumeration)
match authenticate(user, pass) {
    Ok(session) => Ok(session),
    Err(_) => Err("Invalid credentials"), // Generic message
}
```

### 3. Validate All Inputs

```rust
// Validate EVERYTHING from untrusted sources
async fn api_handler(request: Request) -> Response {
    // Headers
    validate_content_type(&request)?;
    
    // Body
    let body: UserInput = validate_and_deserialize(request.body())?;
    
    // Query params
    validate_query_params(&request.query)?;
    
    // User-provided file paths
    let safe_path = safe_join(base, &body.file_path)?;
}
```

### 4. Use Secure Defaults

```rust
// ✅ GOOD: Framework provides secure defaults
let password_hash = hash_password(password)?; // Bcrypt with good defaults

// ✅ GOOD: Use safe helpers
let checksum = sha256(&data); // SHA256, not MD5

// ✅ GOOD: Atomic operations by default
write_atomic(path, data)?;
```

### 5. Log Security Events

```rust
use dev_engineeringlabs_rustboot_observability::*;

// Log authentication failures
if !auth_success {
    log::warn!(
        "Failed login attempt: user={}, ip={}", 
        username, 
        request_ip
    );
}

// Log rate limit violations
if rate_limit_exceeded {
    metric_counter!("security.rate_limit.exceeded", 1);
}
```

## Common Vulnerabilities Addressed

| Vulnerability | Mitigation | Crate |
|--------------|------------|-------|
| **SQL Injection** | Input validation, parameterized queries | `rustboot-validation` |
| **XSS** | Input validation, output escaping | `rustboot-validation` |
| **Path Traversal** | `safe_join()` validation | `rustboot-fileio` |
| **Weak Passwords** | Bcrypt hashing | `rustboot-crypto` |
| **Brute Force** | Rate limiting | `rustboot-ratelimit` |
| **DoS Attacks** | Rate limiting, circuit breakers | `rustboot-ratelimit`, `rustboot-resilience` |
| **Data Integrity** | SHA256 checksums, HMAC | `rustboot-crypto` |
| **Data Corruption** | Atomic writes | `rustboot-fileio` |
| **Unauthorized Access** | RBAC, permission checks | `rustboot-security` |
| **Session Hijacking** | JWT validation, secure sessions | `rustboot-security` |
| **Exposed Secrets** | Secret encryption, secure storage | `rustboot-security` |
| **Audit Trail Gaps** | Security event logging | `rustboot-security` |

## Security Checklist

### Development
- [ ] Validate all user inputs with `rustboot-validation`
- [ ] Use `safe_join()` for user-provided file paths
- [ ] Hash passwords with Bcrypt (`hash_password()`)
- [ ] Implement authentication with JWT tokens (`rustboot-security`)
- [ ] Use RBAC for authorization (`rustboot-security`)
- [ ] Encrypt secrets with AES-256-GCM (`rustboot-security`)
- [ ] Log security events with audit trail (`rustboot-security`)
- [ ] Never log sensitive data (passwords, tokens, keys)
- [ ] Use HMAC for API signatures
- [ ] Implement rate limiting on public endpoints

### Production
- [ ] Enable HTTPS/TLS (transport layer security)
- [ ] Use environment variables for secrets (not hardcoded)
- [ ] Enable security logging and monitoring
- [ ] Regular security dependency updates
- [ ] Implement proper error handling (don't expose internals)
- [ ] Set up rate limiting on all public APIs

## Security Features Status

### ✅ Available Now
- **Cryptography**: `rustboot-crypto` - SHA256, HMAC, Bcrypt
- **Input Validation**: `rustboot-validation` - XSS/SQL injection prevention
- **Rate Limiting**: `rustboot-ratelimit` - DoS/brute force protection
- **Secure File Ops**: `rustboot-fileio` - Path traversal prevention, atomic writes

### 🚧 In Development
- **Authentication**: `rustboot-security` - JWT, sessions, OAuth2, MFA
- **Authorization**: `rustboot-security` - RBAC, ABAC, permissions
- **Secrets Management**: `rustboot-security` - Encryption, rotation, vault integration
- **Security Auditing**: `rustboot-security` - Compliance logging, audit trails

See [rustboot-security/backlog.md](../crates/rustboot-security/backlog.md) for detailed roadmap.

### 📋 Planned Enhancements

See individual crate backlogs for additional planned security features:
- **Auth Middleware**: [rustboot-middleware/backlog.md](../crates/rustboot-middleware/backlog.md)
- **Secret Management Integration**: [rustboot-config/backlog.md](../crates/rustboot-config/backlog.md)
- **Advanced Crypto**: [rustboot-crypto/backlog.md](../crates/rustboot-crypto/backlog.md) - AES, RSA, JWT

---

**Last Updated**: 2025-12-22  
**Version**: 1.1
