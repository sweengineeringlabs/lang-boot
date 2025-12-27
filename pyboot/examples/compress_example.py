"""
PyBoot Examples - Compression

Demonstrates compression utilities.
"""

from dev.engineeringlabs.pyboot.compress import (
    compress,
    decompress,
    CompressionLevel,
    CompressionFormat,
    GzipCompressor,
)


# Example 1: Basic compression
print("=" * 50)
print("Example 1: Basic Compression")
print("=" * 50)

original = b"Hello, World! " * 100  # Repeat to show compression benefit
compressed = compress(original)
decompressed = decompress(compressed)

print(f"Original size:    {len(original)} bytes")
print(f"Compressed size:  {len(compressed)} bytes")
print(f"Compression ratio: {len(compressed) / len(original):.2%}")
print(f"Decompressed matches original: {decompressed == original}")
print()


# Example 2: Compression levels
print("=" * 50)
print("Example 2: Compression Levels")
print("=" * 50)

data = b"Sample data for compression testing. " * 1000

for level in CompressionLevel:
    compressed = compress(data, level=level)
    print(f"{level.name:10} -> {len(compressed):5} bytes ({len(compressed)/len(data):.1%})")
print()


# Example 3: Using GzipCompressor class
print("=" * 50)
print("Example 3: GzipCompressor Class")
print("=" * 50)

compressor = GzipCompressor(level=CompressionLevel.BEST)

text = "This is some text that will be compressed and decompressed."
original_bytes = text.encode('utf-8')
compressed_bytes = compressor.compress(original_bytes)
decompressed_bytes = compressor.decompress(compressed_bytes)

print(f"Original text: {text}")
print(f"Original size: {len(original_bytes)} bytes")
print(f"Compressed size: {len(compressed_bytes)} bytes")
print(f"Decompressed: {decompressed_bytes.decode('utf-8')}")
print()


# Example 4: Compressing JSON data
print("=" * 50)
print("Example 4: JSON Compression")
print("=" * 50)

import json

data = {
    "users": [
        {"id": i, "name": f"User {i}", "email": f"user{i}@example.com"}
        for i in range(100)
    ]
}

json_bytes = json.dumps(data).encode('utf-8')
compressed = compress(json_bytes, level=CompressionLevel.DEFAULT)

print(f"JSON size:       {len(json_bytes)} bytes")
print(f"Compressed size: {len(compressed)} bytes")
print(f"Savings:         {(1 - len(compressed)/len(json_bytes)):.1%}")
