# Core Module Overview

## WHAT: Core Types and Utilities

Foundation types, result monads, and common utilities used across all modules.

Key capabilities:
- **Result Types** - Explicit error handling without exceptions
- **Option Types** - Null-safe value containers
- **Common Utilities** - String, collection, time utilities

## WHY: Consistent Foundation

**Problems Solved**:
1. **Null Pointer Exceptions** - Option types prevent nulls
2. **Exception Handling** - Result types for explicit errors
3. **Code Duplication** - Shared utilities

**When to Use**: Automatically included as dependency

## HOW: Usage Guide

```java
Result<User, Error> result = userService.findById(id);

result.map(User::getName)
      .orElse("Unknown");

if (result.isOk()) {
    User user = result.unwrap();
}
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| All modules | Foundation dependency |

---

**Status**: Stable
