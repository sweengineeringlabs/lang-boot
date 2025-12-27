"""Stream protocols."""

from typing import Protocol, TypeVar, AsyncIterator, Callable, Awaitable

T = TypeVar("T")
R = TypeVar("R")


class Publisher(Protocol[T]):
    """Publishes items to subscribers."""
    
    async def subscribe(self, subscriber: "Subscriber[T]") -> None:
        """Subscribe to items."""
        ...


class Subscriber(Protocol[T]):
    """Receives items from publisher."""
    
    async def on_next(self, item: T) -> None:
        """Handle next item."""
        ...
    
    async def on_error(self, error: Exception) -> None:
        """Handle error."""
        ...
    
    async def on_complete(self) -> None:
        """Handle completion."""
        ...


class Processor(Protocol[T, R]):
    """Transforms stream items."""
    
    async def process(self, item: T) -> R:
        """Process single item."""
        ...
