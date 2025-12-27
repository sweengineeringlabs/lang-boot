# Validation Overview

## WHAT: Type-Safe Input Validation

The `rustboot-validation` crate provides a fluent, type-safe validation framework for Rust applications.

Key features:
- **Fluent builders** - Chainable validation rules
- **Type-safe** - Compile-time validation logic
- **Composable** - Combine validators
- **Extensible** - Custom validation rules

## WHY: Prevent Invalid Data and Security Issues

**Problems Solved**:
1. Manual validation code scattered across codebase
2. Inconsistent error messages
3. Security vulnerabilities (XSS, SQL injection via invalid input)
4. Runtime errors from invalid data

**When to Use**: Any application accepting user input, API parameters, configuration values, or untrusted data.

## HOW: Fluent Validation Builders

### Basic Example

```rust
use dev_engineeringlabs_rustboot_validation::*;

// Email validation
let validator = StringValidationBuilder::new("email")
    .not_empty()
    .email()
    .build();

validator.validate(user_input)?;
```

### String Validation

```rust
let validator = StringValidationBuilder::new("username")
    .not_empty()
    .min_length(3)
    .max_length(50)
    .matches(r"^[a-zA-Z0-9_]+$")
    .build();
```

**Available**:
- `not_empty()` - Reject empty strings
- `min_length()`, `max_length()` - Length constraints
- `matches()` - Regex pattern matching
- `email()` - Email format validation

**Planned**: See [backlog.md](../backlog.md)

## Relationship to Other Crates

| Crate | Focus | Relationship |
|-------|-------|--------------|
| `rustboot-http` | HTTP handling | Validates request parameters |
| `rustboot-config` | Configuration | Validates config values |
| `rustboot-security` | Security | Used for auth input validation |

## Examples and Tests

**Examples**: See [`examples/`](../examples/) directory
- [`user_registration.rs`](../examples/user_registration.rs) - Complete user registration validation
- [`api_validation.rs`](../examples/api_validation.rs) - REST API request validation

**Tests**: See [`tests/`](../tests/) directory  
- [`integration.rs`](../tests/integration.rs) - Integration tests with real-world scenarios

**For testing guidance**: See [Rust Test Organization Guide](../../docs/4-development/guide/rust-test-organization.md)

---

**Status**: Stable  
**Backlog**: See [backlog.md](../backlog.md)
