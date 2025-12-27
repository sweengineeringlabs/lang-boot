"""Health checker interface."""

from abc import ABC, abstractmethod

from dev.engineeringlabs.pyboot.health.api.status import ComponentHealth


class HealthChecker(ABC):
    """
    Abstract interface for health checks.

    Implement this to create health checks for specific components.

    Example:
        class DatabaseHealthChecker(HealthChecker):
            def __init__(self, db: Database):
                self._db = db

            @property
            def name(self) -> str:
                return "database"

            async def check(self) -> ComponentHealth:
                try:
                    await self._db.ping()
                    return ComponentHealth.healthy("database")
                except Exception as e:
                    return ComponentHealth.unhealthy("database", str(e))
    """

    @property
    @abstractmethod
    def name(self) -> str:
        """Get the component name."""
        ...

    @abstractmethod
    async def check(self) -> ComponentHealth:
        """
        Perform a health check.

        Returns:
            ComponentHealth with check results.
        """
        ...


__all__ = ["HealthChecker"]
