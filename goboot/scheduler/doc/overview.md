# Scheduler Module Overview

## WHAT: Task Scheduling

Cron-like scheduling and periodic tasks.

Key capabilities:
- **Cron** - Standard expressions
- **Fixed Rate** - Periodic execution
- **One-time** - Delayed execution
- **Distributed** - Leader election

## WHY: Automated Tasks

**Problems Solved**: Manual triggers, reliability

**When to Use**: Background jobs, maintenance

## HOW: Usage Guide

```go
sched := scheduler.New()

// Cron
sched.Cron("0 0 * * *", dailyCleanup)

// Fixed rate
sched.Every(5*time.Minute, syncData)

// One-time
sched.After(1*time.Hour, sendReminder)

sched.Start()
```

---

**Status**: Stable
