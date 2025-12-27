# Rustboot Framework Overview

| | |
|------|------|
| **WHAT** | Infrastructure library for Rust applications |
| **WHY** | Reusable cross-cutting concerns (validation, caching, DI, resilience, observability) |
| **HOW** | Compile-time macros and traits (zero runtime reflection) |

## Quick Navigation

- **Architecture & Design** → [3-design/architecture.md](3-design/architecture.md)
- **Development Guides** → [4-development/developer-guide.md](4-development/developer-guide.md)
- **Crate Documentation** → See below

## Crate Documentation

Detailed overview for each crate (WHAT-WHY-HOW structure):

### High Priority Crates
- [rustboot-macros](../crates/rustboot-macros/README.md) - Procedural macros (derive and attributes)
- [rustboot-database](../crates/rustboot-database/doc/overview.md) - Database abstractions, query patterns
- [rustboot-http](../crates/rustboot-http/doc/overview.md) - HTTP client utilities
- [rustboot-messaging](../crates/rustboot-messaging/doc/overview.md) - Message queue abstractions
- [rustboot-security](../crates/rustboot-security/doc/overview.md) - Authentication, authorization, secrets, auditing
- [rustboot-validation](../crates/rustboot-validation/doc/overview.md) - Input validation framework

### Medium Priority Crates
- [rustboot-cache](../crates/rustboot-cache/doc/overview.md) - Caching abstractions
- [rustboot-config](../crates/rustboot-config/doc/overview.md) - Configuration management
- [rustboot-di](../crates/rustboot-di/doc/overview.md) - Dependency injection
- [rustboot-middleware](../crates/rustboot-middleware/doc/overview.md) - Middleware pipeline
- [rustboot-observability](../crates/rustboot-observability/doc/overview.md) - Logging, metrics, tracing
- [rustboot-ratelimit](../crates/rustboot-ratelimit/doc/overview.md) - Rate limiting
- [rustboot-resilience](../crates/rustboot-resilience/doc/overview.md) - Circuit breakers, retries
- [rustboot-serialization](../crates/rustboot-serialization/doc/overview.md) - Serialization formats
- [rustboot-state-machine](../crates/rustboot-state-machine/doc/overview.md) - State machines

### Utility Crates
- [rustboot-async](../crates/rustboot-async/doc/overview.md) - Async utilities
- [rustboot-compress](../crates/rustboot-compress/doc/overview.md) - Compression
- [rustboot-crypto](../crates/rustboot-crypto/doc/overview.md) - Cryptography primitives
- [rustboot-datetime](../crates/rustboot-datetime/doc/overview.md) - Date/time utilities
- [rustboot-fileio](../crates/rustboot-fileio/doc/overview.md) - File I/O
- [rustboot-toolchain](../crates/rustboot-toolchain/doc/overview.md) - Development tooling
- [rustboot-uuid](../crates/rustboot-uuid/doc/overview.md) - UUID generation

## Backlog & Roadmap

- [Framework Backlog Index](backlog.md) - Links to all crate backlogs
- [Framework-Wide Backlog](framework-backlog.md) - Cross-cutting improvements

## Documentation Standards

- [Documentation Templates](templates/README.md) - WHAT-WHY-HOW templates for SEA projects

---

**Total Crates**: 22  
**Architecture**: SEA (Stratified Encapsulation Architecture)  
**Documentation Format**: WHAT-WHY-HOW structure
