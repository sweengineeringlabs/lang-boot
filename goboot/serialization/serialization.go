// Package serialization provides serialization utilities for the goboot framework.
//
// This module provides:
//   - API layer: Format, Serializer interface
//   - Core layer: JSONSerializer, XMLSerializer, Registry
//
// Example:
//
//	import "dev.engineeringlabs/goboot/serialization"
//
//	type User struct {
//	    Name  string `json:"name"`
//	    Email string `json:"email"`
//	}
//
//	// Using JSON serializer
//	data, _ := serialization.JSON.Marshal(User{Name: "John", Email: "john@example.com"})
//
//	// Using registry
//	registry := serialization.NewRegistry()
//	data, _ = registry.Marshal(serialization.FormatJSON, user)
package serialization

import (
	"dev.engineeringlabs/goboot/serialization/api"
	"dev.engineeringlabs/goboot/serialization/core"
)

// Re-export API types
type (
	// Format represents a serialization format.
	Format = api.Format
	// Serializer is the interface for serializers.
	Serializer = api.Serializer
	// StreamSerializer extends Serializer with streaming support.
	StreamSerializer = api.StreamSerializer
)

// Re-export API constants
const (
	FormatJSON    = api.FormatJSON
	FormatYAML    = api.FormatYAML
	FormatTOML    = api.FormatTOML
	FormatXML     = api.FormatXML
	FormatMsgPack = api.FormatMsgPack
)

// Re-export API functions
var (
	ContentType         = api.ContentType
	FileExtension       = api.FileExtension
	FormatFromExtension = api.FormatFromExtension
)

// Re-export Core types
type (
	// JSONSerializer implements JSON serialization.
	JSONSerializer = core.JSONSerializer
	// XMLSerializer implements XML serialization.
	XMLSerializer = core.XMLSerializer
	// Registry manages serializers by format.
	Registry = core.Registry
)

// Re-export Core functions
var (
	NewJSONSerializer         = core.NewJSONSerializer
	NewJSONSerializerNoEscape = core.NewJSONSerializerNoEscape
	NewXMLSerializer          = core.NewXMLSerializer
	NewRegistry               = core.NewRegistry
)

// Default serializers
var (
	JSON = core.JSON
	XML  = core.XML
)
