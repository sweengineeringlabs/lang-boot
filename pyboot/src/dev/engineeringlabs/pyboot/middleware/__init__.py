"""
Middleware Module - Pipeline pattern for request/response processing.

This module provides:
- Middleware pipeline
- Request/response processing
- Context propagation
- Error handling middleware
- Built-in middleware (logging, timing, auth)

Example:
    from dev.engineeringlabs.pyboot.middleware import Pipeline, Middleware, Context
    
    # Define middleware
    class LoggingMiddleware(Middleware):
        async def process(self, ctx: Context, next: NextFn) -> Response:
            print(f"Request: {ctx.request}")
            response = await next(ctx)
            print(f"Response: {response}")
            return response
    
    # Build pipeline
    pipeline = Pipeline()
    pipeline.use(LoggingMiddleware())
    pipeline.use(TimingMiddleware())
    pipeline.use(AuthMiddleware())
    
    # Process request
    response = await pipeline.execute(request)
"""

from dev.engineeringlabs.pyboot.middleware.api import (
    # Types
    Context,
    Request,
    Response,
    NextFn,
    # Middleware protocol
    Middleware,
    # Exceptions
    MiddlewareError,
    PipelineError,
)

from dev.engineeringlabs.pyboot.middleware.core import (
    Pipeline,
    # Built-in middleware
    LoggingMiddleware,
    TimingMiddleware,
    ErrorHandlerMiddleware,
    RetryMiddleware,
)

__all__ = [
    # API
    "Context",
    "Request", 
    "Response",
    "NextFn",
    "Middleware",
    "MiddlewareError",
    "PipelineError",
    # Core
    "Pipeline",
    "LoggingMiddleware",
    "TimingMiddleware",
    "ErrorHandlerMiddleware",
    "RetryMiddleware",
]
