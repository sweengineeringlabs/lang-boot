# Rustboot Developer Guide

Development guides, patterns, and best practices.

**Audience**: Application Developers, Contributors

## Repository Governance

- [Repository Governance Best Practices](guide/repository-governance.md) - Required files and processes
  - CODE_OF_CONDUCT, SECURITY, SUPPORT, CONTRIBUTING files
  - Open-source vs internal project requirements
  - Issue and PR templates
  - License selection and compliance
  - Community management

## Security Development

- [Security Guide](guide/security-guide.md) - How to implement security in Rustboot applications
  - Cryptography (rustboot-crypto)
  - Authentication & Authorization (rustboot-security)
  - Input Validation (rustboot-validation)
  - Rate Limiting (rustboot-ratelimit)
  - Secure File Operations (rustboot-fileio)
  - Security best practices

## Procedural Macros

- [Macros Guide](../../crates/rustboot-macros/docs/guide.md) - **Complete usage guide**
  - When to use macros vs manual code
  - Detailed usage for each macro
  - Macro composition patterns
  - Troubleshooting and best practices

- [Macros Overview](../../crates/rustboot-macros/docs/overview.md) - Architecture and implementation
- [Macros README](../../crates/rustboot-macros/README.md) - Quick start

### Common Patterns

**Dependency Injection**:
```rust
#[derive(Injectable)]
struct UserService {
    repository: Arc<dyn UserRepository>,
}
```

**Input Validation**:
```rust
#[derive(Validate)]
struct CreateUser {
    #[validate(length(min = 3, max = 50))]
    username: String,
    
    #[validate(email)]
    email: String,
}
```

**Cross-Cutting Concerns**:
```rust
#[traced(level = "info")]
#[retry(max_attempts = 3)]
#[cached(ttl = 600)]
async fn expensive_operation(&self) -> Result<Data> {
    // Automatically traced, retried, and cached
}
```

## Rust Development Patterns

### Testing
- [Rust Test Organization](guide/rust-test-organization.md) - Idiomatic Rust testing
  - Unit tests (inline with `#[cfg(test)]`)
  - Integration tests (`tests/` directory)
  - Test attributes and patterns
  - Running tests with Cargo

### Project Structure
- [Rust Packaging vs Java](guide/rust-packaging-vs-java.md) - Why Rust differs from Java
  - No `src/main` and `src/test` separation
  - Crate-based vs domain-based naming
  - Conditional compilation
  - Module system differences

- [Crates, Packages, and Modules](guide/crates-packages-modules.md) - Rust's module system
  - Understanding crates vs packages
  - Module organization patterns
  - Visibility and privacy
  - Re-exports and public API design

## Code Quality

### Linting
- [Clippy Guide](guide/clippy-guide.md) - Using Clippy for code quality
  - Lint categories and what runs by default
  - Common lints in Rustboot codebase
  - Configuration and customization
  - Best practices for handling warnings
  - CI/CD integration

## Development Workflow

### Release Management
- [Release Versioning Guide](../6-deployment/guide/release-versioning.md) - Version management and release process
  - Semantic Versioning (SemVer) principles
  - Version bump decision matrix
  - Release workflow and checklist
  - Git tagging and branch strategies
  - CHANGELOG management
  - Breaking change policies
  - Rust-specific versioning (Cargo.toml)

### Contributing
[Add guides for:]
- Code review process
- CI/CD pipeline

## Crate-Specific Development

For crate-specific implementation details, see:
- [Crate Overviews](../overview.md#crate-documentation) - Links to all `crates/*/docs/overview.md`

---

**Related**: [Architecture Documentation](../3-design/architecture.md) - Design decisions and architecture
