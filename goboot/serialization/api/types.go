// Package api contains the public interfaces and types for the serialization module.
package api

import (
	"io"
)

// Format represents a serialization format.
type Format string

const (
	// JSON format.
	FormatJSON Format = "json"
	// YAML format.
	FormatYAML Format = "yaml"
	// TOML format.
	FormatTOML Format = "toml"
	// XML format.
	FormatXML Format = "xml"
	// MessagePack format.
	FormatMsgPack Format = "msgpack"
)

// Serializer is the interface for serializers.
type Serializer interface {
	// Format returns the serialization format.
	Format() Format

	// Marshal serializes a value to bytes.
	Marshal(v any) ([]byte, error)

	// Unmarshal deserializes bytes to a value.
	Unmarshal(data []byte, v any) error

	// MarshalIndent serializes a value with indentation.
	MarshalIndent(v any, prefix, indent string) ([]byte, error)
}

// StreamSerializer extends Serializer with streaming support.
type StreamSerializer interface {
	Serializer

	// Encode writes a value to a writer.
	Encode(w io.Writer, v any) error

	// Decode reads a value from a reader.
	Decode(r io.Reader, v any) error
}

// ContentType returns the MIME content type for a format.
func ContentType(format Format) string {
	switch format {
	case FormatJSON:
		return "application/json"
	case FormatYAML:
		return "application/yaml"
	case FormatTOML:
		return "application/toml"
	case FormatXML:
		return "application/xml"
	case FormatMsgPack:
		return "application/msgpack"
	default:
		return "application/octet-stream"
	}
}

// FileExtension returns the file extension for a format.
func FileExtension(format Format) string {
	switch format {
	case FormatJSON:
		return ".json"
	case FormatYAML:
		return ".yaml"
	case FormatTOML:
		return ".toml"
	case FormatXML:
		return ".xml"
	case FormatMsgPack:
		return ".msgpack"
	default:
		return ""
	}
}

// FormatFromExtension returns the format for a file extension.
func FormatFromExtension(ext string) Format {
	switch ext {
	case ".json":
		return FormatJSON
	case ".yaml", ".yml":
		return FormatYAML
	case ".toml":
		return FormatTOML
	case ".xml":
		return FormatXML
	case ".msgpack":
		return FormatMsgPack
	default:
		return ""
	}
}
