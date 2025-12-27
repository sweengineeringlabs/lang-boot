"""Database configuration."""

from dataclasses import dataclass
from urllib.parse import urlparse


@dataclass(frozen=True, slots=True)
class DatabaseConfig:
    """Configuration for database connection.

    Attributes:
        url: Database connection URL (e.g., postgresql://user:pass@host/db)
        pool_size: Connection pool size
        pool_min_size: Minimum pool size
        max_overflow: Maximum connections beyond pool_size
        pool_timeout: Timeout waiting for connection from pool
        connect_timeout: Connection timeout
        command_timeout: Query timeout
        ssl: Enable SSL
        echo: Log SQL queries
    """

    url: str
    pool_size: int = 10
    pool_min_size: int = 2
    max_overflow: int = 5
    pool_timeout: float = 30.0
    connect_timeout: float = 10.0
    command_timeout: float = 30.0
    ssl: bool = False
    echo: bool = False

    @property
    def driver(self) -> str:
        """Extract driver from URL."""
        parsed = urlparse(self.url)
        scheme = parsed.scheme
        if "+" in scheme:
            return scheme.split("+")[1]
        return scheme

    @property
    def database_type(self) -> str:
        """Extract database type from URL."""
        parsed = urlparse(self.url)
        scheme = parsed.scheme
        if "+" in scheme:
            return scheme.split("+")[0]
        return scheme

    @property
    def host(self) -> str:
        """Extract host from URL."""
        parsed = urlparse(self.url)
        return parsed.hostname or "localhost"

    @property
    def port(self) -> int | None:
        """Extract port from URL."""
        parsed = urlparse(self.url)
        return parsed.port

    @property
    def database_name(self) -> str:
        """Extract database name from URL."""
        parsed = urlparse(self.url)
        return parsed.path.lstrip("/")

    @classmethod
    def default(cls, url: str) -> "DatabaseConfig":
        """Get default configuration."""
        return cls(url=url)

    @classmethod
    def for_testing(cls, url: str = "sqlite:///:memory:") -> "DatabaseConfig":
        """Get configuration for testing."""
        return cls(
            url=url,
            pool_size=2,
            pool_min_size=1,
            max_overflow=0,
            echo=True,
        )


__all__ = ["DatabaseConfig"]
