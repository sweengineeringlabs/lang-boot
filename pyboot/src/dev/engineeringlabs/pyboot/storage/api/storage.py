"""Storage interface definition."""

from abc import ABC, abstractmethod
from typing import AsyncIterator

from dev.engineeringlabs.pyboot.storage.api.models import FileInfo


class Storage(ABC):
    """
    Abstract storage interface.

    Provides async methods for file operations.

    Example:
        storage = LocalStorage("/data")

        # Write
        await storage.write("path/to/file.txt", b"content")

        # Read
        content = await storage.read("path/to/file.txt")

        # List
        async for file_info in storage.list("path/"):
            print(file_info.name)

        # Delete
        await storage.delete("path/to/file.txt")
    """

    @property
    @abstractmethod
    def name(self) -> str:
        """Get the storage name."""
        ...

    @abstractmethod
    async def read(self, path: str) -> bytes:
        """
        Read file contents.

        Args:
            path: File path

        Returns:
            File contents as bytes

        Raises:
            StorageError: If file doesn't exist or read fails
        """
        ...

    @abstractmethod
    async def write(self, path: str, content: bytes) -> None:
        """
        Write file contents.

        Args:
            path: File path
            content: File contents

        Raises:
            StorageError: If write fails
        """
        ...

    @abstractmethod
    async def delete(self, path: str) -> bool:
        """
        Delete a file.

        Args:
            path: File path

        Returns:
            True if deleted, False if not found
        """
        ...

    @abstractmethod
    async def exists(self, path: str) -> bool:
        """
        Check if a file exists.

        Args:
            path: File path

        Returns:
            True if exists, False otherwise
        """
        ...

    @abstractmethod
    async def info(self, path: str) -> FileInfo | None:
        """
        Get file information.

        Args:
            path: File path

        Returns:
            FileInfo or None if not found
        """
        ...

    @abstractmethod
    def list(self, prefix: str = "") -> AsyncIterator[FileInfo]:
        """
        List files with optional prefix.

        Args:
            prefix: Path prefix to filter by

        Yields:
            FileInfo for each matching file
        """
        ...

    async def read_text(self, path: str, encoding: str = "utf-8") -> str:
        """Read file as text."""
        content = await self.read(path)
        return content.decode(encoding)

    async def write_text(
        self,
        path: str,
        content: str,
        encoding: str = "utf-8",
    ) -> None:
        """Write text to file."""
        await self.write(path, content.encode(encoding))

    async def copy(self, source: str, destination: str) -> None:
        """Copy a file."""
        content = await self.read(source)
        await self.write(destination, content)

    async def move(self, source: str, destination: str) -> None:
        """Move a file."""
        await self.copy(source, destination)
        await self.delete(source)


__all__ = ["Storage"]
