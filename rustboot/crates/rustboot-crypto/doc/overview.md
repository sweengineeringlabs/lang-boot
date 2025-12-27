# Crypto Overview

## WHAT: Hashing & Password Security

**SHA256**: Secure hashing | **HMAC**: Message authentication | **Bcrypt**: Password hashing

## WHY: Security Best Practices

**Problems**: Insecure passwords, data integrity | **Solutions**: Industry-standard algorithms

## HOW: Use Cases

```rust
// File integrity
let checksum = sha256(&file_data);

// API signatures
let signature = hmac_sha256(secret_key, request_body);

// User passwords
let hash = hash_password(user_password)?;
verify_password(login_attempt, &stored_hash)?;
```

**Never**: Roll your own crypto, store plain passwords, use MD5/SHA1.


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [crypto_basic.rs](../examples/crypto_basic.rs) - Basic usage demonstration
- See directory for additional examples

**Purpose**: Show users HOW to use this module in real applications.

### Tests

**Location**: [	ests/](../tests/) directory

**Current tests**:
- [integration.rs](../tests/integration.rs) - Integration tests using public API

**Purpose**: Show users HOW to test code that uses this module.

### Testing Guidance

**For developers using this module**: See [Rust Test Organization](../../docs/4-development/guide/rust-test-organization.md)

**For contributors**: Run tests with:
```bash
cargo test -p dev-engineeringlabs-rustboot-crypto
cargo run --example crypto_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)