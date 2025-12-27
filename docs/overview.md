# Lang-Boot Overview

> **Multi-language infrastructure framework ecosystem** for building production-ready applications with reusable cross-cutting concerns.

---

## TLDR

Lang-Boot provides **4 framework implementations** (Rust, Python, Java, Go) with consistent APIs for common application infrastructure. All frameworks follow the **SEA (Stratified Encapsulation Architecture)** pattern.

---

## Frameworks

| Framework | Language | Version | Modules | Overview |
|-----------|----------|---------|---------|----------|
| 🦀 **Rustboot** | Rust | 1.75+ | 31 crates | [rustboot/docs/overview.md](../rustboot/docs/overview.md) |
| 🐍 **Pyboot** | Python | 3.11+ | 37 modules | [pyboot/docs/overview.md](../pyboot/docs/overview.md) |
| ☕ **JBoot** | Java | 17+ | 25 modules | [jboot/docs/overview.md](../jboot/docs/overview.md) |
| 🦫 **Goboot** | Go | 1.21+ | 28 packages | [goboot/docs/overview.md](../goboot/docs/overview.md) |

Each framework overview covers:
- **WHAT**: Module capabilities
- **WHY**: Problems solved, when to use
- **HOW**: Usage examples

For implementation details, see `{lang}/docs/4-development/developer-guide.md`.

---

## Why Lang-Boot?

### Problems Solved

1. **Repetitive boilerplate** — Every project re-implements logging, config, validation
2. **Inconsistent patterns** — Teams use different approaches for same problems
3. **Missing best practices** — Production concerns like circuit breakers often skipped
4. **Polyglot challenges** — Multi-language teams have no shared vocabulary

### Benefits

| Benefit | Description |
|---------|-------------|
| **Consistency** | Same module structure and naming across languages |
| **Best Practices** | Production-proven patterns built-in |
| **Extensibility** | SPI layer for custom implementations |
| **Zero Lock-in** | Use individual modules without full framework |

---

## Architecture: SEA Pattern

All frameworks follow **Stratified Encapsulation Architecture**:

```
┌─────────────────────────────────────┐
│           FACADE (Public API)       │  ← What users import
├─────────────────────────────────────┤
│              CORE                   │  ← Implementations
├─────────────────────────────────────┤
│              API                    │  ← Internal contracts
├─────────────────────────────────────┤
│              SPI                    │  ← Extension points
├─────────────────────────────────────┤
│            COMMON                   │  ← Shared utilities
└─────────────────────────────────────┘
```

---

## Module Categories

All frameworks provide these cross-cutting concerns:

| Category | Modules |
|----------|---------|
| **Core** | Errors, Result types, Common utilities |
| **Foundation** | Config, DI, Validation |
| **Resilience** | Circuit breakers, Retry, Timeout, Rate limiting |
| **Web/API** | HTTP client, Web framework, OpenAPI, CLI |
| **Data** | Database, Caching, Storage |
| **Messaging** | Event bus, Pub/sub, Queues |
| **Security** | Auth, Authorization, Crypto, Secrets |
| **Observability** | Logging, Metrics, Tracing, Health |
| **Utilities** | DateTime, Serialization, Testing |
| **Advanced** | State machines, Scheduler, Feature flags |

---

## Language Comparison

| Feature | Rust | Python | Java | Go |
|---------|------|--------|------|-----|
| **Compilation** | AOT | Interpreted | JIT | AOT |
| **GC** | None (ownership) | Ref counting + GC | JVM GC | Concurrent GC |
| **Concurrency** | async/await | asyncio | Virtual threads | Goroutines |
| **Startup** | Fast | Slow | Slow (warmup) | Fast |
| **Use Cases** | Systems, CLI | Scripts, ML | Enterprise | Cloud, CLI |

---

## Getting Started

1. **Choose your framework** based on language preference
2. **Read the framework overview** (links above)
3. **Follow the developer guide** for implementation details
4. **Run examples** to see it in action

---

## Project Structure

```
lang-boot/
├── docs/                    ← You are here
│   └── overview.md
├── rustboot/                ← Rust framework
│   ├── docs/overview.md     ← Concepts
│   └── docs/4-development/  ← Implementation
├── pyboot/                  ← Python framework
├── jboot/                   ← Java framework
└── goboot/                  ← Go framework
```

---

## Contributing

See individual framework CONTRIBUTING.md files:
- [Rustboot](../rustboot/CONTRIBUTING.md)
- [Pyboot](../pyboot/CONTRIBUTING.md)
- [JBoot](../jboot/CONTRIBUTING.md)
- [Goboot](../goboot/CONTRIBUTING.md)

---

## License

| Framework | License |
|-----------|---------|
| Rustboot | MIT |
| Pyboot | Apache-2.0 |
| JBoot | MIT |
| Goboot | Apache-2.0 |
