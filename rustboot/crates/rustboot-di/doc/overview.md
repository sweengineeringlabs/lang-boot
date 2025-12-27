# DI Overview

## WHAT: Dependency Injection Container

Manage dependencies and lifetimes in Rust applications.

## WHY: Decouple Components

**Problems**: Tight coupling, hard-to-test singletons, manual wiring

## HOW: Container Registration

```rust
use dev_engineeringlabs_rustboot_di::*;

let container = Container::new();
container.register(MyService::new());
let service = container.resolve::<MyService>()?;
```

**Status**: Stable | **Backlog**: See [backlog.md](../backlog.md)


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [di_basic.rs](../examples/di_basic.rs) - Basic usage demonstration
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
cargo test -p dev-engineeringlabs-rustboot-di
cargo run --example di_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)