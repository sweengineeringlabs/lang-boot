"""
Config Module - Configuration management.

This module provides:
- API layer: Settings, ConfigValue, ConfigSource
- Core layer: ConfigLoader, global settings management

Example:
    from dev.engineeringlabs.pyboot.config import Settings, ConfigLoader

    loader = ConfigLoader()
    settings = loader.load_with_env("config.yaml", prefix="APP_")

    timeout = settings.get("api.timeout").as_int(default=30)
"""

from dev.engineeringlabs.pyboot.config.api import (
    ConfigSource,
    ConfigValue,
    Settings,
)

from dev.engineeringlabs.pyboot.config.core import (
    ConfigLoader,
    configure,
    get_settings,
)

__all__ = [
    # API
    "ConfigSource",
    "ConfigValue",
    "Settings",
    # Core
    "ConfigLoader",
    "configure",
    "get_settings",
]
