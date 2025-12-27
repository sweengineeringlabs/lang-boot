"""Web Core - Web framework implementations."""

from typing import Callable, Any, Awaitable
from dataclasses import dataclass, field
from dev.engineeringlabs.pyboot.web.api import Request, Response, Route, WebError, HTTPStatus


class Router:
    """HTTP router."""
    
    def __init__(self) -> None:
        self._routes: list[Route] = []
    
    def add_route(self, method: str, path: str, handler: Callable[..., Awaitable[Response]]) -> None:
        """Add a route."""
        self._routes.append(Route(method=method.upper(), path=path, handler=handler))
    
    def get(self, path: str) -> Callable[[Callable[..., Awaitable[Response]]], Callable[..., Awaitable[Response]]]:
        """Register a GET route."""
        def decorator(handler: Callable[..., Awaitable[Response]]) -> Callable[..., Awaitable[Response]]:
            self.add_route("GET", path, handler)
            return handler
        return decorator
    
    def post(self, path: str) -> Callable[[Callable[..., Awaitable[Response]]], Callable[..., Awaitable[Response]]]:
        """Register a POST route."""
        def decorator(handler: Callable[..., Awaitable[Response]]) -> Callable[..., Awaitable[Response]]:
            self.add_route("POST", path, handler)
            return handler
        return decorator
    
    def match(self, method: str, path: str) -> Route | None:
        """Find matching route."""
        for route in self._routes:
            if route.method == method.upper() and route.path == path:
                return route
        return None
    
    async def handle(self, request: Request) -> Response:
        """Handle a request."""
        route = self.match(request.method, request.path)
        if not route:
            return Response(status=HTTPStatus.NOT_FOUND)
        if route.handler:
            return await route.handler(request)
        return Response()


@dataclass
class CORSConfig:
    """CORS configuration."""
    allow_origins: list[str] = field(default_factory=lambda: ["*"])
    allow_methods: list[str] = field(default_factory=lambda: ["GET", "POST", "PUT", "DELETE"])
    allow_headers: list[str] = field(default_factory=lambda: ["Content-Type"])
    max_age: int = 86400


class CORSMiddleware:
    """CORS middleware."""
    
    def __init__(self, config: CORSConfig | None = None) -> None:
        self.config = config or CORSConfig()
    
    def process_request(self, request: Request) -> Response | None:
        """Process request (handle preflight)."""
        if request.method == "OPTIONS":
            return Response(
                status=HTTPStatus.NO_CONTENT,
                headers={
                    "Access-Control-Allow-Origin": ",".join(self.config.allow_origins),
                    "Access-Control-Allow-Methods": ",".join(self.config.allow_methods),
                    "Access-Control-Allow-Headers": ",".join(self.config.allow_headers),
                    "Access-Control-Max-Age": str(self.config.max_age),
                },
            )
        return None
    
    def process_response(self, response: Response) -> Response:
        """Add CORS headers to response."""
        response.headers["Access-Control-Allow-Origin"] = ",".join(self.config.allow_origins)
        return response


# Convenience decorators
def route(method: str, path: str) -> Callable[[Callable[..., Awaitable[Response]]], Callable[..., Awaitable[Response]]]:
    """Route decorator."""
    def decorator(handler: Callable[..., Awaitable[Response]]) -> Callable[..., Awaitable[Response]]:
        handler._route = Route(method=method.upper(), path=path, handler=handler)  # type: ignore
        return handler
    return decorator


def get(path: str) -> Callable[[Callable[..., Awaitable[Response]]], Callable[..., Awaitable[Response]]]:
    """GET route decorator."""
    return route("GET", path)


def post(path: str) -> Callable[[Callable[..., Awaitable[Response]]], Callable[..., Awaitable[Response]]]:
    """POST route decorator."""
    return route("POST", path)


def put(path: str) -> Callable[[Callable[..., Awaitable[Response]]], Callable[..., Awaitable[Response]]]:
    """PUT route decorator."""
    return route("PUT", path)


def delete(path: str) -> Callable[[Callable[..., Awaitable[Response]]], Callable[..., Awaitable[Response]]]:
    """DELETE route decorator."""
    return route("DELETE", path)


__all__ = [
    "Router",
    "CORSMiddleware",
    "route",
    "get",
    "post",
    "put",
    "delete",
]
