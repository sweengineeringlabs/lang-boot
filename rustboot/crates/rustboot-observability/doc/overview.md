# Observability Overview

## WHAT: Logging, Metrics, and Tracing

Unified observability for monitoring applications.

## WHY: Production Debugging

**Problems**: No visibility, hard to debug, missing metrics

## HOW: Structured Logging

```rust
use dev_engineeringlabs_rustboot_observability::*;

log::info!("User logged in", user_id = user.id);
metric_counter!("logins", 1);
```

**Status**: Stable | **Backlog**: See [backlog.md](../backlog.md)


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [observability_basic.rs](../examples/observability_basic.rs) - Basic usage demonstration
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
cargo test -p dev-engineeringlabs-rustboot-observability
cargo run --example observability_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)