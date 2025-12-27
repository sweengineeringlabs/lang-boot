"""State machine implementation."""

from typing import Any, TypeVar, Generic
from enum import Enum
from dataclasses import dataclass, field
from datetime import datetime

from dev.engineeringlabs.pyboot.state_machine.api.types import (
    State,
    Transition,
    Event,
    StateMachineConfig,
)
from dev.engineeringlabs.pyboot.state_machine.api.exceptions import (
    InvalidTransitionError,
    GuardFailedError,
)

S = TypeVar("S", bound=Enum)


@dataclass
class StateContext(Generic[S]):
    """Runtime context for a state machine instance.
    
    Attributes:
        state: Current state.
        data: User data associated with this instance.
        history: List of (state, timestamp) for state history.
    """
    state: S
    data: dict[str, Any] = field(default_factory=dict)
    history: list[tuple[S, datetime]] = field(default_factory=list)
    
    def __post_init__(self) -> None:
        if not self.history:
            self.history.append((self.state, datetime.now()))
    
    def get(self, key: str, default: Any = None) -> Any:
        """Get data value."""
        return self.data.get(key, default)
    
    def set(self, key: str, value: Any) -> None:
        """Set data value."""
        self.data[key] = value


class StateMachine(Generic[S]):
    """State machine with transitions and guards.
    
    Example:
        from enum import Enum
        from dev.engineeringlabs.pyboot.state_machine import StateMachine, Transition
        
        class OrderState(Enum):
            PENDING = "pending"
            PAID = "paid"
            SHIPPED = "shipped"
        
        machine = StateMachine(
            initial=OrderState.PENDING,
            transitions=[
                Transition(OrderState.PENDING, OrderState.PAID, "pay"),
                Transition(OrderState.PAID, OrderState.SHIPPED, "ship"),
            ],
        )
        
        ctx = machine.create({"order_id": "123"})
        machine.trigger(ctx, "pay")
        print(ctx.state)  # OrderState.PAID
    """
    
    def __init__(
        self,
        initial: S,
        transitions: list[Transition[S]] | None = None,
        config: StateMachineConfig[S] | None = None,
    ) -> None:
        if config:
            self._config = config
        else:
            self._config = StateMachineConfig(
                initial=initial,
                transitions=transitions or [],
            )
        self._transitions_by_state: dict[S, list[Transition[S]]] = {}
        self._build_index()
    
    def _build_index(self) -> None:
        """Build transition index for fast lookup."""
        for t in self._config.transitions:
            if t.from_state not in self._transitions_by_state:
                self._transitions_by_state[t.from_state] = []
            self._transitions_by_state[t.from_state].append(t)
    
    @property
    def initial(self) -> S:
        """Get initial state."""
        return self._config.initial
    
    def create(self, data: dict[str, Any] | None = None) -> StateContext[S]:
        """Create a new state machine instance.
        
        Args:
            data: Initial context data.
            
        Returns:
            StateContext for this instance.
        """
        return StateContext(state=self._config.initial, data=data or {})
    
    def can_trigger(self, ctx: StateContext[S], event_name: str, payload: dict[str, Any] | None = None) -> bool:
        """Check if event can be triggered."""
        event = Event(name=event_name, payload=payload or {})
        transition = self._find_transition(ctx.state, event_name)
        
        if transition is None:
            return False
        
        return transition.can_transition(ctx, event)
    
    def trigger(
        self,
        ctx: StateContext[S],
        event_name: str,
        payload: dict[str, Any] | None = None,
    ) -> S:
        """Trigger an event and transition state.
        
        Args:
            ctx: State context.
            event_name: Event name.
            payload: Event payload.
            
        Returns:
            New state.
            
        Raises:
            InvalidTransitionError: If no valid transition exists.
            GuardFailedError: If guard condition fails.
        """
        event = Event(name=event_name, payload=payload or {})
        transition = self._find_transition(ctx.state, event_name)
        
        if transition is None:
            available = self.available_events(ctx.state)
            raise InvalidTransitionError(
                str(ctx.state.value),
                event_name,
                available,
            )
        
        if not transition.can_transition(ctx, event):
            raise GuardFailedError(
                str(transition.from_state.value),
                str(transition.to_state.value),
                event_name,
            )
        
        # Execute exit callbacks
        for action in self._config.on_exit.get(ctx.state, []):
            action(ctx, event)
        
        # Execute transition action
        transition.execute(ctx, event)
        
        # Update state
        old_state = ctx.state
        ctx.state = transition.to_state
        ctx.history.append((ctx.state, datetime.now()))
        
        # Execute enter callbacks
        for action in self._config.on_enter.get(ctx.state, []):
            action(ctx, event)
        
        return ctx.state
    
    def _find_transition(self, state: S, event_name: str) -> Transition[S] | None:
        """Find transition for state and event."""
        transitions = self._transitions_by_state.get(state, [])
        for t in transitions:
            if t.event == event_name:
                return t
        return None
    
    def available_events(self, state: S) -> list[str]:
        """Get available events for a state."""
        transitions = self._transitions_by_state.get(state, [])
        return [t.event for t in transitions]
    
    def get_states(self) -> list[S]:
        """Get all states in the machine."""
        states = set()
        for t in self._config.transitions:
            states.add(t.from_state)
            states.add(t.to_state)
        return list(states)


def create_machine(
    initial: S,
    transitions: list[Transition[S]],
) -> StateMachine[S]:
    """Factory function to create a state machine."""
    return StateMachine(initial=initial, transitions=transitions)
