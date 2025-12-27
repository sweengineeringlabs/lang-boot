// Package api contains the public interfaces and types for the di module.
package api

import (
	"reflect"
)

// Scope defines the lifecycle of a dependency.
type Scope int

const (
	// Transient creates a new instance each time.
	Transient Scope = iota
	// Singleton creates a single shared instance.
	Singleton
	// Scoped creates an instance per scope (e.g., per request).
	Scoped
)

// String returns the string representation of a Scope.
func (s Scope) String() string {
	switch s {
	case Transient:
		return "TRANSIENT"
	case Singleton:
		return "SINGLETON"
	case Scoped:
		return "SCOPED"
	default:
		return "UNKNOWN"
	}
}

// Registration holds the information for a dependency registration.
type Registration struct {
	// Type is the interface type being registered.
	Type reflect.Type
	// Factory creates instances of the type.
	Factory func(Container) (any, error)
	// Scope defines the lifecycle.
	Scope Scope
	// Name is an optional qualifier for named bindings.
	Name string
}

// Container is the interface for dependency injection containers.
type Container interface {
	// Register registers a dependency.
	Register(registration Registration) error
	// Resolve resolves a dependency by type.
	Resolve(target any) error
	// ResolveNamed resolves a named dependency.
	ResolveNamed(name string, target any) error
	// CreateScope creates a child scope.
	CreateScope() Container
}
