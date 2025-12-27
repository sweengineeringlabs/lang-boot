"""
PyBoot Examples - Async Utilities

Demonstrates async task management and execution.
"""

import asyncio
from dev.engineeringlabs.pyboot.async_utils import (
    spawn,
    spawn_blocking,
    gather,
    TaskHandle,
)
from dev.engineeringlabs.pyboot.async_utils.core import TaskExecutor, TaskPool


async def main():
    # Example 1: Spawn async tasks
    print("=" * 50)
    print("Example 1: Spawn Async Tasks")
    print("=" * 50)
    
    async def fetch_data(id: int) -> dict:
        await asyncio.sleep(0.1)  # Simulate IO
        return {"id": id, "data": f"result_{id}"}
    
    handle = await spawn(fetch_data(1))
    result = await handle.result()
    print(f"Task result: {result}")
    print()

    # Example 2: Gather multiple tasks
    print("=" * 50)
    print("Example 2: Gather Multiple Tasks")
    print("=" * 50)
    
    async def process(n: int) -> int:
        await asyncio.sleep(0.05)
        return n * 2
    
    results = await gather(process(1), process(2), process(3), process(4), process(5))
    print(f"Gathered results: {results}")
    print()

    # Example 3: Spawn blocking function
    print("=" * 50)
    print("Example 3: Spawn Blocking Function")
    print("=" * 50)
    
    import time
    
    def cpu_bound_task(n: int) -> int:
        """Simulate CPU-bound work."""
        time.sleep(0.1)
        return sum(range(n))
    
    result = await spawn_blocking(cpu_bound_task, 1000)
    print(f"Blocking task result: {result}")
    print()

    # Example 4: Task Pool
    print("=" * 50)
    print("Example 4: Task Pool (Concurrency Control)")
    print("=" * 50)
    
    pool = TaskPool(max_concurrent=3)
    
    async def slow_task(id: int) -> str:
        print(f"  Task {id} starting...")
        await asyncio.sleep(0.2)
        print(f"  Task {id} done")
        return f"result_{id}"
    
    # Submit 5 tasks but only 3 run at a time
    for i in range(5):
        await pool.submit(slow_task(i))
    
    results = await pool.gather_all()
    print(f"All results: {results}")
    print()

    # Example 5: Task Executor
    print("=" * 50)
    print("Example 5: Task Executor")
    print("=" * 50)
    
    executor = TaskExecutor(max_workers=2)
    
    async def async_work() -> str:
        await asyncio.sleep(0.1)
        return "async done"
    
    def sync_work() -> str:
        time.sleep(0.1)
        return "sync done"
    
    async_result = await executor.run(async_work())
    sync_result = await executor.run_blocking(sync_work)
    
    print(f"Async result: {async_result}")
    print(f"Sync result: {sync_result}")
    
    executor.shutdown()


if __name__ == "__main__":
    asyncio.run(main())
