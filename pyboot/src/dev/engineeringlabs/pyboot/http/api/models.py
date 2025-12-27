"""HTTP request and response models."""

import json
from dataclasses import dataclass, field
from enum import Enum
from typing import Any


class HttpMethod(str, Enum):
    """HTTP methods."""
    GET = "GET"
    POST = "POST"
    PUT = "PUT"
    PATCH = "PATCH"
    DELETE = "DELETE"
    HEAD = "HEAD"
    OPTIONS = "OPTIONS"


@dataclass(slots=True)
class HttpRequest:
    """HTTP request model."""

    method: HttpMethod
    url: str
    headers: dict[str, str] = field(default_factory=dict)
    params: dict[str, Any] = field(default_factory=dict)
    body: bytes | None = None
    json_body: dict[str, Any] | None = None
    timeout: float | None = None

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "method": self.method.value,
            "url": self.url,
            "headers": self.headers,
            "params": self.params,
            "has_body": self.body is not None or self.json_body is not None,
        }


@dataclass(slots=True)
class HttpResponse:
    """HTTP response model."""

    status_code: int
    headers: dict[str, str]
    body: bytes
    request: HttpRequest
    elapsed_ms: float = 0.0

    @property
    def ok(self) -> bool:
        """Check if response is successful (2xx)."""
        return 200 <= self.status_code < 300

    @property
    def is_redirect(self) -> bool:
        """Check if response is a redirect (3xx)."""
        return 300 <= self.status_code < 400

    @property
    def is_client_error(self) -> bool:
        """Check if response is a client error (4xx)."""
        return 400 <= self.status_code < 500

    @property
    def is_server_error(self) -> bool:
        """Check if response is a server error (5xx)."""
        return 500 <= self.status_code < 600

    def text(self, encoding: str = "utf-8") -> str:
        """Get response body as text."""
        return self.body.decode(encoding)

    def json(self) -> Any:
        """Parse response body as JSON."""
        return json.loads(self.body)

    def raise_for_status(self) -> None:
        """Raise an exception if response is not successful."""
        from dev.engineeringlabs.pyboot.http.api.exceptions import HttpError

        if not self.ok:
            raise HttpError(
                f"HTTP {self.status_code}",
                status_code=self.status_code,
                response=self,
            )

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "status_code": self.status_code,
            "headers": dict(self.headers),
            "body_length": len(self.body),
            "elapsed_ms": self.elapsed_ms,
        }


__all__ = ["HttpMethod", "HttpRequest", "HttpResponse"]
