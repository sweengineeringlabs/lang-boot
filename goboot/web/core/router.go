// Package core contains the implementation details for the web module.
package core

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
	"sync"

	"dev.engineeringlabs/goboot/web/api"
)

// DefaultRouter is the default router implementation.
type DefaultRouter struct {
	routes     map[string]map[string]api.Handler
	middleware []api.Middleware
	prefix     string
	mu         sync.RWMutex
}

// NewRouter creates a new DefaultRouter.
func NewRouter() *DefaultRouter {
	return &DefaultRouter{
		routes:     make(map[string]map[string]api.Handler),
		middleware: make([]api.Middleware, 0),
	}
}

// Handle registers a handler for a pattern.
func (r *DefaultRouter) Handle(method, pattern string, handler api.Handler) {
	r.mu.Lock()
	defer r.mu.Unlock()

	fullPattern := r.prefix + pattern
	if r.routes[method] == nil {
		r.routes[method] = make(map[string]api.Handler)
	}
	r.routes[method][fullPattern] = handler
}

// Use adds middleware to the router.
func (r *DefaultRouter) Use(middleware ...api.Middleware) {
	r.middleware = append(r.middleware, middleware...)
}

// Group creates a new route group with a prefix.
func (r *DefaultRouter) Group(prefix string) api.Router {
	return &DefaultRouter{
		routes:     r.routes,
		middleware: r.middleware,
		prefix:     r.prefix + prefix,
	}
}

// GET registers a GET handler.
func (r *DefaultRouter) GET(pattern string, handler api.Handler) {
	r.Handle(http.MethodGet, pattern, handler)
}

// POST registers a POST handler.
func (r *DefaultRouter) POST(pattern string, handler api.Handler) {
	r.Handle(http.MethodPost, pattern, handler)
}

// PUT registers a PUT handler.
func (r *DefaultRouter) PUT(pattern string, handler api.Handler) {
	r.Handle(http.MethodPut, pattern, handler)
}

// DELETE registers a DELETE handler.
func (r *DefaultRouter) DELETE(pattern string, handler api.Handler) {
	r.Handle(http.MethodDelete, pattern, handler)
}

// PATCH registers a PATCH handler.
func (r *DefaultRouter) PATCH(pattern string, handler api.Handler) {
	r.Handle(http.MethodPatch, pattern, handler)
}

// ServeHTTP implements http.Handler.
func (r *DefaultRouter) ServeHTTP(w http.ResponseWriter, req *http.Request) {
	ctx := api.NewContext(w, req)

	// Find handler
	r.mu.RLock()
	methodRoutes := r.routes[req.Method]
	var handler api.Handler
	var params map[string]string

	for pattern, h := range methodRoutes {
		if match, p := matchPattern(pattern, req.URL.Path); match {
			handler = h
			params = p
			break
		}
	}
	r.mu.RUnlock()

	if handler == nil {
		http.NotFound(w, req)
		return
	}

	ctx.Params = params

	// Apply middleware
	finalHandler := handler
	for i := len(r.middleware) - 1; i >= 0; i-- {
		finalHandler = r.middleware[i](finalHandler)
	}

	// Execute handler
	if err := finalHandler(ctx); err != nil {
		handleError(ctx, err)
	}
}

// matchPattern matches a URL pattern to a path and extracts parameters.
func matchPattern(pattern, path string) (bool, map[string]string) {
	params := make(map[string]string)

	patternParts := strings.Split(strings.Trim(pattern, "/"), "/")
	pathParts := strings.Split(strings.Trim(path, "/"), "/")

	if len(patternParts) != len(pathParts) {
		return false, nil
	}

	for i, part := range patternParts {
		if strings.HasPrefix(part, ":") {
			// Parameter
			paramName := strings.TrimPrefix(part, ":")
			params[paramName] = pathParts[i]
		} else if part != pathParts[i] {
			return false, nil
		}
	}

	return true, params
}

func handleError(ctx *api.Context, err error) {
	if httpErr, ok := err.(*HTTPError); ok {
		ctx.Response.WriteHeader(httpErr.Code)
		json.NewEncoder(ctx.Response).Encode(map[string]string{"error": httpErr.Message})
	} else {
		ctx.Response.WriteHeader(http.StatusInternalServerError)
		json.NewEncoder(ctx.Response).Encode(map[string]string{"error": err.Error()})
	}
}

// HTTPError represents an HTTP error.
type HTTPError struct {
	Code    int
	Message string
	Details map[string]any
}

func (e *HTTPError) Error() string {
	return fmt.Sprintf("HTTP %d: %s", e.Code, e.Message)
}

// WithDetail adds a detail to the error and returns the error for chaining.
func (e *HTTPError) WithDetail(key string, value any) *HTTPError {
	if e.Details == nil {
		e.Details = make(map[string]any)
	}
	e.Details[key] = value
	return e
}

// NewHTTPError creates a new HTTPError.
func NewHTTPError(code int, message string) *HTTPError {
	return &HTTPError{Code: code, Message: message}
}

// Error constructors
var (
	ErrBadRequest          = func(msg string) error { return NewHTTPError(400, msg) }
	ErrUnauthorized        = func(msg string) error { return NewHTTPError(401, msg) }
	ErrForbidden           = func(msg string) error { return NewHTTPError(403, msg) }
	ErrNotFound            = func(msg string) error { return NewHTTPError(404, msg) }
	ErrInternalServerError = func(msg string) error { return NewHTTPError(500, msg) }
)

// Response helpers for Context

// JSON sends a JSON response.
func JSON(ctx *api.Context, code int, data any) error {
	ctx.Response.Header().Set("Content-Type", "application/json")
	ctx.Response.WriteHeader(code)
	return json.NewEncoder(ctx.Response).Encode(data)
}

// String sends a string response.
func String(ctx *api.Context, code int, s string) error {
	ctx.Response.Header().Set("Content-Type", "text/plain")
	ctx.Response.WriteHeader(code)
	_, err := ctx.Response.Write([]byte(s))
	return err
}

// NoContent sends a 204 No Content response.
func NoContent(ctx *api.Context) error {
	ctx.Response.WriteHeader(http.StatusNoContent)
	return nil
}

// Redirect sends a redirect response.
func Redirect(ctx *api.Context, url string, code int) error {
	http.Redirect(ctx.Response, ctx.Request, url, code)
	return nil
}
