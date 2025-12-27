package api

import (
	"testing"
)

func TestResponse_IsSuccess(t *testing.T) {
	tests := []struct {
		code     int
		expected bool
	}{
		{200, true},
		{201, true},
		{204, true},
		{299, true},
		{199, false},
		{300, false},
		{400, false},
		{500, false},
	}

	for _, tt := range tests {
		resp := &Response{StatusCode: tt.code}
		if resp.IsSuccess() != tt.expected {
			t.Errorf("Status %d: expected IsSuccess=%v", tt.code, tt.expected)
		}
	}
}

func TestResponse_IsClientError(t *testing.T) {
	tests := []struct {
		code     int
		expected bool
	}{
		{400, true},
		{401, true},
		{403, true},
		{404, true},
		{499, true},
		{399, false},
		{500, false},
		{200, false},
	}

	for _, tt := range tests {
		resp := &Response{StatusCode: tt.code}
		if resp.IsClientError() != tt.expected {
			t.Errorf("Status %d: expected IsClientError=%v", tt.code, tt.expected)
		}
	}
}

func TestResponse_IsServerError(t *testing.T) {
	tests := []struct {
		code     int
		expected bool
	}{
		{500, true},
		{502, true},
		{503, true},
		{599, true},
		{499, false},
		{600, false},
		{200, false},
	}

	for _, tt := range tests {
		resp := &Response{StatusCode: tt.code}
		if resp.IsServerError() != tt.expected {
			t.Errorf("Status %d: expected IsServerError=%v", tt.code, tt.expected)
		}
	}
}

func TestDefaultClientConfig(t *testing.T) {
	config := DefaultClientConfig()

	if config.Timeout == 0 {
		t.Error("Timeout should have default value")
	}
	if config.DefaultHeaders == nil {
		t.Error("DefaultHeaders should not be nil")
	}
}

func TestRequestOptions(t *testing.T) {
	req := &Request{}

	WithHeader("X-Custom", "value")(req)
	if req.Headers["X-Custom"] != "value" {
		t.Error("WithHeader should set header")
	}

	WithJSON()(req)
	if req.Headers["Content-Type"] != "application/json" {
		t.Error("WithJSON should set Content-Type")
	}

	WithBearer("token123")(req)
	if req.Headers["Authorization"] != "Bearer token123" {
		t.Error("WithBearer should set Authorization header")
	}
}
