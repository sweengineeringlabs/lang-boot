"""
Test assertions - Fluent assertion helpers.
"""

from typing import Any, TypeVar, Generic, Callable
import re

T = TypeVar("T")


class AssertionBuilder(Generic[T]):
    """Fluent assertion builder for readable test assertions.
    
    Example:
        assert_that(user.name).equals("Alice")
        assert_that(items).has_length(3)
        assert_that(result).is_not_none().contains("success")
    """
    
    def __init__(self, value: T, description: str = "") -> None:
        self._value = value
        self._description = description
        self._negated = False
    
    @property
    def not_(self) -> "AssertionBuilder[T]":
        """Negate the next assertion."""
        self._negated = True
        return self
    
    def _check(self, condition: bool, message: str) -> "AssertionBuilder[T]":
        """Check condition and raise if failed."""
        if self._negated:
            condition = not condition
            message = f"NOT {message}"
        self._negated = False
        
        if not condition:
            desc = f" ({self._description})" if self._description else ""
            raise AssertionError(f"{message}{desc}")
        return self
    
    def equals(self, expected: Any) -> "AssertionBuilder[T]":
        """Assert value equals expected."""
        return self._check(
            self._value == expected,
            f"Expected {expected!r}, got {self._value!r}"
        )
    
    def is_none(self) -> "AssertionBuilder[T]":
        """Assert value is None."""
        return self._check(self._value is None, f"Expected None, got {self._value!r}")
    
    def is_not_none(self) -> "AssertionBuilder[T]":
        """Assert value is not None."""
        return self._check(self._value is not None, "Expected non-None value")
    
    def is_true(self) -> "AssertionBuilder[T]":
        """Assert value is True."""
        return self._check(self._value is True, f"Expected True, got {self._value!r}")
    
    def is_false(self) -> "AssertionBuilder[T]":
        """Assert value is False."""
        return self._check(self._value is False, f"Expected False, got {self._value!r}")
    
    def is_instance_of(self, cls: type) -> "AssertionBuilder[T]":
        """Assert value is instance of type."""
        return self._check(
            isinstance(self._value, cls),
            f"Expected instance of {cls.__name__}, got {type(self._value).__name__}"
        )
    
    def has_length(self, expected: int) -> "AssertionBuilder[T]":
        """Assert collection has expected length."""
        actual = len(self._value)  # type: ignore
        return self._check(
            actual == expected,
            f"Expected length {expected}, got {actual}"
        )
    
    def contains(self, item: Any) -> "AssertionBuilder[T]":
        """Assert collection contains item."""
        return self._check(
            item in self._value,  # type: ignore
            f"Expected {self._value!r} to contain {item!r}"
        )
    
    def matches(self, pattern: str) -> "AssertionBuilder[T]":
        """Assert string matches regex pattern."""
        return self._check(
            bool(re.search(pattern, str(self._value))),
            f"Expected {self._value!r} to match pattern {pattern!r}"
        )
    
    def is_greater_than(self, other: Any) -> "AssertionBuilder[T]":
        """Assert value is greater than other."""
        return self._check(
            self._value > other,  # type: ignore
            f"Expected {self._value!r} > {other!r}"
        )
    
    def is_less_than(self, other: Any) -> "AssertionBuilder[T]":
        """Assert value is less than other."""
        return self._check(
            self._value < other,  # type: ignore
            f"Expected {self._value!r} < {other!r}"
        )
    
    def raises(self, exception_type: type[Exception]) -> "AssertionBuilder[T]":
        """Assert callable raises exception."""
        if not callable(self._value):
            raise TypeError("Value must be callable for raises()")
        
        try:
            self._value()  # type: ignore
            return self._check(False, f"Expected {exception_type.__name__} to be raised")
        except exception_type:
            return self._check(True, "")
        except Exception as e:
            return self._check(
                False,
                f"Expected {exception_type.__name__}, got {type(e).__name__}"
            )
    
    def satisfies(self, predicate: Callable[[T], bool]) -> "AssertionBuilder[T]":
        """Assert value satisfies a predicate."""
        return self._check(
            predicate(self._value),
            f"Value {self._value!r} did not satisfy predicate"
        )


def assert_that(value: T, description: str = "") -> AssertionBuilder[T]:
    """Create a fluent assertion builder."""
    return AssertionBuilder(value, description)


def expect(value: T, description: str = "") -> AssertionBuilder[T]:
    """Alias for assert_that for BDD-style tests."""
    return AssertionBuilder(value, description)
