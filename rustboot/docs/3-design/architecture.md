# Rustboot Architecture Documentation

Architecture, design decisions, and security documentation.

**Audience**: Architects, Technical Leadership, Security Teams

## Architecture Overview

Rustboot follows the [SEA (Stratified Encapsulation Architecture)](https://github.com/phdsystems/rustratify) pattern with 22 specialized crates organized by responsibility.

### Key Components

- **Macros**: Procedural macros for compile-time code generation (DI, validation, caching, tracing)
- **Core Framework**: Validation, DI, middleware, state machines
- **Infrastructure**: Database, HTTP, messaging, caching
- **Security**: Authentication, authorization, secrets management, auditing
- **Resilience**: Retry, circuit breaker, rate limiting, timeout
- **Observability**: Logging, metrics, tracing
- **Utilities**: Async, crypto, datetime, compression, file I/O, UUID

## Security

### Overview & Planning
- [Security Overview](security-overview.md) - Architecture, capabilities, and roadmap
- [Security Audit Guide](security-audit.md) - OWASP coverage, compliance (SOC 2, HIPAA, GDPR)

### Architecture Decisions
- [ADR-001: Security Module Organization](adr/001-security-module-organization.md) - Why security is distributed + dedicated crate

## Design Guides

[Add additional design guides here as they are created]

## Crate Architecture

For individual crate architecture, see:
- [Crate Overviews](../overview.md#crate-documentation) - Links to all `crates/*/docs/overview.md`

---

**Related**: [Developer Guide](../4-development/developer-guide.md) - Implementation guides and best practices
