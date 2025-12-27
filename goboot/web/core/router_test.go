package core

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"dev.engineeringlabs/goboot/web/api"
)

func TestDefaultRouter_HandleAndServe(t *testing.T) {
	router := NewRouter()

	handlerCalled := false
	router.Handle(http.MethodGet, "/test", func(ctx *api.Context) error {
		handlerCalled = true
		return JSON(ctx, 200, map[string]string{"status": "ok"})
	})

	req := httptest.NewRequest(http.MethodGet, "/test", nil)
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	if !handlerCalled {
		t.Error("Handler should be called")
	}
	if w.Code != 200 {
		t.Errorf("Expected status 200, got %d", w.Code)
	}
}

func TestDefaultRouter_MethodRouting(t *testing.T) {
	router := NewRouter()

	router.GET("/resource", func(ctx *api.Context) error {
		return String(ctx, 200, "GET")
	})
	router.POST("/resource", func(ctx *api.Context) error {
		return String(ctx, 201, "POST")
	})
	router.PUT("/resource", func(ctx *api.Context) error {
		return String(ctx, 200, "PUT")
	})
	router.DELETE("/resource", func(ctx *api.Context) error {
		return NoContent(ctx)
	})

	tests := []struct {
		method   string
		expected int
	}{
		{http.MethodGet, 200},
		{http.MethodPost, 201},
		{http.MethodPut, 200},
		{http.MethodDelete, 204},
	}

	for _, tt := range tests {
		req := httptest.NewRequest(tt.method, "/resource", nil)
		w := httptest.NewRecorder()
		router.ServeHTTP(w, req)

		if w.Code != tt.expected {
			t.Errorf("%s: expected %d, got %d", tt.method, tt.expected, w.Code)
		}
	}
}

func TestDefaultRouter_URLParams(t *testing.T) {
	router := NewRouter()

	var capturedID string
	router.GET("/users/:id", func(ctx *api.Context) error {
		capturedID = ctx.Param("id")
		return String(ctx, 200, capturedID)
	})

	req := httptest.NewRequest(http.MethodGet, "/users/123", nil)
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	if capturedID != "123" {
		t.Errorf("Expected '123', got '%s'", capturedID)
	}
}

func TestDefaultRouter_MultipleParams(t *testing.T) {
	router := NewRouter()

	var org, repo string
	router.GET("/orgs/:org/repos/:repo", func(ctx *api.Context) error {
		org = ctx.Param("org")
		repo = ctx.Param("repo")
		return NoContent(ctx)
	})

	req := httptest.NewRequest(http.MethodGet, "/orgs/acme/repos/widgets", nil)
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	if org != "acme" {
		t.Errorf("Expected org 'acme', got '%s'", org)
	}
	if repo != "widgets" {
		t.Errorf("Expected repo 'widgets', got '%s'", repo)
	}
}

func TestDefaultRouter_NotFound(t *testing.T) {
	router := NewRouter()

	router.GET("/exists", func(ctx *api.Context) error {
		return NoContent(ctx)
	})

	req := httptest.NewRequest(http.MethodGet, "/not-exists", nil)
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	if w.Code != 404 {
		t.Errorf("Expected 404, got %d", w.Code)
	}
}

func TestDefaultRouter_Group(t *testing.T) {
	router := NewRouter()

	apiGroup := router.Group("/api")
	v1 := apiGroup.Group("/v1")

	v1.Handle(http.MethodGet, "/users", func(ctx *api.Context) error {
		return String(ctx, 200, "users")
	})

	req := httptest.NewRequest(http.MethodGet, "/api/v1/users", nil)
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	if w.Code != 200 {
		t.Errorf("Expected 200, got %d", w.Code)
	}
	if w.Body.String() != "users" {
		t.Errorf("Expected 'users', got '%s'", w.Body.String())
	}
}

func TestDefaultRouter_Middleware(t *testing.T) {
	router := NewRouter()

	order := make([]string, 0)

	router.Use(func(next api.Handler) api.Handler {
		return func(ctx *api.Context) error {
			order = append(order, "before1")
			err := next(ctx)
			order = append(order, "after1")
			return err
		}
	})

	router.Use(func(next api.Handler) api.Handler {
		return func(ctx *api.Context) error {
			order = append(order, "before2")
			err := next(ctx)
			order = append(order, "after2")
			return err
		}
	})

	router.GET("/test", func(ctx *api.Context) error {
		order = append(order, "handler")
		return NoContent(ctx)
	})

	req := httptest.NewRequest(http.MethodGet, "/test", nil)
	w := httptest.NewRecorder()

	router.ServeHTTP(w, req)

	expected := []string{"before1", "before2", "handler", "after2", "after1"}
	if len(order) != len(expected) {
		t.Errorf("Order length mismatch: %v", order)
	}
	for i, v := range expected {
		if order[i] != v {
			t.Errorf("Order[%d]: expected '%s', got '%s'", i, v, order[i])
		}
	}
}

func TestHTTPError(t *testing.T) {
	err := NewHTTPError(404, "Not Found")

	if err.Code != 404 {
		t.Errorf("Expected code 404, got %d", err.Code)
	}
	if err.Message != "Not Found" {
		t.Errorf("Expected 'Not Found', got '%s'", err.Message)
	}
	if err.Error() != "HTTP 404: Not Found" {
		t.Errorf("Unexpected Error(): %s", err.Error())
	}
}

func TestContext_QueryParams(t *testing.T) {
	req := httptest.NewRequest(http.MethodGet, "/test?page=1&limit=20", nil)
	w := httptest.NewRecorder()
	ctx := api.NewContext(w, req)

	if ctx.Query("page") != "1" {
		t.Error("Query 'page' should be '1'")
	}
	if ctx.Query("limit") != "20" {
		t.Error("Query 'limit' should be '20'")
	}
	if ctx.Query("missing") != "" {
		t.Error("Missing query should be empty")
	}
	if ctx.QueryDefault("missing", "default") != "default" {
		t.Error("QueryDefault should return default")
	}
}

func TestContext_SetGet(t *testing.T) {
	req := httptest.NewRequest(http.MethodGet, "/test", nil)
	w := httptest.NewRecorder()
	ctx := api.NewContext(w, req)

	ctx.Set("key", "value")
	val, ok := ctx.Get("key")

	if !ok {
		t.Error("Key should exist")
	}
	if val != "value" {
		t.Errorf("Expected 'value', got '%v'", val)
	}
}

func TestResponseHelpers(t *testing.T) {
	t.Run("JSON", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/", nil)
		w := httptest.NewRecorder()
		ctx := api.NewContext(w, req)

		JSON(ctx, 200, map[string]string{"key": "value"})

		if w.Code != 200 {
			t.Errorf("Expected 200, got %d", w.Code)
		}
		if w.Header().Get("Content-Type") != "application/json" {
			t.Error("Content-Type should be application/json")
		}
	})

	t.Run("String", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/", nil)
		w := httptest.NewRecorder()
		ctx := api.NewContext(w, req)

		String(ctx, 200, "hello")

		if w.Body.String() != "hello" {
			t.Errorf("Expected 'hello', got '%s'", w.Body.String())
		}
	})

	t.Run("NoContent", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/", nil)
		w := httptest.NewRecorder()
		ctx := api.NewContext(w, req)

		NoContent(ctx)

		if w.Code != 204 {
			t.Errorf("Expected 204, got %d", w.Code)
		}
	})
}

func TestMatchPattern(t *testing.T) {
	tests := []struct {
		pattern  string
		path     string
		match    bool
		params   map[string]string
	}{
		{"/users", "/users", true, map[string]string{}},
		{"/users", "/posts", false, nil},
		{"/users/:id", "/users/123", true, map[string]string{"id": "123"}},
		{"/users/:id/posts", "/users/123/posts", true, map[string]string{"id": "123"}},
		{"/orgs/:org/repos/:repo", "/orgs/acme/repos/widgets", true, map[string]string{"org": "acme", "repo": "widgets"}},
		{"/users/:id", "/users", false, nil},
		{"/users", "/users/123", false, nil},
	}

	for _, tt := range tests {
		match, params := matchPattern(tt.pattern, tt.path)
		if match != tt.match {
			t.Errorf("Pattern '%s' path '%s': expected match=%v, got %v", tt.pattern, tt.path, tt.match, match)
		}
		if match {
			for k, v := range tt.params {
				if params[k] != v {
					t.Errorf("Pattern '%s' path '%s': expected param %s=%s, got %s", tt.pattern, tt.path, k, v, params[k])
				}
			}
		}
	}
}
