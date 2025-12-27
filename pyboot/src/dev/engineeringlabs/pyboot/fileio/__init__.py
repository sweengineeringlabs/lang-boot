"""
FileIO Module - File I/O utilities.

Provides utilities for file operations:
- File reading/writing
- Path utilities
- File watching
"""

from dev.engineeringlabs.pyboot.fileio.api import (
    FileError,
    FileNotFoundError,
    PermissionError,
)

from dev.engineeringlabs.pyboot.fileio.core import (
    read_file,
    write_file,
    read_json,
    write_json,
    read_yaml,
    write_yaml,
    ensure_dir,
    FileWatcher,
)

__all__ = [
    # API
    "FileError",
    "FileNotFoundError",
    "PermissionError",
    # Core
    "read_file",
    "write_file",
    "read_json",
    "write_json",
    "read_yaml",
    "write_yaml",
    "ensure_dir",
    "FileWatcher",
]
