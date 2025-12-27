"""Tests for compress module."""

import pytest
from dev.engineeringlabs.pyboot.compress import (
    compress,
    decompress,
    CompressionLevel,
    CompressionFormat,
    GzipCompressor,
    CompressError,
)


class TestCompress:
    """Tests for compress/decompress functions."""
    
    def test_roundtrip(self):
        """Test compress/decompress roundtrip."""
        original = b"Hello, World!" * 100
        compressed = compress(original)
        decompressed = decompress(compressed)
        
        assert decompressed == original
    
    def test_compression_reduces_size(self):
        """Test compression reduces size for repetitive data."""
        original = b"A" * 1000
        compressed = compress(original)
        
        assert len(compressed) < len(original)
    
    def test_different_levels(self):
        """Test different compression levels work."""
        data = b"Test data " * 100
        
        for level in CompressionLevel:
            compressed = compress(data, level=level)
            decompressed = decompress(compressed)
            assert decompressed == data
    
    def test_fastest_is_larger_than_best(self):
        """Test fastest compression produces larger output."""
        data = b"Test data for compression " * 1000
        
        fastest = compress(data, level=CompressionLevel.FASTEST)
        best = compress(data, level=CompressionLevel.BEST)
        
        assert len(fastest) >= len(best)
    
    def test_empty_data(self):
        """Test compressing empty data."""
        original = b""
        compressed = compress(original)
        decompressed = decompress(compressed)
        
        assert decompressed == original
    
    def test_binary_data(self):
        """Test compressing binary data."""
        original = bytes(range(256)) * 10
        compressed = compress(original)
        decompressed = decompress(compressed)
        
        assert decompressed == original


class TestGzipCompressor:
    """Tests for GzipCompressor class."""
    
    def test_roundtrip(self):
        """Test GzipCompressor roundtrip."""
        compressor = GzipCompressor()
        original = b"Test data"
        
        compressed = compressor.compress(original)
        decompressed = compressor.decompress(compressed)
        
        assert decompressed == original
    
    def test_with_level(self):
        """Test GzipCompressor with different levels."""
        for level in CompressionLevel:
            compressor = GzipCompressor(level=level)
            data = b"Test " * 100
            
            compressed = compressor.compress(data)
            decompressed = compressor.decompress(compressed)
            
            assert decompressed == data


class TestCompressionFormat:
    """Tests for CompressionFormat enum."""
    
    def test_gzip_format(self):
        """Test GZIP format works."""
        data = b"test data"
        compressed = compress(data, format=CompressionFormat.GZIP)
        decompressed = decompress(compressed, format=CompressionFormat.GZIP)
        
        assert decompressed == data
    
    def test_unsupported_format(self):
        """Test unsupported format raises error."""
        with pytest.raises(CompressError):
            compress(b"data", format=CompressionFormat.ZSTD)


class TestCompressionLevel:
    """Tests for CompressionLevel enum."""
    
    def test_all_levels_exist(self):
        """Test all expected levels exist."""
        assert CompressionLevel.FASTEST
        assert CompressionLevel.FAST
        assert CompressionLevel.DEFAULT
        assert CompressionLevel.BEST
