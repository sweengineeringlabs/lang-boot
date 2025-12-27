"""Middleware protocol."""

from typing import Protocol, runtime_checkable
from dev.engineeringlabs.pyboot.middleware.api.types import Context, Response, NextFn


@runtime_checkable
class Middleware(Protocol):
    """Middleware protocol for pipeline processing.
    
    Example:
        class AuthMiddleware(Middleware):
            async def process(self, ctx: Context, next: NextFn) -> Response:
                token = ctx.request.headers.get("Authorization")
                if not token:
                    return Response.error(401, "Unauthorized")
                ctx.set("user", validate_token(token))
                return await next(ctx)
    """
    
    async def process(self, ctx: Context, next: NextFn) -> Response:
        """Process request and optionally call next middleware.
        
        Args:
            ctx: Request context.
            next: Next middleware in chain.
            
        Returns:
            Response object.
        """
        ...
