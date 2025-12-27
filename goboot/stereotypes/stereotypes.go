// Package stereotypes provides stereotype/annotation utilities for the goboot framework.
//
// Since Go doesn't have decorators or annotations like Python/Java, this module
// provides Go-idiomatic alternatives:
//   - Marker interfaces for type classification
//   - Decorator functions for wrapping behavior
//   - Annotation registry for runtime metadata
//   - Lifecycle hooks for component initialization
//
// Example - Marker Interfaces:
//
//	type UserService struct {
//	    stereotypes.BaseService
//	}
//
//	// UserService now has Stereotype() method returning "service"
//
// Example - Decorator Functions:
//
//	// Wrap function with retry logic
//	fetchData := stereotypes.Wrap(
//	    originalFetch,
//	    stereotypes.Retryable[Data](3, time.Second),
//	    stereotypes.Timed[Data]("fetchData", log.Printf),
//	)
//
// Example - Annotation Registry:
//
//	registry := stereotypes.NewAnnotationRegistry()
//	registry.Register("UserService", stereotypes.NewAnnotation("Transactional").
//	    With("isolation", "READ_COMMITTED"))
//
// Example - Lifecycle Hooks:
//
//	type DatabasePool struct {
//	    stereotypes.BaseComponent
//	}
//
//	func (p *DatabasePool) PostConstruct() error {
//	    return p.connect()
//	}
//
//	func (p *DatabasePool) PreDestroy() error {
//	    return p.disconnect()
//	}
package stereotypes

import (
	"dev.engineeringlabs/goboot/stereotypes/api"
	"dev.engineeringlabs/goboot/stereotypes/core"
)

// Re-export API types
type (
	// Stereotype represents a type stereotype/marker.
	Stereotype = api.Stereotype
	// Marker is an interface that marks a type with a stereotype.
	Marker = api.Marker
	// Component is a marker interface for general components.
	Component = api.Component
	// Service is a marker interface for business services.
	Service = api.Service
	// Repository is a marker interface for repositories.
	Repository = api.Repository
	// Controller is a marker interface for controllers.
	Controller = api.Controller
	// Tag represents a struct field tag.
	Tag = api.Tag
	// Annotation represents runtime annotation metadata.
	Annotation = api.Annotation
	// AnnotationRegistry stores annotations for types.
	AnnotationRegistry = api.AnnotationRegistry
	// Lifecycle represents component lifecycle hooks.
	Lifecycle = api.Lifecycle
	// PostConstruct is called after construction.
	PostConstruct = api.PostConstruct
	// PreDestroy is called before destruction.
	PreDestroy = api.PreDestroy
)

// Re-export API constants
const (
	StereotypeComponent  = api.StereotypeComponent
	StereotypeService    = api.StereotypeService
	StereotypeRepository = api.StereotypeRepository
	StereotypeController = api.StereotypeController
	StereotypeGateway    = api.StereotypeGateway
	StereotypeHandler    = api.StereotypeHandler
	StereotypeMiddleware = api.StereotypeMiddleware
	StereotypeFactory    = api.StereotypeFactory
	StereotypeSingleton  = api.StereotypeSingleton
)

// Re-export API functions
var (
	ParseTag      = api.ParseTag
	NewAnnotation = api.NewAnnotation
)

// Re-export Core types
type (
	// DefaultAnnotationRegistry is the default annotation registry.
	DefaultAnnotationRegistry = core.DefaultAnnotationRegistry
	// BaseComponent provides a base implementation for components.
	BaseComponent = core.BaseComponent
	// BaseService provides a base implementation for services.
	BaseService = core.BaseService
	// BaseRepository provides a base implementation for repositories.
	BaseRepository = core.BaseRepository
	// BaseController provides a base implementation for controllers.
	BaseController = core.BaseController
)

// Re-export Core functions
var (
	NewAnnotationRegistry = core.NewAnnotationRegistry
	NewBaseComponent      = core.NewBaseComponent
	NewBaseService        = core.NewBaseService
	NewBaseRepository     = core.NewBaseRepository
	NewBaseController     = core.NewBaseController
	GetTypeName           = core.GetTypeName
	GetStereotype         = core.GetStereotype
	InitializeLifecycle   = core.InitializeLifecycle
	DestroyLifecycle      = core.DestroyLifecycle
	Validated             = core.Validated
)

// Note: For generic decorator functions, use the core package directly:
//   core.Retryable[T](maxAttempts, delay)
//   core.Timed[T](name, logFunc)
//   core.Cached[T](cache, keyFunc)
//   core.Logged[T](name, logFunc)
//   core.Synchronized[T](mu)

// Wrap wraps a function with decorators.
func Wrap[T any](fn T, decorators ...core.Decorator[T]) T {
	return core.Wrap(fn, decorators...)
}
