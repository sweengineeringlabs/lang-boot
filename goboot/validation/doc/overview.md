# Validation Module Overview

## WHAT: Input Validation

Fluent validation with constraints.

Key capabilities:
- **Fluent API** - Readable rules
- **Constraints** - Built-in validators
- **Custom** - Custom validators
- **Struct Tags** - Tag-based validation

## WHY: Data Integrity

**Problems Solved**: Invalid input, error messages

**When to Use**: All user input

## HOW: Usage Guide

```go
v := validation.New()

v.Field("email").
    Required().
    Email()

v.Field("age").
    Range(18, 120)

result := v.Validate(data)
if !result.IsValid() {
    for _, err := range result.Errors() {
        fmt.Println(err.Field, err.Message)
    }
}
```

---

**Status**: Stable
