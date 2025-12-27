# Streams Module Overview

## WHAT: Reactive Streams

Stream processing with operators.

Key capabilities:
- **Publishers** - Event sources
- **Operators** - Map, filter, etc.
- **Backpressure** - Flow control
- **Channels** - Go channel bridge

## WHY: Async Data Processing

**Problems Solved**: Memory pressure, composition

**When to Use**: Event streams, real-time

## HOW: Usage Guide

```go
stream := streams.FromSlice([]int{1, 2, 3, 4, 5})

stream.
    Map(func(x int) int { return x * 2 }).
    Filter(func(x int) bool { return x > 4 }).
    Subscribe(func(x int) {
        fmt.Println(x)
    })
```

---

**Status**: Stable
