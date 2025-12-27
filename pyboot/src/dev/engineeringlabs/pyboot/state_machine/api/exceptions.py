"""State machine exceptions - Standalone error types."""


class StateMachineError(Exception):
    """Base exception for state machine errors."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


class InvalidTransitionError(StateMachineError):
    """Exception for invalid state transitions."""
    
    def __init__(self, from_state: str, to_state: str, event: str | None = None) -> None:
        msg = f"Invalid transition from '{from_state}' to '{to_state}'"
        if event:
            msg += f" via event '{event}'"
        super().__init__(msg)
        self.from_state = from_state
        self.to_state = to_state
        self.event = event


class GuardFailedError(StateMachineError):
    """Exception when a transition guard fails."""
    
    def __init__(self, guard_name: str, transition: str) -> None:
        super().__init__(f"Guard '{guard_name}' failed for transition: {transition}")
        self.guard_name = guard_name
        self.transition = transition


__all__ = ["StateMachineError", "InvalidTransitionError", "GuardFailedError"]
