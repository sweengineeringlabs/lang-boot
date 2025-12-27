"""Authorization decorators."""

import functools
from collections.abc import Awaitable, Callable
from typing import Any, TypeVar

from dev.engineeringlabs.pyboot.security.api.exceptions import (
    AuthenticationError,
    InsufficientPermissionsError,
    InsufficientRolesError,
)
from dev.engineeringlabs.pyboot.security.core.context import get_current_principal

T = TypeVar("T")


def require_permission(
    *permissions: str,
    require_all: bool = False,
) -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """
    Decorator to require permissions.

    Args:
        *permissions: Required permission names
        require_all: If True, require all permissions; if False, require any

    Example:
        @require_permission("users:read")
        async def get_users():
            ...

        @require_permission("users:read", "users:write", require_all=True)
        async def manage_users():
            ...
    """
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            principal = get_current_principal()

            if principal is None or not principal.authenticated:
                raise AuthenticationError("Authentication required")

            if require_all:
                if not principal.has_all_permissions(*permissions):
                    missing = [p for p in permissions if not principal.has_permission(p)]
                    raise InsufficientPermissionsError(missing[0] if missing else permissions[0])
            else:
                if not principal.has_any_permission(*permissions):
                    raise InsufficientPermissionsError(permissions[0])

            return await func(*args, **kwargs)

        return wrapper

    return decorator


def require_role(
    *roles: str,
    require_all: bool = False,
) -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """
    Decorator to require roles.

    Args:
        *roles: Required role names
        require_all: If True, require all roles; if False, require any

    Example:
        @require_role("admin")
        async def admin_action():
            ...

        @require_role("admin", "superuser", require_all=False)
        async def privileged_action():
            ...
    """
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            principal = get_current_principal()

            if principal is None or not principal.authenticated:
                raise AuthenticationError("Authentication required")

            if require_all:
                if not principal.has_all_roles(*roles):
                    missing = [r for r in roles if not principal.has_role(r)]
                    raise InsufficientRolesError(missing[0] if missing else roles[0])
            else:
                if not principal.has_any_role(*roles):
                    raise InsufficientRolesError(roles[0])

            return await func(*args, **kwargs)

        return wrapper

    return decorator


def require_authenticated() -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """
    Decorator to require authentication.

    Example:
        @require_authenticated()
        async def protected_action():
            ...
    """
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            principal = get_current_principal()

            if principal is None or not principal.authenticated:
                raise AuthenticationError("Authentication required")

            return await func(*args, **kwargs)

        return wrapper

    return decorator


__all__ = [
    "require_permission",
    "require_role",
    "require_authenticated",
]
