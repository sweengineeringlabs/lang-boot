"""
Ratelimit Module - Rate limiting utilities.

Provides rate limiting strategies:
- Token bucket
- Leaky bucket
- Fixed window
- Sliding window
"""

from dev.engineeringlabs.pyboot.ratelimit.api import (
    RateLimitConfig,
    RateLimitStrategy,
    RateLimitError,
    RateLimitExceededError,
)

from dev.engineeringlabs.pyboot.ratelimit.core import (
    RateLimiter,
    TokenBucket,
    LeakyBucket,
    FixedWindow,
    SlidingWindow,
    rate_limited,
)

__all__ = [
    # API
    "RateLimitConfig",
    "RateLimitStrategy",
    "RateLimitError",
    "RateLimitExceededError",
    # Core
    "RateLimiter",
    "TokenBucket",
    "LeakyBucket",
    "FixedWindow",
    "SlidingWindow",
    "rate_limited",
]
