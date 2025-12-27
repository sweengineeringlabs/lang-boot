// Package openapi provides OpenAPI/Swagger documentation generation utilities.
//
// This module enables automatic API documentation from Go handlers.
//
// Example:
//
//	spec := openapi.NewSpec("My API", "1.0.0")
//	spec.AddPath("/users", openapi.Path{
//	    Get: &openapi.Operation{
//	        Summary: "List users",
//	        Responses: openapi.Responses{
//	            "200": {Description: "Success", Schema: openapi.ArrayOf(User{})},
//	        },
//	    },
//	})
//
//	// Generate OpenAPI JSON
//	json := spec.ToJSON()
package openapi

import "dev.engineeringlabs/goboot/openapi/api"

// Re-export API types
type (
	// Spec represents an OpenAPI specification.
	Spec = api.Spec
	// Info contains API metadata.
	Info = api.Info
	// Path represents an API path.
	Path = api.Path
	// Operation represents an HTTP operation.
	Operation = api.Operation
	// Parameter represents an operation parameter.
	Parameter = api.Parameter
	// Response represents an operation response.
	Response = api.Response
	// Schema represents a JSON schema.
	Schema = api.Schema
	// Tag represents an API tag.
	Tag = api.Tag
	// Contact represents contact information.
	Contact = api.Contact
	// License represents license information.
	License = api.License
	// Server represents a server URL.
	Server = api.Server
	// SecurityScheme represents a security scheme.
	SecurityScheme = api.SecurityScheme
)

// Responses is a map of status codes to responses.
type Responses = map[string]Response

// NewSpec creates a new OpenAPI specification.
func NewSpec(title, version string) *Spec {
	return &Spec{
		OpenAPI: "3.0.3",
		Info: Info{
			Title:   title,
			Version: version,
		},
		Paths: make(map[string]Path),
	}
}

// ArrayOf creates an array schema for the given type.
func ArrayOf(item interface{}) Schema {
	return Schema{
		Type:  "array",
		Items: SchemaFrom(item),
	}
}

// SchemaFrom creates a schema from a Go struct.
func SchemaFrom(v interface{}) *Schema {
	// Implementation would use reflection to build schema
	return &Schema{Type: "object"}
}
