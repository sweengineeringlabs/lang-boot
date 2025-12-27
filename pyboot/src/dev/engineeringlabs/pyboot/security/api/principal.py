"""Principal and permission models."""

from dataclasses import dataclass, field
from typing import Any


@dataclass(frozen=True, slots=True)
class Permission:
    """A permission that can be granted to a principal."""

    name: str
    resource: str | None = None
    action: str | None = None

    def matches(self, other: "Permission") -> bool:
        """Check if this permission matches another."""
        # Exact match
        if self.name == other.name:
            return True

        # Wildcard matching
        if self.name == "*":
            return True

        # Resource:action matching
        if self.resource and other.resource:
            if self.resource == "*" or self.resource == other.resource:
                if self.action == "*" or self.action == other.action:
                    return True

        return False

    def __str__(self) -> str:
        if self.resource and self.action:
            return f"{self.resource}:{self.action}"
        return self.name

    @classmethod
    def from_string(cls, permission_str: str) -> "Permission":
        """Parse a permission from string format."""
        if ":" in permission_str:
            parts = permission_str.split(":", 1)
            resource = parts[0]
            action = parts[1] if len(parts) > 1 else None
            return cls(name=permission_str, resource=resource, action=action)
        return cls(name=permission_str)


@dataclass(slots=True)
class Principal:
    """
    A security principal (user, service, etc.).

    Example:
        principal = Principal(
            id="user123",
            name="John Doe",
            roles=["admin", "user"],
            permissions=[Permission("users:read"), Permission("users:write")],
        )

        if principal.has_permission("users:read"):
            # Allow access
            ...
    """

    id: str
    name: str | None = None
    email: str | None = None
    roles: set[str] = field(default_factory=set)
    permissions: set[Permission] = field(default_factory=set)
    attributes: dict[str, Any] = field(default_factory=dict)
    authenticated: bool = True

    def has_role(self, role: str) -> bool:
        """Check if principal has a role."""
        return role in self.roles

    def has_any_role(self, *roles: str) -> bool:
        """Check if principal has any of the roles."""
        return bool(self.roles.intersection(roles))

    def has_all_roles(self, *roles: str) -> bool:
        """Check if principal has all of the roles."""
        return all(role in self.roles for role in roles)

    def has_permission(self, permission: str | Permission) -> bool:
        """Check if principal has a permission."""
        if isinstance(permission, str):
            permission = Permission.from_string(permission)

        for perm in self.permissions:
            if perm.matches(permission):
                return True
        return False

    def has_any_permission(self, *permissions: str | Permission) -> bool:
        """Check if principal has any of the permissions."""
        return any(self.has_permission(p) for p in permissions)

    def has_all_permissions(self, *permissions: str | Permission) -> bool:
        """Check if principal has all of the permissions."""
        return all(self.has_permission(p) for p in permissions)

    def with_role(self, role: str) -> "Principal":
        """Create a new principal with an additional role."""
        new_roles = self.roles.copy()
        new_roles.add(role)
        return Principal(
            id=self.id,
            name=self.name,
            email=self.email,
            roles=new_roles,
            permissions=self.permissions.copy(),
            attributes=self.attributes.copy(),
            authenticated=self.authenticated,
        )

    def with_permission(self, permission: str | Permission) -> "Principal":
        """Create a new principal with an additional permission."""
        if isinstance(permission, str):
            permission = Permission.from_string(permission)
        new_perms = self.permissions.copy()
        new_perms.add(permission)
        return Principal(
            id=self.id,
            name=self.name,
            email=self.email,
            roles=self.roles.copy(),
            permissions=new_perms,
            attributes=self.attributes.copy(),
            authenticated=self.authenticated,
        )

    @classmethod
    def anonymous(cls) -> "Principal":
        """Create an anonymous principal."""
        return cls(id="anonymous", authenticated=False)

    @classmethod
    def system(cls) -> "Principal":
        """Create a system principal with all permissions."""
        return cls(
            id="system",
            name="System",
            roles={"system"},
            permissions={Permission("*")},
        )


__all__ = ["Principal", "Permission"]
