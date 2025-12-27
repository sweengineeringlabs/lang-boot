"""Validation API layer."""

from dev.engineeringlabs.pyboot.validation.api.validator import Validator
from dev.engineeringlabs.pyboot.validation.api.result import ValidationResult, FieldError
from dev.engineeringlabs.pyboot.validation.api.exceptions import ValidationError
from dev.engineeringlabs.pyboot.validation.api.validators import (
    required,
    not_empty,
    min_length,
    max_length,
    pattern,
    email,
    url,
    min_value,
    max_value,
    in_range,
    one_of,
    uuid,
    phone,
    ip_address,
)

__all__ = [
    "Validator",
    "ValidationResult",
    "FieldError",
    "ValidationError",
    # Validators
    "required",
    "not_empty",
    "min_length",
    "max_length",
    "pattern",
    "email",
    "url",
    "min_value",
    "max_value",
    "in_range",
    "one_of",
    "uuid",
    "phone",
    "ip_address",
]

