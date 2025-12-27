"""
State Machine Module - Type-safe state transitions with guards.

This module provides:
- State machine definition
- Transitions with guards
- Event-driven state changes
- State persistence
- Transition hooks

Example:
    from dev.engineeringlabs.pyboot.state_machine import StateMachine, State, Transition
    
    # Define states
    class OrderState(State):
        PENDING = "pending"
        PAID = "paid"
        SHIPPED = "shipped"
        DELIVERED = "delivered"
        CANCELLED = "cancelled"
    
    # Define state machine
    machine = StateMachine(
        initial=OrderState.PENDING,
        transitions=[
            Transition(OrderState.PENDING, OrderState.PAID, event="pay"),
            Transition(OrderState.PAID, OrderState.SHIPPED, event="ship"),
            Transition(OrderState.SHIPPED, OrderState.DELIVERED, event="deliver"),
            Transition(OrderState.PENDING, OrderState.CANCELLED, event="cancel", guard=can_cancel),
        ],
    )
    
    # Use the machine
    order = machine.create(context={"order_id": "123"})
    order.trigger("pay")
    print(order.state)  # OrderState.PAID
"""

from dev.engineeringlabs.pyboot.state_machine.api import (
    # Core types
    State,
    Transition,
    Event,
    Guard,
    Action,
    # Machine
    StateMachineConfig,
    # Exceptions
    StateMachineError,
    InvalidTransitionError,
    GuardFailedError,
)

from dev.engineeringlabs.pyboot.state_machine.core import (
    StateMachine,
    StateContext,
    create_machine,
)

__all__ = [
    # API
    "State",
    "Transition",
    "Event",
    "Guard",
    "Action",
    "StateMachineConfig",
    "StateMachineError",
    "InvalidTransitionError",
    "GuardFailedError",
    # Core
    "StateMachine",
    "StateContext",
    "create_machine",
]
