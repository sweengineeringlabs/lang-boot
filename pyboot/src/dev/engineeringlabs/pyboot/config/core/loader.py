"""Configuration loader implementation."""

import json
from pathlib import Path
from typing import Any

from dev.engineeringlabs.pyboot.config.api.settings import ConfigSource, Settings

# Global settings instance
_global_settings: Settings | None = None


class ConfigLoader:
    """
    Configuration loader for various file formats.

    Supports:
    - YAML files (.yaml, .yml)
    - JSON files (.json)
    - Environment variables

    Example:
        loader = ConfigLoader()

        # Load from file
        settings = loader.load("config.yaml")

        # Load from file with environment override
        settings = loader.load_with_env("config.yaml", prefix="APP_")
    """

    def load(self, path: str | Path) -> Settings:
        """
        Load configuration from a file.

        Args:
            path: Path to configuration file (YAML or JSON)

        Returns:
            Settings object with loaded configuration

        Raises:
            FileNotFoundError: If file doesn't exist
            ValueError: If file format is not supported
        """
        path = Path(path)
        if not path.exists():
            raise FileNotFoundError(f"Configuration file not found: {path}")

        data = self._load_file(path)
        settings = Settings()
        settings.load_from_dict(data, source=ConfigSource.FILE)
        return settings

    def load_with_env(
        self,
        path: str | Path | None = None,
        prefix: str = "",
        defaults: dict[str, Any] | None = None,
    ) -> Settings:
        """
        Load configuration from file and environment variables.

        Priority: override > environment > file > default

        Args:
            path: Optional path to configuration file
            prefix: Prefix for environment variables
            defaults: Default values to set

        Returns:
            Settings object with merged configuration
        """
        settings = Settings()

        # 1. Load defaults
        if defaults:
            for key, value in defaults.items():
                settings.set_default(key, value)

        # 2. Load from file
        if path:
            path = Path(path)
            if path.exists():
                data = self._load_file(path)
                settings.load_from_dict(data, source=ConfigSource.FILE)

        # 3. Load from environment (highest priority for env vars)
        settings.load_from_env(prefix=prefix)

        return settings

    def _load_file(self, path: Path) -> dict[str, Any]:
        """Load configuration from a file."""
        suffix = path.suffix.lower()

        if suffix in (".yaml", ".yml"):
            return self._load_yaml(path)
        elif suffix == ".json":
            return self._load_json(path)
        elif suffix == ".toml":
            return self._load_toml(path)
        else:
            raise ValueError(f"Unsupported configuration file format: {suffix}")

    def _load_yaml(self, path: Path) -> dict[str, Any]:
        """Load YAML file."""
        try:
            import yaml
        except ImportError:
            raise ImportError("PyYAML is required for YAML support. Install with: pip install pyyaml")

        with open(path, encoding="utf-8") as f:
            return yaml.safe_load(f) or {}

    def _load_json(self, path: Path) -> dict[str, Any]:
        """Load JSON file."""
        with open(path, encoding="utf-8") as f:
            return json.load(f)

    def _load_toml(self, path: Path) -> dict[str, Any]:
        """Load TOML file."""
        try:
            import tomli
        except ImportError:
            try:
                import tomllib as tomli  # Python 3.11+
            except ImportError:
                raise ImportError(
                    "TOML support requires tomli or Python 3.11+. "
                    "Install with: pip install pyboot-config[toml]"
                )

        with open(path, "rb") as f:
            return tomli.load(f)


def configure(
    path: str | Path | None = None,
    prefix: str = "",
    defaults: dict[str, Any] | None = None,
) -> Settings:
    """
    Configure global settings.

    This is a convenience function for setting up the global settings instance.

    Args:
        path: Optional path to configuration file
        prefix: Prefix for environment variables
        defaults: Default values to set

    Returns:
        The global Settings instance

    Example:
        from dev.engineeringlabs.pyboot.config import configure, get_settings

        # At startup
        configure(
            path="config.yaml",
            prefix="APP_",
            defaults={"app.name": "MyApp"}
        )

        # Anywhere else
        settings = get_settings()
        name = settings.get_string("app.name")
    """
    global _global_settings
    loader = ConfigLoader()
    _global_settings = loader.load_with_env(path, prefix, defaults)
    return _global_settings


def get_settings() -> Settings:
    """
    Get the global settings instance.

    Returns:
        The global Settings instance

    Raises:
        RuntimeError: If configure() hasn't been called
    """
    global _global_settings
    if _global_settings is None:
        _global_settings = Settings()
    return _global_settings


__all__ = [
    "ConfigLoader",
    "configure",
    "get_settings",
]
