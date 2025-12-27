"""Connection pool implementation."""

import asyncio
from collections import deque
from contextlib import asynccontextmanager
from typing import Any, AsyncIterator

from dev.engineeringlabs.pyboot.database.api.config import DatabaseConfig
from dev.engineeringlabs.pyboot.database.api.database import Database
from dev.engineeringlabs.pyboot.database.api.connection import Connection, Transaction
from dev.engineeringlabs.pyboot.database.api.exceptions import PoolExhaustedError


class PooledConnection(Connection):
    """A connection from the pool."""

    def __init__(self, raw_conn: Any, pool: "ConnectionPool") -> None:
        self._raw = raw_conn
        self._pool = pool
        self._in_use = True

    async def execute(self, query: str, *args: Any) -> Any:
        """Execute a query."""
        return await self._raw.execute(query, *args)

    async def fetch_one(self, query: str, *args: Any) -> Any | None:
        """Fetch a single row."""
        return await self._raw.fetchrow(query, *args)

    async def fetch_all(self, query: str, *args: Any) -> list[Any]:
        """Fetch all rows."""
        return await self._raw.fetch(query, *args)

    async def fetch_val(self, query: str, *args: Any) -> Any | None:
        """Fetch a single value."""
        return await self._raw.fetchval(query, *args)

    async def close(self) -> None:
        """Return connection to pool."""
        self._in_use = False
        await self._pool._return_connection(self)


class PooledTransaction(Transaction):
    """A transaction from a pooled connection."""

    def __init__(self, conn: PooledConnection, raw_tx: Any) -> None:
        self._conn = conn
        self._raw = raw_tx
        self._committed = False
        self._rolled_back = False

    @property
    def connection(self) -> Connection:
        """Get the underlying connection."""
        return self._conn

    async def execute(self, query: str, *args: Any) -> Any:
        """Execute within transaction."""
        return await self._raw.execute(query, *args)

    async def fetch_one(self, query: str, *args: Any) -> Any | None:
        """Fetch a single row."""
        return await self._raw.fetchrow(query, *args)

    async def fetch_all(self, query: str, *args: Any) -> list[Any]:
        """Fetch all rows."""
        return await self._raw.fetch(query, *args)

    async def commit(self) -> None:
        """Commit the transaction."""
        if not self._committed and not self._rolled_back:
            await self._raw.commit()
            self._committed = True

    async def rollback(self) -> None:
        """Rollback the transaction."""
        if not self._committed and not self._rolled_back:
            await self._raw.rollback()
            self._rolled_back = True


class ConnectionPool(Database):
    """
    Connection pool implementation.

    Manages a pool of database connections for efficient reuse.

    Example:
        pool = ConnectionPool(
            config=DatabaseConfig(
                url="postgresql://user:pass@localhost/db",
                pool_size=10,
            )
        )

        async with pool:
            async with pool.connection() as conn:
                result = await conn.fetch_all("SELECT * FROM users")
    """

    def __init__(self, config: DatabaseConfig) -> None:
        self._config = config
        self._pool: deque[Any] = deque()
        self._in_use: set[Any] = set()
        self._lock = asyncio.Lock()
        self._connected = False
        self._raw_pool: Any = None

    @property
    def config(self) -> DatabaseConfig:
        """Get the database configuration."""
        return self._config

    @property
    def is_connected(self) -> bool:
        """Check if database is connected."""
        return self._connected

    @property
    def pool_size(self) -> int:
        """Get current pool size."""
        return len(self._pool) + len(self._in_use)

    @property
    def available(self) -> int:
        """Get number of available connections."""
        return len(self._pool)

    async def connect(self) -> None:
        """Connect to the database and initialize pool."""
        if self._connected:
            return

        # This is a stub - actual implementation would use
        # the appropriate driver (asyncpg, aiomysql, etc.)
        self._connected = True

    async def disconnect(self) -> None:
        """Disconnect and close all connections."""
        if not self._connected:
            return

        async with self._lock:
            # Close all connections
            for conn in list(self._pool):
                try:
                    await conn.close()
                except Exception:
                    pass
            self._pool.clear()

            for conn in list(self._in_use):
                try:
                    await conn.close()
                except Exception:
                    pass
            self._in_use.clear()

            if self._raw_pool:
                await self._raw_pool.close()
                self._raw_pool = None

            self._connected = False

    @asynccontextmanager
    async def connection(self) -> AsyncIterator[Connection]:
        """Get a connection from the pool."""
        conn = await self._acquire()
        try:
            yield PooledConnection(conn, self)
        finally:
            await self._return_connection_raw(conn)

    @asynccontextmanager
    async def transaction(self) -> AsyncIterator[Transaction]:
        """Start a transaction."""
        async with self.connection() as conn:
            # Start transaction
            # This is a stub - actual implementation depends on driver
            tx = PooledTransaction(conn, conn)
            try:
                yield tx
            except Exception:
                await tx.rollback()
                raise
            else:
                await tx.commit()

    async def execute(self, query: str, *args: Any) -> Any:
        """Execute a query."""
        async with self.connection() as conn:
            return await conn.execute(query, *args)

    async def fetch_one(self, query: str, *args: Any) -> Any | None:
        """Fetch a single row."""
        async with self.connection() as conn:
            return await conn.fetch_one(query, *args)

    async def fetch_all(self, query: str, *args: Any) -> list[Any]:
        """Fetch all rows."""
        async with self.connection() as conn:
            return await conn.fetch_all(query, *args)

    async def _acquire(self) -> Any:
        """Acquire a connection from the pool."""
        async with self._lock:
            # Try to get from pool
            if self._pool:
                conn = self._pool.popleft()
                self._in_use.add(conn)
                return conn

            # Check if we can create a new connection
            current_size = len(self._in_use)
            max_size = self._config.pool_size + self._config.max_overflow

            if current_size >= max_size:
                raise PoolExhaustedError(
                    pool_size=self._config.pool_size,
                    timeout=self._config.pool_timeout,
                )

            # Create new connection (stub)
            conn = object()  # Placeholder
            self._in_use.add(conn)
            return conn

    async def _return_connection(self, conn: PooledConnection) -> None:
        """Return a pooled connection."""
        await self._return_connection_raw(conn._raw)

    async def _return_connection_raw(self, conn: Any) -> None:
        """Return a raw connection to the pool."""
        async with self._lock:
            if conn in self._in_use:
                self._in_use.remove(conn)

                # Return to pool if not at max size
                if len(self._pool) < self._config.pool_size:
                    self._pool.append(conn)


__all__ = ["ConnectionPool"]
