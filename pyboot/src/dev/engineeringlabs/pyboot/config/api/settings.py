"""Configuration settings models."""

import os
from dataclasses import dataclass, field
from enum import Enum
from typing import Any


class ConfigSource(str, Enum):
    """Source of configuration values with priority ordering."""

    DEFAULT = "default"        # Priority 0 - lowest
    FILE = "file"              # Priority 1
    ENVIRONMENT = "environment"  # Priority 2
    OVERRIDE = "override"      # Priority 3 - highest


@dataclass(frozen=True, slots=True)
class ConfigValue:
    """A configuration value with metadata.

    Example:
        value = ConfigValue(key="api.timeout", value="30", source=ConfigSource.ENVIRONMENT)
        timeout = value.as_int(default=60)  # Returns 30
    """

    key: str
    value: Any
    source: ConfigSource = ConfigSource.DEFAULT
    description: str | None = None

    def as_str(self, default: str = "") -> str:
        """Get value as string."""
        if self.value is None:
            return default
        return str(self.value)

    def as_int(self, default: int = 0) -> int:
        """Get value as integer."""
        if self.value is None:
            return default
        try:
            return int(self.value)
        except (ValueError, TypeError):
            return default

    def as_float(self, default: float = 0.0) -> float:
        """Get value as float."""
        if self.value is None:
            return default
        try:
            return float(self.value)
        except (ValueError, TypeError):
            return default

    def as_bool(self, default: bool = False) -> bool:
        """Get value as boolean."""
        if self.value is None:
            return default
        if isinstance(self.value, bool):
            return self.value
        if isinstance(self.value, str):
            return self.value.lower() in ("true", "1", "yes", "on")
        return bool(self.value)

    def as_list(self, default: list[str] | None = None, separator: str = ",") -> list[str]:
        """Get value as list (splits string by separator)."""
        if self.value is None:
            return default or []
        if isinstance(self.value, list):
            return self.value
        if isinstance(self.value, str):
            return [item.strip() for item in self.value.split(separator) if item.strip()]
        return default or []


@dataclass
class Settings:
    """
    Application settings container.

    Manages configuration from multiple sources with priority:
    override > environment > file > default

    Example:
        settings = Settings()
        settings.set_default("api.timeout", 30)
        settings.load_from_env(prefix="APP_")

        timeout = settings.get("api.timeout").as_int()
    """

    _values: dict[str, ConfigValue] = field(default_factory=dict)

    def get(self, key: str, default: Any = None) -> ConfigValue:
        """Get a configuration value."""
        if key in self._values:
            return self._values[key]
        return ConfigValue(key=key, value=default, source=ConfigSource.DEFAULT)

    def get_string(self, key: str, default: str = "") -> str:
        """Get a string configuration value."""
        return self.get(key).as_str(default)

    def get_int(self, key: str, default: int = 0) -> int:
        """Get an integer configuration value."""
        return self.get(key).as_int(default)

    def get_float(self, key: str, default: float = 0.0) -> float:
        """Get a float configuration value."""
        return self.get(key).as_float(default)

    def get_bool(self, key: str, default: bool = False) -> bool:
        """Get a boolean configuration value."""
        return self.get(key).as_bool(default)

    def set(
        self,
        key: str,
        value: Any,
        source: ConfigSource = ConfigSource.OVERRIDE,
        description: str | None = None,
    ) -> None:
        """Set a configuration value."""
        self._values[key] = ConfigValue(
            key=key,
            value=value,
            source=source,
            description=description,
        )

    def set_default(self, key: str, value: Any, description: str | None = None) -> None:
        """Set a default value (won't override existing)."""
        if key not in self._values:
            self._values[key] = ConfigValue(
                key=key,
                value=value,
                source=ConfigSource.DEFAULT,
                description=description,
            )

    def load_from_env(self, prefix: str = "") -> int:
        """
        Load configuration from environment variables.

        Environment variables are converted to config keys:
        - Strip prefix
        - Convert to lowercase
        - Replace underscores with dots

        Args:
            prefix: Optional prefix for environment variables.

        Returns:
            Number of values loaded.

        Example:
            # With APP_DATABASE_HOST=localhost
            settings.load_from_env(prefix="APP_")
            # Creates key: database.host = localhost
        """
        loaded = 0
        for key, value in os.environ.items():
            if prefix and not key.startswith(prefix):
                continue

            config_key = key[len(prefix):].lower().replace("_", ".")

            # Only set if not already overridden
            if config_key not in self._values or self._values[config_key].source != ConfigSource.OVERRIDE:
                self._values[config_key] = ConfigValue(
                    key=config_key,
                    value=value,
                    source=ConfigSource.ENVIRONMENT,
                )
                loaded += 1

        return loaded

    def load_from_dict(
        self,
        data: dict[str, Any],
        source: ConfigSource = ConfigSource.FILE,
    ) -> int:
        """
        Load configuration from a dictionary.

        Nested dictionaries are flattened with dot notation.

        Args:
            data: Dictionary of configuration values.
            source: Source to assign to loaded values.

        Returns:
            Number of values loaded.

        Example:
            settings.load_from_dict({
                "database": {
                    "host": "localhost",
                    "port": 5432,
                }
            })
            # Creates: database.host, database.port
        """
        loaded = 0
        for key, value in self._flatten_dict(data).items():
            if key not in self._values or self._should_override(key, source):
                self._values[key] = ConfigValue(key=key, value=value, source=source)
                loaded += 1
        return loaded

    def _should_override(self, key: str, new_source: ConfigSource) -> bool:
        """Check if new source should override existing value."""
        if key not in self._values:
            return True
        existing = self._values[key]
        # Higher priority sources override lower priority
        priority = {
            ConfigSource.DEFAULT: 0,
            ConfigSource.FILE: 1,
            ConfigSource.ENVIRONMENT: 2,
            ConfigSource.OVERRIDE: 3,
        }
        return priority[new_source] > priority[existing.source]

    def _flatten_dict(
        self,
        data: dict[str, Any],
        prefix: str = "",
    ) -> dict[str, Any]:
        """Flatten a nested dictionary with dot notation."""
        result: dict[str, Any] = {}
        for key, value in data.items():
            full_key = f"{prefix}.{key}" if prefix else key
            if isinstance(value, dict):
                result.update(self._flatten_dict(value, full_key))
            else:
                result[full_key] = value
        return result

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary of raw values."""
        return {k: v.value for k, v in self._values.items()}

    def keys(self) -> list[str]:
        """Get all configuration keys."""
        return list(self._values.keys())

    def __contains__(self, key: str) -> bool:
        """Check if key exists."""
        return key in self._values

    def __len__(self) -> int:
        """Get number of configuration values."""
        return len(self._values)


__all__ = [
    "ConfigSource",
    "ConfigValue",
    "Settings",
]
