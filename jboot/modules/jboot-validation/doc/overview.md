# Validation Module Overview

## WHAT: Input Validation

Fluent validation builders with custom constraints and i18n.

Key capabilities:
- **Fluent API** - Readable validation rules
- **Constraints** - Built-in and custom validators
- **i18n** - Internationalized messages
- **Groups** - Validation groups

## WHY: Data Integrity

**Problems Solved**:
1. **Invalid Input** - Early validation
2. **Error Messages** - User-friendly errors
3. **Reusability** - Shared validation rules

**When to Use**: All user input, API requests

## HOW: Usage Guide

```java
var validator = Validator.builder()
    .field("email")
        .notEmpty()
        .email()
    .field("age")
        .range(18, 120)
    .field("password")
        .notEmpty()
        .minLength(8)
    .build();

ValidationResult result = validator.validate(data);

if (!result.isValid()) {
    result.getErrors().forEach(e -> 
        System.err.println(e.getField() + ": " + e.getMessage())
    );
}
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-web | Request validation |
| jboot-error | Validation errors |

---

**Status**: Stable
