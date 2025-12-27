"""Stream exceptions - Standalone error types for stream operations."""


class StreamError(Exception):
    """Base exception for stream errors."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


class BackpressureError(StreamError):
    """Exception when buffer is full."""
    
    def __init__(self, buffer_size: int) -> None:
        super().__init__(f"Buffer full (size: {buffer_size}). Apply backpressure.")
        self.buffer_size = buffer_size


__all__ = ["StreamError", "BackpressureError"]
