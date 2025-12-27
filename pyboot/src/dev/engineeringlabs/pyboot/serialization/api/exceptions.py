"""Serialization exceptions - Standalone error types."""


class SerializationError(Exception):
    """Base exception for serialization errors."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


class EncodingError(SerializationError):
    """Exception when encoding fails."""
    
    def __init__(self, message: str, *, format: str | None = None, cause: Exception | None = None) -> None:
        super().__init__(message, cause=cause)
        self.format = format


class DecodingError(SerializationError):
    """Exception when decoding fails."""
    
    def __init__(self, message: str, *, format: str | None = None, cause: Exception | None = None) -> None:
        super().__init__(message, cause=cause)
        self.format = format


__all__ = ["SerializationError", "EncodingError", "DecodingError"]
