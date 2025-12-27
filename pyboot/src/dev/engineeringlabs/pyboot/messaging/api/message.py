"""Message models."""

import time
import uuid
from dataclasses import dataclass, field
from typing import Any


@dataclass(frozen=True, slots=True)
class Topic:
    """A message topic/channel."""

    name: str
    description: str = ""

    def matches(self, pattern: str) -> bool:
        """Check if topic matches a pattern (supports wildcards)."""
        if pattern == "*":
            return True
        if pattern.endswith(".*"):
            prefix = pattern[:-2]
            return self.name == prefix or self.name.startswith(f"{prefix}.")
        if pattern.endswith("#"):
            prefix = pattern[:-1]
            return self.name.startswith(prefix)
        return self.name == pattern


@dataclass(slots=True)
class Message:
    """A message in the event bus.

    Attributes:
        topic: The topic/channel name
        payload: Message payload (any JSON-serializable data)
        id: Unique message ID
        timestamp: When the message was created
        headers: Optional message headers
        correlation_id: ID to correlate related messages
        reply_to: Topic to reply to
    """

    topic: str
    payload: Any
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    timestamp: float = field(default_factory=time.time)
    headers: dict[str, str] = field(default_factory=dict)
    correlation_id: str | None = None
    reply_to: str | None = None

    def with_header(self, key: str, value: str) -> "Message":
        """Create a new message with an additional header."""
        new_headers = dict(self.headers)
        new_headers[key] = value
        return Message(
            topic=self.topic,
            payload=self.payload,
            id=self.id,
            timestamp=self.timestamp,
            headers=new_headers,
            correlation_id=self.correlation_id,
            reply_to=self.reply_to,
        )

    def with_correlation_id(self, correlation_id: str) -> "Message":
        """Create a new message with a correlation ID."""
        return Message(
            topic=self.topic,
            payload=self.payload,
            id=self.id,
            timestamp=self.timestamp,
            headers=self.headers,
            correlation_id=correlation_id,
            reply_to=self.reply_to,
        )

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        result: dict[str, Any] = {
            "topic": self.topic,
            "payload": self.payload,
            "id": self.id,
            "timestamp": self.timestamp,
        }
        if self.headers:
            result["headers"] = self.headers
        if self.correlation_id:
            result["correlation_id"] = self.correlation_id
        if self.reply_to:
            result["reply_to"] = self.reply_to
        return result

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> "Message":
        """Create from dictionary."""
        return cls(
            topic=data["topic"],
            payload=data["payload"],
            id=data.get("id", str(uuid.uuid4())),
            timestamp=data.get("timestamp", time.time()),
            headers=data.get("headers", {}),
            correlation_id=data.get("correlation_id"),
            reply_to=data.get("reply_to"),
        )


__all__ = ["Topic", "Message"]
