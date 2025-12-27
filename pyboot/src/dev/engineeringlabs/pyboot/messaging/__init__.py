"""
Messaging Module - Async messaging and event bus.

This module provides:
- EventBus: In-memory pub/sub event bus
- Message: Standard message format
- Handlers: Event handler decorators

Example:
    from dev.engineeringlabs.pyboot.messaging import EventBus, Message, on_event

    bus = EventBus()

    # Register handler
    @on_event("user.created")
    async def handle_user_created(event: Message):
        print(f"User created: {event.payload}")

    # Publish event
    await bus.publish("user.created", {"id": "123", "name": "John"})
"""

from dev.engineeringlabs.pyboot.messaging.api import (
    Message,
    MessageHandler,
    EventBus,
    Topic,
    on_event,
)

from dev.engineeringlabs.pyboot.messaging.core import (
    InMemoryEventBus,
    get_event_bus,
    set_event_bus,
)

__all__ = [
    # API
    "Message",
    "MessageHandler",
    "EventBus",
    "Topic",
    "on_event",
    # Core
    "InMemoryEventBus",
    "get_event_bus",
    "set_event_bus",
]
