# DateTime Overview

## WHAT: Time Utilities

**Timestamps**: Current time in various formats | **Formatting**: RFC3339, human-readable durations

## WHY: Consistency

**Problems**: Timezone issues, inconsistent formatting | **Solutions**: UTC timestamps, standard formats

## HOW: Common Tasks

```rust
// Logging
let timestamp = now();
log!("{} - Event occurred", format_timestamp(&timestamp));

// API responses
let created_at = now_millis();

// Human-readable
format_duration(elapsed); // "5m 23s"
```

**Best Practice**: Always use UTC, format at display time.


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [datetime_basic.rs](../examples/datetime_basic.rs) - Basic usage demonstration
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
cargo test -p dev-engineeringlabs-rustboot-datetime
cargo run --example datetime_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)