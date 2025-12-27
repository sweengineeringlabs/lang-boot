"""Event bus interface."""

from abc import ABC, abstractmethod
from typing import Any

from dev.engineeringlabs.pyboot.messaging.api.message import Message


class EventBus(ABC):
    """
    Abstract event bus interface.

    Example:
        bus = get_event_bus()

        # Subscribe to events
        async def handler(message: Message):
            print(f"Received: {message.payload}")

        await bus.subscribe("user.*", handler)

        # Publish events
        await bus.publish("user.created", {"id": "123"})
    """

    @abstractmethod
    async def publish(
        self,
        topic: str,
        payload: Any,
        headers: dict[str, str] | None = None,
    ) -> None:
        """
        Publish a message to a topic.

        Args:
            topic: Topic name
            payload: Message payload
            headers: Optional message headers
        """
        ...

    @abstractmethod
    async def publish_message(self, message: Message) -> None:
        """
        Publish a pre-built message.

        Args:
            message: Message to publish
        """
        ...

    @abstractmethod
    async def subscribe(
        self,
        topic: str,
        handler: Any,  # MessageHandler
    ) -> str:
        """
        Subscribe to a topic.

        Args:
            topic: Topic pattern (supports wildcards)
            handler: Message handler function

        Returns:
            Subscription ID
        """
        ...

    @abstractmethod
    async def unsubscribe(self, subscription_id: str) -> bool:
        """
        Unsubscribe from a topic.

        Args:
            subscription_id: ID returned from subscribe()

        Returns:
            True if unsubscribed, False if not found
        """
        ...

    @abstractmethod
    async def close(self) -> None:
        """Close the event bus."""
        ...

    async def request(
        self,
        topic: str,
        payload: Any,
        timeout: float = 30.0,
    ) -> Message:
        """
        Send a request and wait for a response.

        Args:
            topic: Topic to send to
            payload: Request payload
            timeout: Timeout in seconds

        Returns:
            Response message

        Raises:
            TimeoutError: If no response received
        """
        raise NotImplementedError("Request-response not supported by this bus")


__all__ = ["EventBus"]
