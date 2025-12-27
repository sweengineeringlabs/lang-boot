"""Tests for ratelimit module."""

import pytest
import asyncio
from dev.engineeringlabs.pyboot.ratelimit import (
    RateLimitConfig,
    RateLimitStrategy,
    TokenBucket,
    FixedWindow,
    SlidingWindow,
    LeakyBucket,
    rate_limited,
    RateLimitExceededError,
)


class TestTokenBucket:
    """Tests for TokenBucket rate limiter."""
    
    @pytest.mark.asyncio
    async def test_allows_within_limit(self):
        """Test requests within limit are allowed."""
        config = RateLimitConfig(max_requests=5, window_seconds=1.0)
        limiter = TokenBucket(config)
        
        for _ in range(5):
            assert await limiter.acquire()
    
    @pytest.mark.asyncio
    async def test_denies_over_limit(self):
        """Test requests over limit are denied."""
        config = RateLimitConfig(max_requests=3, window_seconds=1.0)
        limiter = TokenBucket(config)
        
        for _ in range(3):
            await limiter.acquire()
        
        assert not await limiter.acquire()
    
    @pytest.mark.asyncio
    async def test_tokens_refill(self):
        """Test tokens refill over time."""
        config = RateLimitConfig(max_requests=2, window_seconds=0.1)
        limiter = TokenBucket(config)
        
        await limiter.acquire()
        await limiter.acquire()
        assert not await limiter.acquire()
        
        await asyncio.sleep(0.15)
        assert await limiter.acquire()


class TestFixedWindow:
    """Tests for FixedWindow rate limiter."""
    
    @pytest.mark.asyncio
    async def test_allows_within_limit(self):
        """Test requests within limit are allowed."""
        config = RateLimitConfig(max_requests=3, window_seconds=1.0)
        limiter = FixedWindow(config)
        
        for _ in range(3):
            assert await limiter.acquire()
    
    @pytest.mark.asyncio
    async def test_denies_over_limit(self):
        """Test requests over limit are denied."""
        config = RateLimitConfig(max_requests=2, window_seconds=1.0)
        limiter = FixedWindow(config)
        
        await limiter.acquire()
        await limiter.acquire()
        assert not await limiter.acquire()
    
    @pytest.mark.asyncio
    async def test_window_resets(self):
        """Test window resets after time."""
        config = RateLimitConfig(max_requests=1, window_seconds=0.1)
        limiter = FixedWindow(config)
        
        await limiter.acquire()
        assert not await limiter.acquire()
        
        await asyncio.sleep(0.15)
        assert await limiter.acquire()


class TestSlidingWindow:
    """Tests for SlidingWindow rate limiter."""
    
    @pytest.mark.asyncio
    async def test_allows_within_limit(self):
        """Test requests within limit are allowed."""
        config = RateLimitConfig(max_requests=3, window_seconds=1.0)
        limiter = SlidingWindow(config)
        
        for _ in range(3):
            assert await limiter.acquire()
    
    @pytest.mark.asyncio
    async def test_denies_over_limit(self):
        """Test requests over limit are denied."""
        config = RateLimitConfig(max_requests=2, window_seconds=1.0)
        limiter = SlidingWindow(config)
        
        await limiter.acquire()
        await limiter.acquire()
        assert not await limiter.acquire()
    
    @pytest.mark.asyncio
    async def test_sliding_behavior(self):
        """Test sliding window behavior."""
        config = RateLimitConfig(max_requests=2, window_seconds=0.2)
        limiter = SlidingWindow(config)
        
        await limiter.acquire()
        await asyncio.sleep(0.1)
        await limiter.acquire()
        assert not await limiter.acquire()
        
        # After first request expires
        await asyncio.sleep(0.15)
        assert await limiter.acquire()


class TestRateLimitedDecorator:
    """Tests for @rate_limited decorator."""
    
    @pytest.mark.asyncio
    async def test_allows_within_limit(self):
        """Test decorated function works within limit."""
        @rate_limited(max_requests=3, window_seconds=1.0)
        async def api_call():
            return "success"
        
        for _ in range(3):
            assert await api_call() == "success"
    
    @pytest.mark.asyncio
    async def test_raises_on_limit_exceeded(self):
        """Test decorated function raises when limit exceeded."""
        @rate_limited(max_requests=1, window_seconds=1.0)
        async def api_call():
            return "success"
        
        await api_call()
        
        with pytest.raises(RateLimitExceededError):
            await api_call()


class TestRateLimitConfig:
    """Tests for RateLimitConfig."""
    
    def test_default_values(self):
        """Test default config values."""
        config = RateLimitConfig()
        assert config.strategy == RateLimitStrategy.TOKEN_BUCKET
        assert config.max_requests == 100
        assert config.window_seconds == 60.0
    
    def test_custom_values(self):
        """Test custom config values."""
        config = RateLimitConfig(
            strategy=RateLimitStrategy.SLIDING_WINDOW,
            max_requests=10,
            window_seconds=5.0,
        )
        assert config.strategy == RateLimitStrategy.SLIDING_WINDOW
        assert config.max_requests == 10
        assert config.window_seconds == 5.0
