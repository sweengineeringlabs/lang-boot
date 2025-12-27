# DateTime Module Overview

## WHAT: Date/Time Utilities

Clock abstractions, date utilities, and timezone handling.

Key capabilities:
- **Clock Abstraction** - Testable time
- **Formatting** - Standard date/time formats
- **Parsing** - Flexible input parsing
- **Timezone** - Timezone conversions

## WHY: Testable Time

**Problems Solved**:
1. **Time-Dependent Tests** - Injectable clocks
2. **Formatting Inconsistency** - Standard formats
3. **Timezone Bugs** - Explicit timezone handling

**When to Use**: Any time-dependent logic

## HOW: Usage Guide

```java
// Injectable clock
Clock clock = Clock.systemUTC();
Instant now = clock.instant();

// In tests
Clock fixedClock = Clock.fixed(Instant.parse("2024-01-01T00:00:00Z"));

// Formatting
String formatted = DateTime.format(instant, "yyyy-MM-dd");
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-cache | TTL calculations |
| jboot-scheduler | Scheduling |

---

**Status**: Stable
