"""Schedule definitions."""

import time
from abc import ABC, abstractmethod
from dataclasses import dataclass


class Schedule(ABC):
    """Abstract schedule interface."""

    @abstractmethod
    def next_run_time(self) -> float:
        """Get the next run time (Unix timestamp)."""
        ...

    @abstractmethod
    def is_one_time(self) -> bool:
        """Check if this is a one-time schedule."""
        ...


@dataclass(frozen=True, slots=True)
class Interval(Schedule):
    """
    Interval-based schedule.

    Example:
        Interval(seconds=30)  # Every 30 seconds
        Interval(minutes=5)   # Every 5 minutes
        Interval(hours=1)     # Every hour
    """

    seconds: float = 0
    minutes: float = 0
    hours: float = 0
    days: float = 0

    def total_seconds(self) -> float:
        """Get total interval in seconds."""
        return (
            self.seconds
            + self.minutes * 60
            + self.hours * 3600
            + self.days * 86400
        )

    def next_run_time(self) -> float:
        """Get the next run time."""
        return time.time() + self.total_seconds()

    def is_one_time(self) -> bool:
        return False


@dataclass(frozen=True, slots=True)
class Cron(Schedule):
    """
    Cron-based schedule.

    Example:
        Cron("0 9 * * 1-5")   # 9 AM on weekdays
        Cron("*/5 * * * *")  # Every 5 minutes
        Cron("0 0 1 * *")    # First day of month at midnight
    """

    expression: str

    def next_run_time(self) -> float:
        """Get the next run time."""
        try:
            from croniter import croniter
            cron = croniter(self.expression, time.time())
            return cron.get_next(float)
        except ImportError:
            # Fallback: run in 60 seconds if croniter not installed
            return time.time() + 60

    def is_one_time(self) -> bool:
        return False


@dataclass(frozen=True, slots=True)
class Once(Schedule):
    """
    One-time schedule with delay.

    Example:
        Once(delay=60)   # Run once after 60 seconds
        Once(at=time.time() + 3600)  # Run at specific time
    """

    delay: float = 0
    at: float | None = None

    def next_run_time(self) -> float:
        """Get the run time."""
        if self.at:
            return self.at
        return time.time() + self.delay

    def is_one_time(self) -> bool:
        return True


@dataclass(frozen=True, slots=True)
class Daily(Schedule):
    """
    Daily schedule at specific time.

    Example:
        Daily(hour=9, minute=0)  # Every day at 9:00 AM
    """

    hour: int = 0
    minute: int = 0
    second: int = 0

    def next_run_time(self) -> float:
        """Get the next run time."""
        import datetime

        now = datetime.datetime.now()
        target = now.replace(
            hour=self.hour,
            minute=self.minute,
            second=self.second,
            microsecond=0,
        )

        if target <= now:
            target += datetime.timedelta(days=1)

        return target.timestamp()

    def is_one_time(self) -> bool:
        return False


__all__ = ["Schedule", "Interval", "Cron", "Once", "Daily"]
