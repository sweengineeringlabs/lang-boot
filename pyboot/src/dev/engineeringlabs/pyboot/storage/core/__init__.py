"""Storage core implementations."""

from dev.engineeringlabs.pyboot.storage.core.local import LocalStorage
from dev.engineeringlabs.pyboot.storage.core.registry import get_storage, set_storage

__all__ = [
    "LocalStorage",
    "get_storage",
    "set_storage",
]
