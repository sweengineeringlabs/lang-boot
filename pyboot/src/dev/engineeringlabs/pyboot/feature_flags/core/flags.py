"""
Feature flags implementation - Main flag manager.
"""

from typing import Any
from dev.engineeringlabs.pyboot.feature_flags.api.types import Flag, FlagContext, RolloutStrategy
from dev.engineeringlabs.pyboot.feature_flags.api.exceptions import FlagNotFoundError


class FeatureFlags:
    """Feature flag manager.
    
    Manages flag registration, evaluation, and provides a central
    point for feature flag operations.
    
    Example:
        flags = FeatureFlags()
        
        # Register flags
        flags.register(Flag(name="dark_mode", enabled=True))
        flags.register(Flag(
            name="new_feature",
            enabled=True,
            rollout=RolloutStrategy.percentage(25),
        ))
        
        # Check flags
        if flags.is_enabled("dark_mode"):
            enable_dark_mode()
        
        # With context
        ctx = FlagContext(user_id="user_123")
        if flags.is_enabled("new_feature", ctx):
            show_new_feature()
    """
    
    def __init__(self) -> None:
        self._flags: dict[str, Flag] = {}
        self._default_context: FlagContext | None = None
    
    def register(self, flag: Flag) -> None:
        """Register a feature flag."""
        self._flags[flag.name] = flag
    
    def register_many(self, *flags: Flag) -> None:
        """Register multiple flags."""
        for flag in flags:
            self.register(flag)
    
    def unregister(self, name: str) -> None:
        """Remove a flag."""
        self._flags.pop(name, None)
    
    def get(self, name: str) -> Flag | None:
        """Get a flag by name."""
        return self._flags.get(name)
    
    def get_or_raise(self, name: str) -> Flag:
        """Get a flag or raise if not found."""
        flag = self._flags.get(name)
        if flag is None:
            raise FlagNotFoundError(name)
        return flag
    
    def is_enabled(
        self,
        name: str,
        context: FlagContext | None = None,
        default: bool = False,
    ) -> bool:
        """Check if a flag is enabled.
        
        Args:
            name: Flag name.
            context: Evaluation context.
            default: Default value if flag not found.
            
        Returns:
            True if flag is enabled for context.
        """
        flag = self._flags.get(name)
        if flag is None:
            return default
        
        ctx = context or self._default_context
        return flag.is_enabled(ctx)
    
    def get_value(
        self,
        name: str,
        context: FlagContext | None = None,
        default: Any = None,
    ) -> Any:
        """Get flag value if enabled, otherwise default."""
        flag = self._flags.get(name)
        if flag is None:
            return default
        
        ctx = context or self._default_context
        if flag.is_enabled(ctx):
            return flag.default_value if flag.default_value is not None else True
        return default
    
    def set_default_context(self, context: FlagContext) -> None:
        """Set default context for evaluations."""
        self._default_context = context
    
    def list_flags(self) -> list[Flag]:
        """Get all registered flags."""
        return list(self._flags.values())
    
    def clear(self) -> None:
        """Remove all flags."""
        self._flags.clear()


# Global instance
_global_flags = FeatureFlags()


def is_enabled(
    name: str,
    *,
    user_id: str | None = None,
    groups: list[str] | None = None,
    default: bool = False,
    **attributes: Any,
) -> bool:
    """Check if a feature flag is enabled.
    
    Convenience function using the global flag manager.
    
    Example:
        if is_enabled("new_checkout"):
            use_new_checkout()
        
        if is_enabled("beta_feature", user_id="123"):
            show_beta()
    """
    context = None
    if user_id or groups or attributes:
        context = FlagContext(
            user_id=user_id,
            groups=groups,
            attributes=attributes,
        )
    return _global_flags.is_enabled(name, context, default)


def get_flag(name: str) -> Flag | None:
    """Get a flag from global manager."""
    return _global_flags.get(name)


def register_flag(flag: Flag) -> None:
    """Register a flag in global manager."""
    _global_flags.register(flag)


def get_global_flags() -> FeatureFlags:
    """Get the global feature flags manager."""
    return _global_flags
