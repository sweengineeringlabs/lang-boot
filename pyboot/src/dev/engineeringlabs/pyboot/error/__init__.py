"""
Error Module - Common error types and utilities.

Provides shared error patterns:
- Base exception types
- Error codes
- Result monad
- Error chaining utilities
"""

from dev.engineeringlabs.pyboot.error.api import (
    ErrorCode,
    PybootError,
)

from dev.engineeringlabs.pyboot.error.core import (
    Result,
    Ok,
    Err,
    chain_errors,
    wrap_error,
)

__all__ = [
    # API
    "ErrorCode",
    "PybootError",
    # Core
    "Result",
    "Ok",
    "Err",
    "chain_errors",
    "wrap_error",
]
