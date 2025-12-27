"""Session protocols."""

from typing import Protocol, runtime_checkable
from dev.engineeringlabs.pyboot.session.api.types import Session, SessionData


@runtime_checkable
class SessionStore(Protocol):
    """Protocol for session storage backends."""
    
    async def get(self, session_id: str) -> SessionData | None:
        """Get session data."""
        ...
    
    async def set(self, session_id: str, data: SessionData, ttl: int | None = None) -> None:
        """Store session data."""
        ...
    
    async def delete(self, session_id: str) -> None:
        """Delete session."""
        ...
    
    async def exists(self, session_id: str) -> bool:
        """Check if session exists."""
        ...


@runtime_checkable
class SessionSerializer(Protocol):
    """Protocol for session serialization."""
    
    def serialize(self, data: SessionData) -> bytes:
        """Serialize session data."""
        ...
    
    def deserialize(self, data: bytes) -> SessionData:
        """Deserialize session data."""
        ...
