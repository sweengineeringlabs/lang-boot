"""Session store implementations."""

from datetime import datetime, timedelta
from typing import Any
from dev.engineeringlabs.pyboot.session.api.types import SessionData


class MemorySessionStore:
    """In-memory session store.
    
    Example:
        store = MemorySessionStore(ttl=3600)
        await store.set("session_123", SessionData(data={"user": "alice"}))
        data = await store.get("session_123")
    """
    
    def __init__(self, ttl: int = 3600) -> None:
        self._sessions: dict[str, tuple[SessionData, datetime]] = {}
        self._ttl = ttl
    
    async def get(self, session_id: str) -> SessionData | None:
        """Get session data."""
        entry = self._sessions.get(session_id)
        if entry is None:
            return None
        
        data, expires_at = entry
        if datetime.now() > expires_at:
            del self._sessions[session_id]
            return None
        
        return data
    
    async def set(self, session_id: str, data: SessionData, ttl: int | None = None) -> None:
        """Store session data."""
        expires_at = datetime.now() + timedelta(seconds=ttl or self._ttl)
        self._sessions[session_id] = (data, expires_at)
    
    async def delete(self, session_id: str) -> None:
        """Delete session."""
        self._sessions.pop(session_id, None)
    
    async def exists(self, session_id: str) -> bool:
        """Check if session exists."""
        return await self.get(session_id) is not None
    
    async def cleanup(self) -> int:
        """Remove expired sessions. Returns count removed."""
        now = datetime.now()
        expired = [
            sid for sid, (_, expires) in self._sessions.items()
            if now > expires
        ]
        for sid in expired:
            del self._sessions[sid]
        return len(expired)
    
    @property
    def count(self) -> int:
        """Number of active sessions."""
        return len(self._sessions)
