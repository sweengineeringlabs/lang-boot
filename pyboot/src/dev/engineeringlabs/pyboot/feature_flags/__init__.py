"""
Feature Flags Module - Runtime feature toggles and gradual rollouts.

This module provides:
- Simple on/off feature flags
- Percentage-based rollouts
- User/context targeting
- Flag providers (memory, file, remote)

Example:
    from dev.engineeringlabs.pyboot.feature_flags import is_enabled, FeatureFlags
    from dev.engineeringlabs.pyboot.feature_flags import Flag, RolloutStrategy
    
    # Simple check
    if is_enabled("new_dashboard"):
        show_new_dashboard()
    
    # With context
    if is_enabled("beta_features", user_id="user_123"):
        enable_beta()
    
    # Configure flags
    flags = FeatureFlags()
    flags.register(Flag(
        name="dark_mode",
        enabled=True,
        rollout=RolloutStrategy.percentage(50),
    ))
"""

from dev.engineeringlabs.pyboot.feature_flags.api import (
    # Types
    Flag,
    FlagContext,
    RolloutStrategy,
    # Exceptions
    FeatureFlagError,
    FlagNotFoundError,
)

from dev.engineeringlabs.pyboot.feature_flags.core import (
    # Main interface
    FeatureFlags,
    # Functions
    is_enabled,
    get_flag,
    register_flag,
    # Providers
    MemoryFlagProvider,
    FileFlagProvider,
)

__all__ = [
    # API Types
    "Flag",
    "FlagContext",
    "RolloutStrategy",
    # API Exceptions
    "FeatureFlagError",
    "FlagNotFoundError",
    # Core
    "FeatureFlags",
    "is_enabled",
    "get_flag",
    "register_flag",
    # Providers
    "MemoryFlagProvider",
    "FileFlagProvider",
]
