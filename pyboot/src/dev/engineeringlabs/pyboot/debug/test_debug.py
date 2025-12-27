"""Tests for debug module."""

import pytest
import time
import io
import sys
from dev.engineeringlabs.pyboot.debug import (
    debug_log,
    timed,
    memory_usage,
    Timer,
    Profiler,
    DebugLevel,
    DebugConfig,
)


class TestDebugLog:
    """Tests for debug_log function."""
    
    def test_debug_log_outputs(self, capsys):
        """Test debug_log outputs to stderr."""
        debug_log("Test message", level=DebugLevel.INFO)
        captured = capsys.readouterr()
        assert "Test message" in captured.err
    
    def test_debug_log_includes_level(self, capsys):
        """Test output includes level."""
        debug_log("Message", level=DebugLevel.WARN)
        captured = capsys.readouterr()
        assert "WARN" in captured.err
    
    def test_debug_log_with_context(self, capsys):
        """Test output includes context."""
        debug_log("Message", user="alice", id=123)
        captured = capsys.readouterr()
        assert "user=alice" in captured.err
        assert "id=123" in captured.err


class TestTimed:
    """Tests for timed decorator."""
    
    def test_timed_returns_result(self):
        """Test timed decorator passes through result."""
        @timed
        def add(a, b):
            return a + b
        
        result = add(2, 3)
        assert result == 5
    
    def test_timed_preserves_function_name(self):
        """Test timed preserves function name."""
        @timed
        def my_function():
            pass
        
        assert my_function.__name__ == "my_function"
    
    def test_timed_logs_time(self, capsys):
        """Test timed logs execution time."""
        @timed
        def wait():
            time.sleep(0.01)
        
        wait()
        captured = capsys.readouterr()
        assert "wait" in captured.err
        assert "elapsed" in captured.err.lower() or "ms" in captured.err


class TestTimer:
    """Tests for Timer context manager."""
    
    def test_timer_measures_time(self):
        """Test Timer measures elapsed time."""
        with Timer() as timer:
            time.sleep(0.05)
        
        assert timer.elapsed >= 0.04
        assert timer.elapsed < 0.15
    
    def test_timer_with_name(self, capsys):
        """Test Timer with name outputs."""
        with Timer("my_operation"):
            pass
        
        captured = capsys.readouterr()
        assert "my_operation" in captured.err


class TestProfiler:
    """Tests for Profiler."""
    
    def test_record_timing(self):
        """Test recording timings."""
        profiler = Profiler()
        profiler.record("operation", 0.1)
        profiler.record("operation", 0.2)
        
        summary = profiler.summary()
        assert "operation" in summary
        assert summary["operation"]["count"] == 2
    
    def test_summary_statistics(self):
        """Test summary statistics."""
        profiler = Profiler()
        profiler.record("test", 0.1)
        profiler.record("test", 0.2)
        profiler.record("test", 0.3)
        
        summary = profiler.summary()
        stats = summary["test"]
        
        assert stats["count"] == 3
        assert abs(stats["avg_ms"] - 200) < 1  # ~200ms average
        assert stats["min_ms"] == pytest.approx(100, abs=1)
        assert stats["max_ms"] == pytest.approx(300, abs=1)
    
    def test_multiple_operations(self):
        """Test tracking multiple operations."""
        profiler = Profiler()
        profiler.record("op_a", 0.1)
        profiler.record("op_b", 0.2)
        profiler.record("op_a", 0.15)
        
        summary = profiler.summary()
        assert len(summary) == 2
        assert summary["op_a"]["count"] == 2
        assert summary["op_b"]["count"] == 1


class TestMemoryUsage:
    """Tests for memory_usage function."""
    
    def test_returns_dict(self):
        """Test returns dictionary."""
        result = memory_usage()
        assert isinstance(result, dict)


class TestDebugLevel:
    """Tests for DebugLevel enum."""
    
    def test_all_levels_exist(self):
        """Test all expected levels exist."""
        assert DebugLevel.TRACE
        assert DebugLevel.DEBUG
        assert DebugLevel.INFO
        assert DebugLevel.WARN
        assert DebugLevel.ERROR
    
    def test_level_ordering(self):
        """Test levels are ordered."""
        assert DebugLevel.TRACE.value < DebugLevel.DEBUG.value
        assert DebugLevel.DEBUG.value < DebugLevel.INFO.value
        assert DebugLevel.INFO.value < DebugLevel.WARN.value
        assert DebugLevel.WARN.value < DebugLevel.ERROR.value


class TestDebugConfig:
    """Tests for DebugConfig."""
    
    def test_default_values(self):
        """Test default config values."""
        config = DebugConfig()
        assert config.level == DebugLevel.DEBUG
        assert config.enabled is True
    
    def test_custom_values(self):
        """Test custom config values."""
        config = DebugConfig(level=DebugLevel.INFO, enabled=False)
        assert config.level == DebugLevel.INFO
        assert config.enabled is False
