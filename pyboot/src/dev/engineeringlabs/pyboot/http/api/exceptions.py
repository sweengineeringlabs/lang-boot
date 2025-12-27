"""HTTP exceptions."""

from typing import Any, TYPE_CHECKING

if TYPE_CHECKING:
    from dev.engineeringlabs.pyboot.http.api.models import HttpResponse


class HttpError(Exception):
    """Base exception for HTTP errors."""

    def __init__(
        self,
        message: str,
        status_code: int | None = None,
        response: "HttpResponse | None" = None,
        cause: Exception | None = None,
    ) -> None:
        super().__init__(message)
        self.message = message
        self.status_code = status_code
        self.response = response
        self.cause = cause

    def __str__(self) -> str:
        if self.status_code:
            return f"[HTTP {self.status_code}] {self.message}"
        return self.message

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        result: dict[str, Any] = {
            "message": self.message,
        }
        if self.status_code:
            result["status_code"] = self.status_code
        if self.cause:
            result["cause"] = str(self.cause)
        return result


class ConnectionError(HttpError):
    """Raised when connection fails."""

    def __init__(
        self,
        message: str = "Connection failed",
        cause: Exception | None = None,
    ) -> None:
        super().__init__(message, cause=cause)


class TimeoutError(HttpError):
    """Raised when request times out."""

    def __init__(
        self,
        message: str = "Request timed out",
        timeout: float | None = None,
        cause: Exception | None = None,
    ) -> None:
        super().__init__(message, cause=cause)
        self.timeout = timeout


class TooManyRedirectsError(HttpError):
    """Raised when too many redirects occur."""

    def __init__(
        self,
        message: str = "Too many redirects",
        max_redirects: int | None = None,
    ) -> None:
        super().__init__(message)
        self.max_redirects = max_redirects


__all__ = [
    "HttpError",
    "ConnectionError",
    "TimeoutError",
    "TooManyRedirectsError",
]
