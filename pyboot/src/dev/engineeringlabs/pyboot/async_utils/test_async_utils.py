"""Tests for async_utils module."""

import pytest
import asyncio
from dev.engineeringlabs.pyboot.async_utils import spawn, spawn_blocking, gather, TaskHandle
from dev.engineeringlabs.pyboot.async_utils.core import TaskExecutor, TaskPool


class TestSpawn:
    """Tests for spawn function."""
    
    @pytest.mark.asyncio
    async def test_spawn_returns_handle(self):
        """Test spawn returns TaskHandle."""
        async def simple():
            return 42
        
        handle = await spawn(simple())
        assert isinstance(handle, TaskHandle)
    
    @pytest.mark.asyncio
    async def test_spawn_result(self):
        """Test getting result from spawned task."""
        async def compute():
            await asyncio.sleep(0.01)
            return "done"
        
        handle = await spawn(compute())
        result = await handle.result()
        assert result == "done"
    
    @pytest.mark.asyncio
    async def test_spawn_cancel(self):
        """Test canceling spawned task."""
        async def long_running():
            await asyncio.sleep(10)
            return "finished"
        
        handle = await spawn(long_running())
        cancelled = handle.cancel()
        assert cancelled
    
    @pytest.mark.asyncio
    async def test_spawn_done(self):
        """Test checking if task is done."""
        async def quick():
            return 1
        
        handle = await spawn(quick())
        await asyncio.sleep(0.05)
        assert handle.done()


class TestSpawnBlocking:
    """Tests for spawn_blocking function."""
    
    @pytest.mark.asyncio
    async def test_spawn_blocking_result(self):
        """Test running blocking function."""
        def cpu_work(n):
            return sum(range(n))
        
        result = await spawn_blocking(cpu_work, 100)
        assert result == sum(range(100))
    
    @pytest.mark.asyncio
    async def test_spawn_blocking_with_kwargs(self):
        """Test blocking function with kwargs."""
        def greet(name, greeting="Hello"):
            return f"{greeting}, {name}!"
        
        result = await spawn_blocking(greet, "World", greeting="Hi")
        assert result == "Hi, World!"


class TestGather:
    """Tests for gather function."""
    
    @pytest.mark.asyncio
    async def test_gather_multiple(self):
        """Test gathering multiple coroutines."""
        async def add(a, b):
            return a + b
        
        results = await gather(add(1, 2), add(3, 4), add(5, 6))
        assert results == [3, 7, 11]
    
    @pytest.mark.asyncio
    async def test_gather_preserves_order(self):
        """Test gather preserves order."""
        async def delayed(value, delay):
            await asyncio.sleep(delay)
            return value
        
        results = await gather(
            delayed("a", 0.03),
            delayed("b", 0.01),
            delayed("c", 0.02),
        )
        assert results == ["a", "b", "c"]


class TestTaskPool:
    """Tests for TaskPool."""
    
    @pytest.mark.asyncio
    async def test_pool_limits_concurrency(self):
        """Test pool limits concurrent tasks."""
        pool = TaskPool(max_concurrent=2)
        running = []
        
        async def track():
            running.append(1)
            await asyncio.sleep(0.05)
            running.pop()
            return True
        
        for _ in range(4):
            await pool.submit(track())
        
        await pool.gather_all()
    
    @pytest.mark.asyncio
    async def test_pool_gather_all(self):
        """Test gathering all pool results."""
        pool = TaskPool(max_concurrent=3)
        
        async def square(n):
            return n * n
        
        for i in range(5):
            await pool.submit(square(i))
        
        results = await pool.gather_all()
        assert sorted(results) == [0, 1, 4, 9, 16]


class TestTaskExecutor:
    """Tests for TaskExecutor."""
    
    @pytest.mark.asyncio
    async def test_executor_run(self):
        """Test running async task."""
        executor = TaskExecutor()
        
        async def task():
            return "result"
        
        result = await executor.run(task())
        assert result == "result"
        executor.shutdown()
    
    @pytest.mark.asyncio
    async def test_executor_run_blocking(self):
        """Test running blocking task."""
        executor = TaskExecutor(max_workers=1)
        
        def blocking():
            return 42
        
        result = await executor.run_blocking(blocking)
        assert result == 42
        executor.shutdown()
