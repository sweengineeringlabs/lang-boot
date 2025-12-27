"""In-memory event bus implementation."""

import asyncio
import fnmatch
import uuid
from dataclasses import dataclass, field
from typing import Any

from dev.engineeringlabs.pyboot.messaging.api.bus import EventBus
from dev.engineeringlabs.pyboot.messaging.api.message import Message


@dataclass
class Subscription:
    """A subscription to a topic."""
    id: str
    pattern: str
    handler: Any  # MessageHandler


class InMemoryEventBus(EventBus):
    """
    In-memory event bus implementation.

    Suitable for single-process applications and testing.

    Example:
        bus = InMemoryEventBus()

        async def handler(message: Message):
            print(f"Received: {message.payload}")

        await bus.subscribe("user.*", handler)
        await bus.publish("user.created", {"id": "123"})
    """

    def __init__(self) -> None:
        self._subscriptions: dict[str, Subscription] = {}
        self._lock = asyncio.Lock()

    async def publish(
        self,
        topic: str,
        payload: Any,
        headers: dict[str, str] | None = None,
    ) -> None:
        """Publish a message to a topic."""
        message = Message(
            topic=topic,
            payload=payload,
            headers=headers or {},
        )
        await self.publish_message(message)

    async def publish_message(self, message: Message) -> None:
        """Publish a pre-built message."""
        async with self._lock:
            subscriptions = list(self._subscriptions.values())

        # Find matching handlers
        handlers = []
        for sub in subscriptions:
            if self._matches(message.topic, sub.pattern):
                handlers.append(sub.handler)

        # Execute handlers concurrently
        if handlers:
            await asyncio.gather(
                *[self._safe_call(handler, message) for handler in handlers],
                return_exceptions=True,
            )

    async def subscribe(
        self,
        topic: str,
        handler: Any,
    ) -> str:
        """Subscribe to a topic."""
        subscription_id = str(uuid.uuid4())
        subscription = Subscription(
            id=subscription_id,
            pattern=topic,
            handler=handler,
        )

        async with self._lock:
            self._subscriptions[subscription_id] = subscription

        return subscription_id

    async def unsubscribe(self, subscription_id: str) -> bool:
        """Unsubscribe from a topic."""
        async with self._lock:
            if subscription_id in self._subscriptions:
                del self._subscriptions[subscription_id]
                return True
            return False

    async def close(self) -> None:
        """Close the event bus."""
        async with self._lock:
            self._subscriptions.clear()

    def _matches(self, topic: str, pattern: str) -> bool:
        """Check if a topic matches a pattern."""
        # Exact match
        if topic == pattern:
            return True

        # Wildcard matching
        # user.* matches user.created but not user.profile.updated
        # user.# matches user.created and user.profile.updated

        if "*" in pattern or "#" in pattern:
            # Convert AMQP-style patterns to fnmatch patterns
            fn_pattern = pattern.replace(".", "\\.").replace("*", "[^\\.]*").replace("#", ".*")
            import re
            return bool(re.match(f"^{fn_pattern}$", topic))

        return False

    async def _safe_call(self, handler: Any, message: Message) -> None:
        """Safely call a handler."""
        try:
            await handler(message)
        except Exception as e:
            # Log error but don't propagate
            # In production, use proper logging
            print(f"Handler error for {message.topic}: {e}")

    async def request(
        self,
        topic: str,
        payload: Any,
        timeout: float = 30.0,
    ) -> Message:
        """Send a request and wait for a response."""
        reply_topic = f"_reply.{uuid.uuid4()}"
        response_future: asyncio.Future[Message] = asyncio.get_event_loop().create_future()

        async def response_handler(message: Message) -> None:
            if not response_future.done():
                response_future.set_result(message)

        # Subscribe to reply topic
        sub_id = await self.subscribe(reply_topic, response_handler)

        try:
            # Send request
            message = Message(
                topic=topic,
                payload=payload,
                reply_to=reply_topic,
            )
            await self.publish_message(message)

            # Wait for response
            return await asyncio.wait_for(response_future, timeout=timeout)
        finally:
            await self.unsubscribe(sub_id)


__all__ = ["InMemoryEventBus"]
