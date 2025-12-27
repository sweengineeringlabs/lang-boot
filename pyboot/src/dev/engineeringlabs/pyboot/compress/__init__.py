"""
Compress Module - Compression and decompression utilities.

Provides utilities for data compression:
- Gzip compression
- Zstd compression
- Compression streams
"""

from dev.engineeringlabs.pyboot.compress.api import (
    CompressionLevel,
    CompressionFormat,
    CompressError,
)

from dev.engineeringlabs.pyboot.compress.core import (
    compress,
    decompress,
    compress_stream,
    decompress_stream,
    GzipCompressor,
    ZstdCompressor,
)

__all__ = [
    # API
    "CompressionLevel",
    "CompressionFormat",
    "CompressError",
    # Core
    "compress",
    "decompress",
    "compress_stream",
    "decompress_stream",
    "GzipCompressor",
    "ZstdCompressor",
]
