# Rate Limit Module Overview

## WHAT: Rate Limiting

Request rate limiting with multiple algorithms and distributed support.

Key capabilities:
- **Token Bucket** - Smooth rate limiting
- **Sliding Window** - Rolling window limits
- **Fixed Window** - Simple time-based limits
- **Distributed** - Redis-backed for clusters

## WHY: Protect Resources

**Problems Solved**:
1. **Overload** - Prevent resource exhaustion
2. **Fair Usage** - Limit per-user rates
3. **Cost Control** - API quota enforcement

**When to Use**: APIs, resource-intensive operations

## HOW: Usage Guide

```java
var rateLimiter = RateLimiter.tokenBucket(RateLimiterConfig.builder()
    .capacity(100)
    .refillRate(10)
    .build());

if (rateLimiter.tryAcquire()) {
    processRequest();
} else {
    throw new RateLimitExceededException();
}
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-web | Request middleware |
| jboot-cache | Distributed state |

---

**Status**: Stable
