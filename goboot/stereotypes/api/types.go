// Package api contains the public interfaces and types for the stereotypes module.
// Stereotypes provide Go-idiomatic alternatives to annotations/decorators.
package api

import (
	"reflect"
)

// Stereotype represents a type stereotype/marker.
type Stereotype string

const (
	// Component marks a general component.
	StereotypeComponent Stereotype = "component"
	// Service marks a business service.
	StereotypeService Stereotype = "service"
	// Repository marks a data repository.
	StereotypeRepository Stereotype = "repository"
	// Controller marks a web controller.
	StereotypeController Stereotype = "controller"
	// Gateway marks an external gateway.
	StereotypeGateway Stereotype = "gateway"
	// Handler marks an event handler.
	StereotypeHandler Stereotype = "handler"
	// Middleware marks middleware.
	StereotypeMiddleware Stereotype = "middleware"
	// Factory marks a factory.
	StereotypeFactory Stereotype = "factory"
	// Singleton marks a singleton.
	StereotypeSingleton Stereotype = "singleton"
)

// Marker is an interface that marks a type with a stereotype.
type Marker interface {
	// Stereotype returns the stereotype of the type.
	Stereotype() Stereotype
}

// Component is a marker interface for general components.
type Component interface {
	Marker
}

// Service is a marker interface for business services.
type Service interface {
	Marker
}

// Repository is a marker interface for repositories.
type Repository interface {
	Marker
}

// Controller is a marker interface for controllers.
type Controller interface {
	Marker
}

// Tag represents a struct field tag.
type Tag struct {
	Name    string
	Value   string
	Options []string
}

// ParseTag parses a struct tag.
func ParseTag(tag string, name string) *Tag {
	t := reflect.StructTag(tag)
	value, ok := t.Lookup(name)
	if !ok {
		return nil
	}

	return &Tag{
		Name:  name,
		Value: value,
	}
}

// Annotation represents runtime annotation metadata.
type Annotation struct {
	Name       string
	Target     string // type, field, method
	Attributes map[string]any
}

// NewAnnotation creates a new Annotation.
func NewAnnotation(name string) *Annotation {
	return &Annotation{
		Name:       name,
		Attributes: make(map[string]any),
	}
}

// With adds an attribute.
func (a *Annotation) With(key string, value any) *Annotation {
	a.Attributes[key] = value
	return a
}

// AnnotationRegistry stores annotations for types.
type AnnotationRegistry interface {
	// Register registers an annotation for a type.
	Register(typeName string, annotation *Annotation)

	// Get gets annotations for a type.
	Get(typeName string) []*Annotation

	// Has checks if a type has an annotation.
	Has(typeName string, annotationName string) bool
}

// Lifecycle represents component lifecycle hooks.
type Lifecycle interface {
	// OnInit is called after construction.
	OnInit() error
	// OnDestroy is called before destruction.
	OnDestroy() error
}

// PostConstruct is called after a component is constructed.
type PostConstruct interface {
	PostConstruct() error
}

// PreDestroy is called before a component is destroyed.
type PreDestroy interface {
	PreDestroy() error
}
