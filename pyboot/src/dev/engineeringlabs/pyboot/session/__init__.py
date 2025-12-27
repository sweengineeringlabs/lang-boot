"""
Session Module - HTTP session management.

This module provides:
- Session storage (memory, Redis)
- Session middleware
- Flash messages
- CSRF protection
- Session encryption

Example:
    from dev.engineeringlabs.pyboot.session import Session, MemorySessionStore, session_middleware
    
    # Configure session store
    store = MemorySessionStore(ttl=3600)
    
    # Use in middleware
    @session_middleware(store)
    async def handler(request):
        session = request.session
        session["user_id"] = "123"
        session.flash("Login successful!")
        return response
    
    # Access session
    user_id = session.get("user_id")
    messages = session.get_flash_messages()
"""

from dev.engineeringlabs.pyboot.session.api import (
    # Types
    Session,
    SessionData,
    SessionConfig,
    # Protocols
    SessionStore,
    SessionSerializer,
    # Exceptions
    SessionError,
    SessionExpiredError,
    SessionNotFoundError,
)

from dev.engineeringlabs.pyboot.session.core import (
    # Stores
    MemorySessionStore,
    # Middleware
    session_middleware,
    get_session,
    # Utilities
    generate_session_id,
)

__all__ = [
    # API
    "Session",
    "SessionData",
    "SessionConfig",
    "SessionStore",
    "SessionSerializer",
    "SessionError",
    "SessionExpiredError",
    "SessionNotFoundError",
    # Core
    "MemorySessionStore",
    "session_middleware",
    "get_session",
    "generate_session_id",
]
