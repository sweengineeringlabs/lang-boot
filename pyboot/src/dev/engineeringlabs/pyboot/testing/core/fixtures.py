"""
Test fixtures - Fixture management and scoping.
"""

from enum import Enum
from typing import Any, Callable, Generator, TypeVar
from contextlib import contextmanager

T = TypeVar("T")


class FixtureScope(str, Enum):
    """Fixture lifecycle scope."""
    FUNCTION = "function"
    CLASS = "class"  
    MODULE = "module"
    SESSION = "session"


class TestFixture:
    """Base class for test fixtures.
    
    Provides setup/teardown lifecycle and fixture methods.
    
    Example:
        class UserFixtures(TestFixture):
            def setup(self):
                self.db = Database()
            
            def teardown(self):
                self.db.close()
            
            @fixture
            def user(self) -> User:
                return User(name="Test")
            
            @fixture(scope=FixtureScope.CLASS)
            def admin(self) -> User:
                return User(name="Admin", role="admin")
    """
    
    _fixtures: dict[str, Any]
    
    def __init__(self) -> None:
        self._fixtures = {}
        self._setup_complete = False
    
    def setup(self) -> None:
        """Override to perform setup before tests."""
        pass
    
    def teardown(self) -> None:
        """Override to perform cleanup after tests."""
        pass
    
    def __enter__(self) -> "TestFixture":
        self.setup()
        self._setup_complete = True
        return self
    
    def __exit__(self, *args: Any) -> None:
        self.teardown()
        self._fixtures.clear()
    
    def get_fixture(self, name: str) -> Any:
        """Get a fixture by name."""
        if name not in self._fixtures:
            method = getattr(self, name, None)
            if method and hasattr(method, "_fixture"):
                self._fixtures[name] = method()
        return self._fixtures.get(name)
    
    def set_fixture(self, name: str, value: Any) -> None:
        """Manually set a fixture value."""
        self._fixtures[name] = value


@contextmanager
def use_fixtures(*fixture_classes: type[TestFixture]) -> Generator[dict[str, Any], None, None]:
    """Context manager to use multiple fixtures.
    
    Example:
        with use_fixtures(UserFixture, DatabaseFixture) as fixtures:
            user = fixtures["user"]
            db = fixtures["database"]
    """
    instances = [cls() for cls in fixture_classes]
    all_fixtures: dict[str, Any] = {}
    
    try:
        for instance in instances:
            instance.setup()
            # Collect all fixtures from instance
            for name in dir(instance):
                method = getattr(instance, name)
                if hasattr(method, "_fixture"):
                    all_fixtures[name] = method()
        yield all_fixtures
    finally:
        for instance in reversed(instances):
            instance.teardown()
