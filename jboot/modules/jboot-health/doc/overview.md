# Health Module Overview

## WHAT: Health Checks

Liveness and readiness probes for container orchestration and monitoring.

Key capabilities:
- **Liveness** - Is the application alive?
- **Readiness** - Is the application ready for traffic?
- **Dependencies** - Check external dependencies
- **Aggregation** - Combine multiple checks

## WHY: Production Readiness

**Problems Solved**:
1. **Silent Failures** - Detect unhealthy state
2. **Orchestration** - Kubernetes probes
3. **Dependency Failures** - Monitor connections

**When to Use**: All production deployments

## HOW: Usage Guide

```java
var health = Health.builder()
    .check("database", () -> db.isConnected())
    .check("redis", () -> redis.ping())
    .check("api", () -> api.healthCheck())
    .build();

HealthResult result = health.check();
// { "status": "UP", "checks": {...} }
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-web | Health endpoints |
| jboot-observability | Metrics export |

---

**Status**: Stable
