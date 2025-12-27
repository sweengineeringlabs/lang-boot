"""Session middleware."""

from typing import Any, Callable, Awaitable
from contextvars import ContextVar

from dev.engineeringlabs.pyboot.session.api.types import Session, SessionData, SessionConfig
from dev.engineeringlabs.pyboot.session.api.protocols import SessionStore
from dev.engineeringlabs.pyboot.session.core.utils import generate_session_id

_current_session: ContextVar[Session | None] = ContextVar("current_session", default=None)


def get_session() -> Session | None:
    """Get current session from context."""
    return _current_session.get()


def session_middleware(
    store: SessionStore,
    config: SessionConfig | None = None,
) -> Callable:
    """Create session middleware.
    
    Example:
        store = MemorySessionStore()
        middleware = session_middleware(store)
        
        @middleware
        async def handler(request):
            session = get_session()
            session["user_id"] = "123"
            return response
    """
    cfg = config or SessionConfig()
    
    def decorator(func: Callable[..., Awaitable[Any]]) -> Callable[..., Awaitable[Any]]:
        async def wrapper(*args: Any, **kwargs: Any) -> Any:
            # Extract session ID from request (simplified)
            request = args[0] if args else kwargs.get("request")
            session_id = None
            
            if hasattr(request, "cookies"):
                session_id = request.cookies.get(cfg.cookie_name)
            elif hasattr(request, "headers"):
                session_id = request.headers.get("X-Session-ID")
            
            # Load or create session
            if session_id:
                data = await store.get(session_id)
            else:
                session_id = generate_session_id()
                data = None
            
            session = Session(
                session_id=session_id,
                data=data,
                config=cfg,
            )
            
            # Set context
            token = _current_session.set(session)
            
            try:
                response = await func(*args, **kwargs)
                
                # Save session if modified
                if session.modified:
                    await store.set(session.session_id, session._data, cfg.ttl)
                
                # Set cookie if new session
                if hasattr(response, "set_cookie") and data is None:
                    response.set_cookie(
                        cfg.cookie_name,
                        session.session_id,
                        path=cfg.cookie_path,
                        secure=cfg.cookie_secure,
                        httponly=cfg.cookie_httponly,
                        samesite=cfg.cookie_samesite,
                        max_age=cfg.ttl,
                    )
                
                return response
            finally:
                _current_session.reset(token)
        
        return wrapper
    return decorator
