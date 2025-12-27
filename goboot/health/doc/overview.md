# Health Module Overview

## WHAT: Health Checks

Liveness and readiness probes for orchestration.

Key capabilities:
- **Liveness** - Is the app alive?
- **Readiness** - Ready for traffic?
- **Dependencies** - Check externals
- **Aggregation** - Combine checks

## WHY: Production Readiness

**Problems Solved**: Silent failures, orchestration

**When to Use**: All production deployments

## HOW: Usage Guide

```go
checker := health.NewChecker()
checker.Add("database", func() error {
    return db.Ping()
})
checker.Add("redis", func() error {
    return redis.Ping()
})

result := checker.Check()
// {"status": "UP", "checks": {...}}
```

---

**Status**: Stable
