"""
Scheduler Module - Task scheduling and job management.

This module provides:
- Job scheduling with intervals
- Cron-like scheduling
- One-time delayed tasks
- Job management

Example:
    from dev.engineeringlabs.pyboot.scheduler import Scheduler, every, cron

    scheduler = Scheduler()

    # Run every 5 minutes
    @scheduler.every(minutes=5)
    async def sync_data():
        await fetch_and_store()

    # Run at specific times
    @scheduler.cron("0 9 * * 1-5")  # 9 AM weekdays
    async def morning_report():
        await send_report()

    # Run once after delay
    @scheduler.once(delay=60)
    async def cleanup():
        await do_cleanup()

    # Start the scheduler
    await scheduler.start()
"""

from dev.engineeringlabs.pyboot.scheduler.api import (
    Job,
    JobStatus,
    Schedule,
    Interval,
    Cron,
    SchedulerError,
)

from dev.engineeringlabs.pyboot.scheduler.core import (
    Scheduler,
    get_scheduler,
    set_scheduler,
    every,
    cron,
    once,
)

__all__ = [
    # API
    "Job",
    "JobStatus",
    "Schedule",
    "Interval",
    "Cron",
    "SchedulerError",
    # Core
    "Scheduler",
    "get_scheduler",
    "set_scheduler",
    "every",
    "cron",
    "once",
]
