"""Local filesystem storage implementation."""

import os
from pathlib import Path
from typing import AsyncIterator

from dev.engineeringlabs.pyboot.storage.api.storage import Storage
from dev.engineeringlabs.pyboot.storage.api.config import StorageConfig
from dev.engineeringlabs.pyboot.storage.api.models import FileInfo
from dev.engineeringlabs.pyboot.storage.api.exceptions import StorageError, FileNotFoundError, FileTooLargeError


class LocalStorage(Storage):
    """
    Local filesystem storage implementation.

    Example:
        storage = LocalStorage("/data/uploads")
        await storage.write("documents/report.pdf", pdf_bytes)
        content = await storage.read("documents/report.pdf")
    """

    def __init__(
        self,
        base_path: str | Path,
        config: StorageConfig | None = None,
    ) -> None:
        self._base_path = Path(base_path)
        self._config = config or StorageConfig.default()
        self._name = f"local:{base_path}"

        # Create base directory if needed
        if self._config.create_directories and not self._base_path.exists():
            self._base_path.mkdir(parents=True, exist_ok=True)

    @property
    def name(self) -> str:
        """Get the storage name."""
        return self._name

    @property
    def base_path(self) -> Path:
        """Get the base path."""
        return self._base_path

    def _resolve_path(self, path: str) -> Path:
        """Resolve a path relative to base."""
        # Prevent path traversal
        resolved = (self._base_path / path).resolve()
        if not str(resolved).startswith(str(self._base_path.resolve())):
            raise StorageError(f"Path traversal detected: {path}", path=path)
        return resolved

    async def read(self, path: str) -> bytes:
        """Read file contents."""
        try:
            import aiofiles
        except ImportError:
            # Fallback to sync
            file_path = self._resolve_path(path)
            if not file_path.exists():
                raise FileNotFoundError(path)
            return file_path.read_bytes()

        file_path = self._resolve_path(path)
        if not file_path.exists():
            raise FileNotFoundError(path)

        async with aiofiles.open(file_path, "rb") as f:
            return await f.read()

    async def write(self, path: str, content: bytes) -> None:
        """Write file contents."""
        # Check file size
        if (
            self._config.max_file_size is not None
            and len(content) > self._config.max_file_size
        ):
            raise FileTooLargeError(path, len(content), self._config.max_file_size)

        file_path = self._resolve_path(path)

        # Create parent directories
        if self._config.create_directories:
            file_path.parent.mkdir(parents=True, exist_ok=True)

        try:
            import aiofiles
        except ImportError:
            # Fallback to sync
            file_path.write_bytes(content)
            return

        async with aiofiles.open(file_path, "wb") as f:
            await f.write(content)

    async def delete(self, path: str) -> bool:
        """Delete a file."""
        file_path = self._resolve_path(path)
        if not file_path.exists():
            return False
        file_path.unlink()
        return True

    async def exists(self, path: str) -> bool:
        """Check if a file exists."""
        file_path = self._resolve_path(path)
        return file_path.exists()

    async def info(self, path: str) -> FileInfo | None:
        """Get file information."""
        file_path = self._resolve_path(path)
        if not file_path.exists():
            return None

        stat = file_path.stat()
        return FileInfo(
            path=path,
            name=file_path.name,
            size=stat.st_size,
            modified_at=stat.st_mtime,
            created_at=stat.st_ctime,
        )

    async def list(self, prefix: str = "") -> AsyncIterator[FileInfo]:
        """List files with optional prefix."""
        search_path = self._resolve_path(prefix) if prefix else self._base_path

        if not search_path.exists():
            return

        for item in search_path.rglob("*"):
            if item.is_file():
                rel_path = str(item.relative_to(self._base_path))
                stat = item.stat()
                yield FileInfo(
                    path=rel_path,
                    name=item.name,
                    size=stat.st_size,
                    modified_at=stat.st_mtime,
                    created_at=stat.st_ctime,
                )


__all__ = ["LocalStorage"]
