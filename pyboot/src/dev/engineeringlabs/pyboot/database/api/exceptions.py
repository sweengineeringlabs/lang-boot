"""Database exceptions."""

from typing import Any


class DatabaseError(Exception):
    """Base exception for database errors."""

    def __init__(
        self,
        message: str,
        query: str | None = None,
        cause: Exception | None = None,
    ) -> None:
        super().__init__(message)
        self.message = message
        self.query = query
        self.cause = cause

    def __str__(self) -> str:
        if self.query:
            return f"{self.message} [Query: {self.query[:100]}...]"
        return self.message


class ConnectionError(DatabaseError):
    """Raised when connection fails."""

    def __init__(
        self,
        message: str = "Database connection failed",
        host: str | None = None,
        cause: Exception | None = None,
    ) -> None:
        super().__init__(message, cause=cause)
        self.host = host


class QueryError(DatabaseError):
    """Raised when a query fails."""

    def __init__(
        self,
        message: str,
        query: str,
        params: tuple[Any, ...] | None = None,
        cause: Exception | None = None,
    ) -> None:
        super().__init__(message, query=query, cause=cause)
        self.params = params


class IntegrityError(DatabaseError):
    """Raised when an integrity constraint is violated."""

    def __init__(
        self,
        message: str,
        constraint: str | None = None,
        cause: Exception | None = None,
    ) -> None:
        super().__init__(message, cause=cause)
        self.constraint = constraint


class TransactionError(DatabaseError):
    """Raised when a transaction operation fails."""
    pass


class PoolExhaustedError(DatabaseError):
    """Raised when the connection pool is exhausted."""

    def __init__(
        self,
        message: str = "Connection pool exhausted",
        pool_size: int | None = None,
        timeout: float | None = None,
        cause: Exception | None = None,
    ) -> None:
        super().__init__(message, cause=cause)
        self.pool_size = pool_size
        self.timeout = timeout


__all__ = [
    "DatabaseError",
    "ConnectionError",
    "QueryError",
    "IntegrityError",
    "TransactionError",
    "PoolExhaustedError",
]
