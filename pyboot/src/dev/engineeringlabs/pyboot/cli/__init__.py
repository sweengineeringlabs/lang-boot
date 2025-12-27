"""
CLI Module - Command-line interface utilities.

Provides utilities for building CLI applications:
- Argument parsing
- Command registration
- Output formatting
"""

from dev.engineeringlabs.pyboot.cli.api import (
    Command,
    Argument,
    Option,
    CLIError,
    CLIResult,
)

from dev.engineeringlabs.pyboot.cli.core import (
    CLI,
    CommandRegistry,
)

__all__ = [
    # API
    "Command",
    "Argument",
    "Option",
    "CLIError",
    "CLIResult",
    # Core
    "CLI",
    "CommandRegistry",
]
