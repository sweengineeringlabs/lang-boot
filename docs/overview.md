# Lang-Boot Overview

> **Multi-language infrastructure framework ecosystem** for building production-ready applications with reusable cross-cutting concerns.

---

## TLDR

Lang-Boot provides **4 framework implementations** (Rust, Python, Java, Go) with consistent APIs for common application infrastructure: validation, caching, DI, resilience, security, and observability. All frameworks follow the **SEA (Stratified Encapsulation Architecture)** pattern for maintainable, extensible code.

---

## Table of Contents

- [What is Lang-Boot?](#what-is-lang-boot)
  - [Language Internals](#language-internals)
- [Why Lang-Boot?](#why-lang-boot)
- [How to Use Lang-Boot](#how-to-use-lang-boot)
- [Audience](#audience)
- [Framework Comparison](#framework-comparison)
  - [Feature Matrix](#feature-matrix)
  - [Maturity Comparison](#maturity-comparison)
  - [Source Code Location](#source-code-location)
  - [Test Location](#test-location)
  - [Build & Tooling Comparison](#build--tooling-comparison)
  - [Dependency Comparison](#dependency-comparison)
  - [License Comparison](#license-comparison)
- [Architecture](#architecture)
- [Module Categories](#module-categories)
- [Framework Deep Dives](#framework-deep-dives)
- [CI/CD Infrastructure](#cicd-infrastructure)
- [Project Structure](#project-structure)
- [Insights & Recommendations](#insights--recommendations)
- [Getting Started](#getting-started)
- [Contributing](#contributing)

---

## What is Lang-Boot?

Lang-Boot is a **polyglot framework ecosystem** that provides standardized, production-grade implementations of common application infrastructure across four programming languages:

| Framework | Language | Version | Modules |
|-----------|----------|---------|---------|
| **Rustboot** | Rust | 1.75+ | 31 crates |
| **Pyboot** | Python | 3.11+ | 37 modules |
| **JBoot** | Java | 17+ | 24 modules |
| **Goboot** | Go | 1.21+ | 25+ packages |

### Core Capabilities

All frameworks provide these cross-cutting concerns:

- **Error Handling** — Result monads, typed errors, error chains
- **Configuration** — Multi-source, environment-aware, type-safe
- **Dependency Injection** — Containers, scopes, lifecycle management
- **Validation** — Fluent builders, custom constraints, i18n
- **Caching** — TTL-based, multi-backend, cache-aside pattern
- **Resilience** — Circuit breakers, retries, timeouts, bulkheads
- **Rate Limiting** — Token bucket, sliding window, leaky bucket
- **HTTP** — Client abstractions, middleware, routing
- **Messaging** — Pub/sub, event-driven, message queues
- **Security** — Authentication, authorization, crypto, secrets, auditing
- **Observability** — Logging, metrics, tracing, health checks
- **Database** — Repository patterns, transactions, query builders
- **State Machines** — FSM with guards, actions, transitions

### Language Internals

Understanding how each language works helps in choosing the right framework for your use case:

#### 🦀 Rust

| Aspect | Details |
|--------|---------|
| **Compilation** | Ahead-of-Time (AOT) via LLVM backend |
| **Output** | Native machine code (binary executable) |
| **Runtime** | Minimal runtime, no garbage collector |
| **Memory** | Ownership system with borrow checker (compile-time safety) |
| **Concurrency** | Zero-cost abstractions, async/await with tokio/async-std |
| **Type System** | Static, strong, algebraic data types (enums, structs) |
| **Error Handling** | `Result<T, E>` and `Option<T>` (no exceptions) |

```
Source (.rs) → rustc → LLVM IR → Machine Code → Binary
```

**Key Characteristics:**
- No garbage collection pauses
- Memory safety without runtime overhead
- Compile-time thread safety via `Send`/`Sync` traits
- Zero-cost abstractions (pay only for what you use)

---

#### 🐍 Python

| Aspect | Details |
|--------|---------|
| **Compilation** | Source → Bytecode (at import time) |
| **Output** | `.pyc` bytecode files (portable) |
| **Runtime** | CPython interpreter (or PyPy, GraalPy) |
| **Memory** | Reference counting + cyclic garbage collector |
| **Concurrency** | GIL limits CPU parallelism; async via asyncio |
| **Type System** | Dynamic, strong; optional type hints (PEP 484) |
| **Error Handling** | Exceptions (`try`/`except`) |

```
Source (.py) → Compiler → Bytecode (.pyc) → PVM (Python Virtual Machine)
```

**Key Characteristics:**
- Interpreted with JIT options (PyPy)
- Global Interpreter Lock (GIL) affects multithreading
- Excellent for rapid development and scripting
- Rich ecosystem for data science and web

---

#### ☕ Java

| Aspect | Details |
|--------|---------|
| **Compilation** | Source → Bytecode (via javac) |
| **Output** | `.class` files (platform-independent bytecode) |
| **Runtime** | JVM (HotSpot, GraalVM, OpenJ9) |
| **Memory** | Automatic garbage collection (G1, ZGC, Shenandoah) |
| **Concurrency** | OS threads + Virtual Threads (Java 21+) |
| **Type System** | Static, strong, nominal typing |
| **Error Handling** | Checked/unchecked exceptions |

```
Source (.java) → javac → Bytecode (.class) → JIT → Machine Code
```

**Key Characteristics:**
- "Write once, run anywhere" via JVM
- Just-In-Time (JIT) compilation for performance
- Virtual Threads enable millions of concurrent tasks
- Mature ecosystem with extensive libraries

---

#### 🦫 Go

| Aspect | Details |
|--------|---------|
| **Compilation** | Ahead-of-Time (AOT) via Go compiler |
| **Output** | Native machine code (statically linked binary) |
| **Runtime** | Minimal runtime with garbage collector |
| **Memory** | Concurrent, tri-color mark-and-sweep GC |
| **Concurrency** | Goroutines (lightweight threads) + channels |
| **Type System** | Static, strong, structural typing (interfaces) |
| **Error Handling** | Multiple return values (value, error pattern) |

```
Source (.go) → go build → Machine Code → Single Binary
```

**Key Characteristics:**
- Fast compilation (seconds, not minutes)
- Single static binary with no dependencies
- Built-in concurrency with goroutines (< 2KB stack)
- Simple language spec (25 keywords)

---

#### Comparison Summary

| Feature | Rust | Python | Java | Go |
|---------|------|--------|------|-----|
| **Compilation** | AOT | Interpreted/Bytecode | JIT | AOT |
| **Garbage Collection** | None (ownership) | Reference counting + GC | Yes (JVM GC) | Yes (concurrent GC) |
| **Memory Safety** | Compile-time | Runtime | Runtime (null checks) | Runtime |
| **Startup Time** | Fast | Slow | Slow (JVM warmup) | Fast |
| **Binary Size** | Medium | N/A (needs interpreter) | N/A (needs JVM) | Large (static) |
| **Concurrency Model** | async/await + threads | asyncio + threads (GIL) | Virtual threads | Goroutines |
| **Learning Curve** | Steep | Gentle | Moderate | Gentle |
| **Use Cases** | Systems, WebAssembly, CLI | Scripts, ML, Web | Enterprise, Android | Cloud, CLI, Microservices |

---

## Why Lang-Boot?

### The Problem

Modern applications require extensive "plumbing" code:

1. **Repetitive boilerplate** — Every project re-implements logging, config, validation
2. **Inconsistent patterns** — Teams use different approaches for the same problems
3. **Missing best practices** — Production concerns like circuit breakers are often skipped
4. **Polyglot challenges** — Multi-language teams have no shared vocabulary

### The Solution

Lang-Boot provides:

| Benefit | Description |
|---------|-------------|
| **Consistency** | Same module structure and naming across languages |
| **Best Practices** | Production-proven patterns built-in |
| **Extensibility** | SPI layer for custom implementations |
| **Type Safety** | Compile-time checks where possible |
| **Zero Lock-in** | Use individual modules without full framework |

### Design Principles

1. **Convention over Configuration** — Sensible defaults, opt-in customization
2. **Explicit over Implicit** — Clear APIs, no magic
3. **Composition over Inheritance** — Small, focused components
4. **Fail Fast** — Early validation, clear error messages
5. **Observable by Default** — Built-in metrics, logging, tracing hooks

---

## How to Use Lang-Boot

### Installation

```bash
# 🦀 Rust
cargo add rustboot

# 🐍 Python
pip install pyboot
pip install pyboot[full]    # All features
pip install pyboot[redis]   # Redis backend

# ☕ Java (Maven)
<dependency>
    <groupId>com.jboot</groupId>
    <artifactId>jboot-core</artifactId>
    <version>0.1.0</version>
</dependency>

# 🦫 Go
go get dev.engineeringlabs/goboot
```

### Quick Examples

#### Validation

```rust
// Rust
use rustboot::prelude::*;

let validator = StringValidationBuilder::new("email")
    .not_empty()
    .email()
    .build();
```

```python
# Python
from dev.engineeringlabs.pyboot.validation import Validator

validator = Validator.builder()
    .field("email").not_empty().email()
    .build()
```

```java
// Java
var validator = Validator.builder()
    .field("email").notEmpty().email()
    .field("age").range(18, 120)
    .build();
```

```go
// Go
validator := validation.NewBuilder().
    Field("email").NotEmpty().Email().
    Build()
```

#### Resilience (Circuit Breaker)

```rust
// Rust
let cb = CircuitBreaker::builder("api")
    .failure_threshold(5)
    .timeout(Duration::from_secs(30))
    .build();

cb.execute(|| risky_operation()).await?;
```

```python
# Python
from dev.engineeringlabs.pyboot.resilience import CircuitBreaker

cb = CircuitBreaker("api", failure_threshold=5, timeout=30)

@cb.protect
async def risky_operation():
    ...
```

```java
// Java
var cb = CircuitBreaker.builder("myService")
    .failureThreshold(5)
    .timeout(Duration.ofSeconds(30))
    .build();

cb.execute(() -> riskyOperation());
```

```go
// Go
cb := resilience.NewCircuitBreaker("api", resilience.DefaultCircuitBreakerConfig())
result := cb.Execute(riskyOperation)
```

---

## Audience

This documentation serves multiple audiences:

| Audience | Focus Areas | Start Here |
|----------|-------------|------------|
| **Application Developers** | Using frameworks in projects | [Quick Examples](#quick-examples) |
| **Framework Contributors** | Extending or improving frameworks | [Architecture](#architecture) |
| **Architects** | Evaluating for adoption | [Why Lang-Boot?](#why-lang-boot) |
| **Researchers** | SEA pattern, polyglot design | [Architecture](#architecture) |

---

## Framework Comparison

### Feature Matrix

| Feature | 🦀 Rustboot | 🐍 Pyboot | ☕ JBoot | 🦫 Goboot |
|---------|-------------|-----------|----------|----------|
| Error Handling | Result<T,E> | Result monad | Result<T> | Result[T] |
| Config | ✅ Multi-source | ✅ Multi-source | ✅ Multi-source | ✅ Multi-source |
| DI | ✅ Container | ✅ Protocol-based | ✅ Container | ✅ Interface-based |
| Validation | ✅ Fluent + Macros | ✅ Fluent | ✅ Fluent + Bean | ✅ Fluent |
| Caching | ✅ TTL + Backends | ✅ TTL + Backends | ✅ TTL + Backends | ✅ TTL + Backends |
| Circuit Breaker | ✅ | ✅ | ✅ | ✅ |
| Rate Limiting | ✅ 3 algorithms | ✅ 3 algorithms | ✅ 3 algorithms | ✅ Token bucket |
| HTTP Client | ✅ | ✅ | ✅ | ✅ |
| Messaging | ✅ Pub/Sub | ✅ Pub/Sub | ✅ Pub/Sub | ✅ Pub/Sub |
| Auth/AuthZ | ✅ | ✅ | ✅ | ✅ |
| Crypto | ✅ | ✅ | ✅ | ✅ |
| Observability | ✅ Full stack | ✅ Full stack | ✅ Full stack | ✅ Full stack |
| State Machine | ✅ Guards | ✅ Guards | ✅ Guards | ✅ Guards |
| Macros/Decorators | ✅ Proc macros | ✅ Decorators | ❌ | ❌ |
| OpenAPI | ✅ | ✅ | ❌ | ❌ |
| CLI | ✅ | ✅ | ❌ | ❌ |

### Maturity Comparison

| Aspect | Rustboot | Pyboot | JBoot | Goboot |
|--------|----------|--------|-------|--------|
| **File Count** | 8,594 | 484 | 83 | 141 |
| **Examples** | 1,351 files | 13 dirs | Minimal | 10 dirs |
| **Documentation** | Extensive | Good | Basic | Good |
| **CI/CD** | Production-grade | GitHub Actions | Basic | Basic |
| **Tests** | Comprehensive | Comprehensive | Growing | Growing |

### Source Code Location

| Framework | Source Path | Entry Point |
|-----------|-------------|-------------|
| **Rustboot** | `rustboot/crates/` | `rustboot/Cargo.toml` |
| **Pyboot** | `pyboot/src/dev/engineeringlabs/pyboot/` | `pyboot/pyproject.toml` |
| **JBoot** | `jboot/modules/*/src/main/java/` | `jboot/pom.xml` |
| **Goboot** | `goboot/*/` (each package) | `goboot/go.mod` |

### Test Location

| Framework | Unit Tests | Integration Tests | Test Command |
|-----------|------------|-------------------|--------------|
| **Rustboot** | `crates/*/src/**/*_test.rs` | `tests/` | `cargo test --workspace` |
| **Pyboot** | `src/test/` | `examples/` | `pytest` |
| **JBoot** | `modules/*/src/test/java/` | `modules/*/src/test/` | `mvn test` |
| **Goboot** | `*/*_test.go` | `examples/` | `go test ./...` |

### Build & Tooling Comparison

| Aspect | Rustboot | Pyboot | JBoot | Goboot |
|--------|----------|--------|-------|--------|
| **Build Tool** | Cargo | pip/poetry | Maven | Go modules |
| **Package Manager** | crates.io | PyPI | Maven Central | pkg.go.dev |
| **Linter** | Clippy | Ruff/Flake8 | Checkstyle | golangci-lint |
| **Formatter** | rustfmt | Black | google-java-format | gofmt |
| **Test Framework** | built-in | pytest | JUnit 5 | built-in |
| **Coverage** | cargo-llvm-cov | coverage.py | JaCoCo | go test -cover |
| **Docs Generator** | rustdoc | Sphinx/MkDocs | Javadoc | godoc |

### Dependency Comparison

| Aspect | Rustboot | Pyboot | JBoot | Goboot |
|--------|----------|--------|-------|--------|
| **Min Runtime** | Rust 1.75+ | Python 3.11+ | Java 17+ | Go 1.21+ |
| **External Deps** | Minimal | Minimal | Minimal | Minimal |
| **Optional Deps** | Feature flags | Extras `[redis]` | Maven profiles | Build tags |
| **Async Runtime** | tokio | asyncio | Virtual threads | goroutines |

### License Comparison

| Framework | License | Commercial Use | Modification | Distribution |
|-----------|---------|----------------|--------------|--------------|
| **Rustboot** | MIT | ✅ Yes | ✅ Yes | ✅ Yes |
| **Pyboot** | Apache-2.0 | ✅ Yes | ✅ Yes | ✅ Yes |
| **JBoot** | MIT | ✅ Yes | ✅ Yes | ✅ Yes |
| **Goboot** | Apache-2.0 | ✅ Yes | ✅ Yes | ✅ Yes |

---

## Architecture

### SEA (Stratified Encapsulation Architecture)

All frameworks follow the SEA pattern for consistent, maintainable code:

```
┌─────────────────────────────────────────────┐
│                  Facade                      │  Public API entry point
├─────────────────────────────────────────────┤
│                   API                        │  Contracts & interfaces
├─────────────────────────────────────────────┤
│                  Core                        │  Default implementations
├─────────────────────────────────────────────┤
│                   SPI                        │  Extension points
├─────────────────────────────────────────────┤
│                 Common                       │  Shared utilities
└─────────────────────────────────────────────┘
```

| Layer | Purpose | Visibility |
|-------|---------|------------|
| **Facade** | Simplified public API, re-exports | Public |
| **API** | Traits/interfaces, contracts | Public |
| **Core** | Default implementations | Internal |
| **SPI** | Service Provider Interface for extensions | Public |
| **Common** | Shared types, utilities, errors | Internal |

### Dependency Direction

```
Facade → API → Core → SPI → Common
         ↓           ↓
      (uses)     (implements)
```

- **API** defines contracts
- **Core** provides default implementations
- **SPI** allows custom implementations
- **Facade** simplifies consumption

---

## Module Categories

### Standard Module Set

All frameworks implement these module categories:

| Category | Modules | Description |
|----------|---------|-------------|
| **Core** | error, stereotypes, decorators/macros | Foundation types |
| **Foundation** | config, di, validation | Application setup |
| **Resilience** | resilience, cache, ratelimit | Fault tolerance |
| **Web/API** | web, http, session, middleware, openapi | HTTP layer |
| **Data** | database, storage, serialization | Persistence |
| **Messaging** | messaging, streams, notifications | Communication |
| **Security** | security, crypto | Auth & protection |
| **Observability** | observability, health, debug | Monitoring |
| **Utilities** | datetime, testing, fileio, compress, uuid | Helpers |
| **Advanced** | scheduler, statemachine, featureflags, cli | Specialized |

### Module Count by Framework

| Framework | Total Modules |
|-----------|---------------|
| Pyboot | 37 |
| Rustboot | 31 |
| Goboot | 25+ |
| JBoot | 24 |

---

## Framework Deep Dives

### 🦀 Rustboot

The most feature-rich implementation with:

- **31 specialized crates** with focused responsibilities
- **Procedural macros** for DI, validation, caching, tracing, retry
- **Production-grade CI/CD** with 6 automated GitHub Actions workflows
- **Docker & Kubernetes** deployment configurations
- **Comprehensive benchmarking** with Criterion
- **Template engine** for documentation generation

**Key Differentiators:**
- Compile-time validation via proc macros
- Zero-cost abstractions
- Extensive type safety

```rust
use rustboot_macros::{Injectable, Validate, cached, traced, retry};

#[derive(Injectable)]
struct UserService {
    repository: Arc<dyn UserRepository>,
    cache: Arc<Cache>,
}

#[derive(Validate)]
struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    username: String,
    
    #[validate(email)]
    email: String,
}

impl UserService {
    #[traced(level = "info")]
    #[retry(max_attempts = 3)]
    #[cached(ttl = 600)]
    async fn get_user(&self, id: u64) -> Result<User> {
        self.repository.find_user(id).await
    }
}
```

### 🐍 Pyboot

Pythonic implementation with:

- **37 modules** covering async utilities, CLI, compression
- **Protocol-based** dependency injection (no magic)
- **Decorators** for cross-cutting concerns
- **OpenAPI** documentation generation
- **Reactive streams** support

**Key Differentiators:**
- Native async/await support
- Type hints for IDE support
- Decorator composition

```python
from dev.engineeringlabs.pyboot.decorators import compose, memoize
from dev.engineeringlabs.pyboot.error import Result, Ok, Err

@memoize
def expensive_computation(n: int) -> int:
    return sum(range(n))

def divide(a: float, b: float) -> Result[float, str]:
    if b == 0:
        return Err("Division by zero")
    return Ok(a / b)
```

### ☕ JBoot

Java implementation with:

- **24 Maven modules** following Java conventions
- **Bean validation** with custom constraints
- **Fluent API** builders throughout
- **SEA pattern** clearly separated

**Key Differentiators:**
- Familiar to Spring developers
- Bean Validation integration
- Maven ecosystem compatibility

```java
var validator = Validator.builder()
    .field("email").notEmpty().email()
    .field("age").range(18, 120)
    .build();

var cache = Cache.inMemory();
cache.set("key", "value", Duration.ofMinutes(5));

var cb = CircuitBreaker.builder("myService")
    .failureThreshold(5)
    .timeout(Duration.ofSeconds(30))
    .build();
```

### 🦫 Goboot

Go implementation with:

- **25+ packages** with Go idioms
- **Interface-based** dependency injection
- **Result monad** for explicit error handling
- **Stereotypes** module (similar to Spring annotations)

**Key Differentiators:**
- Idiomatic Go patterns
- Simple, explicit APIs
- Great for microservices

```go
package main

import (
    "dev.engineeringlabs/goboot/errors"
    "dev.engineeringlabs/goboot/resilience"
)

func main() {
    result := divide(10, 2)
    if result.IsOk() {
        fmt.Printf("Result: %v\n", result.Unwrap())
    }
    
    cb := resilience.NewCircuitBreaker("api", resilience.DefaultCircuitBreakerConfig())
}

func divide(a, b float64) errors.Result[float64] {
    if b == 0 {
        return errors.Err[float64]("Division by zero")
    }
    return errors.Ok(a / b)
}
```

---

## CI/CD Infrastructure

### GitHub Actions Workflows

The project includes production-grade CI/CD (primarily configured for Rustboot):

| Workflow | Purpose | Trigger | Duration |
|----------|---------|---------|----------|
| `ci.yml` | Multi-platform testing (3 OS × 3 Rust) | Push/PR | ~15-20 min |
| `release.yml` | Automated crates.io publishing | Version tags | ~20-30 min |
| `security.yml` | Daily security scanning (4 tools) | Daily/manual | ~10-15 min |
| `benchmark.yml` | Performance regression detection | Push/PR | ~10 min |
| `docs.yml` | GitHub Pages documentation | Push to main | ~5 min |
| `nightly.yml` | Nightly Rust + Miri testing | Daily | ~15 min |

### Security Scanning

| Tool | Purpose |
|------|---------|
| **cargo-audit** | Rust vulnerability database |
| **cargo-deny** | License compliance, advisories |
| **Trivy** | Container/filesystem scanning |
| **Semgrep** | Static analysis patterns |

### Dependency Management

- **Dependabot** for automated PRs
- Weekly Cargo/GitHub Actions updates
- Intelligent grouping (patch/minor/major)

---

## Project Structure

```
lang-boot/
├── .github/                    # Shared CI/CD configuration
│   ├── workflows/              # GitHub Actions (6 workflows)
│   │   ├── ci.yml
│   │   ├── release.yml
│   │   ├── security.yml
│   │   ├── benchmark.yml
│   │   ├── docs.yml
│   │   └── nightly.yml
│   ├── ISSUE_TEMPLATE/         # Issue templates
│   ├── CODEOWNERS
│   ├── dependabot.yml
│   ├── deny.toml
│   └── pull_request_template.md
│
├── docs/                       # This documentation
│   └── overview.md
│
├── rustboot/                   # 🦀 Rust framework
│   ├── crates/                 # 31 individual crates
│   │   ├── rustboot-async/
│   │   ├── rustboot-cache/
│   │   ├── rustboot-config/
│   │   ├── rustboot-crypto/
│   │   ├── rustboot-database/
│   │   ├── rustboot-di/
│   │   ├── rustboot-error/
│   │   ├── rustboot-health/
│   │   ├── rustboot-http/
│   │   ├── rustboot-macros/
│   │   ├── rustboot-messaging/
│   │   ├── rustboot-middleware/
│   │   ├── rustboot-observability/
│   │   ├── rustboot-ratelimit/
│   │   ├── rustboot-resilience/
│   │   ├── rustboot-security/
│   │   ├── rustboot-serialization/
│   │   ├── rustboot-session/
│   │   ├── rustboot-state-machine/
│   │   ├── rustboot-validation/
│   │   ├── rustboot-web/
│   │   └── ... (31 total)
│   ├── examples/               # Example applications
│   ├── docs/                   # Rust-specific docs
│   ├── docker/                 # Docker configurations
│   ├── k8s/                    # Kubernetes manifests
│   └── template-engine/        # Documentation templates
│
├── pyboot/                     # 🐍 Python framework
│   ├── src/dev/engineeringlabs/pyboot/
│   │   ├── async_utils/
│   │   ├── cache/
│   │   ├── cli/
│   │   ├── config/
│   │   ├── crypto/
│   │   ├── database/
│   │   ├── datetime/
│   │   ├── decorators/
│   │   ├── di/
│   │   ├── error/
│   │   ├── feature_flags/
│   │   ├── health/
│   │   ├── http/
│   │   ├── messaging/
│   │   ├── middleware/
│   │   ├── notifications/
│   │   ├── observability/
│   │   ├── openapi/
│   │   ├── resilience/
│   │   ├── scheduler/
│   │   ├── security/
│   │   ├── serialization/
│   │   ├── session/
│   │   ├── state_machine/
│   │   ├── storage/
│   │   ├── streams/
│   │   ├── testing/
│   │   ├── validation/
│   │   └── web/
│   ├── examples/               # Example scripts
│   └── docs/                   # Python-specific docs
│
├── jboot/                      # ☕ Java framework
│   ├── modules/                # 24 Maven modules
│   │   ├── jboot-cache/
│   │   ├── jboot-config/
│   │   ├── jboot-core/
│   │   ├── jboot-crypto/
│   │   ├── jboot-database/
│   │   ├── jboot-di/
│   │   ├── jboot-error/
│   │   ├── jboot-health/
│   │   ├── jboot-http/
│   │   ├── jboot-messaging/
│   │   ├── jboot-observability/
│   │   ├── jboot-ratelimit/
│   │   ├── jboot-resilience/
│   │   ├── jboot-security/
│   │   ├── jboot-serialization/
│   │   ├── jboot-session/
│   │   ├── jboot-statemachine/
│   │   ├── jboot-storage/
│   │   ├── jboot-streams/
│   │   ├── jboot-testing/
│   │   ├── jboot-validation/
│   │   └── jboot-web/
│   ├── pom.xml                 # Parent POM
│   └── docs/                   # Java-specific docs
│
└── goboot/                     # 🦫 Go framework
    ├── cache/
    ├── config/
    ├── crypto/
    ├── database/
    ├── datetime/
    ├── di/
    ├── errors/
    ├── featureflags/
    ├── health/
    ├── http/
    ├── messaging/
    ├── notifications/
    ├── observability/
    ├── resilience/
    ├── scheduler/
    ├── security/
    ├── serialization/
    ├── session/
    ├── statemachine/
    ├── stereotypes/
    ├── storage/
    ├── streams/
    ├── testing/
    ├── validation/
    ├── web/
    ├── examples/               # Example applications
    ├── go.mod
    └── docs/                   # Go-specific docs
```

---

## Getting Started

### Choose Your Framework

1. **New to Lang-Boot?** Start with your preferred language's framework
2. **Polyglot team?** Use all frameworks with consistent patterns
3. **Just need one module?** Import only what you need

### Learning Path

1. **Read the README** of your chosen framework
2. **Run an example** to see it in action
3. **Explore module docs** for specific features
4. **Check tests** for usage patterns

### Per-Framework Documentation

| Framework | Overview | Examples |
|-----------|----------|----------|
| Rustboot | [docs/overview.md](../rustboot/docs/overview.md) | [examples/](../rustboot/examples/) |
| Pyboot | [docs/overview.md](../pyboot/docs/overview.md) | [examples/](../pyboot/examples/) |
| JBoot | [docs/overview.md](../jboot/docs/overview.md) | modules/\*/src/test/ |
| Goboot | [docs/overview.md](../goboot/docs/overview.md) | [examples/](../goboot/examples/) |

---

## Contributing

### Development Setup

See individual framework READMEs for setup:

- [Rustboot CONTRIBUTING.md](../rustboot/CONTRIBUTING.md)
- [Pyboot CONTRIBUTING.md](../pyboot/CONTRIBUTING.md)
- [JBoot CONTRIBUTING.md](../jboot/CONTRIBUTING.md)
- [Goboot CONTRIBUTING.md](../goboot/CONTRIBUTING.md)

### Contribution Guidelines

1. **Follow SEA architecture** — Respect layer boundaries
2. **Add tests** — All new code needs tests
3. **Update docs** — Keep documentation in sync
4. **Cross-pollinate** — Port features across frameworks when applicable

### Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make changes following coding standards
4. Run tests locally
5. Submit PR with description

---

## Insights & Recommendations

### Strengths

| Strength | Description |
|----------|-------------|
| **Consistency** | All frameworks share common module structure and naming conventions |
| **Comprehensive** | Covers all major cross-cutting concerns for production apps |
| **Well-documented** | Especially Rustboot with extensive CI/CD documentation |
| **Production-ready** | Enterprise-grade security scanning and testing |
| **SEA Architecture** | Clean separation of concerns with extension points |
| **Polyglot** | Same patterns across 4 languages enables team portability |

### Areas for Improvement

| Area | Framework | Status | Notes |
|------|-----------|--------|-------|
| **Examples** | JBoot | ✅ Done | Added `examples/README.md` with 8 comprehensive examples |
| **Documentation** | JBoot | ✅ Done | Added openapi, cli, async modules |
| **Stereotypes docs** | Goboot | ✅ Done | Added `stereotypes/doc/overview.md` with Spring/NestJS comparison |
| **Async patterns** | JBoot/Goboot | ✅ Done | Added `jboot-async` and `goboot/async` modules |
| **OpenAPI** | JBoot/Goboot | ✅ Done | Added `jboot-openapi` and `goboot/openapi` modules |
| **CLI** | JBoot/Goboot | ✅ Done | Added `jboot-cli` and `goboot/cli` modules |

### Remaining Work

| Area | Framework | Status | Notes |
|------|-----------|--------|-------|
| **Tests** | JBoot | ✅ Done | Added AsyncTest, CliTest, OpenApiTest |
| **Tests** | Goboot | ✅ Done | Added async_test.go, cli_test.go, openapi_test.go |
| **API layers** | JBoot/Goboot | ✅ Done | Added API interfaces for async, cli, openapi modules |
| **Examples** | Goboot | ✅ Done | Added async_example.go, cli_example.go, openapi_example.go |

### Recommended Adoption Path

1. **Greenfield projects**: Start with full framework adoption
2. **Existing projects**: Adopt individual modules incrementally
3. **Polyglot teams**: Establish shared vocabulary using Lang-Boot patterns
4. **Microservices**: Use language-specific framework per service with consistent patterns

### Future Roadmap Suggestions

| Priority | Feature | Frameworks |
|----------|---------|------------|
| **High** | GraphQL module | All |
| **High** | gRPC module | All |
| **Medium** | WebSocket support | All |
| **Medium** | Distributed tracing | JBoot, Goboot |
| **Low** | WASM support | Rustboot |
| **Low** | ML/AI utilities | Pyboot |

---

## License

Individual frameworks have their own licenses:

| Framework | License |
|-----------|---------|
| Rustboot | MIT |
| Pyboot | Apache-2.0 |
| JBoot | MIT |
| Goboot | Apache-2.0 |

---

**Maintained by**: @elvischidera  
**Last Updated**: 2025-12-27  
**Version**: 1.0
