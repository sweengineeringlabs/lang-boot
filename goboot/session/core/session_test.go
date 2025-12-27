package core

import (
	"context"
	"testing"
	"time"

	"dev.engineeringlabs/goboot/session/api"
)

func TestMemoryStore_SaveAndGet(t *testing.T) {
	store := NewMemoryStore()
	ctx := context.Background()

	session := api.NewSession("test-session", time.Hour)
	session.Set("user", "john")

	store.Save(ctx, session)

	retrieved, err := store.Get(ctx, "test-session")
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	val, _ := retrieved.Get("user")
	if val != "john" {
		t.Error("Session data mismatch")
	}
}

func TestMemoryStore_GetNotFound(t *testing.T) {
	store := NewMemoryStore()
	ctx := context.Background()

	_, err := store.Get(ctx, "nonexistent")
	if err == nil {
		t.Error("Expected error for nonexistent session")
	}
}

func TestMemoryStore_Delete(t *testing.T) {
	store := NewMemoryStore()
	ctx := context.Background()

	session := api.NewSession("test-session", time.Hour)
	store.Save(ctx, session)
	store.Delete(ctx, "test-session")

	exists, _ := store.Exists(ctx, "test-session")
	if exists {
		t.Error("Session should be deleted")
	}
}

func TestMemoryStore_ExpiredSession(t *testing.T) {
	store := NewMemoryStore()
	ctx := context.Background()

	session := &api.Session{
		ID:        "expired",
		Data:      make(map[string]any),
		ExpiresAt: time.Now().Add(-time.Hour), // Already expired
	}
	store.Save(ctx, session)

	_, err := store.Get(ctx, "expired")
	if err == nil {
		t.Error("Expected error for expired session")
	}
}

func TestMemoryStore_GC(t *testing.T) {
	store := NewMemoryStore()
	ctx := context.Background()

	// Add expired session
	store.Save(ctx, &api.Session{
		ID:        "expired",
		Data:      make(map[string]any),
		ExpiresAt: time.Now().Add(-time.Hour),
	})

	// Add valid session
	store.Save(ctx, &api.Session{
		ID:        "valid",
		Data:      make(map[string]any),
		ExpiresAt: time.Now().Add(time.Hour),
	})

	store.GC(ctx)

	exists, _ := store.Exists(ctx, "expired")
	if exists {
		t.Error("Expired session should be removed by GC")
	}

	exists, _ = store.Exists(ctx, "valid")
	if !exists {
		t.Error("Valid session should not be removed by GC")
	}
}

func TestDefaultSessionManager_StartSession(t *testing.T) {
	store := NewMemoryStore()
	config := api.DefaultSessionConfig()
	manager := NewSessionManager(store, config)
	ctx := context.Background()

	session, err := manager.Start(ctx)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if session.ID == "" {
		t.Error("Session ID should not be empty")
	}
}

func TestDefaultSessionManager_Regenerate(t *testing.T) {
	store := NewMemoryStore()
	config := api.DefaultSessionConfig()
	manager := NewSessionManager(store, config)
	ctx := context.Background()

	// Start session
	session, _ := manager.Start(ctx)
	session.Set("key", "value")
	manager.Save(ctx, session)

	oldID := session.ID

	// Regenerate
	newSession, err := manager.Regenerate(ctx, oldID)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	if newSession.ID == oldID {
		t.Error("New session should have different ID")
	}

	// Check data preserved
	val, _ := newSession.Get("key")
	if val != "value" {
		t.Error("Session data should be preserved")
	}

	// Old session should be gone
	_, err = manager.Get(ctx, oldID)
	if err == nil {
		t.Error("Old session should be deleted")
	}
}

func TestSession_GetSet(t *testing.T) {
	session := api.NewSession("test", time.Hour)

	session.Set("key", "value")
	val, ok := session.Get("key")

	if !ok {
		t.Error("Key should exist")
	}
	if val != "value" {
		t.Errorf("Expected 'value', got %v", val)
	}
}

func TestSession_Delete(t *testing.T) {
	session := api.NewSession("test", time.Hour)

	session.Set("key", "value")
	session.Delete("key")

	_, ok := session.Get("key")
	if ok {
		t.Error("Key should be deleted")
	}
}

func TestSession_IsExpired(t *testing.T) {
	expired := &api.Session{ExpiresAt: time.Now().Add(-time.Hour)}
	if !expired.IsExpired() {
		t.Error("Should be expired")
	}

	valid := &api.Session{ExpiresAt: time.Now().Add(time.Hour)}
	if valid.IsExpired() {
		t.Error("Should not be expired")
	}
}
