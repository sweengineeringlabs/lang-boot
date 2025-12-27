"""Security context for current principal."""

import contextvars
from typing import Any

from dev.engineeringlabs.pyboot.security.api.principal import Principal

# Context variable for current principal
_current_principal: contextvars.ContextVar[Principal | None] = contextvars.ContextVar(
    "current_principal", default=None
)


def get_current_principal() -> Principal | None:
    """
    Get the current security principal.

    Returns:
        Current principal or None if not authenticated
    """
    return _current_principal.get()


def set_current_principal(principal: Principal | None) -> contextvars.Token[Principal | None]:
    """
    Set the current security principal.

    Args:
        principal: Principal to set (or None to clear)

    Returns:
        Token for resetting the context
    """
    return _current_principal.set(principal)


def reset_current_principal(token: contextvars.Token[Principal | None]) -> None:
    """Reset the principal to its previous value."""
    _current_principal.reset(token)


def require_authentication() -> Principal:
    """
    Require that a principal is authenticated.

    Returns:
        Current principal

    Raises:
        AuthenticationError: If not authenticated
    """
    from dev.engineeringlabs.pyboot.security.api.exceptions import AuthenticationError

    principal = get_current_principal()
    if principal is None or not principal.authenticated:
        raise AuthenticationError("Authentication required")
    return principal


class principal_context:
    """Context manager for setting the current principal."""

    def __init__(self, principal: Principal) -> None:
        self._principal = principal
        self._token: contextvars.Token[Principal | None] | None = None

    def __enter__(self) -> Principal:
        self._token = set_current_principal(self._principal)
        return self._principal

    def __exit__(self, *args: Any) -> None:
        if self._token is not None:
            reset_current_principal(self._token)


__all__ = [
    "get_current_principal",
    "set_current_principal",
    "reset_current_principal",
    "require_authentication",
    "principal_context",
]
