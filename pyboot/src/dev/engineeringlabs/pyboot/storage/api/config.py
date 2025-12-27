"""Storage configuration."""

from dataclasses import dataclass


@dataclass(frozen=True, slots=True)
class StorageConfig:
    """Configuration for storage.

    Attributes:
        base_path: Base path for file operations
        create_directories: Auto-create directories
        max_file_size: Maximum file size in bytes (None = unlimited)
    """

    base_path: str = ""
    create_directories: bool = True
    max_file_size: int | None = None

    @classmethod
    def default(cls) -> "StorageConfig":
        """Get default configuration."""
        return cls()


__all__ = ["StorageConfig"]
