# Middleware Overview

## WHAT: Middleware Pipeline Pattern

Chain request/response processing middleware.

## WHY: Modular Request Processing

**Problems**: Scattered cross-cutting concerns, duplicate code

## HOW: Middleware Chain

```rust
use dev_engineeringlabs_rustboot_middleware::*;

let pipeline = Pipeline::new()
    .use_middleware(LoggingMiddleware)
    .use_middleware(AuthMiddleware);
```

**Status**: Stable | **Backlog**: See [backlog.md](../backlog.md)


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [middleware_basic.rs](../examples/middleware_basic.rs) - Basic usage demonstration
- See directory for additional examples

**Purpose**: Show users HOW to use this module in real applications.

### Tests

**Location**: [	ests/](../tests/) directory

**Current tests**:
- [integration.rs](../tests/integration.rs) - Integration tests using public API

**Purpose**: Show users HOW to test code that uses this module.

### Testing Guidance

**For developers using this module**: See [Rust Test Organization](../../docs/4-development/guide/rust-test-organization.md)

**For contributors**: Run tests with:
```bash
cargo test -p dev-engineeringlabs-rustboot-middleware
cargo run --example middleware_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)