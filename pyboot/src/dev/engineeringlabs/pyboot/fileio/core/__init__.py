"""FileIO Core - File I/O implementations."""

import json
from pathlib import Path
from typing import Any
from dev.engineeringlabs.pyboot.fileio.api import FileError


def read_file(path: str | Path) -> str:
    """Read file contents as string."""
    try:
        return Path(path).read_text(encoding="utf-8")
    except OSError as e:
        raise FileError(f"Failed to read file: {e}", str(path), cause=e)


def write_file(path: str | Path, content: str) -> None:
    """Write string to file."""
    try:
        Path(path).write_text(content, encoding="utf-8")
    except OSError as e:
        raise FileError(f"Failed to write file: {e}", str(path), cause=e)


def read_json(path: str | Path) -> Any:
    """Read JSON file."""
    try:
        content = read_file(path)
        return json.loads(content)
    except json.JSONDecodeError as e:
        raise FileError(f"Invalid JSON: {e}", str(path), cause=e)


def write_json(path: str | Path, data: Any, indent: int = 2) -> None:
    """Write data as JSON file."""
    content = json.dumps(data, indent=indent)
    write_file(path, content)


def read_yaml(path: str | Path) -> Any:
    """Read YAML file (requires pyyaml)."""
    try:
        import yaml
        content = read_file(path)
        return yaml.safe_load(content)
    except ImportError:
        raise FileError("YAML support requires 'pyyaml' library", str(path))


def write_yaml(path: str | Path, data: Any) -> None:
    """Write data as YAML file (requires pyyaml)."""
    try:
        import yaml
        content = yaml.safe_dump(data, default_flow_style=False)
        write_file(path, content)
    except ImportError:
        raise FileError("YAML support requires 'pyyaml' library", str(path))


def ensure_dir(path: str | Path) -> Path:
    """Ensure directory exists."""
    p = Path(path)
    p.mkdir(parents=True, exist_ok=True)
    return p


class FileWatcher:
    """Watch files for changes (placeholder)."""
    
    def __init__(self, path: str | Path) -> None:
        self.path = Path(path)
        self._running = False
    
    async def start(self) -> None:
        """Start watching."""
        self._running = True
    
    async def stop(self) -> None:
        """Stop watching."""
        self._running = False


__all__ = [
    "read_file",
    "write_file",
    "read_json",
    "write_json",
    "read_yaml",
    "write_yaml",
    "ensure_dir",
    "FileWatcher",
]
