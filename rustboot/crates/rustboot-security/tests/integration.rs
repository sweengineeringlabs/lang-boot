//! Integration tests for rustboot-security
//! 
//! Tests the public API of the crate - only what external users can access

use dev_engineeringlabs_rustboot_security::*;
use std::time::Duration;

// ============================================================================
// Integration Tests - Test public API only
// ============================================================================

#[test]
fn full_jwt_flow() {
    // Generate token
    let token = generate_jwt("user123", Duration::from_secs(3600)).unwrap();
    
    // Validate token
    let claims = validate_jwt(&token).unwrap();
    
    // Verify claims
    assert_eq!(claims.sub, "user123");
    assert!(claims.exp > 0);
    assert!(claims.iat > 0);
}

#[test]
fn jwt_rejects_invalid_token() {
    let result = validate_jwt("not.a.valid.token");
    assert!(result.is_err());
}

#[test]
fn rbac_authorization_flow() {
    // Setup RBAC
    let mut rbac = RoleBasedAccessControl::new();
    
    // Grant permissions
    rbac.grant_permission("admin", "users:write").unwrap();
    rbac.grant_permission("admin", "users:read").unwrap();
    rbac.grant_permission("user", "users:read").unwrap();
    
    // Test permissions
    assert!(rbac.check_permission("admin", "users:write").unwrap());
    assert!(rbac.check_permission("admin", "users:read").unwrap());
    assert!(rbac.check_permission("user", "users:read").unwrap());
    assert!(!rbac.check_permission("user", "users:write").unwrap());
}

#[test]
fn rbac_with_user_context() {
    let mut rbac = RoleBasedAccessControl::new();
    rbac.create_role("developer").unwrap();
    rbac.create_role("reviewer").unwrap();
    rbac.grant_permission("developer", "code:write").unwrap();
    rbac.grant_permission("reviewer", "code:review").unwrap();
    
    let mut user = User::new("alice");
    user.add_role("developer");
    
    let ctx = AuthorizationContext::new(&rbac, &user);
    
    assert!(ctx.has_permission("code:write").unwrap());
    assert!(!ctx.has_permission("code:review").unwrap());
}

#[test]
fn secret_encryption_flow() {
    let secret_data = b"super_secret_api_key";
    
    // Encrypt
    let encrypted = encrypt_secret(secret_data).unwrap();
    assert_ne!(encrypted.as_slice(), secret_data);
    
    // Decrypt
    let decrypted = decrypt_secret(&encrypted).unwrap();
    assert_eq!(decrypted.as_slice(), secret_data);
}

#[test]
fn secret_store_flow() {
    let store = SecretStore::new();
    
    // Store secrets
    let api_key = b"secret_api_key_123".to_vec();
    store.store("api_key", api_key.clone()).unwrap();
    
    // Retrieve
    let retrieved = store.retrieve("api_key").unwrap();
    assert_eq!(retrieved, api_key);
    
    // Verify existence
    assert!(store.exists("api_key"));
    assert!(!store.exists("nonexistent"));
}

#[test]
fn audit_logging_flow() {
    use serde_json::json;
    
    // Log authentication event using convenience function
    audit_login("user123", true).unwrap();
    audit_login("attacker", false).unwrap();
    
    // Log permission events
    audit_permission("user123", "users:write", false).unwrap();
    audit_permission("admin", "users:delete", true).unwrap();
    
    // Log custom event using EventType
    audit_event(
        EventType::DataRead,
        "user123",
        json!({
            "resource": "users",
            "timestamp": 1234567890,
        }),
    ).unwrap();
}

#[test]
fn audit_logger_workflow() {
    let logger = AuditLogger::new();
    
    // Create and log events
    let event1 = SecurityEvent::new(EventType::Login, "alice")
        .with_severity(Severity::Info)
        .with_resource("api/v1/login");
    
    let event2 = SecurityEvent::new(EventType::SecurityViolation, "bob")
        .with_severity(Severity::Critical)
        .with_metadata(serde_json::json!({"reason": "brute_force"}));
    
    logger.log(event1).unwrap();
    logger.log(event2).unwrap();
    
    // Query events
    assert_eq!(logger.count(), 2);
    
    let critical_events = logger.get_events_by_severity(Severity::Critical);
    assert_eq!(critical_events.len(), 1);
    assert_eq!(critical_events[0].subject, "bob");
    
    let alice_events = logger.get_events_by_subject("alice");
    assert_eq!(alice_events.len(), 1);
}

#[test]
fn end_to_end_security_workflow() {
    // 1. Authenticate user
    let token = generate_jwt("alice", Duration::from_secs(3600)).unwrap();
    let claims = validate_jwt(&token).unwrap();
    audit_login(&claims.sub, true).unwrap();
    
    // 2. Setup authorization
    let mut rbac = RoleBasedAccessControl::new();
    rbac.create_role("developer").unwrap();
    rbac.grant_permission("developer", "code:write").unwrap();
    rbac.grant_permission("developer", "deploy:staging").unwrap();
    
    let mut user = User::new(&claims.sub);
    user.add_role("developer");
    
    // 3. Check permissions
    let ctx = AuthorizationContext::new(&rbac, &user);
    assert!(ctx.require_permission("code:write").is_ok());
    audit_permission(&user.id, "code:write", true).unwrap();
    
    // 4. Manage secrets
    let store = SecretStore::new();
    let db_password = b"super_secret_db_password".to_vec();
    store.store("db_password", db_password.clone()).unwrap();
    
    // 5. Retrieve and use secret
    let retrieved_password = store.retrieve("db_password").unwrap();
    assert_eq!(retrieved_password, db_password);
    
    // Complete workflow succeeded
}
