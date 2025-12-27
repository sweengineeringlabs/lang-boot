"""
Testing types - Test case and result structures.
"""

from dataclasses import dataclass, field
from enum import Enum
from typing import Any
from datetime import datetime


class TestStatus(str, Enum):
    """Test execution status."""
    PASSED = "passed"
    FAILED = "failed"
    SKIPPED = "skipped"
    ERROR = "error"


@dataclass
class TestCase:
    """Represents a test case.
    
    Attributes:
        name: Test name.
        description: Test description.
        tags: Test tags for filtering.
        timeout: Timeout in seconds.
    """
    name: str
    description: str = ""
    tags: list[str] = field(default_factory=list)
    timeout: float | None = None
    setup: Any | None = None
    teardown: Any | None = None


@dataclass
class TestResult:
    """Result of a test execution.
    
    Attributes:
        name: Test name.
        status: Pass/fail/skip/error.
        duration: Execution time in seconds.
        error: Error message if failed.
        traceback: Stack trace if error.
    """
    name: str
    status: TestStatus
    duration: float = 0.0
    error: str | None = None
    traceback: str | None = None
    timestamp: datetime = field(default_factory=datetime.now)
    
    @property
    def passed(self) -> bool:
        return self.status == TestStatus.PASSED
    
    @property
    def failed(self) -> bool:
        return self.status == TestStatus.FAILED
