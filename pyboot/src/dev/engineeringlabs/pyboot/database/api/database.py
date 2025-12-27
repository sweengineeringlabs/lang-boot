"""Database interface."""

from abc import ABC, abstractmethod
from typing import Any, AsyncContextManager

from dev.engineeringlabs.pyboot.database.api.config import DatabaseConfig
from dev.engineeringlabs.pyboot.database.api.connection import Connection, Transaction


class Database(ABC):
    """
    Abstract database interface.

    Example:
        db = get_database()

        # Get connection
        async with db.connection() as conn:
            result = await conn.execute("SELECT * FROM users")

        # Transaction
        async with db.transaction() as tx:
            await tx.execute("INSERT INTO users (name) VALUES ($1)", "John")
    """

    @property
    @abstractmethod
    def config(self) -> DatabaseConfig:
        """Get the database configuration."""
        ...

    @property
    @abstractmethod
    def is_connected(self) -> bool:
        """Check if database is connected."""
        ...

    @abstractmethod
    async def connect(self) -> None:
        """Connect to the database."""
        ...

    @abstractmethod
    async def disconnect(self) -> None:
        """Disconnect from the database."""
        ...

    @abstractmethod
    def connection(self) -> AsyncContextManager[Connection]:
        """
        Get a connection from the pool.

        Returns:
            Connection context manager
        """
        ...

    @abstractmethod
    def transaction(self) -> AsyncContextManager[Transaction]:
        """
        Start a transaction.

        Returns:
            Transaction context manager
        """
        ...

    @abstractmethod
    async def execute(self, query: str, *args: Any) -> Any:
        """
        Execute a query.

        Args:
            query: SQL query
            *args: Query parameters

        Returns:
            Query result
        """
        ...

    @abstractmethod
    async def fetch_one(self, query: str, *args: Any) -> Any | None:
        """
        Fetch a single row.

        Args:
            query: SQL query
            *args: Query parameters

        Returns:
            Row or None
        """
        ...

    @abstractmethod
    async def fetch_all(self, query: str, *args: Any) -> list[Any]:
        """
        Fetch all rows.

        Args:
            query: SQL query
            *args: Query parameters

        Returns:
            List of rows
        """
        ...

    async def __aenter__(self) -> "Database":
        await self.connect()
        return self

    async def __aexit__(self, *args: Any) -> None:
        await self.disconnect()


__all__ = ["Database"]
