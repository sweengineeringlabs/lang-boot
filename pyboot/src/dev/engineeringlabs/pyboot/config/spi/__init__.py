"""Config SPI layer - Service Provider Interface for configuration sources."""

from abc import ABC, abstractmethod
from typing import Any


class ConfigurationSource(ABC):
    """
    Abstract interface for configuration sources.

    Implement this to create custom configuration sources like:
    - Remote configuration services (e.g., Consul, etcd)
    - Database-backed configuration
    - Custom file formats

    Example:
        class ConsulConfigSource(ConfigurationSource):
            def __init__(self, consul_client):
                self._client = consul_client

            @property
            def name(self) -> str:
                return "consul"

            def load(self) -> dict[str, Any]:
                return self._client.kv.get_all()

            def supports_refresh(self) -> bool:
                return True
    """

    @property
    @abstractmethod
    def name(self) -> str:
        """Get the source name."""
        ...

    @abstractmethod
    def load(self) -> dict[str, Any]:
        """Load configuration from this source."""
        ...

    def supports_refresh(self) -> bool:
        """Check if this source supports refresh."""
        return False

    async def refresh(self) -> dict[str, Any]:
        """Refresh configuration from this source."""
        return self.load()


__all__ = ["ConfigurationSource"]
