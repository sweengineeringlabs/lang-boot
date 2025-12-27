"""Standalone scheduling decorators."""

from typing import Any, Awaitable, Callable

from dev.engineeringlabs.pyboot.scheduler.core.registry import get_scheduler


def every(
    seconds: float = 0,
    minutes: float = 0,
    hours: float = 0,
    days: float = 0,
    name: str | None = None,
    tags: set[str] | None = None,
    scheduler_name: str | None = None,
) -> Callable[[Callable[[], Awaitable[Any]]], Callable[[], Awaitable[Any]]]:
    """
    Decorator to schedule a function at an interval.

    Uses the default scheduler.

    Example:
        @every(minutes=5)
        async def sync():
            await do_sync()
    """
    def decorator(func: Callable[[], Awaitable[Any]]) -> Callable[[], Awaitable[Any]]:
        scheduler = get_scheduler(scheduler_name)
        return scheduler.every(
            seconds=seconds,
            minutes=minutes,
            hours=hours,
            days=days,
            name=name,
            tags=tags,
        )(func)
    return decorator


def cron(
    expression: str,
    name: str | None = None,
    tags: set[str] | None = None,
    scheduler_name: str | None = None,
) -> Callable[[Callable[[], Awaitable[Any]]], Callable[[], Awaitable[Any]]]:
    """
    Decorator to schedule a function with a cron expression.

    Uses the default scheduler.

    Example:
        @cron("0 9 * * 1-5")  # 9 AM weekdays
        async def morning_report():
            await send_report()
    """
    def decorator(func: Callable[[], Awaitable[Any]]) -> Callable[[], Awaitable[Any]]:
        scheduler = get_scheduler(scheduler_name)
        return scheduler.cron(expression, name=name, tags=tags)(func)
    return decorator


def once(
    delay: float = 0,
    at: float | None = None,
    name: str | None = None,
    tags: set[str] | None = None,
    scheduler_name: str | None = None,
) -> Callable[[Callable[[], Awaitable[Any]]], Callable[[], Awaitable[Any]]]:
    """
    Decorator to schedule a one-time execution.

    Uses the default scheduler.

    Example:
        @once(delay=60)
        async def cleanup():
            await do_cleanup()
    """
    def decorator(func: Callable[[], Awaitable[Any]]) -> Callable[[], Awaitable[Any]]:
        scheduler = get_scheduler(scheduler_name)
        return scheduler.once(delay=delay, at=at, name=name, tags=tags)(func)
    return decorator


__all__ = ["every", "cron", "once"]
