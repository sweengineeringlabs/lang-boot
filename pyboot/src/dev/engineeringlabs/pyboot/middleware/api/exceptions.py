"""Middleware exceptions - Standalone error types."""


class MiddlewareError(Exception):
    """Base exception for middleware errors."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


class MiddlewareChainError(MiddlewareError):
    """Exception when middleware chain fails."""
    
    def __init__(self, middleware_name: str, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(f"Middleware '{middleware_name}' failed: {message}", cause=cause)
        self.middleware_name = middleware_name


__all__ = ["MiddlewareError", "MiddlewareChainError"]
