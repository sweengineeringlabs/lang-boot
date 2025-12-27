# UUID Overview

## WHAT: Unique Identifiers

**V4**: Random, cryptographically secure | **V7**: Time-based, sortable (recommended)

## WHY: Global Uniqueness

**Use Cases**: Database IDs, distributed systems, API keys | **Benefits**: No coordination needed

## HOW: Choose Version

```rust
// V4 (random)
let id = new_v4(); // For general use

// V7 (time-based, NEW!)
let id = new_v7(); // For sortable IDs (databases)
```

**Tip**: Use V7 for database primary keys (better index performance).


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [uuid_basic.rs](../examples/uuid_basic.rs) - Basic usage demonstration
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
cargo test -p dev-engineeringlabs-rustboot-uuid
cargo run --example uuid_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)