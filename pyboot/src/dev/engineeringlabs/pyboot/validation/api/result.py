"""Validation result models."""

from dataclasses import dataclass, field
from typing import Any


@dataclass(frozen=True, slots=True)
class FieldError:
    """An error for a specific field."""

    field: str
    message: str
    value: Any = None
    code: str | None = None

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        result: dict[str, Any] = {
            "field": self.field,
            "message": self.message,
        }
        if self.code:
            result["code"] = self.code
        return result


@dataclass(slots=True)
class ValidationResult:
    """Result of validation."""

    errors: list[FieldError] = field(default_factory=list)

    @property
    def is_valid(self) -> bool:
        """Check if validation passed."""
        return len(self.errors) == 0

    @property
    def is_invalid(self) -> bool:
        """Check if validation failed."""
        return len(self.errors) > 0

    def add_error(
        self,
        field: str,
        message: str,
        value: Any = None,
        code: str | None = None,
    ) -> None:
        """Add a field error."""
        self.errors.append(FieldError(field, message, value, code))

    def get_errors(self, field: str) -> list[FieldError]:
        """Get all errors for a field."""
        return [e for e in self.errors if e.field == field]

    def get_first_error(self, field: str) -> FieldError | None:
        """Get the first error for a field."""
        errors = self.get_errors(field)
        return errors[0] if errors else None

    def error_messages(self) -> dict[str, list[str]]:
        """Get error messages grouped by field."""
        result: dict[str, list[str]] = {}
        for error in self.errors:
            if error.field not in result:
                result[error.field] = []
            result[error.field].append(error.message)
        return result

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "valid": self.is_valid,
            "errors": [e.to_dict() for e in self.errors],
        }

    def raise_if_invalid(self) -> None:
        """Raise ValidationError if invalid."""
        from dev.engineeringlabs.pyboot.validation.api.exceptions import ValidationError

        if self.is_invalid:
            raise ValidationError(
                "Validation failed",
                errors=self.errors,
            )


__all__ = ["FieldError", "ValidationResult"]
