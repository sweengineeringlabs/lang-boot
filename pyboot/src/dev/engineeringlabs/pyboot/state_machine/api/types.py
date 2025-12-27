"""State machine types."""

from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Callable, TypeVar, Generic

S = TypeVar("S", bound=Enum)


class State(Enum):
    """Base class for state enums. Extend this for your states."""
    pass


@dataclass
class Event:
    """Event that triggers a transition."""
    name: str
    payload: dict[str, Any] = field(default_factory=dict)


Guard = Callable[[Any, "Event"], bool]
"""Guard function: (context, event) -> bool. Returns True if transition allowed."""

Action = Callable[[Any, "Event"], None]
"""Action function: (context, event) -> None. Executed on transition."""


@dataclass
class Transition(Generic[S]):
    """State transition definition.
    
    Attributes:
        from_state: Source state.
        to_state: Target state.
        event: Event name that triggers this transition.
        guard: Optional guard function.
        action: Optional action to execute on transition.
    """
    from_state: S
    to_state: S
    event: str
    guard: Guard | None = None
    action: Action | None = None
    
    def can_transition(self, context: Any, evt: Event) -> bool:
        """Check if transition is allowed."""
        if self.guard is None:
            return True
        return self.guard(context, evt)
    
    def execute(self, context: Any, evt: Event) -> None:
        """Execute transition action."""
        if self.action:
            self.action(context, evt)


@dataclass
class StateMachineConfig(Generic[S]):
    """State machine configuration.
    
    Attributes:
        initial: Initial state.
        transitions: List of allowed transitions.
        on_enter: Callbacks when entering a state.
        on_exit: Callbacks when exiting a state.
    """
    initial: S
    transitions: list[Transition[S]] = field(default_factory=list)
    on_enter: dict[S, list[Action]] = field(default_factory=dict)
    on_exit: dict[S, list[Action]] = field(default_factory=dict)
    
    def add_transition(
        self,
        from_state: S,
        to_state: S,
        event: str,
        guard: Guard | None = None,
        action: Action | None = None,
    ) -> "StateMachineConfig[S]":
        """Add a transition."""
        self.transitions.append(Transition(from_state, to_state, event, guard, action))
        return self
    
    def on_state_enter(self, state: S, action: Action) -> "StateMachineConfig[S]":
        """Add callback for entering a state."""
        if state not in self.on_enter:
            self.on_enter[state] = []
        self.on_enter[state].append(action)
        return self
    
    def on_state_exit(self, state: S, action: Action) -> "StateMachineConfig[S]":
        """Add callback for exiting a state."""
        if state not in self.on_exit:
            self.on_exit[state] = []
        self.on_exit[state].append(action)
        return self
