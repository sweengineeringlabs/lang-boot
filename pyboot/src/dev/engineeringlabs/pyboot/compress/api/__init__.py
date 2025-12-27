"""Compress API - Compression types and errors."""

from enum import Enum, auto


class CompressionLevel(Enum):
    """Compression level."""
    FASTEST = auto()
    FAST = auto()
    DEFAULT = auto()
    BEST = auto()


class CompressionFormat(Enum):
    """Compression format."""
    GZIP = "gzip"
    ZSTD = "zstd"
    LZ4 = "lz4"
    BZIP2 = "bzip2"


class CompressError(Exception):
    """Base error for compression operations."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


__all__ = [
    "CompressionLevel",
    "CompressionFormat",
    "CompressError",
]
