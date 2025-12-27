"""Stream types."""

from dataclasses import dataclass, field
from typing import Any, TypeVar, Generic
from enum import Enum

T = TypeVar("T")


class StreamState(str, Enum):
    """Stream state."""
    ACTIVE = "active"
    COMPLETED = "completed"
    ERROR = "error"
    CANCELLED = "cancelled"


@dataclass
class StreamItem(Generic[T]):
    """Item in a stream with metadata."""
    value: T
    index: int = 0
    metadata: dict[str, Any] = field(default_factory=dict)


@dataclass
class StreamConfig:
    """Stream configuration.
    
    Attributes:
        buffer_size: Maximum buffer size for backpressure.
        concurrency: Max concurrent operations.
        timeout: Operation timeout in seconds.
    """
    buffer_size: int = 100
    concurrency: int = 10
    timeout: float | None = None
    on_error: str = "stop"  # "stop", "skip", "retry"
