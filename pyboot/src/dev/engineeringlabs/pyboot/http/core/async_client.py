"""Async HTTP client implementation using httpx."""

import time
from typing import Any

from dev.engineeringlabs.pyboot.http.api.client import HttpClient
from dev.engineeringlabs.pyboot.http.api.config import HttpConfig
from dev.engineeringlabs.pyboot.http.api.models import HttpMethod, HttpRequest, HttpResponse
from dev.engineeringlabs.pyboot.http.api.exceptions import HttpError, ConnectionError, TimeoutError


class AsyncHttpClient(HttpClient):
    """
    Async HTTP client implementation using httpx.

    Supports:
    - Automatic retries with exponential backoff
    - Circuit breaker (optional)
    - Configurable timeouts
    - Connection pooling

    Example:
        async with AsyncHttpClient("https://api.example.com") as client:
            response = await client.get("/users")
            users = response.json()
    """

    def __init__(
        self,
        base_url: str,
        config: HttpConfig | None = None,
    ) -> None:
        self._base_url = base_url.rstrip("/")
        self._config = config or HttpConfig.default()
        self._client: Any = None
        self._circuit_breaker: Any = None

        # Set up circuit breaker if enabled
        if self._config.circuit_breaker:
            from dev.engineeringlabs.pyboot.resilience import CircuitBreaker, CircuitBreakerConfig
            self._circuit_breaker = CircuitBreaker(
                name=f"http:{base_url}",
                config=CircuitBreakerConfig(
                    failure_threshold=self._config.circuit_failure_threshold,
                    timeout_seconds=self._config.circuit_timeout,
                ),
            )

    @property
    def base_url(self) -> str:
        """Get the base URL."""
        return self._base_url

    @property
    def config(self) -> HttpConfig:
        """Get the client configuration."""
        return self._config

    async def _get_client(self) -> Any:
        """Get or create the httpx client."""
        if self._client is None:
            try:
                import httpx
            except ImportError:
                raise ImportError(
                    "httpx is required for AsyncHttpClient. "
                    "Install with: pip install pyboot-http"
                )

            self._client = httpx.AsyncClient(
                base_url=self._base_url,
                timeout=httpx.Timeout(
                    self._config.timeout,
                    connect=self._config.connect_timeout,
                ),
                follow_redirects=self._config.follow_redirects,
                max_redirects=self._config.max_redirects,
                verify=self._config.verify_ssl,
                headers=self._config.headers,
            )
        return self._client

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
        """Send an HTTP request."""
        import httpx

        # Build request
        request = HttpRequest(
            method=method,
            url=f"{self._base_url}{path}",
            headers=headers or {},
            params=params or {},
            body=body,
            json_body=json,
            timeout=timeout,
        )

        async def execute() -> HttpResponse:
            client = await self._get_client()
            start_time = time.perf_counter()

            try:
                response = await client.request(
                    method=method.value,
                    url=path,
                    params=params,
                    headers=headers,
                    json=json,
                    content=body,
                    timeout=timeout,
                )

                elapsed_ms = (time.perf_counter() - start_time) * 1000

                return HttpResponse(
                    status_code=response.status_code,
                    headers=dict(response.headers),
                    body=response.content,
                    request=request,
                    elapsed_ms=elapsed_ms,
                )

            except httpx.ConnectError as e:
                raise ConnectionError(str(e), cause=e)
            except httpx.TimeoutException as e:
                raise TimeoutError(str(e), timeout=timeout, cause=e)
            except httpx.HTTPError as e:
                raise HttpError(str(e), cause=e)

        # Execute with retry
        return await self._execute_with_retry(execute)

    async def _execute_with_retry(self, execute: Any) -> HttpResponse:
        """Execute request with retry logic."""
        import asyncio
        import random

        last_error: Exception | None = None
        delay = self._config.retry_delay

        for attempt in range(self._config.retries + 1):
            try:
                if self._circuit_breaker:
                    return await self._circuit_breaker.execute(execute)
                else:
                    return await execute()
            except (ConnectionError, TimeoutError) as e:
                last_error = e
                if attempt < self._config.retries:
                    # Add jitter
                    wait_time = delay * (0.5 + random.random())
                    await asyncio.sleep(wait_time)
                    delay *= self._config.retry_backoff
            except Exception:
                raise

        raise last_error  # type: ignore[misc]

    async def close(self) -> None:
        """Close the client and release resources."""
        if self._client is not None:
            await self._client.aclose()
            self._client = None


def create_client(
    base_url: str,
    config: HttpConfig | None = None,
) -> AsyncHttpClient:
    """
    Create an HTTP client.

    Args:
        base_url: Base URL for requests
        config: Client configuration

    Returns:
        AsyncHttpClient instance
    """
    return AsyncHttpClient(base_url, config)


__all__ = ["AsyncHttpClient", "create_client"]
