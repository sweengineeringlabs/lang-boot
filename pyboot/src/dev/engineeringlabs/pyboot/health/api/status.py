"""Health status models."""

import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Any


class HealthStatus(str, Enum):
    """Health status of a component."""

    HEALTHY = "healthy"
    DEGRADED = "degraded"
    UNHEALTHY = "unhealthy"
    UNKNOWN = "unknown"


@dataclass(frozen=True, slots=True)
class ComponentHealth:
    """Health information for a component."""

    name: str
    status: HealthStatus
    message: str | None = None
    latency_ms: float | None = None
    last_check: float | None = None
    metadata: dict[str, Any] = field(default_factory=dict)

    @classmethod
    def healthy(
        cls,
        name: str,
        message: str | None = None,
        latency_ms: float | None = None,
    ) -> "ComponentHealth":
        """Create a healthy component status."""
        return cls(
            name=name,
            status=HealthStatus.HEALTHY,
            message=message,
            latency_ms=latency_ms,
            last_check=time.time(),
        )

    @classmethod
    def unhealthy(
        cls,
        name: str,
        message: str,
        latency_ms: float | None = None,
    ) -> "ComponentHealth":
        """Create an unhealthy component status."""
        return cls(
            name=name,
            status=HealthStatus.UNHEALTHY,
            message=message,
            latency_ms=latency_ms,
            last_check=time.time(),
        )

    @classmethod
    def degraded(
        cls,
        name: str,
        message: str,
        latency_ms: float | None = None,
    ) -> "ComponentHealth":
        """Create a degraded component status."""
        return cls(
            name=name,
            status=HealthStatus.DEGRADED,
            message=message,
            latency_ms=latency_ms,
            last_check=time.time(),
        )

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        result: dict[str, Any] = {
            "name": self.name,
            "status": self.status.value,
        }
        if self.message:
            result["message"] = self.message
        if self.latency_ms is not None:
            result["latency_ms"] = self.latency_ms
        if self.last_check:
            result["last_check"] = self.last_check
        if self.metadata:
            result["metadata"] = self.metadata
        return result


@dataclass(frozen=True, slots=True)
class SystemHealth:
    """Overall system health status."""

    status: HealthStatus
    components: tuple[ComponentHealth, ...] = field(default_factory=tuple)
    version: str | None = None
    uptime_seconds: float | None = None
    timestamp: float = field(default_factory=time.time)

    @classmethod
    def from_components(
        cls,
        components: list[ComponentHealth],
        version: str | None = None,
        uptime_seconds: float | None = None,
    ) -> "SystemHealth":
        """Create system health from component healths."""
        if not components:
            status = HealthStatus.UNKNOWN
        elif any(c.status == HealthStatus.UNHEALTHY for c in components):
            status = HealthStatus.UNHEALTHY
        elif any(c.status == HealthStatus.DEGRADED for c in components):
            status = HealthStatus.DEGRADED
        elif all(c.status == HealthStatus.HEALTHY for c in components):
            status = HealthStatus.HEALTHY
        else:
            status = HealthStatus.UNKNOWN

        return cls(
            status=status,
            components=tuple(components),
            version=version,
            uptime_seconds=uptime_seconds,
        )

    def is_healthy(self) -> bool:
        """Check if the system is healthy."""
        return self.status == HealthStatus.HEALTHY

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        result: dict[str, Any] = {
            "status": self.status.value,
            "components": [c.to_dict() for c in self.components],
            "timestamp": self.timestamp,
        }
        if self.version:
            result["version"] = self.version
        if self.uptime_seconds is not None:
            result["uptime_seconds"] = self.uptime_seconds
        return result


__all__ = [
    "HealthStatus",
    "ComponentHealth",
    "SystemHealth",
]
