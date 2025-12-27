# Rustboot Framework Backlog Index

This document serves as an index to all backlog items across the Rustboot framework. Each crate maintains its own backlog file, and framework-wide improvements are tracked separately.

## Individual Crate Backlogs

### High Priority Crates
- [rustboot-database](../crates/rustboot-database/backlog.md) - Database abstraction, query builder, migrations
- [rustboot-http](../crates/rustboot-http/backlog.md) - HTTP utilities, WebSocket, HTTP/2 support
- [rustboot-messaging](../crates/rustboot-messaging/backlog.md) - Message queue abstractions, DLQ, retry patterns
- [rustboot-security](../crates/rustboot-security/backlog.md) - Authentication, authorization, secrets, auditing
- [rustboot-validation](../crates/rustboot-validation/backlog.md) - Validation utilities, async validators, i18n

### Medium Priority Crates
- [rustboot-cache](../crates/rustboot-cache/backlog.md) - Caching abstractions, distributed cache, eviction policies
- [rustboot-config](../crates/rustboot-config/backlog.md) - Configuration management, hot-reload, secrets
- [rustboot-di](../crates/rustboot-di/backlog.md) - Dependency injection, scoped lifetimes
- [rustboot-middleware](../crates/rustboot-middleware/backlog.md) - Middleware utilities, auth, CORS
- [rustboot-observability](../crates/rustboot-observability/backlog.md) - Logging, metrics, tracing, health checks
- [rustboot-ratelimit](../crates/rustboot-ratelimit/backlog.md) - Rate limiting patterns, distributed limits
- [rustboot-resilience](../crates/rustboot-resilience/backlog.md) - Circuit breaker, retry, bulkhead patterns
- [rustboot-serialization](../crates/rustboot-serialization/backlog.md) - Serialization formats, CBOR, Protobuf
- [rustboot-state-machine](../crates/rustboot-state-machine/backlog.md) - State machine patterns, hierarchical states

### Low Priority Crates
- [rustboot-async](../crates/rustboot-async/backlog.md) - Async utilities, task management, worker pools
- [rustboot-compress](../crates/rustboot-compress/backlog.md) - Compression utilities, Brotli, Snappy
- [rustboot-crypto](../crates/rustboot-crypto/backlog.md) - Cryptography utilities, AES, RSA, JWT
- [rustboot-datetime](../crates/rustboot-datetime/backlog.md) - Date/time utilities, timezone conversion
- [rustboot-fileio](../crates/rustboot-fileio/backlog.md) - File I/O utilities, file watching, streaming
- [rustboot-uuid](../crates/rustboot-uuid/backlog.md) - UUID generation and utilities

## Framework-Wide Improvements

For cross-cutting concerns that affect multiple crates or the framework as a whole, see:
- [Framework-Wide Backlog](framework-backlog.md)

This includes:
- **Implementation Gaps** - Concrete implementations for HTTP, database, web router, message brokers
- **Missing Middleware** - CORS, security headers, request logging, rate limiting integration
- **Missing Infrastructure** - Health checks, OpenAPI, migrations, session management
- **Incomplete Tests** - Empty test stubs in di, http, resilience, state-machine
- **Unimplemented Code** - Stub examples and unimplemented markers
- Documentation improvements
- Testing infrastructure
- Developer experience enhancements
- Infrastructure and deployment

---

**Completion Estimate**: ~40-50% complete
**Last Updated**: 2025-12-23
**Total Crates**: 23
**Backlog Files**: 21 (20 crate-specific + 1 framework-wide)
