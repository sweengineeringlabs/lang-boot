"""Storage registry for global storage access."""

from dev.engineeringlabs.pyboot.storage.api.storage import Storage
from dev.engineeringlabs.pyboot.storage.core.local import LocalStorage

# Global storage registry
_storages: dict[str, Storage] = {}
_default_storage: Storage | None = None


def get_storage(name: str | None = None) -> Storage:
    """
    Get a storage instance by name.

    Args:
        name: Storage name (None = default storage)

    Returns:
        Storage instance
    """
    global _default_storage

    if name is None:
        if _default_storage is None:
            _default_storage = LocalStorage(".")
        return _default_storage

    if name not in _storages:
        raise KeyError(f"Storage not found: {name}")

    return _storages[name]


def set_storage(storage: Storage, name: str | None = None) -> None:
    """
    Register a storage instance.

    Args:
        storage: Storage instance
        name: Storage name (None = set as default)
    """
    global _default_storage

    if name is None:
        _default_storage = storage
    else:
        _storages[name] = storage


def clear_storages() -> None:
    """Clear all storage registrations."""
    global _default_storage
    _storages.clear()
    _default_storage = None


__all__ = ["get_storage", "set_storage", "clear_storages"]
