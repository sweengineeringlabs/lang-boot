# Rustboot Security Best Practices Guide

This guide covers security best practices when building applications with the Rustboot framework.

## Table of Contents

1. [Input Validation](#input-validation)
2. [Authentication & Authorization](#authentication--authorization)
3. [Data Protection](#data-protection)
4. [HTTP Security Headers](#http-security-headers)
5. [Rate Limiting](#rate-limiting)
6. [Database Security](#database-security)
7. [Secrets Management](#secrets-management)
8. [Logging & Auditing](#logging--auditing)
9. [Dependency Security](#dependency-security)
10. [Production Hardening](#production-hardening)

---

## Input Validation

### Always Validate User Input

Use `rustboot-validation` to validate all incoming data:

```rust
use dev_engineeringlabs_rustboot_validation::{Validator, ValidationError};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl Validator for CreateUserRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        // Username validation
        if self.username.len() < 3 || self.username.len() > 50 {
            errors.push("Username must be 3-50 characters".to_string());
        }
        if !self.username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            errors.push("Username can only contain alphanumeric characters and underscores".to_string());
        }

        // Email validation
        if !self.email.contains('@') || !self.email.contains('.') {
            errors.push("Invalid email format".to_string());
        }

        // Password strength
        if self.password.len() < 12 {
            errors.push("Password must be at least 12 characters".to_string());
        }
        if !self.password.chars().any(|c| c.is_uppercase()) {
            errors.push("Password must contain an uppercase letter".to_string());
        }
        if !self.password.chars().any(|c| c.is_numeric()) {
            errors.push("Password must contain a number".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationError::Multiple(errors))
        }
    }
}
```

### Sanitize Output

Always sanitize data before rendering to prevent XSS:

```rust
use html_escape::encode_text;

fn render_user_content(content: &str) -> String {
    encode_text(content).to_string()
}
```

### Prevent SQL Injection

Always use parameterized queries with `rustboot-database`:

```rust
// GOOD: Parameterized query
let user = sqlx::query_as::<_, User>(
    "SELECT * FROM users WHERE id = $1"
)
.bind(user_id)
.fetch_one(&pool)
.await?;

// BAD: Never concatenate user input
// let query = format!("SELECT * FROM users WHERE id = {}", user_input);
```

---

## Authentication & Authorization

### JWT Best Practices

```rust
use dev_engineeringlabs_rustboot_security::{JwtConfig, JwtService};

// Configure JWT with strong settings
let jwt_config = JwtConfig {
    secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
    issuer: "your-app".to_string(),
    audience: "your-app-users".to_string(),
    expiration_hours: 1,  // Short-lived tokens
    refresh_expiration_days: 7,
};

// Always validate tokens
async fn authenticate(token: &str, jwt_service: &JwtService) -> Result<Claims, AuthError> {
    jwt_service.validate(token)
        .map_err(|_| AuthError::InvalidToken)
}
```

### Password Hashing

Always use strong password hashing:

```rust
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
```

### Role-Based Access Control

```rust
use dev_engineeringlabs_rustboot_security::{Permission, Role, RbacService};

// Define roles and permissions
let rbac = RbacService::new();
rbac.add_role(Role::new("admin").with_permissions(vec![
    Permission::new("users", "create"),
    Permission::new("users", "read"),
    Permission::new("users", "update"),
    Permission::new("users", "delete"),
]));

rbac.add_role(Role::new("user").with_permissions(vec![
    Permission::new("users", "read"),
]));

// Check permissions before actions
async fn delete_user(user: &User, target_id: Uuid, rbac: &RbacService) -> Result<(), Error> {
    if !rbac.has_permission(&user.role, "users", "delete") {
        return Err(Error::Forbidden);
    }
    // Proceed with deletion
    Ok(())
}
```

---

## Data Protection

### Encrypt Sensitive Data

Use `rustboot-crypto` for encryption:

```rust
use dev_engineeringlabs_rustboot_crypto::{Aes256Gcm, EncryptionService};

// Encrypt sensitive data before storage
let encryption = Aes256Gcm::new(&encryption_key);
let encrypted_ssn = encryption.encrypt(user.ssn.as_bytes())?;

// Decrypt when needed
let decrypted = encryption.decrypt(&encrypted_ssn)?;
```

### Secure Data in Transit

Always use HTTPS in production. Configure TLS:

```rust
use axum_server::tls_rustls::RustlsConfig;

let tls_config = RustlsConfig::from_pem_file(
    "certs/cert.pem",
    "certs/key.pem",
)
.await?;

axum_server::bind_rustls("0.0.0.0:443".parse()?, tls_config)
    .serve(app.into_make_service())
    .await?;
```

### Secure Data at Rest

- Encrypt database at the filesystem level
- Use encrypted backups
- Rotate encryption keys regularly

---

## HTTP Security Headers

Use `rustboot-middleware` security headers:

```rust
use dev_engineeringlabs_rustboot_middleware::security::{SecurityHeaders, SecurityConfig};

let security_config = SecurityConfig {
    // Content Security Policy
    content_security_policy: "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'".to_string(),

    // Prevent clickjacking
    x_frame_options: "DENY".to_string(),

    // Prevent MIME type sniffing
    x_content_type_options: "nosniff".to_string(),

    // Enable XSS protection
    x_xss_protection: "1; mode=block".to_string(),

    // HSTS - enforce HTTPS
    strict_transport_security: "max-age=31536000; includeSubDomains; preload".to_string(),

    // Referrer policy
    referrer_policy: "strict-origin-when-cross-origin".to_string(),

    // Permissions policy
    permissions_policy: "geolocation=(), microphone=(), camera=()".to_string(),
};

let security_middleware = SecurityHeaders::new(security_config);
```

---

## Rate Limiting

Protect against abuse with `rustboot-ratelimit`:

```rust
use dev_engineeringlabs_rustboot_ratelimit::{
    RateLimiter, SlidingWindowLimiter, TokenBucketLimiter,
};
use std::time::Duration;

// Global rate limiting
let global_limiter = SlidingWindowLimiter::new(
    1000,                      // 1000 requests
    Duration::from_secs(60),   // per minute
);

// Per-user rate limiting
let user_limiter = TokenBucketLimiter::new(
    100,                       // bucket capacity
    10,                        // refill rate per second
);

// Authentication endpoint (stricter limits)
let auth_limiter = SlidingWindowLimiter::new(
    5,                         // 5 attempts
    Duration::from_secs(300),  // per 5 minutes
);

// Apply in middleware
async fn rate_limit_middleware(
    limiter: &impl RateLimiter,
    client_id: &str,
) -> Result<(), RateLimitError> {
    if !limiter.try_acquire_for(client_id) {
        return Err(RateLimitError::TooManyRequests);
    }
    Ok(())
}
```

---

## Database Security

### Connection Security

```rust
use dev_engineeringlabs_rustboot_database::{DatabaseConfig, Pool};

let config = DatabaseConfig {
    // Use SSL for database connections
    url: "postgres://user:pass@host/db?sslmode=require",

    // Limit connection pool size
    max_connections: 20,
    min_connections: 5,

    // Connection timeout
    connect_timeout: Duration::from_secs(10),

    // Idle timeout
    idle_timeout: Duration::from_secs(300),
};
```

### Principle of Least Privilege

Create database users with minimal permissions:

```sql
-- Read-only user for reporting
CREATE USER reporting_user WITH PASSWORD 'secure_password';
GRANT SELECT ON ALL TABLES IN SCHEMA public TO reporting_user;

-- Application user with limited write access
CREATE USER app_user WITH PASSWORD 'secure_password';
GRANT SELECT, INSERT, UPDATE ON users, orders TO app_user;
-- No DELETE permission unless absolutely necessary
```

---

## Secrets Management

### Environment Variables

Never hardcode secrets:

```rust
// GOOD: Load from environment
let database_url = std::env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");

let jwt_secret = std::env::var("JWT_SECRET")
    .expect("JWT_SECRET must be set");

// BAD: Hardcoded secrets
// let jwt_secret = "my-secret-key";  // NEVER DO THIS
```

### Secrets Rotation

Implement secret rotation support:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct SecretManager {
    current_secret: Arc<RwLock<String>>,
    previous_secret: Arc<RwLock<Option<String>>>,
}

impl SecretManager {
    pub async fn rotate(&self, new_secret: String) {
        let current = self.current_secret.read().await.clone();
        *self.previous_secret.write().await = Some(current);
        *self.current_secret.write().await = new_secret;
    }

    pub async fn validate(&self, token: &str) -> bool {
        // Try current secret first
        if self.validate_with_secret(token, &*self.current_secret.read().await) {
            return true;
        }
        // Fall back to previous secret during rotation
        if let Some(prev) = &*self.previous_secret.read().await {
            return self.validate_with_secret(token, prev);
        }
        false
    }
}
```

---

## Logging & Auditing

### Secure Logging

Never log sensitive data:

```rust
use tracing::{info, warn};

// GOOD: Log action without sensitive data
info!(
    user_id = %user.id,
    action = "password_changed",
    ip = %client_ip,
    "User changed password"
);

// BAD: Never log passwords, tokens, or PII
// info!("User {} changed password to {}", user.email, new_password);
```

### Audit Trail

Maintain an audit log for security-sensitive actions:

```rust
#[derive(Debug, Serialize)]
pub struct AuditEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub actor_id: Option<Uuid>,
    pub resource_type: String,
    pub resource_id: String,
    pub action: String,
    pub ip_address: String,
    pub user_agent: String,
    pub details: serde_json::Value,
}

pub async fn audit_log(event: AuditEvent, pool: &Pool) {
    sqlx::query(
        "INSERT INTO audit_log (timestamp, event_type, actor_id, resource_type,
         resource_id, action, ip_address, user_agent, details)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(event.timestamp)
    .bind(&event.event_type)
    .bind(event.actor_id)
    .bind(&event.resource_type)
    .bind(&event.resource_id)
    .bind(&event.action)
    .bind(&event.ip_address)
    .bind(&event.user_agent)
    .bind(&event.details)
    .execute(pool)
    .await
    .expect("Failed to write audit log");
}
```

---

## Dependency Security

### Regular Audits

Run security audits regularly:

```bash
# Install cargo-audit
cargo install cargo-audit

# Run audit
cargo audit

# Fix vulnerabilities
cargo update
```

### Dependency Pinning

Pin dependencies in production:

```toml
# Cargo.toml
[dependencies]
serde = "=1.0.193"  # Exact version
tokio = "=1.35.0"
```

### Minimal Dependencies

Only include necessary dependencies:

```toml
# Use feature flags to minimize attack surface
[dependencies]
tokio = { version = "1.0", features = ["rt-multi-thread", "net"], default-features = false }
```

---

## Production Hardening

### Disable Debug Mode

```rust
// Cargo.toml
[profile.release]
debug = false
strip = true
lto = true
panic = "abort"
```

### Error Handling

Never expose internal errors to users:

```rust
pub async fn handle_error(err: Error) -> (StatusCode, Json<ApiError>) {
    // Log the full error internally
    tracing::error!(error = ?err, "Internal error occurred");

    // Return generic error to user
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            error: "internal_error".to_string(),
            message: "An unexpected error occurred".to_string(),
            code: 500,
        }),
    )
}
```

### Health Check Security

Protect detailed health endpoints:

```rust
// Public health check - minimal info
async fn health() -> StatusCode {
    StatusCode::OK
}

// Protected detailed health - requires auth
async fn health_detailed(auth: AuthenticatedAdmin) -> Json<DetailedHealth> {
    // Only accessible by authenticated admins
    Json(get_detailed_health().await)
}
```

---

## Security Checklist

Before deploying to production:

- [ ] All user input is validated
- [ ] SQL queries are parameterized
- [ ] Passwords are hashed with Argon2/bcrypt
- [ ] JWT tokens have short expiration
- [ ] HTTPS is enforced
- [ ] Security headers are configured
- [ ] Rate limiting is enabled
- [ ] Secrets are in environment variables
- [ ] Audit logging is enabled
- [ ] Dependencies are audited
- [ ] Debug mode is disabled
- [ ] Error messages don't leak internals
- [ ] Database connections use SSL
- [ ] CORS is properly configured
- [ ] File uploads are validated and sandboxed

---

## Additional Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [CWE/SANS Top 25](https://cwe.mitre.org/top25/)
