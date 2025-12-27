# rustboot-security

Security utilities for authentication, authorization, secrets management, and security auditing.

## Features

- 🔐 **Authentication** - JWT tokens, session management, API keys
- 🛡️ **Authorization** - RBAC, ABAC, permission checks
- 🔑 **Secrets Management** - Secure secret loading, rotation, encryption
- 📝 **Security Auditing** - Audit trails, compliance logging, security metrics

## Installation

```toml
[dependencies]
dev-engineeringlabs-rustboot-security = "0.1"
```

## Quick Start

```rust
use dev_engineeringlabs_rustboot_security::*;

// JWT Authentication
let token = generate_jwt("user_id", Duration::from_secs(3600))?;
let claims = validate_jwt(&token)?;

// Authorization
let rbac = RoleBasedAccessControl::new();
rbac.grant_permission("admin", "users:write")?;
rbac.check_permission("admin", "users:write")?; // ✅ Allowed

// Secrets
let secret = load_secret("API_KEY")?;
let encrypted = encrypt_secret(&secret)?;

// Auditing
audit_event("user.login", user_id, metadata)?;
```

## Modules

### Authentication (`auth`)
- JWT token generation and validation
- Session management
- OAuth2 flows (planned)
- API key management

### Authorization (`authz`)
- Role-Based Access Control (RBAC)
- Attribute-Based Access Control (ABAC) (planned)
- Permission policies
- Access control lists

### Secrets (`secrets`)
- Environment variable loading
- Secret encryption/decryption
- Secret rotation (planned)
- Integration with secret stores (planned)

### Auditing (`audit`)
- Security event logging
- Audit trail tracking
- Compliance logging
- Security metrics

## Documentation

See [docs/overview.md](docs/overview.md) for comprehensive documentation.

## Related Crates

- **rustboot-crypto** - Cryptographic primitives (hashing, HMAC, passwords)
- **rustboot-validation** - Input validation
- **rustboot-ratelimit** - Rate limiting for brute force protection

## Security Guide

See [docs/security-guide.md](../../docs/security-guide.md) for framework-wide security best practices.

## License

MIT
