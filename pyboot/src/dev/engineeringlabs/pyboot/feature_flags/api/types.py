"""
Feature flag types - Flag definitions and rollout strategies.
"""

from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Callable
from datetime import datetime
import hashlib


class RolloutType(str, Enum):
    """Type of rollout strategy."""
    ALL = "all"
    NONE = "none"
    PERCENTAGE = "percentage"
    USER_IDS = "user_ids"
    GROUPS = "groups"
    CUSTOM = "custom"


@dataclass
class RolloutStrategy:
    """Strategy for flag rollout.
    
    Example:
        # Enable for 50% of users
        strategy = RolloutStrategy.percentage(50)
        
        # Enable for specific users
        strategy = RolloutStrategy.user_ids(["user_1", "user_2"])
        
        # Enable for beta testers group
        strategy = RolloutStrategy.groups(["beta", "internal"])
    """
    
    type: RolloutType
    value: Any = None
    custom_evaluator: Callable[[Any], bool] | None = None
    
    @classmethod
    def all(cls) -> "RolloutStrategy":
        """Enable for all users."""
        return cls(type=RolloutType.ALL)
    
    @classmethod
    def none(cls) -> "RolloutStrategy":
        """Disable for all users."""
        return cls(type=RolloutType.NONE)
    
    @classmethod
    def percentage(cls, pct: int) -> "RolloutStrategy":
        """Enable for percentage of users (0-100)."""
        return cls(type=RolloutType.PERCENTAGE, value=max(0, min(100, pct)))
    
    @classmethod
    def user_ids(cls, ids: list[str]) -> "RolloutStrategy":
        """Enable for specific user IDs."""
        return cls(type=RolloutType.USER_IDS, value=set(ids))
    
    @classmethod
    def groups(cls, groups: list[str]) -> "RolloutStrategy":
        """Enable for users in specific groups."""
        return cls(type=RolloutType.GROUPS, value=set(groups))
    
    @classmethod
    def custom(cls, evaluator: Callable[[Any], bool]) -> "RolloutStrategy":
        """Use custom evaluation function."""
        return cls(type=RolloutType.CUSTOM, custom_evaluator=evaluator)
    
    def evaluate(self, context: "FlagContext") -> bool:
        """Evaluate rollout for given context."""
        if self.type == RolloutType.ALL:
            return True
        
        if self.type == RolloutType.NONE:
            return False
        
        if self.type == RolloutType.PERCENTAGE:
            return self._evaluate_percentage(context)
        
        if self.type == RolloutType.USER_IDS:
            return context.user_id in self.value if context.user_id else False
        
        if self.type == RolloutType.GROUPS:
            return bool(set(context.groups or []) & self.value)
        
        if self.type == RolloutType.CUSTOM and self.custom_evaluator:
            return self.custom_evaluator(context)
        
        return False
    
    def _evaluate_percentage(self, context: "FlagContext") -> bool:
        """Consistent percentage evaluation using hash."""
        identifier = context.user_id or context.session_id or ""
        if not identifier:
            return False
        
        # Create consistent hash for user
        hash_input = f"{identifier}:{context.flag_name or ''}".encode()
        hash_value = int(hashlib.md5(hash_input).hexdigest()[:8], 16)
        bucket = hash_value % 100
        
        return bucket < self.value


@dataclass
class FlagContext:
    """Context for flag evaluation.
    
    Provides user/session information for targeted flags.
    
    Example:
        context = FlagContext(
            user_id="user_123",
            groups=["beta_testers"],
            attributes={"country": "US", "tier": "premium"}
        )
    """
    
    user_id: str | None = None
    session_id: str | None = None
    groups: list[str] | None = None
    attributes: dict[str, Any] = field(default_factory=dict)
    flag_name: str | None = None  # Set during evaluation
    
    def with_flag(self, flag_name: str) -> "FlagContext":
        """Create context copy with flag name."""
        return FlagContext(
            user_id=self.user_id,
            session_id=self.session_id,
            groups=self.groups,
            attributes=self.attributes.copy(),
            flag_name=flag_name,
        )


@dataclass
class Flag:
    """Feature flag definition.
    
    Attributes:
        name: Unique flag identifier.
        enabled: Master on/off switch.
        description: Human-readable description.
        rollout: Rollout strategy (who gets the flag).
        default_value: Value when flag is disabled.
        metadata: Additional metadata.
        expires_at: Optional expiration date.
    
    Example:
        flag = Flag(
            name="new_checkout",
            enabled=True,
            description="New checkout flow",
            rollout=RolloutStrategy.percentage(25),
        )
    """
    
    name: str
    enabled: bool = True
    description: str = ""
    rollout: RolloutStrategy = field(default_factory=RolloutStrategy.all)
    default_value: Any = None
    metadata: dict[str, Any] = field(default_factory=dict)
    expires_at: datetime | None = None
    created_at: datetime = field(default_factory=datetime.now)
    
    def is_enabled(self, context: FlagContext | None = None) -> bool:
        """Check if flag is enabled for context."""
        # Check master switch
        if not self.enabled:
            return False
        
        # Check expiration
        if self.expires_at and datetime.now() > self.expires_at:
            return False
        
        # No context = use simple enabled check
        if context is None:
            return self.rollout.type == RolloutType.ALL
        
        # Evaluate rollout
        ctx = context.with_flag(self.name)
        return self.rollout.evaluate(ctx)
