"""Health service implementation."""

import asyncio
import time
from collections.abc import Awaitable, Callable
from typing import Any

from dev.engineeringlabs.pyboot.health.api.checker import HealthChecker
from dev.engineeringlabs.pyboot.health.api.status import ComponentHealth, HealthStatus, SystemHealth


class SimpleHealthChecker(HealthChecker):
    """
    Simple health checker that wraps a callable.

    Example:
        async def check_db():
            await db.ping()
            return True

        checker = SimpleHealthChecker("database", check_db)
    """

    def __init__(
        self,
        name: str,
        check_fn: Callable[[], Awaitable[bool]] | Callable[[], bool],
        description: str | None = None,
    ) -> None:
        self._name = name
        self._check_fn = check_fn
        self._description = description

    @property
    def name(self) -> str:
        """Get the component name."""
        return self._name

    async def check(self) -> ComponentHealth:
        """Perform the health check."""
        start_time = time.time()
        try:
            result = self._check_fn()
            if asyncio.iscoroutine(result):
                result = await result

            latency_ms = (time.time() - start_time) * 1000

            if result:
                return ComponentHealth.healthy(
                    self._name,
                    message=self._description,
                    latency_ms=latency_ms,
                )
            else:
                return ComponentHealth.unhealthy(
                    self._name,
                    message="Check returned False",
                    latency_ms=latency_ms,
                )
        except Exception as e:
            latency_ms = (time.time() - start_time) * 1000
            return ComponentHealth.unhealthy(
                self._name,
                message=str(e),
                latency_ms=latency_ms,
            )


class HealthService:
    """
    Health service for managing and running health checks.

    Example:
        service = HealthService(version="1.0.0")

        # Register checkers
        service.register(DatabaseHealthChecker(db))
        service.register(CacheHealthChecker(cache))

        # Check all
        health = await service.check_all()
        print(health.is_healthy())

        # Check specific component
        component = await service.check("database")
    """

    def __init__(
        self,
        version: str | None = None,
    ) -> None:
        self._version = version
        self._checkers: dict[str, HealthChecker] = {}
        self._start_time = time.time()

    @property
    def version(self) -> str | None:
        """Get the application version."""
        return self._version

    @property
    def uptime_seconds(self) -> float:
        """Get uptime in seconds."""
        return time.time() - self._start_time

    def register(self, checker: HealthChecker) -> None:
        """Register a health checker."""
        self._checkers[checker.name] = checker

    def register_simple(
        self,
        name: str,
        check_fn: Callable[[], Awaitable[bool]] | Callable[[], bool],
        description: str | None = None,
    ) -> None:
        """Register a simple health check function."""
        self._checkers[name] = SimpleHealthChecker(name, check_fn, description)

    def unregister(self, name: str) -> None:
        """Unregister a health checker."""
        self._checkers.pop(name, None)

    def get_checker(self, name: str) -> HealthChecker | None:
        """Get a health checker by name."""
        return self._checkers.get(name)

    def list_checkers(self) -> list[str]:
        """List all registered checker names."""
        return list(self._checkers.keys())

    async def check(self, name: str) -> ComponentHealth:
        """
        Check a specific component.

        Args:
            name: Component name

        Returns:
            ComponentHealth for the component

        Raises:
            KeyError: If component not found
        """
        checker = self._checkers.get(name)
        if checker is None:
            return ComponentHealth(
                name=name,
                status=HealthStatus.UNKNOWN,
                message=f"No health checker registered for '{name}'",
            )
        return await checker.check()

    async def check_all(self) -> SystemHealth:
        """
        Check all registered components.

        Returns:
            SystemHealth with all component statuses
        """
        if not self._checkers:
            return SystemHealth(
                status=HealthStatus.UNKNOWN,
                version=self._version,
                uptime_seconds=self.uptime_seconds,
            )

        # Run all checks concurrently
        tasks = [checker.check() for checker in self._checkers.values()]
        results = await asyncio.gather(*tasks, return_exceptions=True)

        components: list[ComponentHealth] = []
        for checker, result in zip(self._checkers.values(), results):
            if isinstance(result, Exception):
                components.append(ComponentHealth.unhealthy(
                    checker.name,
                    message=f"Check failed: {result}",
                ))
            else:
                components.append(result)

        return SystemHealth.from_components(
            components,
            version=self._version,
            uptime_seconds=self.uptime_seconds,
        )

    async def check_liveness(self) -> bool:
        """
        Simple liveness check.

        Returns True if the service is alive.
        """
        return True

    async def check_readiness(self) -> SystemHealth:
        """
        Readiness check - checks if service is ready to handle requests.

        Returns SystemHealth with all component statuses.
        """
        return await self.check_all()


__all__ = [
    "HealthService",
    "SimpleHealthChecker",
]
