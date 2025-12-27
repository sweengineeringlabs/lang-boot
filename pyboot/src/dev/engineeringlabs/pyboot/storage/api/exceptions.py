"""Storage exceptions."""

from typing import Any


class StorageError(Exception):
    """Base exception for storage errors."""

    def __init__(
        self,
        message: str,
        path: str | None = None,
        cause: Exception | None = None,
    ) -> None:
        super().__init__(message)
        self.message = message
        self.path = path
        self.cause = cause

    def __str__(self) -> str:
        if self.path:
            return f"{self.message}: {self.path}"
        return self.message


class FileNotFoundError(StorageError):
    """Raised when a file is not found."""

    def __init__(self, path: str, cause: Exception | None = None) -> None:
        super().__init__(f"File not found: {path}", path=path, cause=cause)


class FileExistsError(StorageError):
    """Raised when a file already exists."""

    def __init__(self, path: str, cause: Exception | None = None) -> None:
        super().__init__(f"File already exists: {path}", path=path, cause=cause)


class PermissionError(StorageError):
    """Raised when permission is denied."""

    def __init__(self, path: str, cause: Exception | None = None) -> None:
        super().__init__(f"Permission denied: {path}", path=path, cause=cause)


class FileTooLargeError(StorageError):
    """Raised when file exceeds size limit."""

    def __init__(
        self,
        path: str,
        size: int,
        max_size: int,
        cause: Exception | None = None,
    ) -> None:
        super().__init__(
            f"File too large: {size} bytes (max: {max_size})",
            path=path,
            cause=cause,
        )
        self.size = size
        self.max_size = max_size


__all__ = [
    "StorageError",
    "FileNotFoundError",
    "FileExistsError",
    "PermissionError",
    "FileTooLargeError",
]
