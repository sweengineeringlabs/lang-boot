"""Error Core - Result monad and error utilities."""

from typing import TypeVar, Generic, Callable, Any
from dataclasses import dataclass


T = TypeVar("T")
E = TypeVar("E")
U = TypeVar("U")


class Result(Generic[T, E]):
    """Result monad for functional error handling.
    
    A Result is either Ok(value) or Err(error).
    
    Example:
        def divide(a: int, b: int) -> Result[float, str]:
            if b == 0:
                return Err("Division by zero")
            return Ok(a / b)
        
        result = divide(10, 2)
        if result.is_ok:
            print(f"Result: {result.unwrap()}")
        else:
            print(f"Error: {result.unwrap_err()}")
    """
    
    __slots__ = ("_value", "_error", "_is_ok")
    
    def __init__(self, value: T | None = None, error: E | None = None, is_ok: bool = True) -> None:
        self._value = value
        self._error = error
        self._is_ok = is_ok
    
    @property
    def is_ok(self) -> bool:
        """Check if result is Ok."""
        return self._is_ok
    
    @property
    def is_err(self) -> bool:
        """Check if result is Err."""
        return not self._is_ok
    
    def unwrap(self) -> T:
        """Get the value, raise if Err."""
        if not self._is_ok:
            raise ValueError(f"Called unwrap on Err: {self._error}")
        return self._value  # type: ignore
    
    def unwrap_or(self, default: T) -> T:
        """Get the value or return default."""
        return self._value if self._is_ok else default  # type: ignore
    
    def unwrap_err(self) -> E:
        """Get the error, raise if Ok."""
        if self._is_ok:
            raise ValueError(f"Called unwrap_err on Ok: {self._value}")
        return self._error  # type: ignore
    
    def map(self, func: Callable[[T], U]) -> "Result[U, E]":
        """Map the Ok value."""
        if self._is_ok:
            return Ok(func(self._value))  # type: ignore
        return Err(self._error)  # type: ignore
    
    def map_err(self, func: Callable[[E], U]) -> "Result[T, U]":
        """Map the Err value."""
        if self._is_ok:
            return Ok(self._value)  # type: ignore
        return Err(func(self._error))  # type: ignore
    
    def and_then(self, func: Callable[[T], "Result[U, E]"]) -> "Result[U, E]":
        """Chain Result-returning functions."""
        if self._is_ok:
            return func(self._value)  # type: ignore
        return Err(self._error)  # type: ignore
    
    def or_else(self, func: Callable[[E], "Result[T, U]"]) -> "Result[T, U]":
        """Handle Err with a fallback."""
        if self._is_ok:
            return Ok(self._value)  # type: ignore
        return func(self._error)  # type: ignore


def Ok(value: T) -> Result[T, Any]:
    """Create an Ok result."""
    return Result(value=value, is_ok=True)


def Err(error: E) -> Result[Any, E]:
    """Create an Err result."""
    return Result(error=error, is_ok=False)


def chain_errors(*errors: Exception) -> Exception:
    """Chain multiple exceptions together.
    
    Returns the last error with __cause__ chain.
    """
    if not errors:
        raise ValueError("No errors to chain")
    
    result = errors[-1]
    for i in range(len(errors) - 2, -1, -1):
        errors[i].__cause__ = result
        result = errors[i]
    
    return result


def wrap_error(original: Exception, wrapper_type: type[Exception], message: str) -> Exception:
    """Wrap an exception in another exception type.
    
    Example:
        try:
            do_something()
        except IOError as e:
            raise wrap_error(e, ServiceError, "Operation failed")
    """
    wrapped = wrapper_type(message)
    wrapped.__cause__ = original
    return wrapped


__all__ = [
    "Result",
    "Ok",
    "Err",
    "chain_errors",
    "wrap_error",
]
