// Package api contains the public interfaces and types for the session module.
package api

import (
	"context"
	"time"
)

// Session represents a user session.
type Session struct {
	ID        string
	Data      map[string]any
	CreatedAt time.Time
	ExpiresAt time.Time
	LastAccess time.Time
	UserID    string
	IPAddress string
	UserAgent string
}

// NewSession creates a new Session.
func NewSession(id string, ttl time.Duration) *Session {
	now := time.Now()
	return &Session{
		ID:        id,
		Data:      make(map[string]any),
		CreatedAt: now,
		ExpiresAt: now.Add(ttl),
		LastAccess: now,
	}
}

// Get gets a value from the session.
func (s *Session) Get(key string) (any, bool) {
	value, ok := s.Data[key]
	return value, ok
}

// Set sets a value in the session.
func (s *Session) Set(key string, value any) {
	s.Data[key] = value
}

// Delete deletes a value from the session.
func (s *Session) Delete(key string) {
	delete(s.Data, key)
}

// Clear clears all session data.
func (s *Session) Clear() {
	s.Data = make(map[string]any)
}

// IsExpired returns true if the session has expired.
func (s *Session) IsExpired() bool {
	return time.Now().After(s.ExpiresAt)
}

// Extend extends the session.
func (s *Session) Extend(ttl time.Duration) {
	s.ExpiresAt = time.Now().Add(ttl)
}

// Touch updates the last access time.
func (s *Session) Touch() {
	s.LastAccess = time.Now()
}

// SessionStore is the interface for session storage.
type SessionStore interface {
	// Get gets a session by ID.
	Get(ctx context.Context, id string) (*Session, error)

	// Save saves a session.
	Save(ctx context.Context, session *Session) error

	// Delete deletes a session.
	Delete(ctx context.Context, id string) error

	// Exists checks if a session exists.
	Exists(ctx context.Context, id string) (bool, error)

	// GC removes expired sessions.
	GC(ctx context.Context) error
}

// SessionManager is the interface for session management.
type SessionManager interface {
	// Start starts a new session.
	Start(ctx context.Context) (*Session, error)

	// Get gets a session by ID.
	Get(ctx context.Context, id string) (*Session, error)

	// Save saves a session.
	Save(ctx context.Context, session *Session) error

	// Destroy destroys a session.
	Destroy(ctx context.Context, id string) error

	// Regenerate regenerates a session ID.
	Regenerate(ctx context.Context, id string) (*Session, error)

	// GC runs garbage collection.
	GC(ctx context.Context) error
}

// SessionConfig configures session management.
type SessionConfig struct {
	// TTL is the session lifetime.
	TTL time.Duration
	// CookieName is the session cookie name.
	CookieName string
	// CookiePath is the cookie path.
	CookiePath string
	// CookieDomain is the cookie domain.
	CookieDomain string
	// CookieSecure requires HTTPS.
	CookieSecure bool
	// CookieHTTPOnly prevents JavaScript access.
	CookieHTTPOnly bool
	// CookieSameSite is the SameSite attribute.
	CookieSameSite string
	// GCInterval is the garbage collection interval.
	GCInterval time.Duration
}

// DefaultSessionConfig returns a default session configuration.
func DefaultSessionConfig() SessionConfig {
	return SessionConfig{
		TTL:            24 * time.Hour,
		CookieName:     "session_id",
		CookiePath:     "/",
		CookieSecure:   true,
		CookieHTTPOnly: true,
		CookieSameSite: "Lax",
		GCInterval:     time.Hour,
	}
}

// IDGenerator generates session IDs.
type IDGenerator interface {
	// Generate generates a new session ID.
	Generate() string
}
