"""Web API - Web types and errors."""

from typing import Any
from dataclasses import dataclass, field
from enum import IntEnum


class HTTPStatus(IntEnum):
    """HTTP status codes."""
    OK = 200
    CREATED = 201
    NO_CONTENT = 204
    BAD_REQUEST = 400
    UNAUTHORIZED = 401
    FORBIDDEN = 403
    NOT_FOUND = 404
    INTERNAL_ERROR = 500


@dataclass
class Request:
    """HTTP request."""
    method: str
    path: str
    headers: dict[str, str] = field(default_factory=dict)
    query: dict[str, str] = field(default_factory=dict)
    body: bytes | None = None
    json_body: Any = None


@dataclass
class Response:
    """HTTP response."""
    status: HTTPStatus = HTTPStatus.OK
    headers: dict[str, str] = field(default_factory=dict)
    body: bytes | None = None
    json_body: Any = None


@dataclass
class Route:
    """Route definition."""
    method: str
    path: str
    handler: Any = None


class WebError(Exception):
    """Base error for web operations."""
    
    def __init__(
        self,
        message: str,
        status: HTTPStatus = HTTPStatus.INTERNAL_ERROR,
        *,
        cause: Exception | None = None,
    ) -> None:
        super().__init__(message)
        self.message = message
        self.status = status
        self.cause = cause


__all__ = [
    "HTTPStatus",
    "Request",
    "Response",
    "Route",
    "WebError",
]
