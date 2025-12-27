// Package core contains the implementation details for the session module.
package core

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"sync"
	"time"

	"dev.engineeringlabs/goboot/session/api"
)

// MemoryStore is an in-memory session store.
type MemoryStore struct {
	sessions map[string]*api.Session
	mu       sync.RWMutex
}

// NewMemoryStore creates a new MemoryStore.
func NewMemoryStore() *MemoryStore {
	return &MemoryStore{
		sessions: make(map[string]*api.Session),
	}
}

// Get gets a session by ID.
func (s *MemoryStore) Get(ctx context.Context, id string) (*api.Session, error) {
	s.mu.RLock()
	session, ok := s.sessions[id]
	s.mu.RUnlock()

	if !ok {
		return nil, fmt.Errorf("session not found: %s", id)
	}

	if session.IsExpired() {
		s.Delete(ctx, id)
		return nil, fmt.Errorf("session expired: %s", id)
	}

	session.Touch()
	return session, nil
}

// Save saves a session.
func (s *MemoryStore) Save(ctx context.Context, session *api.Session) error {
	s.mu.Lock()
	s.sessions[session.ID] = session
	s.mu.Unlock()
	return nil
}

// Delete deletes a session.
func (s *MemoryStore) Delete(ctx context.Context, id string) error {
	s.mu.Lock()
	delete(s.sessions, id)
	s.mu.Unlock()
	return nil
}

// Exists checks if a session exists.
func (s *MemoryStore) Exists(ctx context.Context, id string) (bool, error) {
	s.mu.RLock()
	session, ok := s.sessions[id]
	s.mu.RUnlock()

	if !ok {
		return false, nil
	}

	return !session.IsExpired(), nil
}

// GC removes expired sessions.
func (s *MemoryStore) GC(ctx context.Context) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	for id, session := range s.sessions {
		if session.IsExpired() {
			delete(s.sessions, id)
		}
	}

	return nil
}

// DefaultSessionManager is the default session manager.
type DefaultSessionManager struct {
	store     api.SessionStore
	config    api.SessionConfig
	idGen     api.IDGenerator
}

// NewSessionManager creates a new DefaultSessionManager.
func NewSessionManager(store api.SessionStore, config api.SessionConfig) *DefaultSessionManager {
	return &DefaultSessionManager{
		store:  store,
		config: config,
		idGen:  &DefaultIDGenerator{},
	}
}

// Start starts a new session.
func (m *DefaultSessionManager) Start(ctx context.Context) (*api.Session, error) {
	id := m.idGen.Generate()
	session := api.NewSession(id, m.config.TTL)

	if err := m.store.Save(ctx, session); err != nil {
		return nil, err
	}

	return session, nil
}

// Get gets a session by ID.
func (m *DefaultSessionManager) Get(ctx context.Context, id string) (*api.Session, error) {
	return m.store.Get(ctx, id)
}

// Save saves a session.
func (m *DefaultSessionManager) Save(ctx context.Context, session *api.Session) error {
	return m.store.Save(ctx, session)
}

// Destroy destroys a session.
func (m *DefaultSessionManager) Destroy(ctx context.Context, id string) error {
	return m.store.Delete(ctx, id)
}

// Regenerate regenerates a session ID.
func (m *DefaultSessionManager) Regenerate(ctx context.Context, id string) (*api.Session, error) {
	// Get existing session
	oldSession, err := m.store.Get(ctx, id)
	if err != nil {
		return nil, err
	}

	// Delete old session
	if err := m.store.Delete(ctx, id); err != nil {
		return nil, err
	}

	// Create new session with same data
	newID := m.idGen.Generate()
	newSession := api.NewSession(newID, m.config.TTL)
	newSession.Data = oldSession.Data
	newSession.UserID = oldSession.UserID
	newSession.IPAddress = oldSession.IPAddress
	newSession.UserAgent = oldSession.UserAgent

	if err := m.store.Save(ctx, newSession); err != nil {
		return nil, err
	}

	return newSession, nil
}

// GC runs garbage collection.
func (m *DefaultSessionManager) GC(ctx context.Context) error {
	return m.store.GC(ctx)
}

// StartGC starts the garbage collector.
func (m *DefaultSessionManager) StartGC(ctx context.Context) {
	go func() {
		ticker := time.NewTicker(m.config.GCInterval)
		defer ticker.Stop()

		for {
			select {
			case <-ticker.C:
				m.GC(ctx)
			case <-ctx.Done():
				return
			}
		}
	}()
}

// DefaultIDGenerator generates session IDs.
type DefaultIDGenerator struct{}

// Generate generates a new session ID.
func (g *DefaultIDGenerator) Generate() string {
	bytes := make([]byte, 32)
	rand.Read(bytes)
	return hex.EncodeToString(bytes)
}

// SessionMiddlewareConfig configures session middleware.
type SessionMiddlewareConfig struct {
	Manager    api.SessionManager
	CookieName string
	Secure     bool
	HTTPOnly   bool
	SameSite   string
	Path       string
	Domain     string
}

// WithSession is a helper to set a session in context.
func WithSession(ctx context.Context, session *api.Session) context.Context {
	return context.WithValue(ctx, sessionContextKey{}, session)
}

// GetSession gets a session from context.
func GetSession(ctx context.Context) (*api.Session, bool) {
	session, ok := ctx.Value(sessionContextKey{}).(*api.Session)
	return session, ok
}

type sessionContextKey struct{}
