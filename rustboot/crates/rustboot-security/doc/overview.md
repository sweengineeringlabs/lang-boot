# Security Crate Overview

> **📝 Important**: This overview links to:
> - Working code examples in `examples/` directory
> - Integration tests in `tests/` directory  
> - Testing guides for developers

## WHAT: Complete Security Suite

The `rustboot-security` crate provides comprehensive security functionality for Rust applications:

Key capabilities:
- **Authentication** - JWT token generation and validation
- **Authorization** - Role-Based Access Control (RBAC) with users and permissions
- **Secrets Management** - Encrypted secret storage and environment variable loading
- **Security Auditing** - Event logging with filtering and compliance tracking

## WHY: Centralized Security Infrastructure

**Problems Solved**:
1. **Fragmented authentication** - Scattered authentication logic across codebase
2. **Inconsistent authorization** - No standardized permission checking
3. **Unsecure secrets** - Plain-text secrets in code or config files
4. **Missing audit trails** - No security event logging for compliance

**Impact if not addressed**:
- Security vulnerabilities from inconsistent auth/authz
- Compliance failures from missing audit trails
- Data breaches from exposed secrets
- Difficult security incident investigation

**When to Use**: Any application requiring user authentication, role-based access control, secure secret storage, or security compliance (SOC 2, HIPAA, GDPR).

**When NOT to Use**: For cryptographic primitives (use `rustboot-crypto` instead).

## HOW: Security Module Guide

### Basic Example

```rust
use dev_engineeringlabs_rustboot_security::*;
use std::time::Duration;

// 1. Authenticate user
let token = generate_jwt("user123", Duration::from_secs(3600))?;
let claims = validate_jwt(&token)?;

// 2. Setup authorization
let mut rbac = RoleBasedAccessControl::new();
rbac.grant_permission("admin", "users:delete")?;

// 3. Check permissions
if rbac.check_permission("admin", "users:delete")? {
    // Allow action
}

// 4. Audit the action
audit_permission("admin", "users:delete", true)?;
```

### Authentication (`auth`)

JWT token generation and validation for user authentication.

```rust
// Generate JWT token
let token = generate_jwt("user123", Duration::from_secs(3600))?;

// Validate token
let claims = validate_jwt(&token)?;
assert_eq!(claims.sub, "user123");
```

**Available**:
- JWT token generation with expiration
- JWT token validation with signature verification
- Token expiration checking
- Claims extraction

**Planned**:
- Proper cryptographic signing (HMAC-SHA256)
- Session management
- OAuth2/OIDC flows
- API key authentication
- Multi-factor authentication (MFA)

### Authorization (`authz`)

Role-Based Access Control (RBAC) for fine-grained permissions.

```rust
// Create RBAC system
let mut rbac = RoleBasedAccessControl::new(); // Has admin, user, guest roles

// Grant permissions to roles
rbac.grant_permission("admin", "users:write")?;
rbac.grant_permission("admin", "users:delete")?;
rbac.grant_permission("user", "users:read")?;

// Assign roles to users
let mut user = User::new("alice");
user.add_role("admin");

// Check permissions
let ctx = AuthorizationContext::new(&rbac, &user);
ctx.require_permission("users:delete")?; // Ok for admin
```

**Available**:
- Role creation and management
- Permission granting and revocation
- User-role assignments
- Permission checking with context
- Role existence checking

**Planned**:
- Attribute-Based Access Control (ABAC)
- Policy-based permissions
- Resource-level permissions
- Role hierarchies
- Permission inheritance

### Secrets Management (`secrets`)

Secure storage and retrieval of sensitive data.

```rust
// In-memory secure storage
let store = SecretStore::new();
store.store("api_key", b"secret123".to_vec())?;
let secret = store.retrieve("api_key")?;

// Environment variables
let db_password = load_secret("DATABASE_PASSWORD")?;
let api_key = load_secret_or_default("API_KEY", "default_key");

// Encryption
let encrypted = encrypt_secret(b"sensitive_data")?;
let decrypted = decrypt_secret(&encrypted)?;
```

**Available**:
- In-memory encrypted secret store
- Environment variable loading
- XOR-based encryption (for demonstration)
- Memory zeroing for security
- Secret listing and deletion

**Planned**:
- AES-256-GCM encryption
- Secret rotation
- External vault integration (HashiCorp Vault, AWS Secrets Manager)
- Encrypted secrets in configuration files
- Key derivation functions (KDF)

### Security Auditing (`audit`)

Comprehensive security event logging for compliance.

```rust
// Log authentication events
audit_login("user123", true)?;  // Successful login
audit_login("attacker", false)?; // Failed login

// Log authorization events
audit_permission("user123", "users:write", false)?;

// Custom security events
let logger = AuditLogger::new();
let event = SecurityEvent::new(EventType::SecurityViolation, "user456")
    .with_severity(Severity::Critical)
    .with_metadata(json!({"reason": "brute_force"}));
logger.log(event)?;

// Query events
let critical_events = logger.get_events_by_severity(Severity::Critical);
let user_events = logger.get_events_by_subject("user123");
```

**Available**:
- Structured security event logging
- Event types (Login, Logout, PermissionGranted, DataRead, etc.)
- Severity levels (Info, Warning, Error, Critical)
- Event filtering by type, subject, severity, time range
- Audit logger with automatic rotation
- Convenience functions for common events

**Planned**:
- Persistent storage (database, files)
- SIEM integration (Splunk, ELK)
- Compliance report generation (SOC 2, HIPAA, GDPR)
- Automated alerting for critical events
- Event retention policies

## Relationship to Other Modules

| Module/Component | Purpose | Relationship |
|------------------|---------|--------------|
| `rustboot-crypto` | Cryptographic primitives | Used by secrets encryption |
| `rustboot-validation` | Input validation | Validates authentication inputs |
| `rustboot-ratelimit` | Rate limiting | Protects authentication endpoints |
| `rustboot-observability` | General logging | Extended for security-specific logging |
| `rustboot-http` | HTTP requests | Secures API endpoints |

**Integration Points**:
- Authentication tokens used in HTTP headers
- Authorization checks in middleware pipelines
- Audit events sent to observability systems
- Secrets loaded for database connections

## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [`examples/`](../examples/) directory

**Current examples**:
- [`authentication.rs`](../examples/authentication.rs) - Basic authentication concepts
- [`authorization.rs`](../examples/authorization.rs) - RBAC and permissions

**Purpose**: Demonstrate authentication, authorization, secrets, and auditing in real scenarios.

### Tests

**Location**: [`tests/`](../tests/) directory

**Current tests**:
- [`integration.rs`](../tests/integration.rs) - Comprehensive integration tests covering:
  - Full JWT authentication flow
  - JWT validation and error handling
  - RBAC authorization workflows
  - User-role-permission contexts
  - Secret encryption and storage
  - Security audit logging
  - End-to-end security workflow

**Test Coverage**:
- **31 unit tests** in module files (all passing ✅)
- **8 integration tests** (all passing ✅)

### Testing Guidance

**For developers using this module**: See [Rust Test Organization](../../docs/4-development/guide/rust-test-organization.md)

**For contributors**: Run tests with:
```bash
cargo test -p dev-engineeringlabs-rustboot-security
cargo run --example authentication
cargo run --example authorization
```

---

**Status**: **Stable** - Core functionality fully implemented and tested  
**Roadmap**: See [backlog.md](../backlog.md) for planned enhancements
