"""Database models."""

from dataclasses import dataclass
from typing import Any


@dataclass(slots=True)
class Row:
    """A database row with dict-like access."""

    _data: dict[str, Any]
    _columns: tuple[str, ...]

    def __init__(self, data: dict[str, Any]) -> None:
        object.__setattr__(self, "_data", data)
        object.__setattr__(self, "_columns", tuple(data.keys()))

    def __getitem__(self, key: str | int) -> Any:
        if isinstance(key, int):
            return self._data[self._columns[key]]
        return self._data[key]

    def __getattr__(self, name: str) -> Any:
        if name.startswith("_"):
            return object.__getattribute__(self, name)
        try:
            return self._data[name]
        except KeyError:
            raise AttributeError(f"Row has no column '{name}'")

    def __contains__(self, key: str) -> bool:
        return key in self._data

    def __iter__(self):
        return iter(self._data.values())

    def __len__(self) -> int:
        return len(self._data)

    @property
    def columns(self) -> tuple[str, ...]:
        """Get column names."""
        return self._columns

    def get(self, key: str, default: Any = None) -> Any:
        """Get a value with default."""
        return self._data.get(key, default)

    def keys(self):
        """Get column names."""
        return self._data.keys()

    def values(self):
        """Get row values."""
        return self._data.values()

    def items(self):
        """Get column-value pairs."""
        return self._data.items()

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return dict(self._data)


__all__ = ["Row"]
