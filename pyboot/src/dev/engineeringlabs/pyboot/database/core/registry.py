"""Database registry for global access."""

from dev.engineeringlabs.pyboot.database.api.database import Database
from dev.engineeringlabs.pyboot.database.api.config import DatabaseConfig
from dev.engineeringlabs.pyboot.database.core.pool import ConnectionPool

# Global database registry
_databases: dict[str, Database] = {}
_default_database: Database | None = None


def get_database(name: str | None = None) -> Database:
    """
    Get a database by name.

    Args:
        name: Database name (None = default database)

    Returns:
        Database instance

    Raises:
        KeyError: If database not found
    """
    global _default_database

    if name is None:
        if _default_database is None:
            raise KeyError("No default database configured. Call set_database() first.")
        return _default_database

    if name not in _databases:
        raise KeyError(f"Database not found: {name}")

    return _databases[name]


def set_database(
    database: Database | DatabaseConfig | str,
    name: str | None = None,
) -> Database:
    """
    Register a database.

    Args:
        database: Database instance, config, or URL
        name: Database name (None = set as default)

    Returns:
        Database instance
    """
    global _default_database

    # Convert to Database if needed
    if isinstance(database, str):
        database = ConnectionPool(DatabaseConfig(url=database))
    elif isinstance(database, DatabaseConfig):
        database = ConnectionPool(database)

    if name is None:
        _default_database = database
    else:
        _databases[name] = database

    return database


def clear_databases() -> None:
    """Clear all database registrations."""
    global _default_database
    _databases.clear()
    _default_database = None


__all__ = ["get_database", "set_database", "clear_databases"]
