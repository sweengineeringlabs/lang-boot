// Package api contains the public interfaces and types for the web module.
package api

import (
	"context"
	"net/http"
)

// Context holds request-scoped data.
type Context struct {
	Request  *http.Request
	Response http.ResponseWriter
	Params   map[string]string
	Values   map[string]any
}

// NewContext creates a new Context.
func NewContext(w http.ResponseWriter, r *http.Request) *Context {
	return &Context{
		Request:  r,
		Response: w,
		Params:   make(map[string]string),
		Values:   make(map[string]any),
	}
}

// Get retrieves a value from the context.
func (c *Context) Get(key string) (any, bool) {
	v, ok := c.Values[key]
	return v, ok
}

// Set stores a value in the context.
func (c *Context) Set(key string, value any) {
	c.Values[key] = value
}

// Param retrieves a URL parameter.
func (c *Context) Param(name string) string {
	return c.Params[name]
}

// Query retrieves a query parameter.
func (c *Context) Query(name string) string {
	return c.Request.URL.Query().Get(name)
}

// QueryDefault retrieves a query parameter with a default value.
func (c *Context) QueryDefault(name, defaultVal string) string {
	val := c.Query(name)
	if val == "" {
		return defaultVal
	}
	return val
}

// Header retrieves a request header.
func (c *Context) Header(name string) string {
	return c.Request.Header.Get(name)
}

// SetHeader sets a response header.
func (c *Context) SetHeader(name, value string) {
	c.Response.Header().Set(name, value)
}

// Handler is the function signature for request handlers.
type Handler func(*Context) error

// Middleware is the function signature for middleware.
type Middleware func(Handler) Handler

// Router is the interface for HTTP routers.
type Router interface {
	// Handle registers a handler for a pattern.
	Handle(method, pattern string, handler Handler)

	// Use adds middleware to the router.
	Use(middleware ...Middleware)

	// Group creates a new route group with a prefix.
	Group(prefix string) Router

	// ServeHTTP implements http.Handler.
	ServeHTTP(w http.ResponseWriter, r *http.Request)
}

// CORSConfig configures CORS middleware.
type CORSConfig struct {
	AllowOrigins     []string
	AllowMethods     []string
	AllowHeaders     []string
	AllowCredentials bool
	ExposeHeaders    []string
	MaxAge           int
}

// DefaultCORSConfig returns a default CORS configuration.
func DefaultCORSConfig() CORSConfig {
	return CORSConfig{
		AllowOrigins: []string{"*"},
		AllowMethods: []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowHeaders: []string{"Origin", "Content-Type", "Accept", "Authorization"},
		MaxAge:       86400,
	}
}

// ContextKey is a type for context keys.
type ContextKey string

const (
	// RequestIDKey is the context key for request ID.
	RequestIDKey ContextKey = "request_id"
	// UserKey is the context key for authenticated user.
	UserKey ContextKey = "user"
)

// FromContext retrieves the web context from a standard context.
func FromContext(ctx context.Context) (*Context, bool) {
	c, ok := ctx.Value(ContextKey("web_context")).(*Context)
	return c, ok
}
