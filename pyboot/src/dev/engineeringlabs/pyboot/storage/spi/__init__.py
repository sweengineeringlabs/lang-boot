"""Storage SPI layer - Service Provider Interface for storage backends."""

from abc import ABC, abstractmethod
from typing import Any

from dev.engineeringlabs.pyboot.storage.api.storage import Storage


class StorageProvider(ABC):
    """
    Abstract interface for storage backend providers.

    Implement this to create custom storage backends like:
    - Amazon S3
    - Google Cloud Storage
    - Azure Blob Storage
    - MinIO

    Example:
        class S3StorageProvider(StorageProvider):
            def __init__(self, s3_client, bucket: str):
                self._client = s3_client
                self._bucket = bucket

            @property
            def name(self) -> str:
                return f"s3:{self._bucket}"

            def create_storage(self, prefix: str = "") -> Storage:
                return S3Storage(self._client, self._bucket, prefix)
    """

    @property
    @abstractmethod
    def name(self) -> str:
        """Get the provider name."""
        ...

    @abstractmethod
    def create_storage(self, config: Any = None) -> Storage:
        """
        Create a storage instance.

        Args:
            config: Optional configuration

        Returns:
            Storage instance
        """
        ...

    def is_available(self) -> bool:
        """Check if this provider is available/configured."""
        return True


__all__ = ["StorageProvider"]
