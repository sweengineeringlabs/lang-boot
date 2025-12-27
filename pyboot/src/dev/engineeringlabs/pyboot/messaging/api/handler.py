"""Message handler decorators and types."""

import functools
from collections.abc import Awaitable, Callable
from typing import Any, Protocol, TypeVar

from dev.engineeringlabs.pyboot.messaging.api.message import Message

T = TypeVar("T")


class MessageHandler(Protocol):
    """Protocol for message handlers."""

    async def __call__(self, message: Message) -> None:
        """Handle a message."""
        ...


# Registry for decorated handlers
_handlers: dict[str, list[tuple[str, Any]]] = {}


def on_event(
    topic: str,
    bus_name: str | None = None,
) -> Callable[[Callable[[Message], Awaitable[None]]], Callable[[Message], Awaitable[None]]]:
    """
    Decorator to register an event handler.

    Args:
        topic: Topic pattern to subscribe to
        bus_name: Optional event bus name

    Example:
        @on_event("user.created")
        async def handle_user_created(message: Message):
            user = message.payload
            await send_welcome_email(user["email"])

        @on_event("order.*")
        async def handle_order_events(message: Message):
            print(f"Order event: {message.topic}")
    """
    def decorator(
        func: Callable[[Message], Awaitable[None]],
    ) -> Callable[[Message], Awaitable[None]]:
        # Store handler for later registration
        bus_key = bus_name or "default"
        if bus_key not in _handlers:
            _handlers[bus_key] = []
        _handlers[bus_key].append((topic, func))

        @functools.wraps(func)
        async def wrapper(message: Message) -> None:
            return await func(message)

        # Mark as event handler
        wrapper._is_event_handler = True  # type: ignore[attr-defined]
        wrapper._event_topic = topic  # type: ignore[attr-defined]
        wrapper._bus_name = bus_name  # type: ignore[attr-defined]

        return wrapper

    return decorator


def get_registered_handlers(bus_name: str = "default") -> list[tuple[str, Any]]:
    """Get all registered handlers for a bus."""
    return _handlers.get(bus_name, [])


def clear_registered_handlers() -> None:
    """Clear all registered handlers."""
    _handlers.clear()


__all__ = [
    "MessageHandler",
    "on_event",
    "get_registered_handlers",
    "clear_registered_handlers",
]
