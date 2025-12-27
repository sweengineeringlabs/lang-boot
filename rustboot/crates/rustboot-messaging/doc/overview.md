# Messaging Overview

## WHAT: Message Queue Abstraction

Pub/sub messaging patterns for async communication.

## WHY: Decouple Services

**Problems**: Tight coupling, synchronous calls, scalability limits

## HOW: Pub/Sub

```rust
use dev_engineeringlabs_rustboot_messaging::*;

let queue = MessageQueue::new();
queue.publish("events", message)?;
queue.subscribe("events", handler).await?;
```

**Status**: Planned | **Backlog**: See [backlog.md](../backlog.md)


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [messaging_basic.rs](../examples/messaging_basic.rs) - Basic usage demonstration
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
cargo test -p dev-engineeringlabs-rustboot-messaging
cargo run --example messaging_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)