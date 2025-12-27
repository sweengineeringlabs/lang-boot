# Rustboot State Machine Examples

This document describes the example programs available for the rustboot-state-machine crate.

## Running the Examples

Once the workspace dependencies are resolved, you can run the examples using:

```bash
# Run from the workspace root
cargo run --example state-machine_basic -p dev-engineeringlabs-rustboot-state-machine
cargo run --example order_processing -p dev-engineeringlabs-rustboot-state-machine
cargo run --example traffic_light -p dev-engineeringlabs-rustboot-state-machine
```

## Available Examples

### 1. Basic Example (`state-machine_basic.rs`)

**Purpose**: Introduces the fundamental concepts of the state machine library.

**Key Concepts Demonstrated**:
- Defining custom state and event enums
- Creating a state machine with initial state
- Adding transitions between states
- Triggering events to change states
- Handling successful and failed transitions
- Using guard conditions to validate transitions
- Checking if a transition is possible without executing it

**Use Case**: A simple media player with states (Idle, Playing, Paused, Stopped)

**Best For**: First-time users learning the API basics

---

### 2. Order Processing Example (`order_processing.rs`)

**Purpose**: Demonstrates a comprehensive, production-ready state machine implementation for an e-commerce order processing system.

**Key Concepts Demonstrated**:
- Complex state graph with multiple transition paths
- Managing context/business data alongside state transitions
- Entry and exit actions for states
- Event history tracking
- Multiple workflow scenarios (success path, error path, cancellation, returns)
- Guard conditions based on business logic
- State machine encapsulation in a higher-level processor
- Detailed status reporting

**Use Case**: E-commerce order lifecycle management

**States**:
- Draft → Pending → PaymentReceived → Confirmed → Processing → Shipped → Delivered
- Alternative paths: Cancellation, Returns, Refunds

**Best For**:
- Understanding real-world state machine patterns
- Learning how to integrate state machines with business logic
- Seeing how to handle complex workflows

**Scenarios Covered**:
1. Successful order flow (draft to delivery)
2. Invalid transition attempts
3. Return and refund flow
4. Order cancellation at different stages
5. Payment rejection and retry

---

### 3. Traffic Light Example (`traffic_light.rs`)

**Purpose**: Shows how to build a time-based cyclic state machine with safety features.

**Key Concepts Demonstrated**:
- Cyclic state transitions (repeating pattern)
- Time tracking within states
- Emergency interrupt transitions
- Resume from safe state
- Integration with external context (vehicle sensors)
- Safety guards
- Invalid transition handling

**Use Case**: Traffic light controller system

**States**: Red → RedYellow → Green → Yellow → Red (cycle repeats)

**Special Features**:
- Emergency mode that forces immediate transition to Red
- Vehicle detection and counting
- Time-in-state tracking
- Safety guards to ensure minimum duration in each state

**Best For**:
- Learning cyclic state machines
- Understanding emergency/interrupt patterns
- Time-based state transitions
- Safety-critical systems

---

## API Overview

Based on these examples, here's a quick reference of the main API:

### Creating a State Machine

```rust
let mut sm = StateMachine::new(InitialState);
```

### Adding Transitions

```rust
sm.add_transition(from_state, event, to_state);
```

### Adding Guards

```rust
sm.add_guard(from_state, event, |current, next| {
    // Return true to allow, false to reject
    true
});
```

### Triggering Events

```rust
match sm.trigger(event) {
    Ok(new_state) => { /* transition successful */ }
    Err(e) => { /* transition failed */ }
}
```

### Checking State

```rust
let current = sm.current_state();
let can_transition = sm.can_trigger(&event);
```

## Common Patterns

### Pattern 1: Simple Linear Flow

```rust
Draft → Pending → Confirmed → Complete
```

Good for: Approval workflows, simple pipelines

### Pattern 2: Cyclic Flow

```rust
State1 → State2 → State3 → State1 (repeats)
```

Good for: Traffic lights, recurring processes, game loops

### Pattern 3: Star Pattern (Emergency/Cancel from any state)

```rust
AnyState --[Cancel]--> CancelledState
AnyState --[Emergency]--> SafeState
```

Good for: Safety systems, user cancellation

### Pattern 4: Diamond/Branching

```rust
        ┌─→ Path1 ─→┐
Start ─→┤           ├─→ End
        └─→ Path2 ─→┘
```

Good for: Conditional workflows, alternate paths

## Design Tips

1. **Keep States Simple**: Each state should represent a clear, distinct condition
2. **Use Guards for Validation**: Don't encode business logic in state structure
3. **Track Context Separately**: Use a wrapper struct to manage business data alongside the state machine
4. **Entry/Exit Actions**: Handle side effects when entering or exiting states
5. **Event History**: Keep a log of transitions for debugging and audit trails
6. **Error Handling**: Always handle Result from trigger() - invalid transitions are errors

## Error Types

- `InvalidTransition`: No transition defined for current state + event
- `GuardRejected`: Transition exists but guard condition returned false

## Performance Considerations

- State and Event types must implement `Clone`, `Eq`, `Hash`, `Debug`
- Transitions are stored in a HashMap for O(1) lookup
- Guards are stored as boxed closures (small overhead)
- State cloning occurs on each transition

## Thread Safety

- StateMachine requires `Send + Sync` on guard closures
- Wrap in `Arc<Mutex<StateMachine>>` for multi-threaded access
- Consider using channels to send events from multiple threads
