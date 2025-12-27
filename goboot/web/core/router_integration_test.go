package core

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"dev.engineeringlabs/goboot/web/api"
)

// Integration tests for the web router

func TestRouter_FullRequestLifecycle(t *testing.T) {
	router := NewRouter()

	// Add logging middleware
	logs := []string{}
	router.Use(func(next api.Handler) api.Handler {
		return func(ctx *api.Context) error {
			logs = append(logs, "before:"+ctx.Request.URL.Path)
			err := next(ctx)
			logs = append(logs, "after:"+ctx.Request.URL.Path)
			return err
		}
	})

	// Add authentication middleware
	router.Use(func(next api.Handler) api.Handler {
		return func(ctx *api.Context) error {
			token := ctx.Request.Header.Get("Authorization")
			if token != "" {
				ctx.Set("authenticated", true)
				ctx.Set("user", strings.TrimPrefix(token, "Bearer "))
			}
			return next(ctx)
		}
	})

	// Register routes
	router.GET("/public", func(ctx *api.Context) error {
		return JSON(ctx, 200, map[string]string{"access": "public"})
	})

	router.GET("/private", func(ctx *api.Context) error {
		auth, _ := ctx.Get("authenticated")
		if auth != true {
			return NewHTTPError(401, "Unauthorized")
		}
		user, _ := ctx.Get("user")
		return JSON(ctx, 200, map[string]any{"user": user})
	})

	router.POST("/users", func(ctx *api.Context) error {
		return JSON(ctx, 201, map[string]string{"created": "true"})
	})

	t.Run("PublicEndpoint", func(t *testing.T) {
		logs = []string{}
		req := httptest.NewRequest(http.MethodGet, "/public", nil)
		w := httptest.NewRecorder()

		router.ServeHTTP(w, req)

		if w.Code != 200 {
			t.Errorf("Expected 200, got %d", w.Code)
		}
		if len(logs) != 2 {
			t.Errorf("Expected 2 log entries, got %d", len(logs))
		}
	})

	t.Run("PrivateEndpoint_Unauthenticated", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/private", nil)
		w := httptest.NewRecorder()

		router.ServeHTTP(w, req)

		if w.Code != 401 {
			t.Errorf("Expected 401, got %d", w.Code)
		}
	})

	t.Run("PrivateEndpoint_Authenticated", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/private", nil)
		req.Header.Set("Authorization", "Bearer john")
		w := httptest.NewRecorder()

		router.ServeHTTP(w, req)

		if w.Code != 200 {
			t.Errorf("Expected 200, got %d", w.Code)
		}
		if !strings.Contains(w.Body.String(), "john") {
			t.Error("Response should contain user")
		}
	})

	t.Run("POST_Endpoint", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodPost, "/users", nil)
		w := httptest.NewRecorder()

		router.ServeHTTP(w, req)

		if w.Code != 201 {
			t.Errorf("Expected 201, got %d", w.Code)
		}
	})
}

func TestRouter_RouteGroups(t *testing.T) {
	router := NewRouter()

	// API v1 - using Handle method instead of GET
	v1 := router.Group("/api/v1")
	v1.Handle(http.MethodGet, "/users", func(ctx *api.Context) error {
		return String(ctx, 200, "v1:users")
	})
	v1.Handle(http.MethodGet, "/posts", func(ctx *api.Context) error {
		return String(ctx, 200, "v1:posts")
	})

	// API v2
	v2 := router.Group("/api/v2")
	v2.Handle(http.MethodGet, "/users", func(ctx *api.Context) error {
		return String(ctx, 200, "v2:users")
	})

	tests := []struct {
		path     string
		expected string
	}{
		{"/api/v1/users", "v1:users"},
		{"/api/v1/posts", "v1:posts"},
		{"/api/v2/users", "v2:users"},
	}

	for _, tt := range tests {
		t.Run(tt.path, func(t *testing.T) {
			req := httptest.NewRequest(http.MethodGet, tt.path, nil)
			w := httptest.NewRecorder()
			router.ServeHTTP(w, req)

			if w.Body.String() != tt.expected {
				t.Errorf("Expected '%s', got '%s'", tt.expected, w.Body.String())
			}
		})
	}
}

func TestRouter_ComplexParams(t *testing.T) {
	router := NewRouter()

	router.GET("/users/:userId/posts/:postId/comments/:commentId", func(ctx *api.Context) error {
		return JSON(ctx, 200, map[string]string{
			"userId":    ctx.Param("userId"),
			"postId":    ctx.Param("postId"),
			"commentId": ctx.Param("commentId"),
		})
	})

	req := httptest.NewRequest(http.MethodGet, "/users/123/posts/456/comments/789", nil)
	w := httptest.NewRecorder()
	router.ServeHTTP(w, req)

	body := w.Body.String()
	if !strings.Contains(body, "123") || !strings.Contains(body, "456") || !strings.Contains(body, "789") {
		t.Errorf("Should contain all param values: %s", body)
	}
}

func TestRouter_QueryParameters(t *testing.T) {
	router := NewRouter()

	router.GET("/search", func(ctx *api.Context) error {
		q := ctx.Query("q")
		page := ctx.QueryDefault("page", "1")
		limit := ctx.QueryDefault("limit", "10")
		return JSON(ctx, 200, map[string]string{
			"q":     q,
			"page":  page,
			"limit": limit,
		})
	})

	t.Run("AllParams", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/search?q=hello&page=2&limit=20", nil)
		w := httptest.NewRecorder()
		router.ServeHTTP(w, req)

		body := w.Body.String()
		if !strings.Contains(body, `"q":"hello"`) {
			t.Error("Should contain query parameter")
		}
		if !strings.Contains(body, `"page":"2"`) {
			t.Error("Should contain page parameter")
		}
	})

	t.Run("DefaultParams", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/search?q=test", nil)
		w := httptest.NewRecorder()
		router.ServeHTTP(w, req)

		body := w.Body.String()
		if !strings.Contains(body, `"page":"1"`) {
			t.Error("Should use default page")
		}
		if !strings.Contains(body, `"limit":"10"`) {
			t.Error("Should use default limit")
		}
	})
}

func TestRouter_MiddlewareOrdering(t *testing.T) {
	router := NewRouter()
	order := []string{}

	// Global middleware
	router.Use(func(next api.Handler) api.Handler {
		return func(ctx *api.Context) error {
			order = append(order, "global:before")
			err := next(ctx)
			order = append(order, "global:after")
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

	expected := []string{
		"global:before",
		"handler",
		"global:after",
	}

	if len(order) != len(expected) {
		t.Errorf("Order length mismatch: %v", order)
	}
	for i, v := range expected {
		if i >= len(order) || order[i] != v {
			t.Errorf("Order[%d] mismatch: expected %s, got %s", i, v, order[i])
		}
	}
}

func TestRouter_ErrorHandling(t *testing.T) {
	router := NewRouter()

	router.GET("/error", func(ctx *api.Context) error {
		return NewHTTPError(500, "Internal Server Error")
	})

	router.GET("/custom-error", func(ctx *api.Context) error {
		return NewHTTPError(422, "Validation Failed").WithDetail("field", "email")
	})

	t.Run("BasicError", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/error", nil)
		w := httptest.NewRecorder()
		router.ServeHTTP(w, req)

		if w.Code != 500 {
			t.Errorf("Expected 500, got %d", w.Code)
		}
	})

	t.Run("CustomError", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/custom-error", nil)
		w := httptest.NewRecorder()
		router.ServeHTTP(w, req)

		if w.Code != 422 {
			t.Errorf("Expected 422, got %d", w.Code)
		}
	})
}

func TestRouter_AllMethods(t *testing.T) {
	router := NewRouter()

	methods := []string{
		http.MethodGet,
		http.MethodPost,
		http.MethodPut,
		http.MethodDelete,
		http.MethodPatch,
	}

	for _, method := range methods {
		m := method
		router.Handle(m, "/resource", func(ctx *api.Context) error {
			return String(ctx, 200, m)
		})
	}

	for _, method := range methods {
		t.Run(method, func(t *testing.T) {
			req := httptest.NewRequest(method, "/resource", nil)
			w := httptest.NewRecorder()
			router.ServeHTTP(w, req)

			if w.Body.String() != method {
				t.Errorf("Expected '%s', got '%s'", method, w.Body.String())
			}
		})
	}
}

// Benchmark tests

func BenchmarkRouter_SimpleRoute(b *testing.B) {
	router := NewRouter()
	router.GET("/test", func(ctx *api.Context) error {
		return NoContent(ctx)
	})

	req := httptest.NewRequest(http.MethodGet, "/test", nil)
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		w := httptest.NewRecorder()
		router.ServeHTTP(w, req)
	}
}

func BenchmarkRouter_ParamRoute(b *testing.B) {
	router := NewRouter()
	router.GET("/users/:id", func(ctx *api.Context) error {
		_ = ctx.Param("id")
		return NoContent(ctx)
	})

	req := httptest.NewRequest(http.MethodGet, "/users/123", nil)
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		w := httptest.NewRecorder()
		router.ServeHTTP(w, req)
	}
}

func BenchmarkRouter_WithMiddleware(b *testing.B) {
	router := NewRouter()
	router.Use(func(next api.Handler) api.Handler {
		return func(ctx *api.Context) error {
			return next(ctx)
		}
	})
	router.GET("/test", func(ctx *api.Context) error {
		return NoContent(ctx)
	})

	req := httptest.NewRequest(http.MethodGet, "/test", nil)
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		w := httptest.NewRecorder()
		router.ServeHTTP(w, req)
	}
}

func BenchmarkRouter_JSON(b *testing.B) {
	router := NewRouter()
	router.GET("/test", func(ctx *api.Context) error {
		return JSON(ctx, 200, map[string]string{"key": "value"})
	})

	req := httptest.NewRequest(http.MethodGet, "/test", nil)
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		w := httptest.NewRecorder()
		router.ServeHTTP(w, req)
	}
}
