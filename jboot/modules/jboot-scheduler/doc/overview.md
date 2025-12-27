# Scheduler Module Overview

## WHAT: Task Scheduling

Cron-like scheduling, periodic tasks, and delayed execution.

Key capabilities:
- **Cron Expressions** - Standard cron scheduling
- **Fixed Rate** - Periodic execution
- **Fixed Delay** - Delay between executions
- **One-time** - Delayed single execution

## WHY: Automated Tasks

**Problems Solved**:
1. **Manual Triggers** - Automated scheduling
2. **Cron Configuration** - Standard expressions
3. **Reliability** - Missed execution handling

**When to Use**: Background jobs, maintenance tasks

## HOW: Usage Guide

```java
var scheduler = Scheduler.create();

// Cron
scheduler.schedule("0 0 * * *", () -> dailyCleanup());

// Fixed rate
scheduler.scheduleAtFixedRate(
    Duration.ofMinutes(5),
    () -> syncData()
);

// One-time
scheduler.scheduleOnce(
    Duration.ofHours(1),
    () -> sendReminder()
);
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-observability | Job metrics |
| jboot-datetime | Time handling |

---

**Status**: Stable
