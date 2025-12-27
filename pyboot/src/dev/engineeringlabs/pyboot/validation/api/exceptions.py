"""Validation exceptions."""

from typing import Any, TYPE_CHECKING

if TYPE_CHECKING:
    from dev.engineeringlabs.pyboot.validation.api.result import FieldError


class ValidationError(Exception):
    """Raised when validation fails."""

    def __init__(
        self,
        message: str = "Validation failed",
        errors: list["FieldError"] | None = None,
    ) -> None:
        super().__init__(message)
        self.message = message
        self.errors = errors or []

    def __str__(self) -> str:
        if self.errors:
            error_msgs = [f"{e.field}: {e.message}" for e in self.errors[:3]]
            if len(self.errors) > 3:
                error_msgs.append(f"... and {len(self.errors) - 3} more")
            return f"{self.message}: {', '.join(error_msgs)}"
        return self.message

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "message": self.message,
            "errors": [e.to_dict() for e in self.errors],
        }

    def get_field_errors(self, field: str) -> list["FieldError"]:
        """Get errors for a specific field."""
        return [e for e in self.errors if e.field == field]


__all__ = ["ValidationError"]
