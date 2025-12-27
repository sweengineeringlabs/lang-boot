# Design Conventions Index

Idiomatic conventions for Rust, Go, Java, and Python.

---

## Conventions

| Document | Description |
|----------|-------------|
| [Testing](testing-conventions.md) | Unit tests, integration tests, co-location |
| [Error Handling](error-handling.md) | Result types, exceptions, error returns |
| [Naming](naming-conventions.md) | snake_case, camelCase, PascalCase |
| [Project Structure](project-structure.md) | Directory layouts, file organization |
| [Documentation](documentation.md) | Doc comments, docstrings, Javadoc |
| [Async & Concurrency](async-concurrency.md) | async/await, goroutines, virtual threads |
| [Visibility & Access](visibility-access.md) | pub, public, private, package-private |
| [Modules & Imports](modules-imports.md) | Import styles, module systems |
| [Dependency Management](dependency-management.md) | Cargo, pip, Maven/Gradle, go mod |
| [Logging](logging.md) | Structured logging, log levels |
| [Configuration](configuration.md) | Environment variables, config files |

---

## Quick Reference

| Aspect | Rust | Go | Java | Python |
|--------|------|-----|------|--------|
| **Naming (vars)** | snake_case | camelCase | camelCase | snake_case |
| **Naming (types)** | PascalCase | PascalCase | PascalCase | PascalCase |
| **Errors** | `Result<T,E>` | `(T, error)` | Exceptions | Exceptions |
| **Tests** | `#[test]` inline | `*_test.go` | `src/test/` | `tests/` |
| **Async** | async/await | goroutines | virtual threads | asyncio |
| **Private** | (default) | lowercase | `private` | `_prefix` |
| **Config** | TOML | YAML | YAML | .env |
| **Package** | Cargo.toml | go.mod | pom.xml | pyproject.toml |
| **Docs** | `///` | `// Comment` | `/** */` | `"""` |
| **Logging** | tracing | slog | SLF4J | structlog |

---

## See Also

- [Architecture Overview](../overview.md)
- [Rustboot Developer Guide](../../rustboot/docs/4-development/developer-guide.md)
- [Pyboot Developer Guide](../../pyboot/docs/4-development/developer-guide.md)
- [JBoot Developer Guide](../../jboot/docs/4-development/developer-guide.md)
- [Goboot Developer Guide](../../goboot/docs/4-development/developer-guide.md)
