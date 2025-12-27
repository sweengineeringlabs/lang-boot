"""Toolchain Core - Build and environment utilities."""

import os
import sys
import platform
from dataclasses import dataclass
from datetime import datetime
from dev.engineeringlabs.pyboot.toolchain.api import Environment, BuildMode


def get_version() -> str:
    """Get the pyboot version."""
    try:
        from main import __version__
        return __version__
    except ImportError:
        return "0.0.0"


def get_python_version() -> str:
    """Get the Python version."""
    return platform.python_version()


def get_environment() -> Environment:
    """Detect the current environment from ENV vars.
    
    Checks: ENV, ENVIRONMENT, APP_ENV, PYTHON_ENV
    """
    env_vars = ["ENV", "ENVIRONMENT", "APP_ENV", "PYTHON_ENV"]
    
    for var in env_vars:
        value = os.environ.get(var, "").lower()
        if value in ("prod", "production"):
            return Environment.PRODUCTION
        elif value in ("stage", "staging"):
            return Environment.STAGING
        elif value in ("test", "testing"):
            return Environment.TEST
        elif value in ("dev", "development"):
            return Environment.DEVELOPMENT
    
    # Default based on __debug__ flag
    return Environment.DEVELOPMENT if __debug__ else Environment.PRODUCTION


def is_debug() -> bool:
    """Check if running in debug mode."""
    return __debug__


def is_production() -> bool:
    """Check if running in production."""
    return get_environment() == Environment.PRODUCTION


@dataclass
class BuildInfo:
    """Build information."""
    version: str
    python_version: str
    platform: str
    environment: Environment
    build_mode: BuildMode
    timestamp: str


def get_build_info() -> BuildInfo:
    """Get build information."""
    return BuildInfo(
        version=get_version(),
        python_version=get_python_version(),
        platform=platform.platform(),
        environment=get_environment(),
        build_mode=BuildMode.DEBUG if is_debug() else BuildMode.RELEASE,
        timestamp=datetime.now().isoformat(),
    )


__all__ = [
    "get_version",
    "get_python_version",
    "get_environment",
    "is_debug",
    "is_production",
    "get_build_info",
]
