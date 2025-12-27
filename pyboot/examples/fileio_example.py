"""
PyBoot Examples - File I/O

Demonstrates file operations and utilities.
"""

import tempfile
import os
from pathlib import Path
from dev.engineeringlabs.pyboot.fileio import (
    read_file,
    write_file,
    read_json,
    write_json,
    ensure_dir,
    FileError,
)


# Create a temp directory for examples
temp_dir = Path(tempfile.mkdtemp())
print(f"Using temp directory: {temp_dir}\n")


# Example 1: Basic file operations
print("=" * 50)
print("Example 1: Basic File Operations")
print("=" * 50)

text_file = temp_dir / "example.txt"
content = "Hello, World!\nThis is a test file."

write_file(text_file, content)
print(f"Wrote to: {text_file}")

read_content = read_file(text_file)
print(f"Read back: {read_content}")
print()


# Example 2: JSON files
print("=" * 50)
print("Example 2: JSON Files")
print("=" * 50)

json_file = temp_dir / "config.json"
data = {
    "name": "myapp",
    "version": "1.0.0",
    "settings": {
        "debug": True,
        "port": 8080,
    },
    "features": ["auth", "cache", "api"],
}

write_json(json_file, data)
print(f"Wrote JSON to: {json_file}")

loaded = read_json(json_file)
print(f"Loaded JSON: {loaded}")
print()


# Example 3: Ensure directory exists
print("=" * 50)
print("Example 3: Ensure Directory")
print("=" * 50)

nested_dir = temp_dir / "path" / "to" / "nested" / "dir"
result = ensure_dir(nested_dir)
print(f"Created directory: {result}")
print(f"Exists: {result.exists()}")
print()


# Example 4: Error handling
print("=" * 50)
print("Example 4: Error Handling")
print("=" * 50)

try:
    read_file(temp_dir / "nonexistent.txt")
except FileError as e:
    print(f"Caught FileError: {e.message}")
    print(f"Path: {e.path}")
print()


# Example 5: Working with multiple files
print("=" * 50)
print("Example 5: Multiple Files")
print("=" * 50)

data_dir = ensure_dir(temp_dir / "data")

for i in range(3):
    file_path = data_dir / f"file_{i}.json"
    write_json(file_path, {"id": i, "value": i * 10})
    print(f"Created: {file_path.name}")

print("\nReading all JSON files:")
for json_path in data_dir.glob("*.json"):
    data = read_json(json_path)
    print(f"  {json_path.name}: {data}")


# Cleanup
import shutil
shutil.rmtree(temp_dir)
print(f"\nCleaned up temp directory")
