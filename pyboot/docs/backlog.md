# Pyboot Framework Backlog Index

This document serves as an index to all backlog items across the pyboot framework. Each module maintains its own needs, and framework-wide improvements are tracked separately.

## Individual Module Backlogs

### High Priority Modules
- config - Configuration management, hot-reload, secrets
- database - Database abstractions, query patterns, pooling
- security - Authentication, authorization, secrets, auditing
- validation - Input validation, async validators
- observability - Logging, metrics, tracing

### Medium Priority Modules
- cache - Caching abstractions, distributed cache, eviction
- di - Dependency injection, scoped lifetimes
- http - HTTP client utilities, HTTP/2
- middleware - Middleware pipeline, CORS
- resilience - Circuit breaker, retry, bulkhead patterns
- ratelimit - Rate limiting patterns
- serialization - Serialization formats
- state_machine - State machine patterns

### Lower Priority Modules
- async_utils - Async utilities, task management
- cli - CLI framework
- compress - Compression utilities
- crypto - Cryptography primitives
- datetime - Date/time utilities
- fileio - File I/O utilities
- uuid - UUID generation

## Framework-Wide Improvements

For cross-cutting concerns that affect multiple modules or the framework as a whole, see:
- [Framework-Wide Backlog](framework-backlog.md)

This includes:
- **Tests** - Unit and integration tests for all modules
- **Examples** - Example scripts for all modules
- **Documentation** - Per-module overview.md files
- **Infrastructure** - CI/CD, publishing, documentation generation

---

**Current Status**: ~40% complete (37 modules implemented, tests/docs in progress)  
**Last Updated**: 2025-12-26  
**Total Modules**: 37
