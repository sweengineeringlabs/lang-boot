package core

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"dev.engineeringlabs/goboot/http/api"
)

func TestDefaultClient_Get(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			t.Errorf("Expected GET, got %s", r.Method)
		}
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("success"))
	}))
	defer server.Close()

	client := NewDefaultClient()
	resp, err := client.Get(context.Background(), server.URL)

	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if resp.StatusCode != 200 {
		t.Errorf("Expected 200, got %d", resp.StatusCode)
	}
	if string(resp.Body) != "success" {
		t.Errorf("Expected 'success', got '%s'", string(resp.Body))
	}
}

func TestDefaultClient_Post(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			t.Errorf("Expected POST, got %s", r.Method)
		}
		w.WriteHeader(http.StatusCreated)
	}))
	defer server.Close()

	client := NewDefaultClient()
	resp, err := client.Post(context.Background(), server.URL, nil)

	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if resp.StatusCode != 201 {
		t.Errorf("Expected 201, got %d", resp.StatusCode)
	}
}

func TestDefaultClient_WithBaseURL(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/users" {
			t.Errorf("Expected /api/users, got %s", r.URL.Path)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	config := api.DefaultClientConfig()
	config.BaseURL = server.URL

	client := NewClient(config)
	resp, err := client.Get(context.Background(), "/api/users")

	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if resp.StatusCode != 200 {
		t.Errorf("Expected 200, got %d", resp.StatusCode)
	}
}

func TestDefaultClient_WithDefaultHeaders(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("X-Api-Key") != "secret" {
			t.Error("Expected X-Api-Key header")
		}
		if r.Header.Get("User-Agent") != "test-client" {
			t.Error("Expected User-Agent header")
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	config := api.DefaultClientConfig()
	config.DefaultHeaders = map[string]string{
		"X-Api-Key":  "secret",
		"User-Agent": "test-client",
	}

	client := NewClient(config)
	client.Get(context.Background(), server.URL)
}

func TestDefaultClient_WithRetries(t *testing.T) {
	attempts := 0
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		attempts++
		if attempts < 3 {
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	config := api.DefaultClientConfig()
	config.MaxRetries = 3
	config.RetryDelay = 10 * time.Millisecond

	client := NewClient(config)
	resp, err := client.Get(context.Background(), server.URL)

	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if resp.StatusCode != 200 {
		t.Errorf("Expected 200 after retries, got %d", resp.StatusCode)
	}
	if attempts != 3 {
		t.Errorf("Expected 3 attempts, got %d", attempts)
	}
}

func TestDefaultClient_ResponseDuration(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		time.Sleep(50 * time.Millisecond)
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	client := NewDefaultClient()
	resp, _ := client.Get(context.Background(), server.URL)

	if resp.Duration < 50*time.Millisecond {
		t.Error("Duration should reflect actual request time")
	}
}

func TestDefaultClient_ResponseHeaders(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("X-Custom-Header", "custom-value")
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	client := NewDefaultClient()
	resp, _ := client.Get(context.Background(), server.URL)

	if resp.Headers["X-Custom-Header"] != "custom-value" {
		t.Error("Expected X-Custom-Header in response")
	}
	if resp.Headers["Content-Type"] != "application/json" {
		t.Error("Expected Content-Type in response")
	}
}

func TestDefaultClient_Delete(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodDelete {
			t.Errorf("Expected DELETE, got %s", r.Method)
		}
		w.WriteHeader(http.StatusNoContent)
	}))
	defer server.Close()

	client := NewDefaultClient()
	resp, _ := client.Delete(context.Background(), server.URL)

	if resp.StatusCode != 204 {
		t.Errorf("Expected 204, got %d", resp.StatusCode)
	}
}

func TestDefaultClient_Put(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPut {
			t.Errorf("Expected PUT, got %s", r.Method)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	client := NewDefaultClient()
	resp, _ := client.Put(context.Background(), server.URL, nil)

	if resp.StatusCode != 200 {
		t.Errorf("Expected 200, got %d", resp.StatusCode)
	}
}

func TestDefaultClient_ContextCancellation(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		time.Sleep(1 * time.Second)
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	client := NewDefaultClient()
	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel()

	_, err := client.Get(ctx, server.URL)
	if err == nil {
		t.Error("Expected error from context cancellation")
	}
}

func TestHTTPError(t *testing.T) {
	resp := &api.Response{
		StatusCode: 404,
		Body:       []byte("not found"),
	}

	err := NewHTTPError(resp)

	if err.StatusCode != 404 {
		t.Errorf("Expected 404, got %d", err.StatusCode)
	}
	if err.Status != "Not Found" {
		t.Errorf("Expected 'Not Found', got '%s'", err.Status)
	}
	if string(err.Body) != "not found" {
		t.Errorf("Expected 'not found', got '%s'", string(err.Body))
	}
}

func TestIsAbsoluteURL(t *testing.T) {
	tests := []struct {
		url      string
		expected bool
	}{
		{"http://example.com", true},
		{"https://example.com", true},
		{"/api/users", false},
		{"api/users", false},
		{"ftp://example.com", false},
	}

	for _, tt := range tests {
		result := isAbsoluteURL(tt.url)
		if result != tt.expected {
			t.Errorf("isAbsoluteURL(%s): expected %v, got %v", tt.url, tt.expected, result)
		}
	}
}
