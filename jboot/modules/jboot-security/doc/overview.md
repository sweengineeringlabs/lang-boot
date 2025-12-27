# Security Module Overview

## WHAT: Authentication & Authorization

Complete security stack with auth, authorization, tokens, and secrets.

Key capabilities:
- **Authentication** - Credential validation
- **Authorization** - Role/permission-based access
- **JWT Tokens** - Token generation/validation
- **Secrets** - Secure secret management

## WHY: Secure Applications

**Problems Solved**:
1. **Auth Complexity** - Unified auth patterns
2. **Token Management** - JWT handling
3. **Secret Leakage** - Secure secret storage

**When to Use**: All applications with authentication

## HOW: Usage Guide

```java
// Authentication
var auth = Authenticator.create();
var result = auth.authenticate(credentials);

// JWT
var jwt = JwtService.create(secretKey);
String token = jwt.generate(claims);
Claims parsed = jwt.verify(token);

// Authorization
@RequiresRole("ADMIN")
public void adminAction() { }
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-crypto | Hashing/encryption |
| jboot-web | Auth middleware |

---

**Status**: Stable
