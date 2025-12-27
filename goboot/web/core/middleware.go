// Package core contains middleware implementations for the web module.
package core

import (
	"log"
	"runtime/debug"
	"strings"
	"sync"
	"time"

	"dev.engineeringlabs/goboot/web/api"
)

// Logger returns a logging middleware.
func Logger() api.Middleware {
	return func(next api.Handler) api.Handler {
		return func(ctx *api.Context) error {
			start := time.Now()
			err := next(ctx)
			duration := time.Since(start)

			log.Printf("%s %s %v", ctx.Request.Method, ctx.Request.URL.Path, duration)
			return err
		}
	}
}

// Recovery returns a panic recovery middleware.
func Recovery() api.Middleware {
	return func(next api.Handler) api.Handler {
		return func(ctx *api.Context) (err error) {
			defer func() {
				if r := recover(); r != nil {
					log.Printf("panic recovered: %v\n%s", r, debug.Stack())
					err = NewHTTPError(500, "Internal Server Error")
				}
			}()
			return next(ctx)
		}
	}
}

// CORS returns a CORS middleware.
func CORS(config api.CORSConfig) api.Middleware {
	return func(next api.Handler) api.Handler {
		return func(ctx *api.Context) error {
			origin := ctx.Request.Header.Get("Origin")

			// Check if origin is allowed
			allowed := false
			for _, o := range config.AllowOrigins {
				if o == "*" || o == origin {
					allowed = true
					break
				}
			}

			if allowed {
				ctx.Response.Header().Set("Access-Control-Allow-Origin", origin)
			}

			if config.AllowCredentials {
				ctx.Response.Header().Set("Access-Control-Allow-Credentials", "true")
			}

			if len(config.ExposeHeaders) > 0 {
				ctx.Response.Header().Set("Access-Control-Expose-Headers", strings.Join(config.ExposeHeaders, ", "))
			}

			// Handle preflight requests
			if ctx.Request.Method == "OPTIONS" {
				ctx.Response.Header().Set("Access-Control-Allow-Methods", strings.Join(config.AllowMethods, ", "))
				ctx.Response.Header().Set("Access-Control-Allow-Headers", strings.Join(config.AllowHeaders, ", "))
				if config.MaxAge > 0 {
					ctx.Response.Header().Set("Access-Control-Max-Age", string(rune(config.MaxAge)))
				}
				ctx.Response.WriteHeader(204)
				return nil
			}

			return next(ctx)
		}
	}
}

// RequestID returns a request ID middleware.
func RequestID() api.Middleware {
	return func(next api.Handler) api.Handler {
		return func(ctx *api.Context) error {
			requestID := ctx.Request.Header.Get("X-Request-ID")
			if requestID == "" {
				requestID = generateRequestID()
			}
			ctx.Set(string(api.RequestIDKey), requestID)
			ctx.Response.Header().Set("X-Request-ID", requestID)
			return next(ctx)
		}
	}
}

func generateRequestID() string {
	return time.Now().Format("20060102150405.000000")
}

// Timeout returns a timeout middleware.
func Timeout(d time.Duration) api.Middleware {
	return func(next api.Handler) api.Handler {
		return func(ctx *api.Context) error {
			done := make(chan error, 1)
			go func() {
				done <- next(ctx)
			}()

			select {
			case err := <-done:
				return err
			case <-time.After(d):
				return NewHTTPError(504, "Gateway Timeout")
			}
		}
	}
}

// SecureHeaders returns a middleware that adds security headers.
func SecureHeaders() api.Middleware {
	return func(next api.Handler) api.Handler {
		return func(ctx *api.Context) error {
			ctx.Response.Header().Set("X-Content-Type-Options", "nosniff")
			ctx.Response.Header().Set("X-Frame-Options", "DENY")
			ctx.Response.Header().Set("X-XSS-Protection", "1; mode=block")
			ctx.Response.Header().Set("Referrer-Policy", "strict-origin-when-cross-origin")
			return next(ctx)
		}
	}
}

// RateLimiter returns a simple in-memory rate limiting middleware.
func RateLimiter(requestsPerSecond int) api.Middleware {
	var tokens = requestsPerSecond
	var lastRefresh = time.Now()
	var mu sync.Mutex

	return func(next api.Handler) api.Handler {
		return func(ctx *api.Context) error {
			mu.Lock()
			now := time.Now()
			elapsed := now.Sub(lastRefresh)
			refill := int(elapsed.Seconds()) * requestsPerSecond
			tokens = minInt(tokens+refill, requestsPerSecond)
			lastRefresh = now

			if tokens <= 0 {
				mu.Unlock()
				return NewHTTPError(429, "Too Many Requests")
			}
			tokens--
			mu.Unlock()

			return next(ctx)
		}
	}
}

func minInt(a, b int) int {
	if a < b {
		return a
	}
	return b
}
