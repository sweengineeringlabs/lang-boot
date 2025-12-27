"""Parsing Core - Parser implementations."""

import json
from typing import Any, Protocol
from dev.engineeringlabs.pyboot.parsing.api import ParseError, ParseResult


class Parser(Protocol):
    """Parser protocol."""
    
    def parse(self, content: str) -> ParseResult[Any]:
        """Parse content."""
        ...


class JsonParser:
    """JSON parser."""
    
    def parse(self, content: str) -> ParseResult[Any]:
        """Parse JSON content."""
        try:
            return ParseResult(value=json.loads(content))
        except json.JSONDecodeError as e:
            return ParseResult(error=ParseError(
                f"Invalid JSON: {e.msg}",
                line=e.lineno,
                column=e.colno,
                cause=e,
            ))


class YamlParser:
    """YAML parser (requires pyyaml)."""
    
    def parse(self, content: str) -> ParseResult[Any]:
        """Parse YAML content."""
        try:
            import yaml
            return ParseResult(value=yaml.safe_load(content))
        except ImportError:
            return ParseResult(error=ParseError("YAML support requires 'pyyaml' library"))
        except Exception as e:
            return ParseResult(error=ParseError(f"Invalid YAML: {e}", cause=e))


class TomlParser:
    """TOML parser (requires tomllib for Python 3.11+)."""
    
    def parse(self, content: str) -> ParseResult[Any]:
        """Parse TOML content."""
        try:
            import tomllib
            return ParseResult(value=tomllib.loads(content))
        except ImportError:
            return ParseResult(error=ParseError("TOML support requires Python 3.11+"))
        except Exception as e:
            return ParseResult(error=ParseError(f"Invalid TOML: {e}", cause=e))


def parse_json(content: str) -> Any:
    """Parse JSON content (raises on error)."""
    result = JsonParser().parse(content)
    return result.unwrap()


def parse_yaml(content: str) -> Any:
    """Parse YAML content (raises on error)."""
    result = YamlParser().parse(content)
    return result.unwrap()


def parse_toml(content: str) -> Any:
    """Parse TOML content (raises on error)."""
    result = TomlParser().parse(content)
    return result.unwrap()


__all__ = [
    "Parser",
    "JsonParser",
    "YamlParser",
    "TomlParser",
    "parse_json",
    "parse_yaml",
    "parse_toml",
]
