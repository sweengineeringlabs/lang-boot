"""Cache entry and statistics models."""

import time
from dataclasses import dataclass, field
from typing import Any


@dataclass(slots=True)
class CacheEntry:
    """A cache entry with metadata."""

    key: str
    value: Any
    created_at: float = field(default_factory=time.time)
    expires_at: float | None = None
    hits: int = 0

    @property
    def ttl(self) -> float | None:
        """Get remaining TTL in seconds."""
        if self.expires_at is None:
            return None
        remaining = self.expires_at - time.time()
        return max(0, remaining)

    @property
    def is_expired(self) -> bool:
        """Check if entry is expired."""
        if self.expires_at is None:
            return False
        return time.time() > self.expires_at

    def touch(self) -> None:
        """Increment hit count."""
        self.hits += 1

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "key": self.key,
            "value": self.value,
            "created_at": self.created_at,
            "expires_at": self.expires_at,
            "ttl": self.ttl,
            "hits": self.hits,
            "is_expired": self.is_expired,
        }


@dataclass(frozen=True, slots=True)
class CacheStats:
    """Cache statistics."""

    hits: int = 0
    misses: int = 0
    size: int = 0
    max_size: int | None = None
    evictions: int = 0

    @property
    def hit_rate(self) -> float:
        """Get cache hit rate (0.0 to 1.0)."""
        total = self.hits + self.misses
        if total == 0:
            return 0.0
        return self.hits / total

    @property
    def miss_rate(self) -> float:
        """Get cache miss rate (0.0 to 1.0)."""
        return 1.0 - self.hit_rate

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "hits": self.hits,
            "misses": self.misses,
            "size": self.size,
            "max_size": self.max_size,
            "evictions": self.evictions,
            "hit_rate": self.hit_rate,
            "miss_rate": self.miss_rate,
        }


__all__ = ["CacheEntry", "CacheStats"]
