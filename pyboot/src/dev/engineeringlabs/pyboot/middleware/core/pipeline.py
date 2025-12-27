"""Pipeline implementation."""

from typing import Callable, Awaitable
from dev.engineeringlabs.pyboot.middleware.api.types import Context, Response, NextFn, Request
from dev.engineeringlabs.pyboot.middleware.api.protocols import Middleware


class Pipeline:
    """Middleware pipeline for request processing.
    
    Example:
        pipeline = Pipeline()
        pipeline.use(LoggingMiddleware())
        pipeline.use(AuthMiddleware())
        pipeline.use(ValidationMiddleware())
        
        # Set handler
        async def handler(ctx: Context) -> Response:
            return Response.ok({"message": "Hello"})
        
        pipeline.handler(handler)
        
        # Execute
        response = await pipeline.execute(Request(path="/api/users"))
    """
    
    def __init__(self) -> None:
        self._middleware: list[Middleware] = []
        self._handler: Callable[[Context], Awaitable[Response]] | None = None
    
    def use(self, middleware: Middleware) -> "Pipeline":
        """Add middleware to pipeline."""
        self._middleware.append(middleware)
        return self
    
    def handler(self, handler: Callable[[Context], Awaitable[Response]]) -> "Pipeline":
        """Set final handler."""
        self._handler = handler
        return self
    
    async def execute(self, request: Request) -> Response:
        """Execute pipeline with request.
        
        Args:
            request: Request to process.
            
        Returns:
            Response from handler.
        """
        ctx = Context(request=request)
        
        if not self._middleware:
            if self._handler:
                return await self._handler(ctx)
            return Response.ok()
        
        # Build chain from end to start
        async def final_handler(c: Context) -> Response:
            if self._handler:
                return await self._handler(c)
            return Response.ok()
        
        chain = final_handler
        
        for middleware in reversed(self._middleware):
            chain = self._wrap(middleware, chain)
        
        return await chain(ctx)
    
    def _wrap(
        self,
        middleware: Middleware,
        next_fn: NextFn,
    ) -> NextFn:
        """Wrap middleware with next function."""
        async def wrapped(ctx: Context) -> Response:
            return await middleware.process(ctx, next_fn)
        return wrapped
    
    def clear(self) -> None:
        """Clear all middleware."""
        self._middleware.clear()
        self._handler = None
