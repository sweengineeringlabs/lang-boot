# Observability Module Overview

## WHAT: Logging, Metrics, Tracing

Unified observability stack.

Key capabilities:
- **Logging** - Structured logging
- **Metrics** - Prometheus metrics
- **Tracing** - OpenTelemetry
- **Correlation** - Request IDs

## WHY: Production Visibility

**Problems Solved**: Debugging, performance

**When to Use**: All production apps

## HOW: Usage Guide

```go
// Logging
log := observability.Logger("myservice")
log.Info("Processing", "orderId", orderId)

// Metrics
observability.Counter("orders_created").Inc()
observability.Histogram("request_duration").Observe(duration)

// Tracing
ctx, span := observability.StartSpan(ctx, "processOrder")
defer span.End()
```

---

**Status**: Stable
