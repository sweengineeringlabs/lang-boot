"""Security exceptions."""


class SecurityError(Exception):
    """Base exception for security errors."""

    def __init__(self, message: str, code: str | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.code = code


class AuthenticationError(SecurityError):
    """Raised when authentication fails."""

    def __init__(
        self,
        message: str = "Authentication failed",
        code: str = "AUTH_FAILED",
    ) -> None:
        super().__init__(message, code)


class InvalidCredentialsError(AuthenticationError):
    """Raised when credentials are invalid."""

    def __init__(self, message: str = "Invalid credentials") -> None:
        super().__init__(message, "INVALID_CREDENTIALS")


class TokenExpiredError(AuthenticationError):
    """Raised when a token has expired."""

    def __init__(self, message: str = "Token has expired") -> None:
        super().__init__(message, "TOKEN_EXPIRED")


class InvalidTokenError(AuthenticationError):
    """Raised when a token is invalid."""

    def __init__(self, message: str = "Invalid token") -> None:
        super().__init__(message, "INVALID_TOKEN")


class AuthorizationError(SecurityError):
    """Raised when authorization fails."""

    def __init__(
        self,
        message: str = "Access denied",
        code: str = "ACCESS_DENIED",
        required_permission: str | None = None,
    ) -> None:
        super().__init__(message, code)
        self.required_permission = required_permission


class InsufficientPermissionsError(AuthorizationError):
    """Raised when user lacks required permissions."""

    def __init__(
        self,
        required_permission: str,
        message: str | None = None,
    ) -> None:
        super().__init__(
            message or f"Missing required permission: {required_permission}",
            "INSUFFICIENT_PERMISSIONS",
            required_permission,
        )


class InsufficientRolesError(AuthorizationError):
    """Raised when user lacks required roles."""

    def __init__(
        self,
        required_role: str,
        message: str | None = None,
    ) -> None:
        super().__init__(
            message or f"Missing required role: {required_role}",
            "INSUFFICIENT_ROLES",
        )
        self.required_role = required_role


__all__ = [
    "SecurityError",
    "AuthenticationError",
    "InvalidCredentialsError",
    "TokenExpiredError",
    "InvalidTokenError",
    "AuthorizationError",
    "InsufficientPermissionsError",
    "InsufficientRolesError",
]
