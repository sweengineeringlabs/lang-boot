package cli

import (
	"testing"
)

func TestNewApp(t *testing.T) {
	app := NewApp("testapp", "Test application")

	if app.Name != "testapp" {
		t.Errorf("expected name 'testapp', got %s", app.Name)
	}
	if app.Description != "Test application" {
		t.Errorf("expected description 'Test application', got %s", app.Description)
	}
	if app.Commands == nil {
		t.Error("expected Commands map to be initialized")
	}
}

func TestVersion(t *testing.T) {
	app := NewApp("testapp", "Test")
	Version("1.0.0")(app)

	if app.Version != "1.0.0" {
		t.Errorf("expected version '1.0.0', got %s", app.Version)
	}
}

func TestAuthor(t *testing.T) {
	app := NewApp("testapp", "Test")
	Author("John Doe", "john@example.com")(app)

	if app.Author != "John Doe" {
		t.Errorf("expected author 'John Doe', got %s", app.Author)
	}
	if app.Email != "john@example.com" {
		t.Errorf("expected email 'john@example.com', got %s", app.Email)
	}
}
