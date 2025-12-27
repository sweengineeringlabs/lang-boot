"""
PyBoot Examples - Rate Limiting

Demonstrates rate limiting with different strategies.
"""

import asyncio
from dev.engineeringlabs.pyboot.ratelimit import (
    RateLimitConfig,
    RateLimitStrategy,
    TokenBucket,
    FixedWindow,
    SlidingWindow,
    rate_limited,
    RateLimitExceededError,
)


async def main():
    # Example 1: Token Bucket
    print("=" * 50)
    print("Example 1: Token Bucket Rate Limiter")
    print("=" * 50)
    
    config = RateLimitConfig(
        strategy=RateLimitStrategy.TOKEN_BUCKET,
        max_requests=5,
        window_seconds=1.0,
    )
    limiter = TokenBucket(config)
    
    for i in range(8):
        allowed = await limiter.acquire()
        print(f"Request {i+1}: {'✓ Allowed' if allowed else '✗ Denied'}")
    
    print("\nWaiting 1 second for token refill...")
    await asyncio.sleep(1.0)
    
    allowed = await limiter.acquire()
    print(f"After wait: {'✓ Allowed' if allowed else '✗ Denied'}")
    print()

    # Example 2: Fixed Window
    print("=" * 50)
    print("Example 2: Fixed Window Rate Limiter")
    print("=" * 50)
    
    config = RateLimitConfig(
        strategy=RateLimitStrategy.FIXED_WINDOW,
        max_requests=3,
        window_seconds=2.0,
    )
    limiter = FixedWindow(config)
    
    for i in range(5):
        allowed = await limiter.acquire()
        print(f"Request {i+1}: {'✓ Allowed' if allowed else '✗ Denied'}")
    print()

    # Example 3: Rate limited decorator
    print("=" * 50)
    print("Example 3: @rate_limited Decorator")
    print("=" * 50)
    
    @rate_limited(max_requests=3, window_seconds=1.0)
    async def api_call(endpoint: str) -> str:
        return f"Response from {endpoint}"
    
    for i in range(5):
        try:
            result = await api_call(f"/api/users/{i}")
            print(f"Call {i+1}: {result}")
        except RateLimitExceededError as e:
            print(f"Call {i+1}: Rate limited - {e.message}")
    print()

    # Example 4: Sliding Window
    print("=" * 50)
    print("Example 4: Sliding Window Rate Limiter")
    print("=" * 50)
    
    config = RateLimitConfig(
        strategy=RateLimitStrategy.SLIDING_WINDOW,
        max_requests=4,
        window_seconds=2.0,
    )
    limiter = SlidingWindow(config)
    
    print("Making requests at different times...")
    for i in range(3):
        allowed = await limiter.acquire()
        print(f"t=0.{i}s: Request {'✓ Allowed' if allowed else '✗ Denied'}")
        await asyncio.sleep(0.1)
    
    await asyncio.sleep(0.5)
    
    for i in range(3):
        allowed = await limiter.acquire()
        print(f"t=0.8s: Request {'✓ Allowed' if allowed else '✗ Denied'}")


if __name__ == "__main__":
    asyncio.run(main())
