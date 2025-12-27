"""Scheduler registry for global access."""

from dev.engineeringlabs.pyboot.scheduler.core.scheduler import Scheduler

# Global scheduler registry
_schedulers: dict[str, Scheduler] = {}
_default_scheduler: Scheduler | None = None


def get_scheduler(name: str | None = None) -> Scheduler:
    """
    Get a scheduler by name.

    Args:
        name: Scheduler name (None = default scheduler)

    Returns:
        Scheduler instance
    """
    global _default_scheduler

    if name is None:
        if _default_scheduler is None:
            _default_scheduler = Scheduler(name="default")
        return _default_scheduler

    if name not in _schedulers:
        _schedulers[name] = Scheduler(name=name)

    return _schedulers[name]


def set_scheduler(scheduler: Scheduler, name: str | None = None) -> None:
    """
    Register a scheduler.

    Args:
        scheduler: Scheduler instance
        name: Scheduler name (None = set as default)
    """
    global _default_scheduler

    if name is None:
        _default_scheduler = scheduler
    else:
        _schedulers[name] = scheduler


def clear_schedulers() -> None:
    """Clear all scheduler registrations."""
    global _default_scheduler
    _schedulers.clear()
    _default_scheduler = None


__all__ = ["get_scheduler", "set_scheduler", "clear_schedulers"]
