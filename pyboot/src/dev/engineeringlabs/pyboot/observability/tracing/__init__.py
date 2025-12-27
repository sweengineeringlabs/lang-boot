"""Tracing module for distributed tracing."""

import functools
import time
import uuid
from collections.abc import Awaitable, Callable
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, TypeVar


class SpanStatus(str, Enum):
    """Status of a span."""
    UNSET = "unset"
    OK = "ok"
    ERROR = "error"


@dataclass(slots=True)
class SpanContext:
    """Context for a span, used for propagation."""
    trace_id: str
    span_id: str
    parent_span_id: str | None = None
    baggage: dict[str, str] = field(default_factory=dict)

    @classmethod
    def create(cls, parent: "SpanContext | None" = None) -> "SpanContext":
        """Create a new span context."""
        trace_id = parent.trace_id if parent else str(uuid.uuid4())
        return cls(
            trace_id=trace_id,
            span_id=str(uuid.uuid4())[:16],
            parent_span_id=parent.span_id if parent else None,
            baggage=dict(parent.baggage) if parent else {},
        )


@dataclass
class Span:
    """
    A trace span representing a unit of work.

    Example:
        tracer = get_tracer("my-service")

        with tracer.start_span("process-request") as span:
            span.set_attribute("request_id", "abc123")
            do_work()
    """
    name: str
    context: SpanContext
    start_time: float = field(default_factory=time.time)
    end_time: float | None = None
    status: SpanStatus = SpanStatus.UNSET
    status_message: str | None = None
    attributes: dict[str, Any] = field(default_factory=dict)
    events: list[dict[str, Any]] = field(default_factory=list)

    def set_attribute(self, key: str, value: Any) -> None:
        """Set a span attribute."""
        self.attributes[key] = value

    def set_attributes(self, attributes: dict[str, Any]) -> None:
        """Set multiple span attributes."""
        self.attributes.update(attributes)

    def add_event(self, name: str, attributes: dict[str, Any] | None = None) -> None:
        """Add an event to the span."""
        self.events.append({
            "name": name,
            "timestamp": time.time(),
            "attributes": attributes or {},
        })

    def set_status(self, status: SpanStatus, message: str | None = None) -> None:
        """Set the span status."""
        self.status = status
        self.status_message = message

    def set_ok(self) -> None:
        """Set status to OK."""
        self.set_status(SpanStatus.OK)

    def set_error(self, message: str) -> None:
        """Set status to ERROR."""
        self.set_status(SpanStatus.ERROR, message)

    def end(self) -> None:
        """End the span."""
        self.end_time = time.time()

    @property
    def duration_ms(self) -> float | None:
        """Get the span duration in milliseconds."""
        if self.end_time is None:
            return None
        return (self.end_time - self.start_time) * 1000

    def __enter__(self) -> "Span":
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: Any,
    ) -> None:
        if exc_type is not None:
            self.set_error(str(exc_val))
            self.set_attribute("exception.type", exc_type.__name__)
            self.set_attribute("exception.message", str(exc_val))
        elif self.status == SpanStatus.UNSET:
            self.set_ok()
        self.end()


class Tracer:
    """
    Tracer for creating and managing spans.

    Example:
        tracer = Tracer("my-service")

        with tracer.start_span("operation") as span:
            span.set_attribute("key", "value")
            do_work()
    """

    def __init__(self, name: str) -> None:
        self._name = name
        self._current_span: Span | None = None
        self._spans: list[Span] = []

    @property
    def name(self) -> str:
        """Get the tracer name."""
        return self._name

    @property
    def current_span(self) -> Span | None:
        """Get the current active span."""
        return self._current_span

    def start_span(
        self,
        name: str,
        parent: SpanContext | None = None,
        attributes: dict[str, Any] | None = None,
    ) -> Span:
        """
        Start a new span.

        Args:
            name: Span name
            parent: Optional parent span context
            attributes: Optional initial attributes

        Returns:
            The new span
        """
        # Use current span as parent if not specified
        if parent is None and self._current_span:
            parent = self._current_span.context

        context = SpanContext.create(parent)
        span = Span(
            name=name,
            context=context,
            attributes=attributes or {},
        )
        self._spans.append(span)
        self._current_span = span
        return span

    def get_spans(self) -> list[Span]:
        """Get all recorded spans."""
        return self._spans.copy()

    def clear_spans(self) -> None:
        """Clear all recorded spans."""
        self._spans.clear()
        self._current_span = None


# Global tracers registry
_tracers: dict[str, Tracer] = {}

T = TypeVar("T")


def get_tracer(name: str) -> Tracer:
    """Get or create a tracer by name."""
    if name not in _tracers:
        _tracers[name] = Tracer(name)
    return _tracers[name]


def trace(
    name: str | None = None,
    tracer_name: str = "default",
    attributes: dict[str, Any] | None = None,
) -> Callable[[Callable[..., T]], Callable[..., T]]:
    """
    Decorator for tracing a function.

    Args:
        name: Span name (defaults to function name)
        tracer_name: Name of the tracer to use
        attributes: Optional span attributes

    Example:
        @trace("process-item")
        async def process(item: Item) -> Result:
            return await do_work(item)

        @trace(attributes={"component": "database"})
        def query(sql: str) -> list:
            return execute(sql)
    """
    def decorator(func: Callable[..., T]) -> Callable[..., T]:
        span_name = name or func.__name__
        tracer = get_tracer(tracer_name)

        @functools.wraps(func)
        def sync_wrapper(*args: Any, **kwargs: Any) -> T:
            with tracer.start_span(span_name, attributes=attributes):
                return func(*args, **kwargs)

        @functools.wraps(func)
        async def async_wrapper(*args: Any, **kwargs: Any) -> T:
            with tracer.start_span(span_name, attributes=attributes):
                return await func(*args, **kwargs)

        import asyncio
        if asyncio.iscoroutinefunction(func):
            return async_wrapper  # type: ignore[return-value]
        return sync_wrapper

    return decorator


__all__ = [
    "Span",
    "SpanContext",
    "SpanStatus",
    "Tracer",
    "get_tracer",
    "trace",
]
