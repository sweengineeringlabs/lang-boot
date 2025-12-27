# Session Module Overview

## WHAT: Session Management

HTTP session handling with multiple backends and security features.

Key capabilities:
- **Session Store** - Memory, Redis, database backends
- **Expiration** - Configurable session TTL
- **Security** - Secure session tokens
- **Attributes** - Typed session attributes

## WHY: Stateful Applications

**Problems Solved**:
1. **State Management** - User session tracking
2. **Scalability** - Distributed session stores
3. **Security** - Session fixation protection

**When to Use**: Web applications with login

## HOW: Usage Guide

```java
var sessionStore = SessionStore.redis(redisClient);

// In request handler
var session = sessionStore.get(sessionId);
session.set("userId", userId);
session.set("cart", shoppingCart);

User user = session.get("userId", User.class);
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-web | Session middleware |
| jboot-security | Auth sessions |

---

**Status**: Stable
