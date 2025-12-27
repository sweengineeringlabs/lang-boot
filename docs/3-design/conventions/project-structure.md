# Project Structure Conventions

Idiomatic directory layouts for each language.

---

## 🦀 Rust

### Single Crate

```
my-crate/
├── Cargo.toml           # Package manifest
├── src/
│   ├── lib.rs           # Library root (or main.rs for binary)
│   ├── module.rs        # Module file
│   └── module/          # Module with submodules
│       ├── mod.rs
│       └── submodule.rs
├── tests/               # Integration tests
│   └── integration.rs
├── examples/            # Example binaries
│   └── basic.rs
├── benches/             # Benchmarks
│   └── benchmark.rs
└── target/              # Build output (gitignored)
```

### Workspace (Multiple Crates)

```
my-workspace/
├── Cargo.toml           # Workspace root
├── crates/
│   ├── core/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── cli/
│       ├── Cargo.toml
│       └── src/main.rs
└── target/              # Shared build output
```

---

## 🦫 Go

### Single Module

```
myproject/
├── go.mod               # Module definition
├── go.sum               # Dependency checksums
├── main.go              # Entry point (package main)
├── handler.go           # Package files
├── handler_test.go      # Tests (co-located)
├── internal/            # Private packages (not importable)
│   └── database/
│       └── db.go
├── pkg/                 # Public reusable packages
│   └── util/
│       └── util.go
└── cmd/                 # Multiple binaries
    └── myapp/
        └── main.go
```

### Standard Layout

```
├── api/                 # OpenAPI specs, proto files
├── cmd/                 # Main applications
├── internal/            # Private code
├── pkg/                 # Public library code
├── web/                 # Web assets
└── scripts/             # Build scripts
```

---

## ☕ Java (Maven)

### Standard Maven Layout

```
my-project/
├── pom.xml              # Maven config
├── src/
│   ├── main/
│   │   ├── java/        # Source code
│   │   │   └── com/example/
│   │   │       ├── Application.java
│   │   │       └── service/
│   │   │           └── UserService.java
│   │   └── resources/   # Config, templates
│   │       └── application.yml
│   └── test/
│       ├── java/        # Test code
│       │   └── com/example/
│       │       └── service/
│       │           └── UserServiceTest.java
│       └── resources/   # Test resources
└── target/              # Build output (gitignored)
```

### Multi-Module Maven

```
parent/
├── pom.xml              # Parent POM
├── core/
│   ├── pom.xml
│   └── src/
├── api/
│   ├── pom.xml
│   └── src/
└── app/
    ├── pom.xml
    └── src/
```

---

## 🐍 Python

### Package Layout

```
my-project/
├── pyproject.toml       # Modern config (PEP 517)
├── setup.py             # Legacy (optional)
├── src/
│   └── mypackage/       # Source package
│       ├── __init__.py
│       ├── core.py
│       └── utils/
│           ├── __init__.py
│           └── helpers.py
├── tests/               # Test directory
│   ├── __init__.py
│   ├── test_core.py
│   └── conftest.py      # pytest fixtures
├── docs/                # Documentation
└── requirements.txt     # Dependencies (or use pyproject.toml)
```

### Flat Layout (simpler)

```
my-project/
├── mypackage/
│   ├── __init__.py
│   └── core.py
├── tests/
│   └── test_core.py
├── pyproject.toml
└── README.md
```

---

## Comparison

| Aspect | Rust | Go | Java | Python |
|--------|------|-----|------|--------|
| Manifest | `Cargo.toml` | `go.mod` | `pom.xml` | `pyproject.toml` |
| Source | `src/` | Root or `cmd/` | `src/main/java/` | `src/` or root |
| Tests | `tests/` + inline | `*_test.go` | `src/test/java/` | `tests/` |
| Private | N/A (crate boundary) | `internal/` | Package-private | `_` prefix |
| Build output | `target/` | N/A (cached) | `target/` | `dist/` |
