"""Connection and transaction interfaces."""

from abc import ABC, abstractmethod
from typing import Any


class Connection(ABC):
    """
    Database connection interface.

    Example:
        async with db.connection() as conn:
            result = await conn.execute("SELECT * FROM users WHERE id = $1", user_id)
            user = await result.fetchone()
    """

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
        """Fetch a single row."""
        ...

    @abstractmethod
    async def fetch_all(self, query: str, *args: Any) -> list[Any]:
        """Fetch all rows."""
        ...

    @abstractmethod
    async def fetch_val(self, query: str, *args: Any) -> Any | None:
        """Fetch a single value."""
        ...

    @abstractmethod
    async def close(self) -> None:
        """Close the connection."""
        ...


class Transaction(ABC):
    """
    Database transaction interface.

    Example:
        async with db.transaction() as tx:
            await tx.execute("INSERT INTO users (name) VALUES ($1)", "John")
            await tx.execute("INSERT INTO logs (action) VALUES ($1)", "user_created")
            # Automatically commits on success, rolls back on exception
    """

    @property
    @abstractmethod
    def connection(self) -> Connection:
        """Get the underlying connection."""
        ...

    @abstractmethod
    async def execute(self, query: str, *args: Any) -> Any:
        """Execute a query within the transaction."""
        ...

    @abstractmethod
    async def fetch_one(self, query: str, *args: Any) -> Any | None:
        """Fetch a single row."""
        ...

    @abstractmethod
    async def fetch_all(self, query: str, *args: Any) -> list[Any]:
        """Fetch all rows."""
        ...

    @abstractmethod
    async def commit(self) -> None:
        """Commit the transaction."""
        ...

    @abstractmethod
    async def rollback(self) -> None:
        """Rollback the transaction."""
        ...

    async def __aenter__(self) -> "Transaction":
        return self

    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: Any,
    ) -> None:
        if exc_type is not None:
            await self.rollback()
        else:
            await self.commit()


__all__ = ["Connection", "Transaction"]
