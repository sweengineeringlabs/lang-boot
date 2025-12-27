"""Database SPI layer - Service Provider Interface for database drivers."""

from abc import ABC, abstractmethod
from typing import Any, AsyncContextManager

from dev.engineeringlabs.pyboot.database.api.config import DatabaseConfig
from dev.engineeringlabs.pyboot.database.api.connection import Connection


class DatabaseDriver(ABC):
    """
    Abstract interface for database drivers.

    Implement this to create support for specific databases:
    - PostgreSQL (asyncpg)
    - MySQL (aiomysql)
    - SQLite (aiosqlite)
    - SQL Server

    Example:
        class AsyncpgDriver(DatabaseDriver):
            @property
            def name(self) -> str:
                return "asyncpg"

            @property
            def database_type(self) -> str:
                return "postgresql"

            async def connect(self, config: DatabaseConfig) -> Any:
                import asyncpg
                return await asyncpg.create_pool(config.url)

            async def disconnect(self, pool: Any) -> None:
                await pool.close()

            def connection(self, pool: Any) -> AsyncContextManager[Connection]:
                return pool.acquire()
    """

    @property
    @abstractmethod
    def name(self) -> str:
        """Get the driver name (e.g., 'asyncpg', 'aiomysql')."""
        ...

    @property
    @abstractmethod
    def database_type(self) -> str:
        """Get the database type (e.g., 'postgresql', 'mysql')."""
        ...

    @abstractmethod
    async def connect(self, config: DatabaseConfig) -> Any:
        """
        Create a connection pool.

        Args:
            config: Database configuration

        Returns:
            Connection pool or similar
        """
        ...

    @abstractmethod
    async def disconnect(self, pool: Any) -> None:
        """
        Close a connection pool.

        Args:
            pool: Connection pool
        """
        ...

    @abstractmethod
    def connection(self, pool: Any) -> AsyncContextManager[Connection]:
        """
        Get a connection from the pool.

        Args:
            pool: Connection pool

        Returns:
            Connection context manager
        """
        ...

    def is_available(self) -> bool:
        """Check if this driver is available."""
        return True


__all__ = ["DatabaseDriver"]
