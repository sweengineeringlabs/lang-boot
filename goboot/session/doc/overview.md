# Session Module Overview

## WHAT: Session Management

HTTP session handling with multiple backends.

Key capabilities:
- **Stores** - Memory, Redis, DB
- **Expiration** - TTL support
- **Security** - Secure tokens
- **Attributes** - Typed values

## WHY: Stateful Applications

**Problems Solved**: State management, scalability

**When to Use**: Web apps with login

## HOW: Usage Guide

```go
store := session.NewRedisStore(redisClient)

// Get session
sess := store.Get(sessionID)
sess.Set("userId", userId)
sess.Set("cart", cart)

// Retrieve
userId := sess.Get("userId").(int64)
```

---

**Status**: Stable
