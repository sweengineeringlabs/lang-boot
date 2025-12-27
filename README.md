# Lang-Boot

**Multi-language infrastructure framework ecosystem** — Reusable cross-cutting concerns for production applications.

---

## TLDR

Lang-Boot provides **4 framework implementations** (Rust, Python, Java, Go) with consistent APIs for common infrastructure: validation, caching, DI, resilience, security, and observability. All follow the **SEA (Stratified Encapsulation Architecture)** pattern.

---

## Table of Contents

- [Frameworks](#frameworks)
- [Quick Start](#quick-start)
- [Features](#features)
- [Documentation](#documentation)
- [License](#license)

---

## Frameworks

| Framework | Language | Modules | Status |
|-----------|----------|---------|--------|
| [🦀 Rustboot](./rustboot) | Rust 1.75+ | 31 crates | Production-ready |
| [🐍 Pyboot](./pyboot) | Python 3.11+ | 37 modules | Stable |
| [☕ JBoot](./jboot) | Java 17+ | 24 modules | Growing |
| [🦫 Goboot](./goboot) | Go 1.21+ | 25+ packages | Growing |

## Quick Start

```bash
# Rust
cargo add rustboot

# Python
pip install pyboot

# Java (Maven)
<dependency>
    <groupId>com.jboot</groupId>
    <artifactId>jboot-core</artifactId>
</dependency>

# Go
go get dev.engineeringlabs/goboot
```

## Features

- 🔄 **Resilience** — Circuit breakers, retries, timeouts
- 💾 **Caching** — Multi-backend, TTL-based
- 💉 **Dependency Injection** — Type-safe containers
- ✅ **Validation** — Fluent builders, custom constraints
- 🔐 **Security** — Auth, authz, crypto, secrets
- 📊 **Observability** — Logging, metrics, tracing
- 🌐 **Web** — HTTP, middleware, sessions

## Documentation

See [docs/overview.md](./docs/overview.md) for complete documentation.

## License

MIT / Apache-2.0 (see individual frameworks)
