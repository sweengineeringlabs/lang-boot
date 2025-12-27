"""Session types."""

from dataclasses import dataclass, field
from typing import Any
from datetime import datetime, timedelta


@dataclass
class SessionConfig:
    """Session configuration."""
    ttl: int = 3600  # seconds
    cookie_name: str = "session_id"
    cookie_path: str = "/"
    cookie_secure: bool = True
    cookie_httponly: bool = True
    cookie_samesite: str = "Lax"


@dataclass
class SessionData:
    """Session data storage."""
    data: dict[str, Any] = field(default_factory=dict)
    flash_messages: list[str] = field(default_factory=list)
    created_at: datetime = field(default_factory=datetime.now)
    accessed_at: datetime = field(default_factory=datetime.now)


class Session:
    """HTTP session with data and flash messages.
    
    Example:
        session = Session(session_id="abc123")
        session["user_id"] = "user_123"
        session["cart"] = ["item1", "item2"]
        
        session.flash("Login successful!")
        messages = session.get_flash_messages()
    """
    
    def __init__(
        self,
        session_id: str,
        data: SessionData | None = None,
        config: SessionConfig | None = None,
    ) -> None:
        self._session_id = session_id
        self._data = data or SessionData()
        self._config = config or SessionConfig()
        self._modified = False
    
    @property
    def session_id(self) -> str:
        return self._session_id
    
    @property
    def modified(self) -> bool:
        return self._modified
    
    @property
    def created_at(self) -> datetime:
        return self._data.created_at
    
    def get(self, key: str, default: Any = None) -> Any:
        """Get session value."""
        self._data.accessed_at = datetime.now()
        return self._data.data.get(key, default)
    
    def set(self, key: str, value: Any) -> None:
        """Set session value."""
        self._data.data[key] = value
        self._data.accessed_at = datetime.now()
        self._modified = True
    
    def delete(self, key: str) -> None:
        """Delete session value."""
        self._data.data.pop(key, None)
        self._modified = True
    
    def clear(self) -> None:
        """Clear all session data."""
        self._data.data.clear()
        self._modified = True
    
    def __getitem__(self, key: str) -> Any:
        return self.get(key)
    
    def __setitem__(self, key: str, value: Any) -> None:
        self.set(key, value)
    
    def __delitem__(self, key: str) -> None:
        self.delete(key)
    
    def __contains__(self, key: str) -> bool:
        return key in self._data.data
    
    # Flash messages
    def flash(self, message: str) -> None:
        """Add flash message."""
        self._data.flash_messages.append(message)
        self._modified = True
    
    def get_flash_messages(self) -> list[str]:
        """Get and clear flash messages."""
        messages = self._data.flash_messages.copy()
        self._data.flash_messages.clear()
        self._modified = True
        return messages
    
    def has_flash(self) -> bool:
        """Check if there are flash messages."""
        return len(self._data.flash_messages) > 0
    
    # Serialization
    def to_dict(self) -> dict[str, Any]:
        """Export session data."""
        return {
            "session_id": self._session_id,
            "data": self._data.data,
            "flash_messages": self._data.flash_messages,
            "created_at": self._data.created_at.isoformat(),
            "accessed_at": self._data.accessed_at.isoformat(),
        }
    
    @classmethod
    def from_dict(cls, data: dict[str, Any], config: SessionConfig | None = None) -> "Session":
        """Create session from dict."""
        session_data = SessionData(
            data=data.get("data", {}),
            flash_messages=data.get("flash_messages", []),
            created_at=datetime.fromisoformat(data.get("created_at", datetime.now().isoformat())),
            accessed_at=datetime.fromisoformat(data.get("accessed_at", datetime.now().isoformat())),
        )
        return cls(
            session_id=data.get("session_id", ""),
            data=session_data,
            config=config,
        )
