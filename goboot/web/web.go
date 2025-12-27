// Package web provides HTTP server utilities and middleware for the goboot framework.
//
// This module provides:
//   - API layer: Context, Handler, Middleware, Router interfaces
//   - Core layer: DefaultRouter, middleware implementations
//
// Example:
//
//	import "dev.engineeringlabs/goboot/web"
//
//	router := web.NewRouter()
//	router.Use(web.Logger(), web.Recovery())
//
//	router.GET("/users/:id", func(ctx *web.Context) error {
//	    id := ctx.Param("id")
//	    return web.JSON(ctx, 200, map[string]string{"id": id})
//	})
//
//	http.ListenAndServe(":8080", router)
package web

import (
	"dev.engineeringlabs/goboot/web/api"
	"dev.engineeringlabs/goboot/web/core"
)

// Re-export API types
type (
	// Context holds request-scoped data.
	Context = api.Context
	// Handler is the function signature for request handlers.
	Handler = api.Handler
	// Middleware is the function signature for middleware.
	Middleware = api.Middleware
	// Router is the interface for HTTP routers.
	Router = api.Router
	// CORSConfig configures CORS middleware.
	CORSConfig = api.CORSConfig
	// ContextKey is a type for context keys.
	ContextKey = api.ContextKey
)

// Re-export API constants
const (
	RequestIDKey = api.RequestIDKey
	UserKey      = api.UserKey
)

// Re-export API functions
var (
	NewContext        = api.NewContext
	DefaultCORSConfig = api.DefaultCORSConfig
	FromContext       = api.FromContext
)

// Re-export Core types
type (
	// DefaultRouter is the default router implementation.
	DefaultRouter = core.DefaultRouter
	// HTTPError represents an HTTP error.
	HTTPError = core.HTTPError
)

// Re-export Core functions
var (
	NewRouter     = core.NewRouter
	NewHTTPError  = core.NewHTTPError

	// Error constructors
	ErrBadRequest          = core.ErrBadRequest
	ErrUnauthorized        = core.ErrUnauthorized
	ErrForbidden           = core.ErrForbidden
	ErrNotFound            = core.ErrNotFound
	ErrInternalServerError = core.ErrInternalServerError

	// Response helpers
	JSON      = core.JSON
	String    = core.String
	NoContent = core.NoContent
	Redirect  = core.Redirect

	// Middleware
	Logger        = core.Logger
	Recovery      = core.Recovery
	CORS          = core.CORS
	RequestID     = core.RequestID
	Timeout       = core.Timeout
	SecureHeaders = core.SecureHeaders
	RateLimiter   = core.RateLimiter
)
