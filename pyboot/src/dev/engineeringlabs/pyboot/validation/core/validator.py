"""Core validation functions."""

from typing import Any, Sequence

from dev.engineeringlabs.pyboot.validation.api.validator import Validator
from dev.engineeringlabs.pyboot.validation.api.result import ValidationResult


def validate(
    *field_validations: tuple[str, Any, Sequence[Validator]],
) -> ValidationResult:
    """
    Validate multiple fields at once.

    Args:
        *field_validations: Tuples of (field_name, value, validators)

    Returns:
        ValidationResult with any errors

    Example:
        result = validate(
            ("email", user_email, [required(), email()]),
            ("password", user_password, [required(), min_length(8)]),
            ("age", user_age, [min_value(18)]),
        )

        if result.is_invalid:
            for error in result.errors:
                print(f"{error.field}: {error.message}")
    """
    result = ValidationResult()

    for field_name, value, validators in field_validations:
        for validator in validators:
            error_message = validator.validate(value)
            if error_message:
                result.add_error(field_name, error_message, value)
                break  # Stop on first error for this field

    return result


def validate_dict(
    data: dict[str, Any],
    schema: dict[str, Sequence[Validator]],
) -> ValidationResult:
    """
    Validate a dictionary against a schema.

    Args:
        data: Dictionary to validate
        schema: Mapping of field names to validators

    Returns:
        ValidationResult with any errors

    Example:
        schema = {
            "email": [required(), email()],
            "password": [required(), min_length(8)],
            "name": [required(), max_length(100)],
        }

        result = validate_dict(user_data, schema)
        result.raise_if_invalid()
    """
    result = ValidationResult()

    for field_name, validators in schema.items():
        value = data.get(field_name)
        for validator in validators:
            error_message = validator.validate(value)
            if error_message:
                result.add_error(field_name, error_message, value)
                break

    return result


def validate_all(
    *field_validations: tuple[str, Any, Sequence[Validator]],
) -> ValidationResult:
    """
    Validate fields, collecting ALL errors (not just first per field).

    Args:
        *field_validations: Tuples of (field_name, value, validators)

    Returns:
        ValidationResult with all errors
    """
    result = ValidationResult()

    for field_name, value, validators in field_validations:
        for validator in validators:
            error_message = validator.validate(value)
            if error_message:
                result.add_error(field_name, error_message, value)

    return result


__all__ = ["validate", "validate_dict", "validate_all"]
