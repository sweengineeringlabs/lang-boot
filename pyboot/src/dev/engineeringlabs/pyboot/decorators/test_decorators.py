"""Tests for decorators module."""

import pytest
import warnings
from dev.engineeringlabs.pyboot.decorators import compose, conditional, debug, memoize, once, deprecated


class TestCompose:
    """Tests for compose decorator."""
    
    def test_compose_single_decorator(self):
        """Test compose with single decorator."""
        def add_one(func):
            def wrapper(x):
                return func(x) + 1
            return wrapper
        
        @compose(add_one)
        def identity(x):
            return x
        
        assert identity(5) == 6
    
    def test_compose_multiple_decorators(self):
        """Test compose with multiple decorators."""
        def double(func):
            def wrapper(x):
                return func(x) * 2
            return wrapper
        
        def add_one(func):
            def wrapper(x):
                return func(x) + 1
            return wrapper
        
        # compose(double, add_one) = double(add_one(func))
        # So: double(add_one(identity))(5) = double(6) = 12
        @compose(double, add_one)
        def identity(x):
            return x
        
        assert identity(5) == 12
    
    def test_compose_order_matters(self):
        """Test that decorator order is preserved."""
        results = []
        
        def first(func):
            def wrapper(*args):
                results.append("first")
                return func(*args)
            return wrapper
        
        def second(func):
            def wrapper(*args):
                results.append("second")
                return func(*args)
            return wrapper
        
        @compose(first, second)
        def noop():
            results.append("func")
        
        noop()
        assert results == ["first", "second", "func"]


class TestConditional:
    """Tests for conditional decorator."""
    
    def test_conditional_true(self):
        """Test conditional when condition is True."""
        def add_one(func):
            def wrapper(x):
                return func(x) + 1
            return wrapper
        
        @conditional(True, add_one)
        def identity(x):
            return x
        
        assert identity(5) == 6
    
    def test_conditional_false(self):
        """Test conditional when condition is False."""
        def add_one(func):
            def wrapper(x):
                return func(x) + 1
            return wrapper
        
        @conditional(False, add_one)
        def identity(x):
            return x
        
        assert identity(5) == 5
    
    def test_conditional_callable(self):
        """Test conditional with callable condition."""
        flag = [False]
        
        def add_one(func):
            def wrapper(x):
                return func(x) + 1
            return wrapper
        
        @conditional(lambda: flag[0], add_one)
        def identity(x):
            return x
        
        assert identity(5) == 5  # Condition evaluated at decoration time


class TestMemoize:
    """Tests for memoize decorator."""
    
    def test_memoize_caches_result(self):
        """Test that memoize caches function results."""
        call_count = [0]
        
        @memoize
        def expensive(x):
            call_count[0] += 1
            return x * 2
        
        assert expensive(5) == 10
        assert expensive(5) == 10
        assert call_count[0] == 1  # Only called once
    
    def test_memoize_different_args(self):
        """Test memoize with different arguments."""
        call_count = [0]
        
        @memoize
        def add(a, b):
            call_count[0] += 1
            return a + b
        
        assert add(1, 2) == 3
        assert add(1, 2) == 3
        assert add(2, 3) == 5
        assert call_count[0] == 2
    
    def test_memoize_cache_clear(self):
        """Test memoize cache can be cleared."""
        @memoize
        def identity(x):
            return x
        
        identity(1)
        identity(2)
        assert len(identity.cache) == 2
        
        identity.cache_clear()
        assert len(identity.cache) == 0


class TestOnce:
    """Tests for once decorator."""
    
    def test_once_executes_once(self):
        """Test that once only executes function once."""
        call_count = [0]
        
        @once
        def initialize():
            call_count[0] += 1
            return "initialized"
        
        assert initialize() == "initialized"
        assert initialize() == "initialized"
        assert initialize() == "initialized"
        assert call_count[0] == 1
    
    def test_once_caches_result(self):
        """Test that once returns cached result."""
        results = []
        
        @once
        def get_config():
            config = {"key": "value"}
            results.append(config)
            return config
        
        r1 = get_config()
        r2 = get_config()
        
        assert r1 is r2
        assert len(results) == 1


class TestDeprecated:
    """Tests for deprecated decorator."""
    
    def test_deprecated_warns(self):
        """Test that deprecated shows warning."""
        @deprecated("Use new_func instead")
        def old_func():
            return "result"
        
        with warnings.catch_warnings(record=True) as w:
            warnings.simplefilter("always")
            result = old_func()
            
            assert len(w) == 1
            assert issubclass(w[0].category, DeprecationWarning)
            assert "old_func is deprecated" in str(w[0].message)
            assert result == "result"
    
    def test_deprecated_with_version(self):
        """Test deprecated with version info."""
        @deprecated("Use new_func", version="2.0")
        def old_func():
            return "result"
        
        with warnings.catch_warnings(record=True) as w:
            warnings.simplefilter("always")
            old_func()
            
            assert "since version 2.0" in str(w[0].message)
