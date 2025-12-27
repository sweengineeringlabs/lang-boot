// Package session provides session management utilities for the goboot framework.
//
// This module provides:
//   - API layer: Session, SessionStore, SessionManager interfaces
//   - Core layer: MemoryStore, DefaultSessionManager
//
// Example:
//
//	import "dev.engineeringlabs/goboot/session"
//
//	store := session.NewMemoryStore()
//	config := session.DefaultSessionConfig()
//	manager := session.NewSessionManager(store, config)
//
//	// Start a new session
//	sess, _ := manager.Start(ctx)
//	sess.Set("user_id", "123")
//	manager.Save(ctx, sess)
//
//	// Get session
//	sess, _ = manager.Get(ctx, sessionID)
//	userID, _ := sess.Get("user_id")
//
//	// Destroy session
//	manager.Destroy(ctx, sessionID)
package session

import (
	"dev.engineeringlabs/goboot/session/api"
	"dev.engineeringlabs/goboot/session/core"
)

// Re-export API types
type (
	// Session represents a user session.
	Session = api.Session
	// SessionStore is the interface for session storage.
	SessionStore = api.SessionStore
	// SessionManager is the interface for session management.
	SessionManager = api.SessionManager
	// SessionConfig configures session management.
	SessionConfig = api.SessionConfig
	// IDGenerator generates session IDs.
	IDGenerator = api.IDGenerator
)

// Re-export API functions
var (
	NewSession           = api.NewSession
	DefaultSessionConfig = api.DefaultSessionConfig
)

// Re-export Core types
type (
	// MemoryStore is an in-memory session store.
	MemoryStore = core.MemoryStore
	// DefaultSessionManager is the default session manager.
	DefaultSessionManager = core.DefaultSessionManager
	// DefaultIDGenerator generates session IDs.
	DefaultIDGenerator = core.DefaultIDGenerator
)

// Re-export Core functions
var (
	NewMemoryStore    = core.NewMemoryStore
	NewSessionManager = core.NewSessionManager
	WithSession       = core.WithSession
	GetSession        = core.GetSession
)
