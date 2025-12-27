"""Job model."""

import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Callable, Awaitable

from dev.engineeringlabs.pyboot.scheduler.api.schedule import Schedule


class JobStatus(str, Enum):
    """Job execution status."""
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"


@dataclass
class Job:
    """
    A scheduled job.

    Attributes:
        id: Unique job ID
        name: Job name
        func: Function to execute
        schedule: Schedule configuration
        status: Current status
        enabled: Whether job is active
    """

    id: str
    name: str
    func: Callable[[], Awaitable[Any]]
    schedule: Schedule
    status: JobStatus = JobStatus.PENDING
    enabled: bool = True
    last_run: float | None = None
    next_run: float | None = None
    run_count: int = 0
    error_count: int = 0
    last_error: str | None = None
    tags: set[str] = field(default_factory=set)
    metadata: dict[str, Any] = field(default_factory=dict)

    def update_next_run(self) -> None:
        """Calculate the next run time."""
        self.next_run = self.schedule.next_run_time()

    def mark_started(self) -> None:
        """Mark job as running."""
        self.status = JobStatus.RUNNING
        self.last_run = time.time()

    def mark_completed(self) -> None:
        """Mark job as completed."""
        self.status = JobStatus.PENDING
        self.run_count += 1
        self.update_next_run()

    def mark_failed(self, error: str) -> None:
        """Mark job as failed."""
        self.status = JobStatus.PENDING
        self.error_count += 1
        self.last_error = error
        self.update_next_run()

    def cancel(self) -> None:
        """Cancel the job."""
        self.status = JobStatus.CANCELLED
        self.enabled = False

    def is_due(self) -> bool:
        """Check if job is due to run."""
        if not self.enabled:
            return False
        if self.status == JobStatus.RUNNING:
            return False
        if self.next_run is None:
            return True
        return time.time() >= self.next_run

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "id": self.id,
            "name": self.name,
            "status": self.status.value,
            "enabled": self.enabled,
            "last_run": self.last_run,
            "next_run": self.next_run,
            "run_count": self.run_count,
            "error_count": self.error_count,
            "last_error": self.last_error,
            "tags": list(self.tags),
        }


__all__ = ["Job", "JobStatus"]
