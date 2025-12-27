"""Compress Core - Compression implementations."""

import gzip
import io
from typing import Iterator
from dev.engineeringlabs.pyboot.compress.api import CompressionLevel, CompressionFormat, CompressError


def _level_to_gzip(level: CompressionLevel) -> int:
    """Convert compression level to gzip level."""
    mapping = {
        CompressionLevel.FASTEST: 1,
        CompressionLevel.FAST: 3,
        CompressionLevel.DEFAULT: 6,
        CompressionLevel.BEST: 9,
    }
    return mapping.get(level, 6)


def compress(
    data: bytes,
    format: CompressionFormat = CompressionFormat.GZIP,
    level: CompressionLevel = CompressionLevel.DEFAULT,
) -> bytes:
    """Compress data."""
    if format == CompressionFormat.GZIP:
        return gzip.compress(data, compresslevel=_level_to_gzip(level))
    raise CompressError(f"Unsupported format: {format}")


def decompress(
    data: bytes,
    format: CompressionFormat = CompressionFormat.GZIP,
) -> bytes:
    """Decompress data."""
    if format == CompressionFormat.GZIP:
        return gzip.decompress(data)
    raise CompressError(f"Unsupported format: {format}")


def compress_stream(
    data: Iterator[bytes],
    format: CompressionFormat = CompressionFormat.GZIP,
    level: CompressionLevel = CompressionLevel.DEFAULT,
) -> Iterator[bytes]:
    """Compress a stream of data."""
    if format == CompressionFormat.GZIP:
        buf = io.BytesIO()
        with gzip.GzipFile(fileobj=buf, mode='wb', compresslevel=_level_to_gzip(level)) as gz:
            for chunk in data:
                gz.write(chunk)
        buf.seek(0)
        yield buf.read()
    else:
        raise CompressError(f"Unsupported format: {format}")


def decompress_stream(
    data: Iterator[bytes],
    format: CompressionFormat = CompressionFormat.GZIP,
) -> Iterator[bytes]:
    """Decompress a stream of data."""
    if format == CompressionFormat.GZIP:
        buf = io.BytesIO(b"".join(data))
        with gzip.GzipFile(fileobj=buf, mode='rb') as gz:
            yield gz.read()
    else:
        raise CompressError(f"Unsupported format: {format}")


class GzipCompressor:
    """Gzip compressor."""
    
    def __init__(self, level: CompressionLevel = CompressionLevel.DEFAULT) -> None:
        self.level = level
    
    def compress(self, data: bytes) -> bytes:
        return compress(data, CompressionFormat.GZIP, self.level)
    
    def decompress(self, data: bytes) -> bytes:
        return decompress(data, CompressionFormat.GZIP)


class ZstdCompressor:
    """Zstd compressor (placeholder - requires zstd library)."""
    
    def __init__(self, level: CompressionLevel = CompressionLevel.DEFAULT) -> None:
        self.level = level
    
    def compress(self, data: bytes) -> bytes:
        raise CompressError("Zstd compression requires 'zstd' library")
    
    def decompress(self, data: bytes) -> bytes:
        raise CompressError("Zstd decompression requires 'zstd' library")


__all__ = [
    "compress",
    "decompress",
    "compress_stream",
    "decompress_stream",
    "GzipCompressor",
    "ZstdCompressor",
]
