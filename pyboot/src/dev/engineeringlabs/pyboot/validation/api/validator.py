"""Validator interface."""

from abc import ABC, abstractmethod
from typing import Any


class Validator(ABC):
    """
    Abstract validator interface.

    Example:
        class EmailValidator(Validator):
            def validate(self, value: Any) -> str | None:
                if not value or "@" not in str(value):
                    return "Invalid email format"
                return None
    """

    @abstractmethod
    def validate(self, value: Any) -> str | None:
        """
        Validate a value.

        Args:
            value: Value to validate

        Returns:
            Error message if invalid, None if valid
        """
        ...

    def __call__(self, value: Any) -> str | None:
        """Allow validators to be called directly."""
        return self.validate(value)


__all__ = ["Validator"]
