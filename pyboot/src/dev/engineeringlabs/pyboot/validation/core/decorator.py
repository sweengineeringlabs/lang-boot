"""Validation decorator."""

import functools
import inspect
from collections.abc import Awaitable, Callable
from typing import Any, TypeVar, get_type_hints, get_args, get_origin

from dev.engineeringlabs.pyboot.validation.api.result import ValidationResult
from dev.engineeringlabs.pyboot.validation.api.exceptions import ValidationError
from dev.engineeringlabs.pyboot.validation.api.validator import Validator

try:
    from typing import Annotated
except ImportError:
    from typing_extensions import Annotated

T = TypeVar("T")


def validated(
    raise_on_error: bool = True,
) -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """
    Decorator to validate function arguments using Annotated types.

    Args:
        raise_on_error: If True, raise ValidationError; if False, return result

    Example:
        from typing import Annotated
        from dev.engineeringlabs.pyboot.validation import validated, Required, Email, MinLength

        @validated()
        async def create_user(
            email: Annotated[str, Required(), Email()],
            password: Annotated[str, Required(), MinLength(8)],
            name: str,  # Not validated
        ) -> User:
            ...
    """
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        # Get type hints
        hints = get_type_hints(func, include_extras=True)
        sig = inspect.signature(func)

        # Extract validators from Annotated types
        field_validators: dict[str, list[Validator]] = {}
        for param_name, param_type in hints.items():
            if param_name == "return":
                continue

            # Check if it's Annotated
            if get_origin(param_type) is Annotated:
                args = get_args(param_type)
                validators = [
                    arg for arg in args[1:] if isinstance(arg, Validator)
                ]
                if validators:
                    field_validators[param_name] = validators

        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            # Bind arguments to signature
            bound = sig.bind(*args, **kwargs)
            bound.apply_defaults()

            # Validate
            result = ValidationResult()
            for param_name, validators in field_validators.items():
                value = bound.arguments.get(param_name)
                for validator in validators:
                    error_message = validator.validate(value)
                    if error_message:
                        result.add_error(param_name, error_message, value)
                        break

            if result.is_invalid:
                if raise_on_error:
                    raise ValidationError("Validation failed", errors=result.errors)
                # Could potentially inject result into kwargs if needed
                pass

            return await func(*args, **kwargs)

        return wrapper

    return decorator


def validated_sync(
    raise_on_error: bool = True,
) -> Callable[[Callable[..., T]], Callable[..., T]]:
    """
    Decorator to validate function arguments (sync version).

    Example:
        @validated_sync()
        def process_data(
            value: Annotated[int, MinValue(0), MaxValue(100)],
        ) -> str:
            return f"Value: {value}"
    """
    def decorator(func: Callable[..., T]) -> Callable[..., T]:
        hints = get_type_hints(func, include_extras=True)
        sig = inspect.signature(func)

        field_validators: dict[str, list[Validator]] = {}
        for param_name, param_type in hints.items():
            if param_name == "return":
                continue

            if get_origin(param_type) is Annotated:
                args = get_args(param_type)
                validators = [
                    arg for arg in args[1:] if isinstance(arg, Validator)
                ]
                if validators:
                    field_validators[param_name] = validators

        @functools.wraps(func)
        def wrapper(*args: Any, **kwargs: Any) -> T:
            bound = sig.bind(*args, **kwargs)
            bound.apply_defaults()

            result = ValidationResult()
            for param_name, validators in field_validators.items():
                value = bound.arguments.get(param_name)
                for validator in validators:
                    error_message = validator.validate(value)
                    if error_message:
                        result.add_error(param_name, error_message, value)
                        break

            if result.is_invalid and raise_on_error:
                raise ValidationError("Validation failed", errors=result.errors)

            return func(*args, **kwargs)

        return wrapper

    return decorator


__all__ = ["validated", "validated_sync"]
