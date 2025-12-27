# State Machine Module Overview

## WHAT: Finite State Machines

Type-safe FSM with guards, actions, and transition hooks.

Key capabilities:
- **States** - Enum-based state definitions
- **Transitions** - Guarded state changes
- **Actions** - Entry/exit/transition actions
- **History** - State change history

## WHY: Complex Workflows

**Problems Solved**:
1. **State Spaghetti** - Explicit state transitions
2. **Invalid States** - Guard-protected transitions
3. **Audit Trail** - State change history

**When to Use**: Order workflows, approval processes

## HOW: Usage Guide

```java
var fsm = StateMachine.<State, Event>builder()
    .initialState(State.CREATED)
    .transition(State.CREATED, Event.PAY, State.PAID)
        .guard(order -> order.hasValidPayment())
        .action(order -> order.processPayment())
    .transition(State.PAID, Event.SHIP, State.SHIPPED)
    .build();

fsm.fire(Event.PAY, order);
fsm.getCurrentState(); // PAID
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-observability | State change events |

---

**Status**: Stable
