"""
Storage Module - File storage abstractions.

This module provides:
- Storage interface for file operations
- Local filesystem storage
- SPI for cloud storage providers (S3, GCS, etc.)

Example:
    from dev.engineeringlabs.pyboot.storage import LocalStorage, get_storage

    # Using local storage
    storage = LocalStorage("/data/uploads")
    await storage.write("file.txt", b"Hello, World!")
    content = await storage.read("file.txt")

    # List files
    files = await storage.list("documents/")
"""

from dev.engineeringlabs.pyboot.storage.api import (
    Storage,
    StorageConfig,
    FileInfo,
    StorageError,
)

from dev.engineeringlabs.pyboot.storage.core import (
    LocalStorage,
    get_storage,
    set_storage,
)

from dev.engineeringlabs.pyboot.storage.spi import (
    StorageProvider,
)

__all__ = [
    # API
    "Storage",
    "StorageConfig",
    "FileInfo",
    "StorageError",
    # Core
    "LocalStorage",
    "get_storage",
    "set_storage",
    # SPI
    "StorageProvider",
]
