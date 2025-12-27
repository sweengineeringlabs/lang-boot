"""Scheduler implementation."""

import asyncio
import uuid
from typing import Any, Awaitable, Callable

from dev.engineeringlabs.pyboot.scheduler.api.job import Job, JobStatus
from dev.engineeringlabs.pyboot.scheduler.api.schedule import Schedule, Interval
from dev.engineeringlabs.pyboot.scheduler.api.exceptions import JobNotFoundError


class Scheduler:
    """
    Task scheduler for running periodic jobs.

    Example:
        scheduler = Scheduler()

        @scheduler.every(minutes=5)
        async def sync_data():
            await fetch_and_store()

        await scheduler.start()
    """

    def __init__(self, name: str = "default") -> None:
        self._name = name
        self._jobs: dict[str, Job] = {}
        self._running = False
        self._task: asyncio.Task[None] | None = None
        self._tick_interval = 1.0  # Check every second

    @property
    def name(self) -> str:
        """Get scheduler name."""
        return self._name

    @property
    def is_running(self) -> bool:
        """Check if scheduler is running."""
        return self._running

    @property
    def jobs(self) -> list[Job]:
        """Get all jobs."""
        return list(self._jobs.values())

    def add_job(
        self,
        func: Callable[[], Awaitable[Any]],
        schedule: Schedule,
        name: str | None = None,
        tags: set[str] | None = None,
    ) -> Job:
        """
        Add a job to the scheduler.

        Args:
            func: Async function to execute
            schedule: Schedule configuration
            name: Job name (defaults to function name)
            tags: Optional tags for grouping

        Returns:
            The created Job
        """
        job_id = str(uuid.uuid4())
        job_name = name or func.__name__

        job = Job(
            id=job_id,
            name=job_name,
            func=func,
            schedule=schedule,
            tags=tags or set(),
        )
        job.update_next_run()

        self._jobs[job_id] = job
        return job

    def remove_job(self, job_id: str) -> None:
        """Remove a job."""
        if job_id not in self._jobs:
            raise JobNotFoundError(job_id)
        del self._jobs[job_id]

    def get_job(self, job_id: str) -> Job:
        """Get a job by ID."""
        if job_id not in self._jobs:
            raise JobNotFoundError(job_id)
        return self._jobs[job_id]

    def get_jobs_by_tag(self, tag: str) -> list[Job]:
        """Get all jobs with a specific tag."""
        return [job for job in self._jobs.values() if tag in job.tags]

    def pause_job(self, job_id: str) -> None:
        """Pause a job."""
        job = self.get_job(job_id)
        job.enabled = False

    def resume_job(self, job_id: str) -> None:
        """Resume a paused job."""
        job = self.get_job(job_id)
        job.enabled = True
        job.update_next_run()

    async def run_job(self, job_id: str) -> Any:
        """Run a job immediately."""
        job = self.get_job(job_id)
        return await self._execute_job(job)

    async def start(self) -> None:
        """Start the scheduler."""
        if self._running:
            return

        self._running = True
        self._task = asyncio.create_task(self._run_loop())

    async def stop(self) -> None:
        """Stop the scheduler."""
        self._running = False
        if self._task:
            self._task.cancel()
            try:
                await self._task
            except asyncio.CancelledError:
                pass
            self._task = None

    async def _run_loop(self) -> None:
        """Main scheduler loop."""
        while self._running:
            await self._tick()
            await asyncio.sleep(self._tick_interval)

    async def _tick(self) -> None:
        """Check and run due jobs."""
        for job in self._jobs.values():
            if job.is_due():
                # Run in background
                asyncio.create_task(self._execute_job(job))

    async def _execute_job(self, job: Job) -> Any:
        """Execute a single job."""
        job.mark_started()

        try:
            result = await job.func()
            job.mark_completed()

            # Remove one-time jobs
            if job.schedule.is_one_time():
                job.enabled = False

            return result

        except Exception as e:
            job.mark_failed(str(e))
            raise

    # Decorator methods
    def every(
        self,
        seconds: float = 0,
        minutes: float = 0,
        hours: float = 0,
        days: float = 0,
        name: str | None = None,
        tags: set[str] | None = None,
    ) -> Callable[[Callable[[], Awaitable[Any]]], Callable[[], Awaitable[Any]]]:
        """
        Decorator to schedule a function at an interval.

        Example:
            @scheduler.every(minutes=5)
            async def sync():
                await do_sync()
        """
        def decorator(func: Callable[[], Awaitable[Any]]) -> Callable[[], Awaitable[Any]]:
            schedule = Interval(seconds=seconds, minutes=minutes, hours=hours, days=days)
            self.add_job(func, schedule, name=name, tags=tags)
            return func
        return decorator

    def cron(
        self,
        expression: str,
        name: str | None = None,
        tags: set[str] | None = None,
    ) -> Callable[[Callable[[], Awaitable[Any]]], Callable[[], Awaitable[Any]]]:
        """
        Decorator to schedule a function with a cron expression.

        Example:
            @scheduler.cron("0 9 * * 1-5")  # 9 AM weekdays
            async def morning_report():
                await send_report()
        """
        from dev.engineeringlabs.pyboot.scheduler.api.schedule import Cron

        def decorator(func: Callable[[], Awaitable[Any]]) -> Callable[[], Awaitable[Any]]:
            schedule = Cron(expression=expression)
            self.add_job(func, schedule, name=name, tags=tags)
            return func
        return decorator

    def once(
        self,
        delay: float = 0,
        at: float | None = None,
        name: str | None = None,
        tags: set[str] | None = None,
    ) -> Callable[[Callable[[], Awaitable[Any]]], Callable[[], Awaitable[Any]]]:
        """
        Decorator to schedule a one-time execution.

        Example:
            @scheduler.once(delay=60)
            async def cleanup():
                await do_cleanup()
        """
        from dev.engineeringlabs.pyboot.scheduler.api.schedule import Once

        def decorator(func: Callable[[], Awaitable[Any]]) -> Callable[[], Awaitable[Any]]:
            schedule = Once(delay=delay, at=at)
            self.add_job(func, schedule, name=name, tags=tags)
            return func
        return decorator

    async def __aenter__(self) -> "Scheduler":
        await self.start()
        return self

    async def __aexit__(self, *args: Any) -> None:
        await self.stop()


__all__ = ["Scheduler"]
