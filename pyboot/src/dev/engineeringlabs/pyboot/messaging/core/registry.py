"""Event bus registry for global access."""

from dev.engineeringlabs.pyboot.messaging.api.bus import EventBus
from dev.engineeringlabs.pyboot.messaging.api.handler import get_registered_handlers
from dev.engineeringlabs.pyboot.messaging.core.memory import InMemoryEventBus

# Global event bus registry
_buses: dict[str, EventBus] = {}
_default_bus: EventBus | None = None


def get_event_bus(name: str | None = None) -> EventBus:
    """
    Get an event bus by name.

    Args:
        name: Bus name (None = default bus)

    Returns:
        EventBus instance
    """
    global _default_bus

    if name is None:
        if _default_bus is None:
            _default_bus = InMemoryEventBus()
        return _default_bus

    if name not in _buses:
        _buses[name] = InMemoryEventBus()

    return _buses[name]


def set_event_bus(bus: EventBus, name: str | None = None) -> None:
    """
    Register an event bus.

    Args:
        bus: EventBus instance
        name: Bus name (None = set as default)
    """
    global _default_bus

    if name is None:
        _default_bus = bus
    else:
        _buses[name] = bus


async def register_handlers(bus_name: str | None = None) -> int:
    """
    Register all decorated handlers with the bus.

    Args:
        bus_name: Bus name to register handlers for

    Returns:
        Number of handlers registered
    """
    bus = get_event_bus(bus_name)
    handlers = get_registered_handlers(bus_name or "default")

    for topic, handler in handlers:
        await bus.subscribe(topic, handler)

    return len(handlers)


def clear_buses() -> None:
    """Clear all event bus registrations."""
    global _default_bus
    _buses.clear()
    _default_bus = None


__all__ = [
    "get_event_bus",
    "set_event_bus",
    "register_handlers",
    "clear_buses",
]
