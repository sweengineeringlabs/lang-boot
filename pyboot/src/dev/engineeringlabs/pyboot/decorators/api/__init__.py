"""Decorators API - Decorator types and errors."""


class DecoratorError(Exception):
    """Base error for decorator operations."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


__all__ = [
    "DecoratorError",
]
