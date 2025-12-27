# Rustboot Security Best Practices Guide

This guide covers comprehensive security best practices for building secure applications with the Rustboot framework.

## Table of Contents

1. [Authentication Best Practices](#1-authentication-best-practices)
2. [Authorization Patterns](#2-authorization-patterns)
3. [Input Validation](#3-input-validation)
4. [Security Headers](#4-security-headers)
5. [Secrets Management](#5-secrets-management)
6. [HTTPS/TLS Configuration](#6-httpstls-configuration)
7. [Rate Limiting for Abuse Prevention](#7-rate-limiting-for-abuse-prevention)
8. [Logging Sensitive Data](#8-logging-sensitive-data)
9. [Dependency Security](#9-dependency-security)

---

## 1. Authentication Best Practices

### 1.1 Password Hashing with Bcrypt

**Always hash passwords before storing them.** Rustboot provides built-in bcrypt support via the `rustboot-crypto` crate.

```rust
use dev_engineeringlabs_rustboot_crypto::{hash_password, verify_password};

async fn register_user(username: &str, password: &str) -> Result<(), Error> {
    // Hash the password with bcrypt (cost factor: 12)
    let hashed = hash_password(password)?;

    // Store username and hashed password in database
    db.execute(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
        &[&username, &hashed]
    ).await?;

    Ok(())
}

async fn authenticate_user(username: &str, password: &str) -> Result<bool, Error> {
    // Fetch the stored hash from database
    let stored_hash: String = db.query_one(
        "SELECT password_hash FROM users WHERE username = $1",
        &[&username]
    ).await?;

    // Verify the password against the hash
    let is_valid = verify_password(password, &stored_hash)?;

    Ok(is_valid)
}
```

**Best Practices:**
- Never store plaintext passwords
- Use bcrypt's default cost factor (12) or higher for production
- Consider Argon2 for even stronger password hashing (add as a custom dependency)
- Implement account lockout after multiple failed attempts
- Enforce strong password policies (minimum length, complexity)

### 1.2 JWT Token Handling

Use JWT tokens for stateless authentication. Rustboot provides basic JWT support in `rustboot-security`.

```rust
use dev_engineeringlabs_rustboot_security::auth::{generate_jwt, validate_jwt};
use std::time::Duration;

async fn login(user_id: &str) -> Result<String, Error> {
    // Generate JWT with 1 hour expiration
    let token = generate_jwt(user_id, Duration::from_secs(3600))?;

    Ok(token)
}

async fn authenticate_request(token: &str) -> Result<String, Error> {
    // Validate and extract claims from JWT
    let claims = validate_jwt(token)?;

    // Check if token is expired (automatically done in validate_jwt)
    Ok(claims.sub) // Return user ID
}
```

**Best Practices:**
- **Use short expiration times** (15-60 minutes for access tokens)
- **Implement refresh tokens** for long-lived sessions
- **Sign tokens with strong secrets** (use environment variables)
- **Never put sensitive data in JWT payload** (it's base64-encoded, not encrypted)
- **Validate tokens on every request**
- **Implement token revocation** for logout and security events
- **Use HTTPS only** to prevent token interception

**For Production:**
```rust
// Use a proper JWT library like `jsonwebtoken` for production:
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

fn create_jwt(user_id: &str, secret: &[u8]) -> Result<String, Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() + 3600;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
        iat: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret)
    ).map_err(Into::into)
}
```

### 1.3 Session Security

For stateful authentication, use the `rustboot-session` crate with secure configuration.

```rust
use dev_engineeringlabs_rustboot_session::{
    SessionManager, MemorySessionStore, SessionConfig, SameSite
};
use std::time::Duration;

async fn setup_session_manager() -> SessionManager<MemorySessionStore> {
    let store = MemorySessionStore::new();

    let config = SessionConfig::default()
        .with_ttl(Duration::from_secs(3600))      // 1 hour session
        .with_cookie_name("sid")                   // Custom cookie name
        .with_secure(true)                         // HTTPS only
        .with_http_only(true)                      // Prevent XSS access
        .with_same_site(SameSite::Strict);        // CSRF protection

    SessionManager::new(store, config)
}

async fn handle_login(manager: &SessionManager<MemorySessionStore>, user_id: u64)
    -> Result<String, Error>
{
    // Create new session
    let (session_id, mut data) = manager.create().await?;

    // Store user information
    manager.update(&session_id, |data| {
        data.set("user_id", user_id)?;
        data.set("login_time", SystemTime::now())?;
        Ok(())
    }).await?;

    Ok(session_id.to_string())
}

async fn handle_logout(manager: &SessionManager<MemorySessionStore>, session_id: &str)
    -> Result<(), Error>
{
    // Delete session
    manager.delete(&session_id.parse()?).await?;
    Ok(())
}
```

**Best Practices:**
- **Always set `secure: true`** in production (HTTPS only)
- **Enable `http_only`** to prevent JavaScript access
- **Use `SameSite::Strict` or `SameSite::Lax`** for CSRF protection
- **Regenerate session IDs after login** to prevent session fixation
- **Use persistent stores** (Redis/Database) for production
- **Implement session timeout** and idle timeout
- **Clear sessions on logout**

---

## 2. Authorization Patterns

### 2.1 Role-Based Access Control (RBAC)

Rustboot provides RBAC through the `rustboot-security` crate.

```rust
use dev_engineeringlabs_rustboot_security::authz::{
    RoleBasedAccessControl, User, AuthorizationContext
};

fn setup_rbac() -> RoleBasedAccessControl {
    let mut rbac = RoleBasedAccessControl::new();

    // Create custom roles
    rbac.create_role("developer").unwrap();
    rbac.create_role("manager").unwrap();

    // Grant permissions to roles
    rbac.grant_permission("admin", "users:delete").unwrap();
    rbac.grant_permission("admin", "users:create").unwrap();
    rbac.grant_permission("admin", "users:read").unwrap();

    rbac.grant_permission("manager", "users:read").unwrap();
    rbac.grant_permission("manager", "reports:generate").unwrap();

    rbac.grant_permission("developer", "code:write").unwrap();
    rbac.grant_permission("developer", "code:read").unwrap();

    rbac.grant_permission("user", "profile:read").unwrap();
    rbac.grant_permission("user", "profile:update").unwrap();

    rbac
}

async fn check_authorization(rbac: &RoleBasedAccessControl, user_id: &str)
    -> Result<(), Error>
{
    // Load user from database
    let mut user = User::new(user_id);
    user.add_role("developer");
    user.add_role("user");

    // Create authorization context
    let ctx = AuthorizationContext::new(rbac, &user);

    // Check permissions
    if ctx.has_permission("code:write")? {
        println!("User can write code");
    }

    // Require permission (throws error if not granted)
    ctx.require_permission("code:write")?;

    Ok(())
}
```

### 2.2 Permission Checking Middleware

Create reusable middleware for authorization checks:

```rust
use dev_engineeringlabs_rustboot_middleware::{Middleware, MiddlewareResult, Next};
use std::pin::Pin;

pub struct AuthorizationMiddleware {
    required_permission: String,
    rbac: Arc<RoleBasedAccessControl>,
}

impl AuthorizationMiddleware {
    pub fn new(permission: impl Into<String>, rbac: Arc<RoleBasedAccessControl>) -> Self {
        Self {
            required_permission: permission.into(),
            rbac,
        }
    }
}

impl<Ctx> Middleware<Ctx> for AuthorizationMiddleware
where
    Ctx: HasUser + Send + 'static,
{
    fn handle(
        &self,
        ctx: Ctx,
        next: Next<Ctx>,
    ) -> Pin<Box<dyn Future<Output = MiddlewareResult<Ctx>> + Send>> {
        let rbac = Arc::clone(&self.rbac);
        let permission = self.required_permission.clone();

        Box::pin(async move {
            let user = ctx.get_user();
            let auth_ctx = AuthorizationContext::new(&rbac, &user);

            // Check permission
            auth_ctx.require_permission(&permission)
                .map_err(|e| format!("Authorization failed: {}", e))?;

            // Continue if authorized
            next(ctx).await
        })
    }
}
```

**Best Practices:**
- **Principle of Least Privilege**: Grant minimum permissions needed
- **Centralize permission definitions** in a constants file
- **Use hierarchical permissions** (e.g., `users:read`, `users:write`, `users:delete`)
- **Check permissions on every protected operation**
- **Log authorization failures** for security monitoring
- **Separate authorization from authentication**

---

## 3. Input Validation

### 3.1 Using rustboot-validation

Always validate user input to prevent injection attacks and data corruption.

```rust
use dev_engineeringlabs_rustboot_validation::{
    StringValidationBuilder, NumericValidationBuilder, Validate
};

#[derive(Debug)]
struct UserRegistration {
    username: String,
    email: String,
    password: String,
    age: i32,
}

fn validate_registration(input: &UserRegistration) -> Result<(), ValidationErrors> {
    // Username validation
    StringValidationBuilder::new("username")
        .not_empty()
        .min_length(3)
        .max_length(20)
        .matches(
            |s: &String| s.chars().all(|c| c.is_alphanumeric() || c == '_'),
            "Username must contain only alphanumeric characters and underscores"
        )
        .build()
        .validate(&input.username)?;

    // Email validation
    StringValidationBuilder::new("email")
        .not_empty()
        .email()
        .max_length(255)
        .build()
        .validate(&input.email)?;

    // Password validation
    StringValidationBuilder::new("password")
        .not_empty()
        .min_length(8)
        .max_length(128)
        .matches(
            |s: &String| s.chars().any(|c| c.is_uppercase()),
            "Password must contain at least one uppercase letter"
        )
        .matches(
            |s: &String| s.chars().any(|c| c.is_lowercase()),
            "Password must contain at least one lowercase letter"
        )
        .matches(
            |s: &String| s.chars().any(|c| c.is_numeric()),
            "Password must contain at least one number"
        )
        .matches(
            |s: &String| s.chars().any(|c| !c.is_alphanumeric()),
            "Password must contain at least one special character"
        )
        .build()
        .validate(&input.password)?;

    // Age validation
    NumericValidationBuilder::new("age")
        .min(13)
        .max(150)
        .build()
        .validate(&input.age)?;

    Ok(())
}
```

### 3.2 SQL Injection Prevention

**Use parameterized queries exclusively.** Never construct SQL with string concatenation.

```rust
// UNSAFE - SQL Injection vulnerability
async fn unsafe_query(db: &Database, user_input: &str) {
    let query = format!("SELECT * FROM users WHERE username = '{}'", user_input);
    db.query(&query).await.unwrap(); // DON'T DO THIS!
}

// SAFE - Parameterized query
async fn safe_query(db: &Database, username: &str) -> Result<Vec<User>, Error> {
    let users = db.query(
        "SELECT * FROM users WHERE username = $1",
        &[&username]  // Parameters are automatically escaped
    ).await?;

    Ok(users)
}

// SAFE - Using SQLx with compile-time checked queries
#[cfg(feature = "sqlx")]
async fn sqlx_safe_query(pool: &sqlx::PgPool, username: &str) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users WHERE username = $1",
        username
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}
```

**Best Practices:**
- **Always use parameterized queries** ($1, $2, etc. for PostgreSQL)
- **Never interpolate user input** into SQL strings
- **Use ORMs or query builders** when possible
- **Validate input before database operations**
- **Use least-privilege database accounts**

### 3.3 XSS Prevention

Prevent Cross-Site Scripting by properly encoding output and validating input.

```rust
use dev_engineeringlabs_rustboot_validation::StringValidationBuilder;

fn sanitize_html_input(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn validate_and_sanitize_comment(comment: &str) -> Result<String, Error> {
    // Validate length
    StringValidationBuilder::new("comment")
        .not_empty()
        .max_length(5000)
        .build()
        .validate(&comment.to_string())?;

    // Sanitize HTML
    Ok(sanitize_html_input(comment))
}

// For JSON APIs, use proper serialization
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Comment {
    id: u64,
    text: String,  // Will be automatically escaped in JSON
    author: String,
}

// Serde automatically handles escaping when serializing to JSON
fn return_comment(comment: Comment) -> String {
    serde_json::to_string(&comment).unwrap()
}
```

**Best Practices:**
- **Use templating engines with auto-escaping** (e.g., Tera, Handlebars)
- **Set Content-Security-Policy headers** (see Section 4)
- **Validate and sanitize all user input**
- **Use `serde_json` for JSON responses** (automatic escaping)
- **Never use `innerHTML` on the client with user content**
- **Implement output encoding** based on context (HTML, JavaScript, URL)

---

## 4. Security Headers

### 4.1 Using rustboot-middleware Security Headers

Rustboot provides comprehensive security headers middleware.

```rust
use dev_engineeringlabs_rustboot_middleware::{
    Pipeline,
    security::{SecurityHeadersMiddleware, SecurityHeadersConfig},
};

fn create_production_security_headers() -> SecurityHeadersMiddleware {
    let config = SecurityHeadersConfig::default()
        .with_csp(
            "default-src 'self'; \
             script-src 'self' https://cdn.trusted.com; \
             style-src 'self' 'unsafe-inline'; \
             img-src 'self' data: https:; \
             font-src 'self' data:; \
             connect-src 'self'; \
             frame-ancestors 'none'; \
             base-uri 'self'; \
             form-action 'self'"
        )
        .with_hsts(31536000, true, true)  // 1 year, includeSubDomains, preload
        .with_frame_options("DENY")
        .with_content_type_options("nosniff")
        .with_referrer_policy("strict-origin-when-cross-origin")
        .with_permissions_policy(
            "geolocation=(), microphone=(), camera=(), payment=(), usb=(), \
             magnetometer=(), gyroscope=(), accelerometer=()"
        );

    SecurityHeadersMiddleware::new(config)
}

fn create_development_security_headers() -> SecurityHeadersMiddleware {
    // More permissive for development
    SecurityHeadersMiddleware::permissive()
}

// Environment-based configuration
fn get_security_middleware() -> SecurityHeadersMiddleware {
    match std::env::var("ENVIRONMENT").as_deref() {
        Ok("production") => create_production_security_headers(),
        Ok("staging") => {
            // Staging: secure but with shorter HSTS
            let config = SecurityHeadersConfig::default()
                .with_hsts(86400, false, false);  // 1 day, no preload
            SecurityHeadersMiddleware::new(config)
        }
        _ => create_development_security_headers(),
    }
}
```

### 4.2 Content-Security-Policy (CSP) Configuration

CSP is your primary defense against XSS attacks.

```rust
// Strict CSP - Recommended for new applications
let strict_csp = "default-src 'none'; \
                  script-src 'self'; \
                  style-src 'self'; \
                  img-src 'self'; \
                  font-src 'self'; \
                  connect-src 'self'; \
                  frame-ancestors 'none'; \
                  base-uri 'self'; \
                  form-action 'self'";

// API-only CSP - For REST APIs
let api_csp = "default-src 'none'; frame-ancestors 'none'";

// Progressive CSP - Start here and tighten over time
let progressive_csp = "default-src 'self'; \
                       script-src 'self' 'unsafe-inline' https://cdn.example.com; \
                       report-uri /csp-violation-report";

let config = SecurityHeadersConfig::new()
    .with_csp(strict_csp);
```

**CSP Best Practices:**
- **Start with `default-src 'none'`** and explicitly allow what you need
- **Avoid `'unsafe-inline'` and `'unsafe-eval'`** in production
- **Use nonces or hashes** for inline scripts if needed
- **Implement CSP reporting** to monitor violations
- **Test thoroughly** before deploying strict CSP

### 4.3 HSTS (HTTP Strict Transport Security) Setup

Force HTTPS for all connections.

```rust
let config = SecurityHeadersConfig::new()
    .with_hsts(
        31536000,  // max-age: 1 year in seconds
        true,      // includeSubDomains
        true       // preload
    );
```

**HSTS Best Practices:**
- **Test HTTPS thoroughly first** before enabling HSTS
- **Start with short max-age** (300 seconds) during testing
- **Gradually increase** to 1 year (31536000 seconds)
- **Only enable `preload`** after submitting to hstspreload.org
- **Be cautious with `includeSubDomains`** - ensure all subdomains support HTTPS

**Migration Path:**
```rust
// Phase 1: Testing (5 minutes)
.with_hsts(300, false, false)

// Phase 2: Staging (1 day)
.with_hsts(86400, false, false)

// Phase 3: Production (1 month)
.with_hsts(2592000, true, false)

// Phase 4: HSTS Preload (1 year)
.with_hsts(31536000, true, true)
```

---

## 5. Secrets Management

### 5.1 Environment Variables

Store secrets in environment variables, never in code.

```rust
use std::env;

fn get_database_url() -> Result<String, Error> {
    env::var("DATABASE_URL")
        .map_err(|_| Error::MissingConfig("DATABASE_URL not set"))
}

fn get_jwt_secret() -> Result<Vec<u8>, Error> {
    let secret = env::var("JWT_SECRET")
        .map_err(|_| Error::MissingConfig("JWT_SECRET not set"))?;

    // Validate secret strength
    if secret.len() < 32 {
        return Err(Error::WeakSecret("JWT_SECRET must be at least 32 characters"));
    }

    Ok(secret.into_bytes())
}

// Load from .env file in development (using dotenv crate)
#[cfg(debug_assertions)]
fn load_env() {
    dotenv::dotenv().ok();
}

#[cfg(not(debug_assertions))]
fn load_env() {
    // In production, environment variables should be set by deployment system
}
```

**.env file (NEVER commit this):**
```env
DATABASE_URL=postgresql://user:password@localhost/mydb
JWT_SECRET=your-super-secret-key-minimum-32-characters-long
API_KEY=prod-api-key-12345
ENCRYPTION_KEY=base64-encoded-256-bit-key
```

**.gitignore:**
```
.env
.env.local
.env.production
secrets/
*.key
*.pem
credentials.json
```

### 5.2 Secret Stores

For production, use dedicated secret management systems.

```rust
use dev_engineeringlabs_rustboot_config::{ConfigLoader, EnvSource};

#[derive(Debug, Deserialize)]
struct SecretConfig {
    database_url: String,
    jwt_secret: String,
    api_key: String,
}

async fn load_secrets() -> Result<SecretConfig, Error> {
    // Load from environment
    let config = ConfigLoader::new()
        .with_source(EnvSource::new())
        .load::<SecretConfig>()?;

    Ok(config)
}

// Integration with AWS Secrets Manager (example)
#[cfg(feature = "aws")]
async fn load_from_aws_secrets_manager() -> Result<SecretConfig, Error> {
    use aws_sdk_secretsmanager::Client;

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let response = client
        .get_secret_value()
        .secret_id("prod/myapp/secrets")
        .send()
        .await?;

    let secret_string = response.secret_string()
        .ok_or(Error::MissingSecret)?;

    let config: SecretConfig = serde_json::from_str(secret_string)?;
    Ok(config)
}
```

### 5.3 Key Rotation

Implement regular key rotation for enhanced security.

```rust
use std::time::{SystemTime, Duration};

struct RotatingSecret {
    current: Vec<u8>,
    previous: Option<Vec<u8>>,
    rotation_time: SystemTime,
    rotation_interval: Duration,
}

impl RotatingSecret {
    fn new(secret: Vec<u8>, interval: Duration) -> Self {
        Self {
            current: secret,
            previous: None,
            rotation_time: SystemTime::now(),
            rotation_interval: interval,
        }
    }

    fn rotate(&mut self, new_secret: Vec<u8>) {
        self.previous = Some(self.current.clone());
        self.current = new_secret;
        self.rotation_time = SystemTime::now();
    }

    fn should_rotate(&self) -> bool {
        SystemTime::now()
            .duration_since(self.rotation_time)
            .unwrap_or(Duration::ZERO) > self.rotation_interval
    }

    fn verify_with_any(&self, data: &[u8], signature: &[u8]) -> bool {
        // Try current key
        if verify_signature(data, signature, &self.current) {
            return true;
        }

        // Try previous key (grace period)
        if let Some(ref prev) = self.previous {
            return verify_signature(data, signature, prev);
        }

        false
    }
}
```

**Best Practices:**
- **Never hardcode secrets** in source code
- **Use environment-specific secrets** (dev, staging, prod)
- **Rotate secrets regularly** (every 90 days minimum)
- **Use secret management systems** in production (AWS Secrets Manager, HashiCorp Vault)
- **Implement grace periods** during rotation
- **Log secret access** for audit purposes
- **Encrypt secrets at rest**

---

## 6. HTTPS/TLS Configuration

### 6.1 TLS Best Practices

Always use HTTPS in production.

```rust
// Example with Axum web framework
use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;

async fn setup_https_server() -> Result<(), Error> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    // Load TLS certificates
    let config = RustlsConfig::from_pem_file(
        "certs/cert.pem",
        "certs/key.pem",
    ).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 443));

    println!("HTTPS server listening on {}", addr);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// Redirect HTTP to HTTPS
async fn redirect_http_to_https() {
    let app = Router::new()
        .fallback(|uri: axum::http::Uri| async move {
            let https_uri = format!("https://{}{}",
                std::env::var("DOMAIN").unwrap_or("localhost".to_string()),
                uri.path()
            );
            axum::response::Redirect::permanent(&https_uri)
        });

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### 6.2 Certificate Management

```bash
# Generate self-signed certificate for development
openssl req -x509 -newkey rsa:4096 -nodes \
  -keyout key.pem -out cert.pem -days 365 \
  -subj "/CN=localhost"

# For production, use Let's Encrypt
certbot certonly --standalone -d yourdomain.com
```

**TLS Best Practices:**
- **Use TLS 1.2 or higher** (disable TLS 1.0 and 1.1)
- **Use strong cipher suites** (AES-256-GCM preferred)
- **Obtain certificates from trusted CAs** (Let's Encrypt)
- **Enable HTTP/2** for better performance
- **Implement certificate pinning** for mobile apps
- **Monitor certificate expiration** and automate renewal
- **Use HSTS** to enforce HTTPS (see Section 4.3)

---

## 7. Rate Limiting for Abuse Prevention

### 7.1 Basic Rate Limiting

Protect your API from abuse with rate limiting.

```rust
use dev_engineeringlabs_rustboot_middleware::{
    RateLimitConfig, RateLimitMiddleware, Pipeline,
};
use std::sync::Arc;
use std::time::Duration;

fn setup_rate_limiting() -> Arc<RateLimitMiddleware> {
    // Fixed Window: 100 requests per minute
    let config = RateLimitConfig::FixedWindow {
        max_requests: 100,
        window_size: Duration::from_secs(60),
    };

    Arc::new(RateLimitMiddleware::new(config))
}

// Token Bucket for API with burst support
fn setup_burst_rate_limiting() -> Arc<RateLimitMiddleware> {
    let config = RateLimitConfig::TokenBucket {
        capacity: 100,       // Bucket size
        refill_rate: 10,     // Tokens per interval
        refill_interval: Duration::from_secs(1),
    };

    Arc::new(RateLimitMiddleware::new(config))
}

// Sliding Window for precise rate limiting
fn setup_precise_rate_limiting() -> Arc<RateLimitMiddleware> {
    let config = RateLimitConfig::SlidingWindow {
        max_requests: 1000,
        window_size: Duration::from_secs(3600),  // 1000/hour
    };

    Arc::new(RateLimitMiddleware::new(config))
}
```

### 7.2 API Key-Based Rate Limiting

Different rate limits for different users.

```rust
use dev_engineeringlabs_rustboot_middleware::{
    HeaderKeyExtractor, CustomKeyExtractor, HttpContext,
};

// Rate limit by API key
fn setup_api_key_rate_limiting() -> Arc<RateLimitMiddleware> {
    let config = RateLimitConfig::TokenBucket {
        capacity: 1000,
        refill_rate: 100,
        refill_interval: Duration::from_secs(1),
    };

    let extractor = Arc::new(HeaderKeyExtractor::new("X-API-Key"));
    Arc::new(RateLimitMiddleware::with_key_extractor(config, extractor))
}

// Custom rate limiting per user tier
fn setup_tiered_rate_limiting() -> Arc<RateLimitMiddleware> {
    let extractor = Arc::new(CustomKeyExtractor::new(|ctx: &HttpContext| {
        // Extract user tier from context and apply different limits
        let api_key = ctx.headers.get("X-API-Key")?;
        let tier = get_user_tier(api_key);

        Some(format!("tier:{}:{}", tier, api_key))
    }));

    let config = RateLimitConfig::TokenBucket {
        capacity: 10000,
        refill_rate: 1000,
        refill_interval: Duration::from_secs(1),
    };

    Arc::new(RateLimitMiddleware::with_key_extractor(config, extractor))
}
```

### 7.3 Endpoint-Specific Rate Limiting

```rust
fn setup_application_with_rate_limiting() {
    // Strict rate limiting for authentication endpoints
    let auth_rate_limit = RateLimitConfig::FixedWindow {
        max_requests: 5,
        window_size: Duration::from_secs(60),  // 5 login attempts per minute
    };

    // Moderate rate limiting for API endpoints
    let api_rate_limit = RateLimitConfig::TokenBucket {
        capacity: 100,
        refill_rate: 10,
        refill_interval: Duration::from_secs(1),
    };

    // Relaxed rate limiting for static content
    let static_rate_limit = RateLimitConfig::FixedWindow {
        max_requests: 1000,
        window_size: Duration::from_secs(60),
    };
}
```

**Rate Limiting Best Practices:**
- **Rate limit authentication endpoints** aggressively (5-10 attempts/minute)
- **Use different limits for different endpoints**
- **Implement exponential backoff** for repeated violations
- **Return proper HTTP 429 status** with Retry-After header
- **Log rate limit violations** for security monitoring
- **Consider distributed rate limiting** for multi-server deployments
- **Whitelist trusted IPs** if needed

---

## 8. Logging Sensitive Data

### 8.1 What NOT to Log

**Never log the following:**
- Passwords (plaintext or hashed)
- API keys and tokens
- JWT tokens
- Session IDs
- Credit card numbers
- Social security numbers
- Private keys
- Authentication credentials
- Personal health information
- Full request/response bodies containing sensitive data

### 8.2 Safe Logging Practices

```rust
use tracing::{info, warn, error, debug};

// BAD - Logs password
fn bad_logging(username: &str, password: &str) {
    info!("User login: {} with password {}", username, password);  // DON'T!
}

// GOOD - Masks sensitive data
fn good_logging(username: &str, password: &str) {
    info!("User login attempt: {}", username);
    debug!("Password length: {}", password.len());  // Only log metadata
}

// Redact sensitive fields from structs
#[derive(Debug)]
struct User {
    pub id: u64,
    pub username: String,
    #[allow(dead_code)]
    password_hash: String,  // Don't derive Debug for sensitive fields
}

impl User {
    fn safe_log_format(&self) -> String {
        format!("User(id={}, username={})", self.id, self.username)
    }
}

// Redacting middleware
use dev_engineeringlabs_rustboot_middleware::{Middleware, Next};

pub struct SanitizedLoggingMiddleware;

impl<Ctx> Middleware<Ctx> for SanitizedLoggingMiddleware
where
    Ctx: HasHeaders + Send + 'static,
{
    fn handle(&self, mut ctx: Ctx, next: Next<Ctx>)
        -> Pin<Box<dyn Future<Output = MiddlewareResult<Ctx>> + Send>>
    {
        Box::pin(async move {
            // Redact sensitive headers before logging
            let mut safe_headers = ctx.headers.clone();
            safe_headers.remove("Authorization");
            safe_headers.remove("X-API-Key");
            safe_headers.remove("Cookie");

            info!("Request headers: {:?}", safe_headers);

            next(ctx).await
        })
    }
}
```

### 8.3 Structured Logging with Redaction

```rust
use serde::{Serialize, Serializer};

#[derive(Serialize)]
struct LoginEvent {
    username: String,
    #[serde(serialize_with = "redact")]
    password: String,
    timestamp: u64,
    ip_address: String,
}

fn redact<S>(_: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("[REDACTED]")
}

fn log_login_attempt(event: LoginEvent) {
    info!("Login attempt: {}", serde_json::to_string(&event).unwrap());
    // Output: {"username":"alice","password":"[REDACTED]","timestamp":...}
}
```

### 8.4 Audit Logging

```rust
use tracing::info;
use serde::Serialize;

#[derive(Serialize)]
struct AuditLog {
    event_type: String,
    user_id: String,
    action: String,
    resource: String,
    timestamp: u64,
    ip_address: String,
    success: bool,
}

fn audit_log(log: AuditLog) {
    info!(
        target: "audit",
        event_type = %log.event_type,
        user_id = %log.user_id,
        action = %log.action,
        resource = %log.resource,
        success = log.success,
        "Audit event"
    );
}

// Example usage
fn log_permission_check(user_id: &str, permission: &str, granted: bool) {
    audit_log(AuditLog {
        event_type: "authorization".to_string(),
        user_id: user_id.to_string(),
        action: "check_permission".to_string(),
        resource: permission.to_string(),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        ip_address: "127.0.0.1".to_string(),  // Get from request context
        success: granted,
    });
}
```

**Logging Best Practices:**
- **Use structured logging** (JSON format)
- **Implement log levels properly** (ERROR, WARN, INFO, DEBUG)
- **Never log sensitive data in production**
- **Log security events** (failed logins, permission denials)
- **Implement log retention policies**
- **Protect log files** (proper permissions, encryption at rest)
- **Use centralized logging** in production (ELK, CloudWatch)
- **Monitor logs for security incidents**

---

## 9. Dependency Security

### 9.1 Using cargo-audit

Regularly audit dependencies for known vulnerabilities.

```bash
# Install cargo-audit
cargo install cargo-audit

# Audit your dependencies
cargo audit

# Audit and fix vulnerabilities
cargo audit fix

# Generate audit report
cargo audit --json > audit-report.json
```

**Add to CI/CD pipeline:**
```yaml
# .github/workflows/security.yml
name: Security Audit

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]
  schedule:
    - cron: '0 0 * * *'  # Daily

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
```

### 9.2 Dependency Management Best Practices

```toml
# Cargo.toml - Pin critical dependencies

[dependencies]
# Use exact versions for security-critical dependencies
bcrypt = "=0.15.0"
jsonwebtoken = "=9.2.0"

# Use compatible versions for others
serde = "^1.0"
tokio = "^1.0"

# Avoid wildcard versions
# bad_crate = "*"  # NEVER DO THIS

[dev-dependencies]
# Dev dependencies can be more relaxed
criterion = "0.5"
```

### 9.3 Supply Chain Security

```bash
# Verify dependency checksums
cargo fetch --locked

# Use cargo-deny for dependency policies
cargo install cargo-deny
cargo deny check

# cargo-deny.toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"
notice = "warn"

[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-3-Clause",
]
deny = [
    "GPL-3.0",
]

[bans]
multiple-versions = "warn"
deny = [
    { name = "openssl", version = "<1.1" },
]
```

### 9.4 Regular Updates

```bash
# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Update Cargo.lock with security patches
cargo update -p vulnerable-crate --precise 1.2.3
```

**Dependency Security Best Practices:**
- **Run `cargo audit` regularly** (daily in CI/CD)
- **Keep dependencies up to date** (monthly review)
- **Review dependency tree** (`cargo tree`)
- **Minimize dependencies** (only add what you need)
- **Prefer well-maintained crates** (check last update, downloads)
- **Review security advisories** (RustSec Database)
- **Use dependabot** or renovatebot for automatic updates
- **Verify dependency sources** (crates.io only)
- **Implement Software Bill of Materials (SBOM)**

---

## Security Checklist

Use this checklist before deploying to production:

### Authentication & Authorization
- [ ] Passwords are hashed with bcrypt (cost >= 12)
- [ ] JWT tokens use strong secrets (>= 32 characters)
- [ ] JWT tokens have short expiration times (<= 1 hour)
- [ ] Sessions use secure cookies (Secure, HttpOnly, SameSite)
- [ ] Session IDs are regenerated after login
- [ ] RBAC is implemented for protected resources
- [ ] Authorization checks on every protected endpoint

### Input Validation & Data Security
- [ ] All user input is validated
- [ ] Parameterized queries used exclusively (no SQL injection)
- [ ] Output is properly encoded (no XSS)
- [ ] File uploads are validated and sanitized
- [ ] Request size limits are enforced

### Security Headers
- [ ] Content-Security-Policy is configured
- [ ] HSTS is enabled (1 year, includeSubDomains)
- [ ] X-Frame-Options set to DENY
- [ ] X-Content-Type-Options set to nosniff
- [ ] Referrer-Policy configured
- [ ] Permissions-Policy configured

### HTTPS & TLS
- [ ] HTTPS is enforced in production
- [ ] TLS 1.2+ is required
- [ ] HTTP redirects to HTTPS
- [ ] Valid TLS certificates from trusted CA
- [ ] Certificate expiration monitoring in place

### Secrets Management
- [ ] No secrets in source code
- [ ] No secrets in version control
- [ ] Environment variables used for secrets
- [ ] Secret management system in production
- [ ] Secrets rotation policy in place

### Rate Limiting & Abuse Prevention
- [ ] Rate limiting on authentication endpoints
- [ ] Rate limiting on API endpoints
- [ ] Rate limit headers included in responses
- [ ] IP-based rate limiting configured
- [ ] DDoS protection in place

### Logging & Monitoring
- [ ] Security events are logged
- [ ] No sensitive data in logs
- [ ] Centralized logging configured
- [ ] Log retention policy in place
- [ ] Security monitoring and alerts configured

### Dependencies & Updates
- [ ] `cargo audit` passes with no vulnerabilities
- [ ] Dependencies are up to date
- [ ] Automated dependency updates configured
- [ ] Regular security reviews scheduled

### Database Security
- [ ] Database uses least-privilege accounts
- [ ] Database connections are encrypted
- [ ] Database backups are encrypted
- [ ] SQL injection prevention verified

---

## Additional Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Working Group](https://www.rust-lang.org/governance/wgs/wg-security-response)
- [RustSec Advisory Database](https://rustsec.org/)
- [OWASP Secure Headers Project](https://owasp.org/www-project-secure-headers/)
- [Mozilla Web Security Guidelines](https://infosec.mozilla.org/guidelines/web_security)
- [NIST Cybersecurity Framework](https://www.nist.gov/cyberframework)

---

## Conclusion

Security is an ongoing process, not a one-time task. Regularly review and update your security practices, stay informed about new vulnerabilities, and always follow the principle of least privilege.

Remember: **Security is everyone's responsibility.**

For questions or security concerns, please contact the security team or open a security issue in the repository.
