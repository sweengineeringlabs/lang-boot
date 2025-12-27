# ADR-001: Security Module Organization

**Status**: Accepted  
**Date**: 2025-12-22  
**Deciders**: Architecture Team  

## Context

Rustboot framework has multiple crates with security-related functionality:
- `rustboot-crypto` - Cryptographic operations (hashing, HMAC, password hashing)
- `rustboot-validation` - Input validation (prevents XSS, SQL injection)
- `rustboot-ratelimit` - Rate limiting (prevents DoS, brute force attacks)
- `rustboot-fileio` - Secure file operations (path traversal prevention, atomic writes)

**Design Question**: Should security features be:
1. **Distributed** across specialized crates (current architecture)
2. **Consolidated** under `rustboot-security/security-{module}` structure
3. **Hybrid** approach with both specialized and security-specific crates

## Decision Drivers

- **Discoverability**: Users need to easily find all security features
- **Modularity**: Avoid unnecessary coupling between unrelated concerns
- **Flexibility**: Users should be able to use features independently
- **Security Auditing**: Security code should be identifiable for audits
- **Maintainability**: Clear boundaries for testing and evolution
- **Conceptual Clarity**: Architecture should reflect domain concepts

## Considered Options

### Option 1: Keep Distributed Architecture (SELECTED)

```
crates/
├── rustboot-crypto/         # Pure security
├── rustboot-validation/     # Data quality + security
├── rustboot-ratelimit/      # Resource management + security
├── rustboot-fileio/         # File I/O + security
└── docs/security-guide.md   # Unified documentation
```

**Pros**:
- ✅ Single Responsibility Principle (each crate has clear, focused purpose)
- ✅ Flexibility (use validation without crypto, fileio without validation)
- ✅ Loose coupling (independent versioning, evolution)
- ✅ Cross-cutting by nature (validation serves both security and business rules)
- ✅ Security guide provides unified discoverability without coupling

**Cons**:
- ❌ Security features scattered across multiple crates
- ❌ Requires documentation to provide unified view
- ❌ Security audits must span multiple crates

### Option 2: Consolidated Security Namespace

```
crates/
└── rustboot-security/
    ├── security-crypto/
    ├── security-validation/
    ├── security-ratelimit/
    └── security-fileio/
```

**Pros**:
- ✅ Clear namespace (`use rustboot_security::crypto::*;`)
- ✅ Grouped discoverability (all security in one place)
- ✅ Easier security audits (single parent directory)
- ✅ Version coherence (security modules versioned together)
- ✅ Security as first-class architectural concern

**Cons**:
- ❌ Over-coupling (forces unrelated concerns to be linked)
- ❌ Breaks modularity (validation useful beyond security)
- ❌ High migration cost (restructure existing crates)
- ❌ Usage confusion (implies all features are security-only)
- ❌ Monolithic risk (parent crate could become bloated)

### Option 3: Hybrid Approach

```
crates/
├── rustboot-crypto/         # Keep (could move to security)
├── rustboot-validation/     # Keep (broader than security)
├── rustboot-ratelimit/      # Keep (resource management)
├── rustboot-fileio/         # Keep (file I/O utilities)
└── rustboot-security/       # NEW - Pure security features
    ├── auth/                # Authentication
    ├── authz/               # Authorization
    ├── secrets/             # Secret management
    └── audit/               # Security auditing
```

**Pros**:
- ✅ Maintains current modularity for cross-cutting concerns
- ✅ Clear home for purely security-focused features
- ✅ Flexible (best of both worlds)

**Cons**:
- ⚠️ Inconsistent (some security in dedicated crate, some distributed)
- ⚠️ May confuse where new features belong

## Decision

**Adopt Option 1 (Distributed Architecture) with documentation consolidation.**

### Rationale

1. **Cross-Cutting Reality**: Security features naturally span multiple domains
   - Validation serves data quality AND security
   - Rate limiting serves resource management AND security
   - File I/O serves reliability AND security

2. **Single Responsibility Maintained**: Each crate has clear, focused purpose
   - `rustboot-crypto` = cryptographic operations
   - `rustboot-validation` = input validation
   - `rustboot-ratelimit` = request throttling
   - `rustboot-fileio` = file operations

3. **Flexibility Preserved**: Users can compose exactly what they need
   - Use validation without pulling in crypto deps
   - Use fileio without validation
   - Import only required security features

4. **Documentation Solves Discoverability**: `docs/security-guide.md` provides unified view without architectural coupling

5. **Future Extension Path**: Can add `rustboot-security` crate later for purely security-focused features (auth, authz, secrets, audit)

## Consequences

### Positive

- ✅ Maintains clean separation of concerns
- ✅ Enables independent evolution of each crate
- ✅ Supports flexible composition for users
- ✅ Security guide provides discoverability
- ✅ No migration cost (keeps existing structure)

### Negative

- ⚠️ Security audits must span multiple crates (mitigated by security guide documentation)
- ⚠️ Requires maintaining security-guide.md to keep unified view

### Neutral

- 🔄 Can revisit if pain points emerge
- 🔄 Can add `rustboot-security` crate for purely security-focused features in future

## Implementation

1. ✅ **Completed** (2025-12-22): Created `docs/security-guide.md` consolidating all security features
2. ✅ **Completed** (2025-12-22): Updated README.md to reference security guide
3. ✅ **Completed** (2025-12-22): Created `rustboot-security` crate for auth/authz/secrets/audit
   - Initial structure with 4 modules (auth, authz, secrets, audit)
   - Placeholder implementations with planned backlog
   - Comprehensive documentation and overview

## Compliance

- Security features tagged in documentation
- Vulnerability mapping table maintained in security guide
- Security checklist provided for developers
- Individual crate backlogs reference security enhancements

## References

- [docs/security-guide.md](../security-guide.md) - Consolidated security documentation
- [docs/backlog.md](../backlog.md) - Individual crate backlogs with security items
- [SEA Architecture](https://github.com/phdsystems/rustratify) - Stratified Encapsulation Architecture principles
