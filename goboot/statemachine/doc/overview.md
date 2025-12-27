# State Machine Module Overview

## WHAT: Finite State Machines

Type-safe FSM with guards and actions.

Key capabilities:
- **States** - Enum-based states
- **Transitions** - Guarded changes
- **Actions** - Entry/exit/transition
- **History** - State history

## WHY: Complex Workflows

**Problems Solved**: State spaghetti, invalid states

**When to Use**: Order workflows, approvals

## HOW: Usage Guide

```go
fsm := statemachine.New[OrderState, OrderEvent]()

fsm.Configure(Created).
    Permit(Pay, Paid).
    OnEntry(func(ctx *statemachine.Context) {
        log.Println("Order created")
    })

fsm.Configure(Paid).
    Permit(Ship, Shipped).
    PermitIf(Cancel, Cancelled, hasNoShipment)

fsm.Fire(Pay)
```

---

**Status**: Stable
