// Package core contains the implementation details for the serialization module.
package core

import (
	"bytes"
	"encoding/json"
	"encoding/xml"
	"fmt"
	"io"

	"dev.engineeringlabs/goboot/serialization/api"
)

// JSONSerializer implements JSON serialization.
type JSONSerializer struct {
	escapeHTML bool
}

// NewJSONSerializer creates a new JSONSerializer.
func NewJSONSerializer() *JSONSerializer {
	return &JSONSerializer{escapeHTML: true}
}

// NewJSONSerializerNoEscape creates a JSONSerializer that doesn't escape HTML.
func NewJSONSerializerNoEscape() *JSONSerializer {
	return &JSONSerializer{escapeHTML: false}
}

// Format returns the serialization format.
func (s *JSONSerializer) Format() api.Format {
	return api.FormatJSON
}

// Marshal serializes a value to JSON bytes.
func (s *JSONSerializer) Marshal(v any) ([]byte, error) {
	if s.escapeHTML {
		return json.Marshal(v)
	}
	var buf bytes.Buffer
	enc := json.NewEncoder(&buf)
	enc.SetEscapeHTML(false)
	if err := enc.Encode(v); err != nil {
		return nil, err
	}
	// Remove trailing newline from Encode
	b := buf.Bytes()
	if len(b) > 0 && b[len(b)-1] == '\n' {
		b = b[:len(b)-1]
	}
	return b, nil
}

// Unmarshal deserializes JSON bytes to a value.
func (s *JSONSerializer) Unmarshal(data []byte, v any) error {
	return json.Unmarshal(data, v)
}

// MarshalIndent serializes a value with indentation.
func (s *JSONSerializer) MarshalIndent(v any, prefix, indent string) ([]byte, error) {
	return json.MarshalIndent(v, prefix, indent)
}

// Encode writes a value to a writer.
func (s *JSONSerializer) Encode(w io.Writer, v any) error {
	enc := json.NewEncoder(w)
	enc.SetEscapeHTML(s.escapeHTML)
	return enc.Encode(v)
}

// Decode reads a value from a reader.
func (s *JSONSerializer) Decode(r io.Reader, v any) error {
	return json.NewDecoder(r).Decode(v)
}

// XMLSerializer implements XML serialization.
type XMLSerializer struct{}

// NewXMLSerializer creates a new XMLSerializer.
func NewXMLSerializer() *XMLSerializer {
	return &XMLSerializer{}
}

// Format returns the serialization format.
func (s *XMLSerializer) Format() api.Format {
	return api.FormatXML
}

// Marshal serializes a value to XML bytes.
func (s *XMLSerializer) Marshal(v any) ([]byte, error) {
	return xml.Marshal(v)
}

// Unmarshal deserializes XML bytes to a value.
func (s *XMLSerializer) Unmarshal(data []byte, v any) error {
	return xml.Unmarshal(data, v)
}

// MarshalIndent serializes a value with indentation.
func (s *XMLSerializer) MarshalIndent(v any, prefix, indent string) ([]byte, error) {
	return xml.MarshalIndent(v, prefix, indent)
}

// Encode writes a value to a writer.
func (s *XMLSerializer) Encode(w io.Writer, v any) error {
	return xml.NewEncoder(w).Encode(v)
}

// Decode reads a value from a reader.
func (s *XMLSerializer) Decode(r io.Reader, v any) error {
	return xml.NewDecoder(r).Decode(v)
}

// Registry manages serializers by format.
type Registry struct {
	serializers map[api.Format]api.Serializer
}

// NewRegistry creates a new Registry with default serializers.
func NewRegistry() *Registry {
	r := &Registry{
		serializers: make(map[api.Format]api.Serializer),
	}
	r.Register(NewJSONSerializer())
	r.Register(NewXMLSerializer())
	return r
}

// Register registers a serializer.
func (r *Registry) Register(s api.Serializer) {
	r.serializers[s.Format()] = s
}

// Get returns a serializer for a format.
func (r *Registry) Get(format api.Format) (api.Serializer, error) {
	s, ok := r.serializers[format]
	if !ok {
		return nil, fmt.Errorf("no serializer registered for format: %s", format)
	}
	return s, nil
}

// Marshal serializes a value using the specified format.
func (r *Registry) Marshal(format api.Format, v any) ([]byte, error) {
	s, err := r.Get(format)
	if err != nil {
		return nil, err
	}
	return s.Marshal(v)
}

// Unmarshal deserializes data using the specified format.
func (r *Registry) Unmarshal(format api.Format, data []byte, v any) error {
	s, err := r.Get(format)
	if err != nil {
		return err
	}
	return s.Unmarshal(data, v)
}

// Default serializers
var (
	JSON = NewJSONSerializer()
	XML  = NewXMLSerializer()
)
