"""
HTTP Module - HTTP client with resilience patterns.

This module provides:
- HttpClient: Async HTTP client with built-in resilience
- Request/Response models
- Middleware support

Example:
    from dev.engineeringlabs.pyboot.http import HttpClient, HttpConfig

    client = HttpClient(
        base_url="https://api.example.com",
        config=HttpConfig(
            timeout=30.0,
            retries=3,
            circuit_breaker=True,
        )
    )

    response = await client.get("/users/123")
    data = response.json()
"""

from dev.engineeringlabs.pyboot.http.api import (
    HttpClient,
    HttpConfig,
    HttpRequest,
    HttpResponse,
    HttpMethod,
    HttpError,
)

from dev.engineeringlabs.pyboot.http.core import (
    create_client,
    AsyncHttpClient,
)

__all__ = [
    # API
    "HttpClient",
    "HttpConfig",
    "HttpRequest",
    "HttpResponse",
    "HttpMethod",
    "HttpError",
    # Core
    "create_client",
    "AsyncHttpClient",
]
