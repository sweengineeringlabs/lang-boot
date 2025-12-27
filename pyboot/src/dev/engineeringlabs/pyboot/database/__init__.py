"""
Database Module - Connection pooling and database utilities.

This module provides:
- Connection pool management
- Transaction context managers
- Database interface abstractions
- SPI for database drivers

Example:
    from dev.engineeringlabs.pyboot.database import Database, DatabaseConfig

    # Configure database
    db = Database(
        config=DatabaseConfig(
            url="postgresql://user:pass@localhost/db",
            pool_size=10,
        )
    )

    # Use connection
    async with db.connection() as conn:
        result = await conn.execute("SELECT * FROM users")
        users = await result.fetchall()

    # Use transaction
    async with db.transaction() as tx:
        await tx.execute("INSERT INTO users (name) VALUES ($1)", "John")
        await tx.execute("INSERT INTO logs (action) VALUES ($1)", "user_created")
"""

from dev.engineeringlabs.pyboot.database.api import (
    Database,
    DatabaseConfig,
    Connection,
    Transaction,
    Row,
    DatabaseError,
)

from dev.engineeringlabs.pyboot.database.core import (
    ConnectionPool,
    get_database,
    set_database,
)

from dev.engineeringlabs.pyboot.database.spi import (
    DatabaseDriver,
)

__all__ = [
    # API
    "Database",
    "DatabaseConfig",
    "Connection",
    "Transaction",
    "Row",
    "DatabaseError",
    # Core
    "ConnectionPool",
    "get_database",
    "set_database",
    # SPI
    "DatabaseDriver",
]
