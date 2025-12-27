"""Error API - Error types and codes."""

from enum import Enum, auto
from typing import Any


class ErrorCode(Enum):
    """Standard error codes for programmatic handling."""
    
    # Generic
    UNKNOWN = auto()
    INTERNAL = auto()
    
    # Validation
    VALIDATION = auto()
    INVALID_INPUT = auto()
    MISSING_REQUIRED = auto()
    
    # Resources
    NOT_FOUND = auto()
    ALREADY_EXISTS = auto()
    CONFLICT = auto()
    
    # Access
    UNAUTHORIZED = auto()
    FORBIDDEN = auto()
    PERMISSION_DENIED = auto()
    
    # Operations
    TIMEOUT = auto()
    CANCELLED = auto()
    UNAVAILABLE = auto()
    
    # Configuration
    CONFIGURATION = auto()
    
    # External
    EXTERNAL_SERVICE = auto()
    NETWORK = auto()


class PybootError(Exception):
    """Base exception for Pyboot framework errors.
    
    All framework exceptions can extend this class for consistency.
    
    Attributes:
        message: Human-readable error description
        code: Error code for programmatic handling
        cause: Original exception that caused this error
        details: Additional context as key-value pairs
    """
    
    def __init__(
        self,
        message: str,
        code: ErrorCode = ErrorCode.UNKNOWN,
        *,
        cause: Exception | None = None,
        details: dict[str, Any] | None = None,
    ) -> None:
        super().__init__(message)
        self.message = message
        self.code = code
        self.cause = cause
        self.details = details or {}
    
    def __str__(self) -> str:
        return f"[{self.code.name}] {self.message}"
    
    def __repr__(self) -> str:
        return f"{self.__class__.__name__}(message={self.message!r}, code={self.code.name})"
    
    def to_dict(self) -> dict[str, Any]:
        """Convert error to dictionary for serialization."""
        result: dict[str, Any] = {
            "code": self.code.name,
            "message": self.message,
        }
        if self.details:
            result["details"] = self.details
        if self.cause:
            result["cause"] = str(self.cause)
        return result


__all__ = [
    "ErrorCode",
    "PybootError",
]
