//! Authorization module
//!
//! RBAC, ABAC, permission checks, policy evaluation

use std::collections::{HashMap, HashSet};

/// Role-based access control
#[derive(Debug, Clone)]
pub struct RoleBasedAccessControl {
    /// Maps role names to sets of permissions
    role_permissions: HashMap<String, HashSet<String>>,
}

impl RoleBasedAccessControl {
    /// Create a new RBAC instance with default roles
    pub fn new() -> Self {
        let mut rbac = Self {
            role_permissions: HashMap::new(),
        };
        
        // Initialize with common default roles
        let _ = rbac.create_role("admin");
        let _ = rbac.create_role("user");
        let _ = rbac.create_role("guest");
        
        rbac
    }
    
    /// Create a new role
    pub fn create_role(&mut self, role: &str) -> crate::SecurityResult<()> {
        if self.role_permissions.contains_key(role) {
            return Err(crate::SecurityError::AuthorizationDenied(
                format!("Role '{}' already exists", role)
            ));
        }
        self.role_permissions.insert(role.to_string(), HashSet::new());
        Ok(())
    }
    
    /// Grant a permission to a role
    pub fn grant_permission(&mut self, role: &str, permission: &str) -> crate::SecurityResult<()> {
        let permissions = self.role_permissions
            .get_mut(role)
            .ok_or_else(|| crate::SecurityError::AuthorizationDenied(
                format!("Role '{}' does not exist", role)
            ))?;
        
        permissions.insert(permission.to_string());
        Ok(())
    }
    
    /// Revoke a permission from a role
    pub fn revoke_permission(&mut self, role: &str, permission: &str) -> crate::SecurityResult<()> {
        let permissions = self.role_permissions
            .get_mut(role)
            .ok_or_else(|| crate::SecurityError::AuthorizationDenied(
                format!("Role '{}' does not exist", role)
            ))?;
        
        if !permissions.remove(permission) {
            return Err(crate::SecurityError::AuthorizationDenied(
                format!("Role '{}' does not have permission '{}'", role, permission)
            ));
        }
        
        Ok(())
    }
    
    /// Check if a role has a specific permission
    pub fn check_permission(&self, role: &str, permission: &str) -> crate::SecurityResult<bool> {
        let permissions = self.role_permissions
            .get(role)
            .ok_or_else(|| crate::SecurityError::AuthorizationDenied(
                format!("Role '{}' does not exist", role)
            ))?;
        
        Ok(permissions.contains(permission))
    }
    
    /// Get all permissions for a role
    pub fn get_permissions(&self, role: &str) -> crate::SecurityResult<Vec<String>> {
        let permissions = self.role_permissions
            .get(role)
            .ok_or_else(|| crate::SecurityError::AuthorizationDenied(
                format!("Role '{}' does not exist", role)
            ))?;
        
        Ok(permissions.iter().cloned().collect())
    }
    
    /// Check if role exists
    pub fn role_exists(&self, role: &str) -> bool {
        self.role_permissions.contains_key(role)
    }
    
    /// Get all roles
    pub fn get_roles(&self) -> Vec<String> {
        self.role_permissions.keys().cloned().collect()
    }
}

impl Default for RoleBasedAccessControl {
    fn default() -> Self {
        Self::new()
    }
}

/// User with assigned roles
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub roles: HashSet<String>,
}

impl User {
    /// Create a new user
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            roles: HashSet::new(),
        }
    }
    
    /// Add a role to the user
    pub fn add_role(&mut self, role: impl Into<String>) {
        self.roles.insert(role.into());
    }
    
    /// Remove a role from the user
    pub fn remove_role(&mut self, role: &str) -> bool {
        self.roles.remove(role)
    }
    
    /// Check if user has a role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(role)
    }
    
    /// Check if user has any of the specified roles
    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|r| self.roles.contains(*r))
    }
    
    /// Check if user has all of the specified roles
    pub fn has_all_roles(&self, roles: &[&str]) -> bool {
        roles.iter().all(|r| self.roles.contains(*r))
    }
}

/// Authorization context for evaluating permissions
pub struct AuthorizationContext<'a> {
    pub rbac: &'a RoleBasedAccessControl,
    pub user: &'a User,
}

impl<'a> AuthorizationContext<'a> {
    /// Create a new authorization context
    pub fn new(rbac: &'a RoleBasedAccessControl, user: &'a User) -> Self {
        Self { rbac, user }
    }
    
    /// Check if user has a specific permission through any of their roles
    pub fn has_permission(&self, permission: &str) -> crate::SecurityResult<bool> {
        for role in &self.user.roles {
            if self.rbac.check_permission(role, permission)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
    
    /// Require a specific permission (returns error if not granted)
    pub fn require_permission(&self, permission: &str) -> crate::SecurityResult<()> {
        if self.has_permission(permission)? {
            Ok(())
        } else {
            Err(crate::SecurityError::AuthorizationDenied(
                format!("User '{}' does not have permission '{}'", self.user.id, permission)
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_role() {
        let mut rbac = RoleBasedAccessControl::new();
        assert!(rbac.create_role("developer").is_ok());
        assert!(rbac.role_exists("developer"));
    }

    #[test]
    fn test_grant_and_check_permission() {
        let mut rbac = RoleBasedAccessControl::new();
        rbac.grant_permission("admin", "delete_user").unwrap();
        
        let has_permission = rbac.check_permission("admin", "delete_user").unwrap();
        assert!(has_permission);
        
        let no_permission = rbac.check_permission("user", "delete_user").unwrap();
        assert!(!no_permission);
    }

    #[test]
    fn test_revoke_permission() {
        let mut rbac = RoleBasedAccessControl::new();
        rbac.grant_permission("admin", "delete_user").unwrap();
        rbac.revoke_permission("admin", "delete_user").unwrap();
        
        let has_permission = rbac.check_permission("admin", "delete_user").unwrap();
        assert!(!has_permission);
    }

    #[test]
    fn test_user_roles() {
        let mut user = User::new("user123");
        user.add_role("developer");
        user.add_role("reviewer");
        
        assert!(user.has_role("developer"));
        assert!(user.has_role("reviewer"));
        assert!(!user.has_role("admin"));
        
        assert!(user.has_any_role(&["admin", "developer"]));
        assert!(!user.has_all_roles(&["admin", "developer"]));
        assert!(user.has_all_roles(&["developer", "reviewer"]));
    }

    #[test]
    fn test_authorization_context() {
        let mut rbac = RoleBasedAccessControl::new();
        rbac.create_role("developer").unwrap();
        rbac.create_role("reviewer").unwrap();
        rbac.grant_permission("developer", "write_code").unwrap();
        rbac.grant_permission("reviewer", "review_code").unwrap();
        
        let mut user = User::new("alice");
        user.add_role("developer");
        
        let ctx = AuthorizationContext::new(&rbac, &user);
        
        assert!(ctx.has_permission("write_code").unwrap());
        assert!(!ctx.has_permission("review_code").unwrap());
        
        assert!(ctx.require_permission("write_code").is_ok());
        assert!(ctx.require_permission("review_code").is_err());
    }

    #[test]
    fn test_get_permissions() {
        let mut rbac = RoleBasedAccessControl::new();
        rbac.grant_permission("admin", "read").unwrap();
        rbac.grant_permission("admin", "write").unwrap();
        rbac.grant_permission("admin", "delete").unwrap();
        
        let permissions = rbac.get_permissions("admin").unwrap();
        assert_eq!(permissions.len(), 3);
        assert!(permissions.contains(&"read".to_string()));
        assert!(permissions.contains(&"write".to_string()));
        assert!(permissions.contains(&"delete".to_string()));
    }
}
