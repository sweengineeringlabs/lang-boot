"""HTTP configuration."""

from dataclasses import dataclass, field


@dataclass(frozen=True, slots=True)
class HttpConfig:
    """Configuration for HTTP client.

    Attributes:
        timeout: Request timeout in seconds
        connect_timeout: Connection timeout in seconds
        retries: Number of retry attempts
        retry_delay: Initial delay between retries
        retry_backoff: Backoff multiplier for retries
        circuit_breaker: Enable circuit breaker
        circuit_failure_threshold: Failures before opening circuit
        circuit_timeout: Circuit recovery timeout
        follow_redirects: Follow HTTP redirects
        max_redirects: Maximum number of redirects
        verify_ssl: Verify SSL certificates
        headers: Default headers
    """

    timeout: float = 30.0
    connect_timeout: float = 10.0
    retries: int = 3
    retry_delay: float = 1.0
    retry_backoff: float = 2.0
    circuit_breaker: bool = False
    circuit_failure_threshold: int = 5
    circuit_timeout: float = 30.0
    follow_redirects: bool = True
    max_redirects: int = 10
    verify_ssl: bool = True
    headers: dict[str, str] = field(default_factory=dict)

    @classmethod
    def default(cls) -> "HttpConfig":
        """Get default configuration."""
        return cls()

    @classmethod
    def with_resilience(cls) -> "HttpConfig":
        """Get configuration with all resilience features enabled."""
        return cls(
            retries=3,
            retry_delay=1.0,
            retry_backoff=2.0,
            circuit_breaker=True,
            circuit_failure_threshold=5,
            circuit_timeout=30.0,
        )


__all__ = ["HttpConfig"]
