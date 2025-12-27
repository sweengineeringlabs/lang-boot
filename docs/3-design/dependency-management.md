# Dependency Management Conventions

Idiomatic dependency management for each language.

---

## 🦀 Rust

### Cargo.toml

```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
tokio-test = "0.4"

[build-dependencies]
cc = "1.0"
```

### Commands

```bash
cargo add serde          # Add dependency
cargo update             # Update to latest compatible
cargo tree               # View dependency tree
cargo audit              # Security audit
```

### Conventions

- Use semantic versioning: `"1.0"` means `>=1.0.0, <2.0.0`
- Pin specific versions in applications: `"=1.0.5"`
- Use `Cargo.lock` for reproducible builds (commit for binaries)
- Use features for optional dependencies

---

## 🦫 Go

### go.mod

```go
module github.com/myorg/myproject

go 1.21

require (
    github.com/gin-gonic/gin v1.9.0
    github.com/stretchr/testify v1.8.4
)

require (
    // indirect dependencies
    golang.org/x/net v0.10.0 // indirect
)
```

### Commands

```bash
go mod init github.com/myorg/myproject
go get github.com/gin-gonic/gin
go mod tidy              # Clean up dependencies
go mod download          # Download all deps
go list -m all           # List all modules
```

### Conventions

- Module path = import path
- `go.sum` contains checksums (always commit)
- Use `go mod tidy` before committing
- Indirect dependencies managed automatically
- Use major version suffix for v2+: `github.com/pkg/v2`

---

## ☕ Java (Maven)

### pom.xml

```xml
<project>
    <groupId>com.example</groupId>
    <artifactId>my-project</artifactId>
    <version>1.0.0-SNAPSHOT</version>
    
    <properties>
        <java.version>17</java.version>
    </properties>
    
    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
            <version>3.2.0</version>
        </dependency>
        
        <dependency>
            <groupId>org.junit.jupiter</groupId>
            <artifactId>junit-jupiter</artifactId>
            <version>5.10.0</version>
            <scope>test</scope>
        </dependency>
    </dependencies>
</project>
```

### Commands

```bash
mvn dependency:tree      # View dependency tree
mvn versions:display-dependency-updates
mvn dependency:resolve   # Download dependencies
```

### Conventions

- Use BOM (Bill of Materials) for version management
- Use SNAPSHOT for development versions
- Scope: compile, test, provided, runtime
- Use dependencyManagement in parent POM

---

## 🐍 Python

### pyproject.toml (Modern)

```toml
[project]
name = "my-package"
version = "0.1.0"
requires-python = ">=3.11"
dependencies = [
    "requests>=2.28",
    "pydantic>=2.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.0",
    "black",
    "ruff",
]
```

### requirements.txt (Traditional)

```
requests>=2.28,<3.0
pydantic>=2.0
pytest>=7.0  # dev only
```

### Commands

```bash
pip install -e .                    # Install editable
pip install -e ".[dev]"             # With dev deps
pip freeze > requirements.lock      # Lock versions
pip-compile pyproject.toml          # Generate locked deps
```

### Conventions

- Use `pyproject.toml` (PEP 621)
- Use virtual environments (`venv`, `poetry`, `uv`)
- Lock production dependencies
- Separate dev dependencies
- Use tools: `pip-tools`, `poetry`, `uv`

---

## Comparison

| Aspect | Rust | Go | Java | Python |
|--------|------|-----|------|--------|
| Manifest | `Cargo.toml` | `go.mod` | `pom.xml` | `pyproject.toml` |
| Lock file | `Cargo.lock` | `go.sum` | N/A | `requirements.lock` |
| Registry | crates.io | pkg.go.dev | Maven Central | PyPI |
| Install | `cargo add` | `go get` | pom.xml edit | `pip install` |
| Scopes | deps, dev, build | single | compile/test/etc | deps, optional |

---

## Version Specifiers

| Syntax | Rust | Go | Java | Python |
|--------|------|-----|------|--------|
| Exact | `=1.0.0` | `v1.0.0` | `1.0.0` | `==1.0.0` |
| Compatible | `1.0` | `v1.0.0` | `[1.0,2.0)` | `~=1.0` |
| Minimum | `>=1.0` | `v1.0.0` | `[1.0,)` | `>=1.0` |
| Range | `>=1.0, <2.0` | N/A | `[1.0,2.0)` | `>=1.0,<2.0` |
