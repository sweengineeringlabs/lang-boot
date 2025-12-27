// Package http provides HTTP client utilities for the goboot framework.
//
// This module provides:
//   - API layer: Client interface, Request, Response types
//   - Core layer: DefaultClient implementation
//
// Example:
//
//	import "dev.engineeringlabs/goboot/http"
//
//	client := http.NewDefaultClient()
//	resp, err := client.Get(ctx, "https://api.example.com/users")
//	if err != nil {
//	    log.Fatal(err)
//	}
//	fmt.Println(string(resp.Body))
//
//	// With configuration
//	config := http.DefaultClientConfig()
//	config.BaseURL = "https://api.example.com"
//	config.Timeout = 10 * time.Second
//	client = http.NewClient(config)
package http

import (
	"dev.engineeringlabs/goboot/http/api"
	"dev.engineeringlabs/goboot/http/core"
)

// Re-export API types
type (
	// Request represents an HTTP request.
	Request = api.Request
	// Response represents an HTTP response.
	Response = api.Response
	// Client is the interface for HTTP clients.
	Client = api.Client
	// ClientConfig configures the HTTP client.
	ClientConfig = api.ClientConfig
	// RequestOption is a functional option for configuring requests.
	RequestOption = api.RequestOption
)

// Re-export API functions
var (
	DefaultClientConfig = api.DefaultClientConfig
	WithHeader          = api.WithHeader
	WithTimeout         = api.WithTimeout
	WithContentType     = api.WithContentType
	WithJSON            = api.WithJSON
	WithBearer          = api.WithBearer
)

// Re-export Core types
type (
	// DefaultClient is the default HTTP client implementation.
	DefaultClient = core.DefaultClient
	// HTTPError represents an HTTP error response.
	HTTPError = core.HTTPError
)

// Re-export Core functions
var (
	NewClient        = core.NewClient
	NewDefaultClient = core.NewDefaultClient
	NewHTTPError     = core.NewHTTPError
)
