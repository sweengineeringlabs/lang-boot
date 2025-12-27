"""Middleware types."""

from dataclasses import dataclass, field
from typing import Any, Callable, Awaitable, TypeVar

T = TypeVar("T")


@dataclass
class Request:
    """Generic request object."""
    path: str = ""
    method: str = "GET"
    headers: dict[str, str] = field(default_factory=dict)
    body: Any = None
    params: dict[str, str] = field(default_factory=dict)
    query: dict[str, str] = field(default_factory=dict)


@dataclass
class Response:
    """Generic response object."""
    status: int = 200
    headers: dict[str, str] = field(default_factory=dict)
    body: Any = None
    
    @classmethod
    def ok(cls, body: Any = None) -> "Response":
        return cls(status=200, body=body)
    
    @classmethod
    def error(cls, status: int, message: str) -> "Response":
        return cls(status=status, body={"error": message})


@dataclass
class Context:
    """Middleware context - carries request and state through pipeline."""
    request: Request
    response: Response | None = None
    state: dict[str, Any] = field(default_factory=dict)
    
    def get(self, key: str, default: Any = None) -> Any:
        """Get state value."""
        return self.state.get(key, default)
    
    def set(self, key: str, value: Any) -> None:
        """Set state value."""
        self.state[key] = value


NextFn = Callable[[Context], Awaitable[Response]]
"""Next function type: async (ctx) -> Response"""
