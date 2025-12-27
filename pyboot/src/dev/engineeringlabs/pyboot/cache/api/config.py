"""Cache configuration."""

from dataclasses import dataclass


@dataclass(frozen=True, slots=True)
class CacheConfig:
    """Configuration for cache behavior.

    Attributes:
        default_ttl: Default TTL in seconds (None = no expiry)
        max_size: Maximum number of entries (None = unlimited)
        eviction_policy: Policy when max_size reached ("lru", "lfu", "fifo")
        namespace: Optional key prefix/namespace
    """

    default_ttl: int | None = None
    max_size: int | None = None
    eviction_policy: str = "lru"
    namespace: str = ""

    @classmethod
    def default(cls) -> "CacheConfig":
        """Get default configuration."""
        return cls()


__all__ = ["CacheConfig"]
