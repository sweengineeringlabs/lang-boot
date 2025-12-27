# Observability Module Overview

## WHAT: Logging, Metrics, Tracing

Unified observability with structured logging, metrics collection, and distributed tracing.

Key capabilities:
- **Logging** - Structured, contextual logging
- **Metrics** - Counters, gauges, histograms
- **Tracing** - Distributed trace context
- **Correlation** - Request correlation IDs

## WHY: Production Visibility

**Problems Solved**:
1. **Debugging** - Structured logs with context
2. **Performance** - Metrics dashboards
3. **Distributed Tracing** - Cross-service visibility

**When to Use**: All production applications

## HOW: Usage Guide

```java
// Logging
Logger log = Logger.of(MyClass.class);
log.info("Processing order", Map.of("orderId", orderId));

// Metrics
Metrics.counter("orders.created").increment();
Metrics.timer("order.processing").record(() -> process(order));

// Tracing
try (var span = Tracer.start("processOrder")) {
    span.tag("orderId", orderId);
    process(order);
}
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| All modules | Instrumentation |

---

**Status**: Stable
