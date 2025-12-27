# Error Module Overview

## WHAT: Error Handling

Structured error types, error codes, and context-rich error reporting.

Key capabilities:
- **Error Types** - Domain-specific error classes
- **Error Codes** - Machine-readable codes
- **Context** - Stackable error context
- **Display** - Human-readable messages

## WHY: Better Error Handling

**Problems Solved**:
1. **Generic Exceptions** - Specific error types
2. **Lost Context** - Error chaining
3. **Debugging** - Rich error details

**When to Use**: All applications

## HOW: Usage Guide

```java
public class UserNotFoundError extends AppError {
    public UserNotFoundError(Long id) {
        super("USER_NOT_FOUND", "User not found: " + id);
    }
}

throw new UserNotFoundError(123)
    .withContext("operation", "getUser")
    .withContext("userId", 123);
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| All modules | Error types |

---

**Status**: Stable
