"""Session exceptions - Standalone error types."""


class SessionError(Exception):
    """Base exception for session errors."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


class SessionNotFoundError(SessionError):
    """Exception when session is not found."""
    
    def __init__(self, session_id: str) -> None:
        super().__init__(f"Session not found: {session_id}")
        self.session_id = session_id


class SessionExpiredError(SessionError):
    """Exception when session has expired."""
    
    def __init__(self, session_id: str) -> None:
        super().__init__(f"Session expired: {session_id}")
        self.session_id = session_id


__all__ = ["SessionError", "SessionNotFoundError", "SessionExpiredError"]
