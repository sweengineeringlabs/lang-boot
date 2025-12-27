# Goboot Framework Overview

| | |
|------|------|
| **WHAT** | Infrastructure library for Go applications |
| **WHY** | Reusable cross-cutting concerns (validation, caching, DI, resilience, observability) |
| **HOW** | Pure Go with interfaces and generics (no reflection magic) |

## Quick Navigation

- **Examples** → [examples/](../examples/)
- **Architecture & Design** → [3-design/architecture.md](3-design/architecture.md)
- **Development Guides** → [4-development/developer-guide.md](4-development/developer-guide.md)
- **Module Documentation** → See below

## Module Documentation

Detailed overview for each module (WHAT-WHY-HOW structure):

### Core Infrastructure
- [errors](../errors/) - Error types, Result monad
- [stereotypes](../stereotypes/) - Markers, decorators, annotations ([docs](../stereotypes/doc/overview.md)) ⭐

### Application Foundation
- [async](../async/) - Async utilities, worker pools ([docs](../async/doc/overview.md)) ⭐
- [config](../config/) - Configuration management
- [di](../di/) - Dependency injection
- [validation](../validation/) - Input validation framework

### Resilience & Performance
- [resilience](../resilience/) - Circuit breakers, retries, timeouts
- [cache](../cache/) - Caching abstractions

### Web & API
- [cli](../cli/) - Command-line interface builder ([docs](../cli/doc/overview.md)) ⭐
- [openapi](../openapi/) - OpenAPI/Swagger documentation ([docs](../openapi/doc/overview.md)) ⭐
- [web](../web/) - Web framework, router, middleware
- [http](../http/) - HTTP client utilities
- [session](../session/) - Session management

### Data & Storage
- [database](../database/) - Database abstractions, transactions
- [storage](../storage/) - File storage abstractions

### Messaging & Streams
- [messaging](../messaging/) - Event bus, pub/sub
- [streams](../streams/) - Reactive stream processing

### Security & Crypto
- [security](../security/) - Authentication, authorization, tokens
- [crypto](../crypto/) - Encryption, hashing, HMAC

### Observability
- [observability](../observability/) - Logging, metrics, tracing
- [health](../health/) - Health checks, liveness/readiness

### Utilities
- [serialization](../serialization/) - JSON, XML serialization
- [datetime](../datetime/) - Clocks, date utilities
- [testing](../testing/) - Mocks, assertions, fixtures

### Advanced Features
- [scheduler](../scheduler/) - Task scheduling, cron
- [featureflags](../featureflags/) - Feature toggles, rollouts
- [statemachine](../statemachine/) - FSM with guards
- [notifications](../notifications/) - Email, SMS, webhooks

## Examples

See [`examples/`](../examples/) for working examples:

- `errors_example.go` - Result monad
- `resilience_example.go` - Resilience patterns
- `validation_example.go` - Input validation
- `cache_example.go` - Caching
- `observability_example.go` - Logging and metrics
- `http_example.go` - HTTP client
- `web_example.go` - Web server and middleware
- `security_example.go` - Authentication and authorization
- `database_example.go` - Database patterns
- `messaging_example.go` - Event bus and pub/sub

## Backlog & Roadmap

- [Framework-Wide Backlog](framework-backlog.md) - Cross-cutting improvements

## Documentation Standards

- All modules follow the WHAT-WHY-HOW structure
- Each module has API, Core, and optional SPI layers
- SEA (Stratified Encapsulation Architecture) pattern

---

**Total Modules**: 28  
**Total Test Files**: 28  
**Architecture**: SEA (Stratified Encapsulation Architecture)  
**Documentation Format**: WHAT-WHY-HOW structure

