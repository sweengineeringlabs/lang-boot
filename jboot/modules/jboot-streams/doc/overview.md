# Streams Module Overview

## WHAT: Reactive Streams

Reactive stream processing with operators and backpressure.

Key capabilities:
- **Publishers** - Event sources
- **Operators** - map, filter, flatMap
- **Backpressure** - Flow control
- **Schedulers** - Thread management

## WHY: Async Data Processing

**Problems Solved**:
1. **Memory Pressure** - Backpressure handling
2. **Composition** - Operator chaining
3. **Concurrency** - Scheduler control

**When to Use**: Event streams, real-time data

## HOW: Usage Guide

```java
var stream = Stream.of(1, 2, 3, 4, 5)
    .map(x -> x * 2)
    .filter(x -> x > 4)
    .subscribe(System.out::println);

// Async
Stream.interval(Duration.ofSeconds(1))
    .take(10)
    .subscribe(tick -> System.out.println("Tick: " + tick));
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-messaging | Event streams |
| jboot-http | Response streaming |

---

**Status**: Stable
