"""
Feature flag providers - Storage backends for flags.
"""

import json
from pathlib import Path
from typing import Protocol, runtime_checkable

from dev.engineeringlabs.pyboot.feature_flags.api.types import Flag, RolloutStrategy, RolloutType


@runtime_checkable
class FlagProvider(Protocol):
    """Protocol for flag storage providers."""
    
    def load(self) -> list[Flag]:
        """Load flags from storage."""
        ...
    
    def save(self, flags: list[Flag]) -> None:
        """Save flags to storage."""
        ...


class MemoryFlagProvider:
    """In-memory flag storage.
    
    Stores flags in memory. Useful for testing or
    when flags are configured in code.
    
    Example:
        provider = MemoryFlagProvider()
        provider.add(Flag(name="feature_x", enabled=True))
        flags = provider.load()
    """
    
    def __init__(self) -> None:
        self._flags: dict[str, Flag] = {}
    
    def add(self, flag: Flag) -> None:
        """Add a flag."""
        self._flags[flag.name] = flag
    
    def remove(self, name: str) -> None:
        """Remove a flag."""
        self._flags.pop(name, None)
    
    def load(self) -> list[Flag]:
        """Load all flags."""
        return list(self._flags.values())
    
    def save(self, flags: list[Flag]) -> None:
        """Save flags."""
        self._flags = {f.name: f for f in flags}


class FileFlagProvider:
    """File-based flag storage (JSON/YAML).
    
    Stores flags in a JSON or YAML file.
    
    Example:
        provider = FileFlagProvider("flags.json")
        flags = provider.load()
        
        # flags.json:
        # {
        #   "flags": [
        #     {"name": "dark_mode", "enabled": true},
        #     {"name": "new_feature", "enabled": true, "rollout": {"type": "percentage", "value": 50}}
        #   ]
        # }
    """
    
    def __init__(self, path: str | Path) -> None:
        self._path = Path(path)
    
    def load(self) -> list[Flag]:
        """Load flags from file."""
        if not self._path.exists():
            return []
        
        content = self._path.read_text()
        data = json.loads(content)
        
        return [self._parse_flag(f) for f in data.get("flags", [])]
    
    def save(self, flags: list[Flag]) -> None:
        """Save flags to file."""
        data = {
            "flags": [self._serialize_flag(f) for f in flags]
        }
        self._path.write_text(json.dumps(data, indent=2))
    
    def _parse_flag(self, data: dict) -> Flag:
        """Parse flag from dict."""
        rollout = RolloutStrategy.all()
        if "rollout" in data:
            rollout = self._parse_rollout(data["rollout"])
        
        return Flag(
            name=data["name"],
            enabled=data.get("enabled", True),
            description=data.get("description", ""),
            rollout=rollout,
            default_value=data.get("default_value"),
            metadata=data.get("metadata", {}),
        )
    
    def _parse_rollout(self, data: dict) -> RolloutStrategy:
        """Parse rollout strategy from dict."""
        rollout_type = RolloutType(data.get("type", "all"))
        
        if rollout_type == RolloutType.ALL:
            return RolloutStrategy.all()
        if rollout_type == RolloutType.NONE:
            return RolloutStrategy.none()
        if rollout_type == RolloutType.PERCENTAGE:
            return RolloutStrategy.percentage(data.get("value", 0))
        if rollout_type == RolloutType.USER_IDS:
            return RolloutStrategy.user_ids(data.get("value", []))
        if rollout_type == RolloutType.GROUPS:
            return RolloutStrategy.groups(data.get("value", []))
        
        return RolloutStrategy.all()
    
    def _serialize_flag(self, flag: Flag) -> dict:
        """Serialize flag to dict."""
        result = {
            "name": flag.name,
            "enabled": flag.enabled,
        }
        
        if flag.description:
            result["description"] = flag.description
        
        if flag.rollout.type != RolloutType.ALL:
            result["rollout"] = {
                "type": flag.rollout.type.value,
                "value": (
                    list(flag.rollout.value) 
                    if isinstance(flag.rollout.value, set) 
                    else flag.rollout.value
                ),
            }
        
        if flag.default_value is not None:
            result["default_value"] = flag.default_value
        
        if flag.metadata:
            result["metadata"] = flag.metadata
        
        return result
