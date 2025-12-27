"""HTTP client interface."""

from abc import ABC, abstractmethod
from typing import Any

from dev.engineeringlabs.pyboot.http.api.config import HttpConfig
from dev.engineeringlabs.pyboot.http.api.models import HttpRequest, HttpResponse, HttpMethod


class HttpClient(ABC):
    """
    Abstract HTTP client interface.

    Example:
        client = create_client(
            base_url="https://api.example.com",
            config=HttpConfig(timeout=30.0, retries=3)
        )

        response = await client.get("/users/123")
        user = response.json()

        response = await client.post("/users", json={"name": "John"})
    """

    @property
    @abstractmethod
    def base_url(self) -> str:
        """Get the base URL."""
        ...

    @property
    @abstractmethod
    def config(self) -> HttpConfig:
        """Get the client configuration."""
        ...

    @abstractmethod
    async def request(
        self,
        method: HttpMethod,
        path: str,
        *,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        json: dict[str, Any] | None = None,
        body: bytes | None = None,
        timeout: float | None = None,
    ) -> HttpResponse:
        """
        Send an HTTP request.

        Args:
            method: HTTP method
            path: URL path (appended to base_url)
            params: Query parameters
            headers: Request headers
            json: JSON body
            body: Raw body bytes
            timeout: Request timeout (overrides default)

        Returns:
            HttpResponse
        """
        ...

    async def get(
        self,
        path: str,
        *,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        timeout: float | None = None,
    ) -> HttpResponse:
        """Send a GET request."""
        return await self.request(
            HttpMethod.GET,
            path,
            params=params,
            headers=headers,
            timeout=timeout,
        )

    async def post(
        self,
        path: str,
        *,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        json: dict[str, Any] | None = None,
        body: bytes | None = None,
        timeout: float | None = None,
    ) -> HttpResponse:
        """Send a POST request."""
        return await self.request(
            HttpMethod.POST,
            path,
            params=params,
            headers=headers,
            json=json,
            body=body,
            timeout=timeout,
        )

    async def put(
        self,
        path: str,
        *,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        json: dict[str, Any] | None = None,
        body: bytes | None = None,
        timeout: float | None = None,
    ) -> HttpResponse:
        """Send a PUT request."""
        return await self.request(
            HttpMethod.PUT,
            path,
            params=params,
            headers=headers,
            json=json,
            body=body,
            timeout=timeout,
        )

    async def patch(
        self,
        path: str,
        *,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        json: dict[str, Any] | None = None,
        body: bytes | None = None,
        timeout: float | None = None,
    ) -> HttpResponse:
        """Send a PATCH request."""
        return await self.request(
            HttpMethod.PATCH,
            path,
            params=params,
            headers=headers,
            json=json,
            body=body,
            timeout=timeout,
        )

    async def delete(
        self,
        path: str,
        *,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        timeout: float | None = None,
    ) -> HttpResponse:
        """Send a DELETE request."""
        return await self.request(
            HttpMethod.DELETE,
            path,
            params=params,
            headers=headers,
            timeout=timeout,
        )

    @abstractmethod
    async def close(self) -> None:
        """Close the client and release resources."""
        ...

    async def __aenter__(self) -> "HttpClient":
        return self

    async def __aexit__(self, *args: Any) -> None:
        await self.close()


__all__ = ["HttpClient"]
