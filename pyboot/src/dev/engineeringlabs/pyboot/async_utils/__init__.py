"""
Async Module - Async utilities and task management.

Provides utilities for async programming:
- Task spawning and management
- Async context managers
- Async utilities
"""

from dev.engineeringlabs.pyboot.async_utils.api import (
    spawn,
    spawn_blocking,
    gather,
    TaskHandle,
    TaskError,
)

from dev.engineeringlabs.pyboot.async_utils.core import (
    TaskExecutor,
    TaskPool,
)

__all__ = [
    # API
    "spawn",
    "spawn_blocking",
    "gather",
    "TaskHandle",
    "TaskError",
    # Core
    "TaskExecutor",
    "TaskPool",
]
