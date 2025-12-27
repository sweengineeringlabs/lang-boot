"""Parsing API - Parse types and errors."""

from typing import TypeVar, Generic


T = TypeVar("T")


class ParseError(Exception):
    """Base error for parsing operations."""
    
    def __init__(
        self,
        message: str,
        *,
        line: int | None = None,
        column: int | None = None,
        cause: Exception | None = None,
    ) -> None:
        super().__init__(message)
        self.message = message
        self.line = line
        self.column = column
        self.cause = cause


class ParseResult(Generic[T]):
    """Result of a parse operation."""
    
    def __init__(self, value: T | None = None, error: ParseError | None = None) -> None:
        self._value = value
        self._error = error
    
    @property
    def is_ok(self) -> bool:
        return self._error is None
    
    @property
    def is_err(self) -> bool:
        return self._error is not None
    
    def unwrap(self) -> T:
        if self._error:
            raise self._error
        return self._value  # type: ignore
    
    def unwrap_or(self, default: T) -> T:
        if self._error:
            return default
        return self._value  # type: ignore
    
    def unwrap_err(self) -> ParseError:
        if self._error is None:
            raise ValueError("Called unwrap_err on Ok result")
        return self._error


__all__ = [
    "ParseError",
    "ParseResult",
]
