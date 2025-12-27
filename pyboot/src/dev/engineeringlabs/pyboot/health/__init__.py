"""
Health Module - Health checking infrastructure.

This module provides:
- API layer: HealthChecker interface, status models
- Core layer: HealthService implementation

Example:
    from dev.engineeringlabs.pyboot.health import HealthService, ComponentHealth

    service = HealthService(version="1.0.0")

    # Register custom health checker
    class DbChecker(HealthChecker):
        @property
        def name(self) -> str:
            return "database"

        async def check(self) -> ComponentHealth:
            return ComponentHealth.healthy("database")

    service.register(DbChecker())
    health = await service.check_all()
"""

from dev.engineeringlabs.pyboot.health.api import (
    HealthChecker,
    HealthStatus,
    ComponentHealth,
    SystemHealth,
)

from dev.engineeringlabs.pyboot.health.core import (
    HealthService,
    SimpleHealthChecker,
)

__all__ = [
    # API
    "HealthChecker",
    "HealthStatus",
    "ComponentHealth",
    "SystemHealth",
    # Core
    "HealthService",
    "SimpleHealthChecker",
]
