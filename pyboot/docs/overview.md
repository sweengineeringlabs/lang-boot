# Pyboot Framework Overview

| | |
|------|------|
| **WHAT** | Infrastructure library for Python applications |
| **WHY** | Reusable cross-cutting concerns (validation, caching, DI, resilience, observability) |
| **HOW** | Pure Python with decorators and protocols (no runtime reflection) |

## Quick Navigation

- **Architecture & Design** → [3-design/architecture.md](3-design/architecture.md)
- **Development Guides** → [4-development/developer-guide.md](4-development/developer-guide.md)
- **Module Documentation** → See below

## Module Documentation

Detailed overview for each module (WHAT-WHY-HOW structure):

### Core Infrastructure
- [error](../src/dev/engineeringlabs/pyboot/error/) - Error types, Result monad
- [decorators](../src/dev/engineeringlabs/pyboot/decorators/) - Decorator utilities
- [toolchain](../src/dev/engineeringlabs/pyboot/toolchain/) - Build and environment utilities

### Application Foundation
- [config](../src/dev/engineeringlabs/pyboot/config/) - Configuration management
- [di](../src/dev/engineeringlabs/pyboot/di/) - Dependency injection
- [validation](../src/dev/engineeringlabs/pyboot/validation/) - Input validation framework

### Resilience & Performance
- [resilience](../src/dev/engineeringlabs/pyboot/resilience/) - Circuit breakers, retries, timeouts
- [ratelimit](../src/dev/engineeringlabs/pyboot/ratelimit/) - Rate limiting strategies
- [cache](../src/dev/engineeringlabs/pyboot/cache/) - Caching abstractions

### Web & API
- [web](../src/dev/engineeringlabs/pyboot/web/) - Web framework utilities
- [http](../src/dev/engineeringlabs/pyboot/http/) - HTTP client utilities
- [openapi](../src/dev/engineeringlabs/pyboot/openapi/) - OpenAPI documentation 
- [middleware](../src/dev/engineeringlabs/pyboot/middleware/) - Middleware pipeline

### Data & Storage
- [database](../src/dev/engineeringlabs/pyboot/database/) - Database abstractions
- [storage](../src/dev/engineeringlabs/pyboot/storage/) - File storage abstractions
- [serialization](../src/dev/engineeringlabs/pyboot/serialization/) - Serialization formats

### Observability
- [observability](../src/dev/engineeringlabs/pyboot/observability/) - Logging, metrics, tracing
- [debug](../src/dev/engineeringlabs/pyboot/debug/) - Debug utilities
- [health](../src/dev/engineeringlabs/pyboot/health/) - Health checks

### Messaging & Events
- [messaging](../src/dev/engineeringlabs/pyboot/messaging/) - Message queue abstractions
- [notifications](../src/dev/engineeringlabs/pyboot/notifications/) - Multi-channel notifications
- [streams](../src/dev/engineeringlabs/pyboot/streams/) - Reactive streams

### Security
- [security](../src/dev/engineeringlabs/pyboot/security/) - Authentication, authorization
- [crypto](../src/dev/engineeringlabs/pyboot/crypto/) - Cryptography primitives

### Utilities
- [async_utils](../src/dev/engineeringlabs/pyboot/async_utils/) - Async utilities
- [cli](../src/dev/engineeringlabs/pyboot/cli/) - CLI framework
- [compress](../src/dev/engineeringlabs/pyboot/compress/) - Compression
- [datetime](../src/dev/engineeringlabs/pyboot/datetime/) - Date/time utilities
- [fileio](../src/dev/engineeringlabs/pyboot/fileio/) - File I/O
- [parsing](../src/dev/engineeringlabs/pyboot/parsing/) - JSON, YAML, TOML parsing
- [uuid](../src/dev/engineeringlabs/pyboot/uuid/) - UUID generation

### Special
- [feature_flags](../src/dev/engineeringlabs/pyboot/feature_flags/) - Feature toggles
- [scheduler](../src/dev/engineeringlabs/pyboot/scheduler/) - Task scheduling
- [session](../src/dev/engineeringlabs/pyboot/session/) - Session management
- [state_machine](../src/dev/engineeringlabs/pyboot/state_machine/) - State machines
- [testing](../src/dev/engineeringlabs/pyboot/testing/) - Test utilities

## Examples

See [`examples/`](../examples/) for working examples:

- `async_example.py` - Async task management
- `cli_example.py` - CLI framework
- `compress_example.py` - Compression utilities
- `debug_example.py` - Debug and profiling
- `decorators_example.py` - Decorator composition
- `error_example.py` - Result monad
- `fileio_example.py` - File operations
- `parsing_example.py` - JSON/YAML/TOML parsing
- `ratelimit_example.py` - Rate limiting
- `toolchain_example.py` - Environment detection
- `uuid_example.py` - UUID generation
- `web_example.py` - Web routing

## Backlog & Roadmap

- [Backlog Index](backlog.md) - Links to all module backlogs
- [Framework-Wide Backlog](framework-backlog.md) - Cross-cutting improvements

## Documentation Standards

- [Documentation Templates](templates/README.md) - WHAT-WHY-HOW templates for SEA projects

---

**Total Modules**: 37  
**Architecture**: SEA (Stratified Encapsulation Architecture)  
**Documentation Format**: WHAT-WHY-HOW structure
