"""Ratelimit API - Rate limit types and errors."""

from enum import Enum, auto
from dataclasses import dataclass


class RateLimitStrategy(Enum):
    """Rate limiting strategy."""
    TOKEN_BUCKET = auto()
    LEAKY_BUCKET = auto()
    FIXED_WINDOW = auto()
    SLIDING_WINDOW = auto()


@dataclass
class RateLimitConfig:
    """Rate limit configuration."""
    strategy: RateLimitStrategy = RateLimitStrategy.TOKEN_BUCKET
    max_requests: int = 100
    window_seconds: float = 60.0
    burst_size: int | None = None


class RateLimitError(Exception):
    """Base error for rate limit operations."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


class RateLimitExceededError(RateLimitError):
    """Rate limit exceeded error."""
    
    def __init__(self, retry_after: float | None = None) -> None:
        msg = "Rate limit exceeded"
        if retry_after:
            msg += f", retry after {retry_after:.1f}s"
        super().__init__(msg)
        self.retry_after = retry_after


__all__ = [
    "RateLimitStrategy",
    "RateLimitConfig",
    "RateLimitError",
    "RateLimitExceededError",
]
