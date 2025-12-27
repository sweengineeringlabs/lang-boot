"""Scheduler exceptions."""


class SchedulerError(Exception):
    """Base exception for scheduler errors."""

    def __init__(self, message: str) -> None:
        super().__init__(message)
        self.message = message


class JobNotFoundError(SchedulerError):
    """Raised when a job is not found."""

    def __init__(self, job_id: str) -> None:
        super().__init__(f"Job not found: {job_id}")
        self.job_id = job_id


class JobAlreadyExistsError(SchedulerError):
    """Raised when trying to add a duplicate job."""

    def __init__(self, job_id: str) -> None:
        super().__init__(f"Job already exists: {job_id}")
        self.job_id = job_id


class SchedulerNotRunningError(SchedulerError):
    """Raised when scheduler is not running."""

    def __init__(self) -> None:
        super().__init__("Scheduler is not running")


__all__ = [
    "SchedulerError",
    "JobNotFoundError",
    "JobAlreadyExistsError",
    "SchedulerNotRunningError",
]
