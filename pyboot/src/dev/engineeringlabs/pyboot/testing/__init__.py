"""
Testing Module - Test utilities, fixtures, and mocking helpers.

This module provides:
- Test fixtures and factory functions
- Mock builders for common components
- Async test utilities
- Assertion helpers
- Test data generators

Example:
    from dev.engineeringlabs.pyboot.testing import TestFixture, MockBuilder, async_test
    from dev.engineeringlabs.pyboot.testing import fake, given, then
    
    # Create test fixtures
    @TestFixture
    class UserFixture:
        @fixture
        def user(self) -> User:
            return fake.user()
    
    # Mock dependencies
    mock_repo = MockBuilder(UserRepository).with_method("find", returns=user).build()
    
    # Async test helper
    @async_test
    async def test_user_service():
        result = await service.get_user("123")
        assert result is not None
"""

from dev.engineeringlabs.pyboot.testing.api import (
    # Decorators
    fixture,
    async_test,
    parametrize,
    # Assertions
    assert_that,
    expect,
    # Types
    TestCase,
    TestResult,
)

from dev.engineeringlabs.pyboot.testing.core import (
    # Fixtures
    TestFixture,
    FixtureScope,
    # Mocking
    MockBuilder,
    mock_provider,
    # Fakers
    fake,
    # Utilities
    run_async,
    wait_for,
    capture_logs,
)

__all__ = [
    # API - Decorators
    "fixture",
    "async_test",
    "parametrize",
    # API - Assertions
    "assert_that",
    "expect",
    # API - Types
    "TestCase",
    "TestResult",
    # Core - Fixtures
    "TestFixture",
    "FixtureScope",
    # Core - Mocking
    "MockBuilder",
    "mock_provider",
    # Core - Fakers
    "fake",
    # Core - Utilities
    "run_async",
    "wait_for",
    "capture_logs",
]
