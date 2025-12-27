# State Machine Overview

## WHAT: State Machine Implementation

Type-safe state transitions with guards.

## WHY: Model Complex State Logic

**Problems**: Complex state management, invalid transitions, bugs

## HOW: State Transitions

```rust
use dev_engineeringlabs_rustboot_state_machine::*;

let machine = StateMachine::new(InitialState);
machine.transition(Event::Start)?;
```

**Status**: Stable | **Backlog**: See [backlog.md](../backlog.md)


## Examples and Tests

> **⚠️ Required**: Every module must have working examples and tests to guide users.

### Examples

**Location**: [xamples/](../examples/) directory

**Current examples**:
- [state-machine_basic.rs](../examples/state-machine_basic.rs) - Basic usage demonstration
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
cargo test -p dev-engineeringlabs-rustboot-state-machine
cargo run --example state-machine_basic
```

---

**Status**: Stable  
**Roadmap**: See [backlog.md](../backlog.md)