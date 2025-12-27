// Package core contains the implementation details for the http module.
package core

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"net/http"
	"time"

	"dev.engineeringlabs/goboot/http/api"
)

// DefaultClient is the default HTTP client implementation.
type DefaultClient struct {
	config     api.ClientConfig
	httpClient *http.Client
}

// NewClient creates a new DefaultClient.
func NewClient(config api.ClientConfig) *DefaultClient {
	transport := config.Transport
	if transport == nil {
		transport = http.DefaultTransport
	}

	return &DefaultClient{
		config: config,
		httpClient: &http.Client{
			Timeout:   config.Timeout,
			Transport: transport,
		},
	}
}

// NewDefaultClient creates a new DefaultClient with default configuration.
func NewDefaultClient() *DefaultClient {
	return NewClient(api.DefaultClientConfig())
}

// Do executes an HTTP request.
func (c *DefaultClient) Do(ctx context.Context, req *api.Request) (*api.Response, error) {
	start := time.Now()

	// Build full URL
	url := req.URL
	if c.config.BaseURL != "" && !isAbsoluteURL(url) {
		url = c.config.BaseURL + url
	}

	// Create HTTP request
	httpReq, err := http.NewRequestWithContext(ctx, req.Method, url, req.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	// Add default headers
	for k, v := range c.config.DefaultHeaders {
		httpReq.Header.Set(k, v)
	}

	// Add request headers (override defaults)
	for k, v := range req.Headers {
		httpReq.Header.Set(k, v)
	}

	// Execute with retries if configured
	var resp *http.Response
	var lastErr error
	maxAttempts := c.config.MaxRetries + 1
	if maxAttempts < 1 {
		maxAttempts = 1
	}

	for attempt := 0; attempt < maxAttempts; attempt++ {
		if attempt > 0 {
			select {
			case <-time.After(c.config.RetryDelay):
			case <-ctx.Done():
				return nil, ctx.Err()
			}
		}

		resp, lastErr = c.httpClient.Do(httpReq)
		if lastErr == nil && resp.StatusCode < 500 {
			break
		}
		if resp != nil {
			resp.Body.Close()
		}
	}

	if lastErr != nil {
		return nil, fmt.Errorf("request failed: %w", lastErr)
	}
	defer resp.Body.Close()

	// Read response body
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response body: %w", err)
	}

	// Build response headers
	headers := make(map[string]string)
	for k, v := range resp.Header {
		if len(v) > 0 {
			headers[k] = v[0]
		}
	}

	return &api.Response{
		StatusCode: resp.StatusCode,
		Headers:    headers,
		Body:       body,
		Duration:   time.Since(start),
	}, nil
}

// Get performs a GET request.
func (c *DefaultClient) Get(ctx context.Context, url string) (*api.Response, error) {
	return c.Do(ctx, &api.Request{
		Method: http.MethodGet,
		URL:    url,
	})
}

// Post performs a POST request.
func (c *DefaultClient) Post(ctx context.Context, url string, body io.Reader) (*api.Response, error) {
	return c.Do(ctx, &api.Request{
		Method: http.MethodPost,
		URL:    url,
		Body:   body,
	})
}

// Put performs a PUT request.
func (c *DefaultClient) Put(ctx context.Context, url string, body io.Reader) (*api.Response, error) {
	return c.Do(ctx, &api.Request{
		Method: http.MethodPut,
		URL:    url,
		Body:   body,
	})
}

// Delete performs a DELETE request.
func (c *DefaultClient) Delete(ctx context.Context, url string) (*api.Response, error) {
	return c.Do(ctx, &api.Request{
		Method: http.MethodDelete,
		URL:    url,
	})
}

// PostJSON performs a POST request with JSON body.
func (c *DefaultClient) PostJSON(ctx context.Context, url string, jsonBody []byte) (*api.Response, error) {
	return c.Do(ctx, &api.Request{
		Method:  http.MethodPost,
		URL:     url,
		Body:    bytes.NewReader(jsonBody),
		Headers: map[string]string{"Content-Type": "application/json"},
	})
}

// PutJSON performs a PUT request with JSON body.
func (c *DefaultClient) PutJSON(ctx context.Context, url string, jsonBody []byte) (*api.Response, error) {
	return c.Do(ctx, &api.Request{
		Method:  http.MethodPut,
		URL:     url,
		Body:    bytes.NewReader(jsonBody),
		Headers: map[string]string{"Content-Type": "application/json"},
	})
}

func isAbsoluteURL(url string) bool {
	return len(url) > 7 && (url[:7] == "http://" || url[:8] == "https://")
}

// HTTPError represents an HTTP error response.
type HTTPError struct {
	StatusCode int
	Status     string
	Body       []byte
}

func (e *HTTPError) Error() string {
	return fmt.Sprintf("HTTP %d: %s", e.StatusCode, e.Status)
}

// NewHTTPError creates a new HTTPError from a response.
func NewHTTPError(resp *api.Response) *HTTPError {
	return &HTTPError{
		StatusCode: resp.StatusCode,
		Status:     http.StatusText(resp.StatusCode),
		Body:       resp.Body,
	}
}
