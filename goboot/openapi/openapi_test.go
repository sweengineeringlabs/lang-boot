package openapi

import (
	"testing"
)

func TestNewSpec(t *testing.T) {
	spec := NewSpec("Test API", "1.0.0")

	if spec.OpenAPI != "3.0.3" {
		t.Errorf("expected OpenAPI '3.0.3', got %s", spec.OpenAPI)
	}
	if spec.Info.Title != "Test API" {
		t.Errorf("expected title 'Test API', got %s", spec.Info.Title)
	}
	if spec.Info.Version != "1.0.0" {
		t.Errorf("expected version '1.0.0', got %s", spec.Info.Version)
	}
	if spec.Paths == nil {
		t.Error("expected Paths map to be initialized")
	}
}

func TestArrayOf(t *testing.T) {
	type User struct {
		ID   int    `json:"id"`
		Name string `json:"name"`
	}

	schema := ArrayOf(User{})

	if schema.Type != "array" {
		t.Errorf("expected type 'array', got %s", schema.Type)
	}
	if schema.Items == nil {
		t.Error("expected Items to be set")
	}
}

func TestSchemaFrom(t *testing.T) {
	type User struct {
		ID   int    `json:"id"`
		Name string `json:"name"`
	}

	schema := SchemaFrom(User{})

	if schema == nil {
		t.Error("expected schema to be created")
	}
	if schema.Type != "object" {
		t.Errorf("expected type 'object', got %s", schema.Type)
	}
}
