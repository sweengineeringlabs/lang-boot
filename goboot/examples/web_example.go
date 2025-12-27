//go:build ignore

// Package main demonstrates the web module usage.
package main

import (
	"fmt"
	"net/http"

	"dev.engineeringlabs/goboot/web"
)

func main() {
	fmt.Println("=== Goboot Web Module Example ===\n")

	// Create router
	router := web.NewRouter()

	// Add middleware
	router.Use(web.Logger())
	router.Use(web.Recovery())
	router.Use(web.SecureHeaders())
	router.Use(web.RequestID())

	// Add CORS
	corsConfig := web.DefaultCORSConfig()
	corsConfig.AllowOrigins = []string{"http://localhost:3000"}
	router.Use(web.CORS(corsConfig))

	// Define routes
	router.GET("/", func(ctx *web.Context) error {
		return web.JSON(ctx, 200, map[string]string{
			"message": "Welcome to Goboot!",
		})
	})

	router.GET("/users/:id", func(ctx *web.Context) error {
		id := ctx.Param("id")
		return web.JSON(ctx, 200, map[string]string{
			"id":   id,
			"name": "John Doe",
		})
	})

	router.POST("/users", func(ctx *web.Context) error {
		return web.JSON(ctx, 201, map[string]string{
			"message": "User created",
		})
	})

	// Route groups
	api := router.Group("/api/v1")

	api.Handle(http.MethodGet, "/health", func(ctx *web.Context) error {
		return web.JSON(ctx, 200, map[string]string{
			"status": "healthy",
		})
	})

	api.Handle(http.MethodGet, "/products", func(ctx *web.Context) error {
		page := ctx.QueryDefault("page", "1")
		limit := ctx.QueryDefault("limit", "10")
		return web.JSON(ctx, 200, map[string]string{
			"page":  page,
			"limit": limit,
		})
	})

	// Error handling
	router.GET("/error", func(ctx *web.Context) error {
		return web.ErrNotFound("Resource not found")
	})

	router.GET("/panic", func(ctx *web.Context) error {
		panic("Something went wrong!")
	})

	fmt.Println("Starting server on :8080...")
	fmt.Println("Routes:")
	fmt.Println("  GET  /")
	fmt.Println("  GET  /users/:id")
	fmt.Println("  POST /users")
	fmt.Println("  GET  /api/v1/health")
	fmt.Println("  GET  /api/v1/products?page=1&limit=10")
	fmt.Println("  GET  /error")
	fmt.Println("  GET  /panic")

	http.ListenAndServe(":8080", router)
}
