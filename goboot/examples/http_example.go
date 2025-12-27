//go:build ignore

// Package main demonstrates the http module usage.
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	gohttp "dev.engineeringlabs/goboot/http"
)

func main() {
	fmt.Println("=== Goboot HTTP Module Example ===\n")
	ctx := context.Background()

	// Example 1: Basic GET request
	fmt.Println("1. Basic GET Request:")
	client := gohttp.NewDefaultClient()

	resp, err := client.Get(ctx, "https://httpbin.org/get")
	if err != nil {
		fmt.Printf("   Error: %v\n", err)
	} else {
		fmt.Printf("   Status: %d\n", resp.StatusCode)
		fmt.Printf("   Duration: %v\n", resp.Duration)
		fmt.Printf("   Success: %v\n", resp.IsSuccess())
	}

	// Example 2: Configured client
	fmt.Println("\n2. Configured Client:")
	config := gohttp.DefaultClientConfig()
	config.BaseURL = "https://httpbin.org"
	config.Timeout = 10 * time.Second
	config.DefaultHeaders = map[string]string{
		"User-Agent": "goboot/1.0",
	}

	configuredClient := gohttp.NewClient(config)

	resp, err = configuredClient.Get(ctx, "/headers")
	if err != nil {
		fmt.Printf("   Error: %v\n", err)
	} else {
		fmt.Printf("   Status: %d\n", resp.StatusCode)

		var data map[string]any
		json.Unmarshal(resp.Body, &data)
		fmt.Printf("   Response: %v\n", data["headers"])
	}

	// Example 3: POST with JSON
	fmt.Println("\n3. POST with JSON:")
	jsonBody := []byte(`{"name": "John", "email": "john@example.com"}`)
	resp, err = configuredClient.PostJSON(ctx, "/post", jsonBody)
	if err != nil {
		fmt.Printf("   Error: %v\n", err)
	} else {
		fmt.Printf("   Status: %d\n", resp.StatusCode)
	}

	// Example 4: Client with retries
	fmt.Println("\n4. Client with Retries:")
	retryConfig := gohttp.DefaultClientConfig()
	retryConfig.MaxRetries = 3
	retryConfig.RetryDelay = 500 * time.Millisecond

	retryClient := gohttp.NewClient(retryConfig)
	resp, _ = retryClient.Get(ctx, "https://httpbin.org/status/200")
	fmt.Printf("   Status: %d (with retry support)\n", resp.StatusCode)

	// Example 5: Response analysis
	fmt.Println("\n5. Response Analysis:")
	resp, _ = client.Get(ctx, "https://httpbin.org/status/404")
	fmt.Printf("   Is Success: %v\n", resp.IsSuccess())
	fmt.Printf("   Is Client Error: %v\n", resp.IsClientError())
	fmt.Printf("   Is Server Error: %v\n", resp.IsServerError())
}
