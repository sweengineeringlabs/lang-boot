// Package api contains the public interfaces and types for the http module.
package api

import (
	"context"
	"io"
	"net/http"
	"time"
)

// Request represents an HTTP request.
type Request struct {
	Method  string
	URL     string
	Headers map[string]string
	Body    io.Reader
	Timeout time.Duration
}

// Response represents an HTTP response.
type Response struct {
	StatusCode int
	Headers    map[string]string
	Body       []byte
	Duration   time.Duration
}

// IsSuccess returns true if the status code is 2xx.
func (r *Response) IsSuccess() bool {
	return r.StatusCode >= 200 && r.StatusCode < 300
}

// IsClientError returns true if the status code is 4xx.
func (r *Response) IsClientError() bool {
	return r.StatusCode >= 400 && r.StatusCode < 500
}

// IsServerError returns true if the status code is 5xx.
func (r *Response) IsServerError() bool {
	return r.StatusCode >= 500 && r.StatusCode < 600
}

// Client is the interface for HTTP clients.
type Client interface {
	// Do executes an HTTP request.
	Do(ctx context.Context, req *Request) (*Response, error)

	// Get performs a GET request.
	Get(ctx context.Context, url string) (*Response, error)

	// Post performs a POST request.
	Post(ctx context.Context, url string, body io.Reader) (*Response, error)

	// Put performs a PUT request.
	Put(ctx context.Context, url string, body io.Reader) (*Response, error)

	// Delete performs a DELETE request.
	Delete(ctx context.Context, url string) (*Response, error)
}

// ClientConfig configures the HTTP client.
type ClientConfig struct {
	// Timeout is the default request timeout.
	Timeout time.Duration
	// MaxRetries is the maximum number of retries.
	MaxRetries int
	// RetryDelay is the delay between retries.
	RetryDelay time.Duration
	// BaseURL is the base URL for all requests.
	BaseURL string
	// DefaultHeaders are headers added to all requests.
	DefaultHeaders map[string]string
	// Transport is the underlying HTTP transport.
	Transport http.RoundTripper
}

// DefaultClientConfig returns a default client configuration.
func DefaultClientConfig() ClientConfig {
	return ClientConfig{
		Timeout:        30 * time.Second,
		MaxRetries:     0,
		RetryDelay:     time.Second,
		DefaultHeaders: make(map[string]string),
	}
}

// RequestOption is a functional option for configuring requests.
type RequestOption func(*Request)

// WithHeader adds a header to the request.
func WithHeader(key, value string) RequestOption {
	return func(r *Request) {
		if r.Headers == nil {
			r.Headers = make(map[string]string)
		}
		r.Headers[key] = value
	}
}

// WithTimeout sets the request timeout.
func WithTimeout(d time.Duration) RequestOption {
	return func(r *Request) {
		r.Timeout = d
	}
}

// WithContentType sets the Content-Type header.
func WithContentType(contentType string) RequestOption {
	return WithHeader("Content-Type", contentType)
}

// WithJSON sets the Content-Type to application/json.
func WithJSON() RequestOption {
	return WithContentType("application/json")
}

// WithBearer sets the Authorization header with a Bearer token.
func WithBearer(token string) RequestOption {
	return WithHeader("Authorization", "Bearer "+token)
}
