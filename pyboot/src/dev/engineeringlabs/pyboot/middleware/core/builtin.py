"""Built-in middleware implementations."""

import time
import logging
import asyncio
from typing import Callable

from dev.engineeringlabs.pyboot.middleware.api.types import Context, Response, NextFn


class LoggingMiddleware:
    """Logs requests and responses."""
    
    def __init__(self, logger: logging.Logger | None = None) -> None:
        self._logger = logger or logging.getLogger(__name__)
    
    async def process(self, ctx: Context, next: NextFn) -> Response:
        self._logger.info(f"→ {ctx.request.method} {ctx.request.path}")
        response = await next(ctx)
        self._logger.info(f"← {response.status} {ctx.request.path}")
        return response


class TimingMiddleware:
    """Adds timing information to response."""
    
    def __init__(self, header_name: str = "X-Response-Time") -> None:
        self._header_name = header_name
    
    async def process(self, ctx: Context, next: NextFn) -> Response:
        start = time.perf_counter()
        response = await next(ctx)
        elapsed = (time.perf_counter() - start) * 1000
        response.headers[self._header_name] = f"{elapsed:.2f}ms"
        ctx.set("response_time_ms", elapsed)
        return response


class ErrorHandlerMiddleware:
    """Catches exceptions and returns error responses."""
    
    def __init__(
        self,
        handler: Callable[[Exception], Response] | None = None,
    ) -> None:
        self._handler = handler
    
    async def process(self, ctx: Context, next: NextFn) -> Response:
        try:
            return await next(ctx)
        except Exception as e:
            if self._handler:
                return self._handler(e)
            return Response.error(500, str(e))


class RetryMiddleware:
    """Retries failed requests."""
    
    def __init__(
        self,
        max_attempts: int = 3,
        retry_on: tuple[type[Exception], ...] = (Exception,),
        delay: float = 0.1,
    ) -> None:
        self._max_attempts = max_attempts
        self._retry_on = retry_on
        self._delay = delay
    
    async def process(self, ctx: Context, next: NextFn) -> Response:
        last_error: Exception | None = None
        
        for attempt in range(self._max_attempts):
            try:
                return await next(ctx)
            except self._retry_on as e:
                last_error = e
                if attempt < self._max_attempts - 1:
                    await asyncio.sleep(self._delay * (2 ** attempt))
        
        return Response.error(500, str(last_error))


class CorsMiddleware:
    """Adds CORS headers."""
    
    def __init__(
        self,
        allow_origins: list[str] | None = None,
        allow_methods: list[str] | None = None,
        allow_headers: list[str] | None = None,
    ) -> None:
        self._origins = allow_origins or ["*"]
        self._methods = allow_methods or ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
        self._headers = allow_headers or ["Content-Type", "Authorization"]
    
    async def process(self, ctx: Context, next: NextFn) -> Response:
        response = await next(ctx)
        response.headers["Access-Control-Allow-Origin"] = ", ".join(self._origins)
        response.headers["Access-Control-Allow-Methods"] = ", ".join(self._methods)
        response.headers["Access-Control-Allow-Headers"] = ", ".join(self._headers)
        return response
