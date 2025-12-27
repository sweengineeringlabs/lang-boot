# DateTime Module Overview

## WHAT: Date/Time Utilities

Clock abstractions and date utilities for testable time.

Key capabilities:
- **Clock** - Injectable time source
- **Formatting** - Standard formats
- **Parsing** - Flexible parsing
- **Timezone** - Zone handling

## WHY: Testable Time

**Problems Solved**: Time-dependent tests, formatting

**When to Use**: Any time-dependent logic

## HOW: Usage Guide

```go
// Injectable clock
type Clock interface {
    Now() time.Time
}

// Real clock
clock := datetime.SystemClock()

// Fixed clock for tests
testClock := datetime.Fixed(time.Date(2024, 1, 1, 0, 0, 0, 0, time.UTC))
```

---

**Status**: Stable
