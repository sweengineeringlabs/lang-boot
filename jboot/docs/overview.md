# JBoot Framework Overview

| | |
|------|------|
| **WHAT** | Infrastructure library for Java applications |
| **WHY** | Reusable cross-cutting concerns (validation, caching, DI, resilience, observability) |
| **HOW** | Clean interfaces with pluggable implementations (following SEA architecture) |

## Quick Navigation

- **Examples** → [examples/README.md](../examples/README.md)
- **Architecture & Design** → [3-design/architecture.md](3-design/architecture.md)
- **Development Guides** → [4-development/developer-guide.md](4-development/developer-guide.md)
- **Module Documentation** → See below

## Module Documentation

Detailed overview for each module (WHAT-WHY-HOW structure):

### High Priority Modules
- [jboot-core](../modules/jboot-core/doc/overview.md) - Core types and utilities
- [jboot-validation](../modules/jboot-validation/doc/overview.md) - Input validation framework
- [jboot-database](../modules/jboot-database/doc/overview.md) - Database abstractions
- [jboot-http](../modules/jboot-http/doc/overview.md) - HTTP client utilities
- [jboot-messaging](../modules/jboot-messaging/doc/overview.md) - Message queue abstractions
- [jboot-security](../modules/jboot-security/doc/overview.md) - Authentication, authorization, secrets

### Medium Priority Modules
- [jboot-cache](../modules/jboot-cache/doc/overview.md) - Caching abstractions
- [jboot-config](../modules/jboot-config/doc/overview.md) - Configuration management
- [jboot-di](../modules/jboot-di/doc/overview.md) - Dependency injection utilities
- [jboot-middleware](../modules/jboot-middleware/doc/overview.md) - Middleware pipeline
- [jboot-observability](../modules/jboot-observability/doc/overview.md) - Logging, metrics, tracing
- [jboot-ratelimit](../modules/jboot-ratelimit/doc/overview.md) - Rate limiting
- [jboot-resilience](../modules/jboot-resilience/doc/overview.md) - Circuit breakers, retries
- [jboot-serialization](../modules/jboot-serialization/doc/overview.md) - Serialization formats
- [jboot-statemachine](../modules/jboot-statemachine/doc/overview.md) - State machines

### Utility Modules
- [jboot-async](../modules/jboot-async/doc/overview.md) - Async utilities (virtual threads)
- [jboot-cli](../modules/jboot-cli/doc/overview.md) - Command-line interface builder
- [jboot-crypto](../modules/jboot-crypto/doc/overview.md) - Cryptography primitives
- [jboot-datetime](../modules/jboot-datetime/doc/overview.md) - Date/time utilities
- [jboot-error](../modules/jboot-error/doc/overview.md) - Error handling
- [jboot-health](../modules/jboot-health/doc/overview.md) - Health checks
- [jboot-openapi](../modules/jboot-openapi/doc/overview.md) - OpenAPI documentation
- [jboot-session](../modules/jboot-session/doc/overview.md) - Session management
- [jboot-storage](../modules/jboot-storage/doc/overview.md) - File storage
- [jboot-streams](../modules/jboot-streams/doc/overview.md) - Reactive streams
- [jboot-testing](../modules/jboot-testing/doc/overview.md) - Testing utilities
- [jboot-web](../modules/jboot-web/doc/overview.md) - Web framework utilities

## Backlog & Roadmap

- [Framework Backlog Index](backlog.md) - Links to all module backlogs
- [Framework-Wide Backlog](framework-backlog.md) - Cross-cutting improvements

## Documentation Standards

- [Documentation Templates](templates/README.md) - WHAT-WHY-HOW templates for SEA projects

---

**Total Modules**: 25  
**Architecture**: SEA (Stratified Encapsulation Architecture)  
**Documentation Format**: WHAT-WHY-HOW structure

