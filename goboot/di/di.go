// Package di provides dependency injection for the goboot framework.
//
// This module provides:
//   - API layer: Container interface, Scope, Registration
//   - Core layer: DefaultContainer implementation
//
// Example:
//
//	import "dev.engineeringlabs/goboot/di"
//
//	container := di.NewContainer()
//
//	// Register a service
//	di.RegisterFunc(container, func(c di.Container) (*MyService, error) {
//	    return &MyService{}, nil
//	}, di.Singleton)
//
//	// Resolve the service
//	var service *MyService
//	container.Resolve(&service)
package di

import (
	"dev.engineeringlabs/goboot/di/api"
	"dev.engineeringlabs/goboot/di/core"
)

// Re-export API types
type (
	// Scope defines the lifecycle of a dependency.
	Scope = api.Scope
	// Registration holds the information for a dependency registration.
	Registration = api.Registration
	// Container is the interface for dependency injection containers.
	Container = api.Container
)

// Re-export API constants
const (
	Transient = api.Transient
	Singleton = api.Singleton
	Scoped    = api.Scoped
)

// Re-export Core types
type DefaultContainer = core.DefaultContainer

// Re-export Core functions
var (
	NewContainer     = core.NewContainer
	RegisterFunc     = core.RegisterFunc[any]
	RegisterInstance = core.RegisterInstance[any]
)

// RegisterFuncTyped is a generic helper to register a typed factory function.
func RegisterFuncTyped[T any](c *DefaultContainer, factory func(Container) (T, error), scope Scope) error {
	return core.RegisterFunc(c, factory, scope)
}

// RegisterInstanceTyped is a generic helper to register a typed instance.
func RegisterInstanceTyped[T any](c *DefaultContainer, instance T) error {
	return core.RegisterInstance(c, instance)
}
