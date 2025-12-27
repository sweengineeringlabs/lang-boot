"""
Validation Module - Input validation utilities.

This module provides:
- Validators: Reusable validation rules
- Validation decorator for functions
- Result-based validation

Example:
    from dev.engineeringlabs.pyboot.validation import validate, required, email, min_length
    from dev.engineeringlabs.pyboot.validation import uuid, phone, ip_address

    # Using validators
    errors = validate(
        ("email", user_email, [required(), email()]),
        ("password", user_password, [required(), min_length(8)]),
        ("user_id", user_id, [required(), uuid()]),
        ("phone", user_phone, [phone()]),
        ("server_ip", server_ip, [ip_address()]),
    )

    # Using decorator
    @validated
    async def create_user(
        email: Annotated[str, Email()],
        password: Annotated[str, MinLength(8)],
    ) -> User:
        ...
"""

from dev.engineeringlabs.pyboot.validation.api import (
    Validator,
    ValidationResult,
    ValidationError,
    FieldError,
    # Common validators
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

from dev.engineeringlabs.pyboot.validation.core import (
    validate,
    validate_dict,
    validated,
)

__all__ = [
    # API
    "Validator",
    "ValidationResult",
    "ValidationError",
    "FieldError",
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
    # Core
    "validate",
    "validate_dict",
    "validated",
]

