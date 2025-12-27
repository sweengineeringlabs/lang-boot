// Package core contains the implementation details for the stereotypes module.
package core

import (
	"context"
	"fmt"
	"reflect"
	"sync"
	"time"

	"dev.engineeringlabs/goboot/stereotypes/api"
)

// DefaultAnnotationRegistry is the default annotation registry.
type DefaultAnnotationRegistry struct {
	annotations map[string][]*api.Annotation
	mu          sync.RWMutex
}

// NewAnnotationRegistry creates a new DefaultAnnotationRegistry.
func NewAnnotationRegistry() *DefaultAnnotationRegistry {
	return &DefaultAnnotationRegistry{
		annotations: make(map[string][]*api.Annotation),
	}
}

// Register registers an annotation.
func (r *DefaultAnnotationRegistry) Register(typeName string, annotation *api.Annotation) {
	r.mu.Lock()
	r.annotations[typeName] = append(r.annotations[typeName], annotation)
	r.mu.Unlock()
}

// Get gets annotations for a type.
func (r *DefaultAnnotationRegistry) Get(typeName string) []*api.Annotation {
	r.mu.RLock()
	defer r.mu.RUnlock()
	return r.annotations[typeName]
}

// Has checks if a type has an annotation.
func (r *DefaultAnnotationRegistry) Has(typeName string, annotationName string) bool {
	r.mu.RLock()
	defer r.mu.RUnlock()

	for _, a := range r.annotations[typeName] {
		if a.Name == annotationName {
			return true
		}
	}
	return false
}

// Decorator wraps a function with additional behavior.
type Decorator[T any] func(T) T

// Wrap wraps a function with decorators (applied in reverse order).
func Wrap[T any](fn T, decorators ...Decorator[T]) T {
	for i := len(decorators) - 1; i >= 0; i-- {
		fn = decorators[i](fn)
	}
	return fn
}

// Retryable wraps a function with retry logic.
func Retryable[T any](maxAttempts int, delay time.Duration) Decorator[func(context.Context) (T, error)] {
	return func(fn func(context.Context) (T, error)) func(context.Context) (T, error) {
		return func(ctx context.Context) (T, error) {
			var lastErr error
			var zero T

			for attempt := 0; attempt < maxAttempts; attempt++ {
				result, err := fn(ctx)
				if err == nil {
					return result, nil
				}
				lastErr = err

				if attempt < maxAttempts-1 {
					select {
					case <-ctx.Done():
						return zero, ctx.Err()
					case <-time.After(delay):
					}
				}
			}

			return zero, lastErr
		}
	}
}

// Timed wraps a function to log its execution time.
func Timed[T any](name string, logFn func(string, time.Duration)) Decorator[func(context.Context) (T, error)] {
	return func(fn func(context.Context) (T, error)) func(context.Context) (T, error) {
		return func(ctx context.Context) (T, error) {
			start := time.Now()
			result, err := fn(ctx)
			logFn(name, time.Since(start))
			return result, err
		}
	}
}

// Cached wraps a function with caching.
func Cached[K comparable, V any](cache map[K]V, mu *sync.RWMutex) func(func(K) V) func(K) V {
	return func(fn func(K) V) func(K) V {
		return func(key K) V {
			mu.RLock()
			if v, ok := cache[key]; ok {
				mu.RUnlock()
				return v
			}
			mu.RUnlock()

			result := fn(key)

			mu.Lock()
			cache[key] = result
			mu.Unlock()

			return result
		}
	}
}

// Validated wraps a function to validate inputs.
func Validated(validator func(any) error) Decorator[func(any) error] {
	return func(fn func(any) error) func(any) error {
		return func(input any) error {
			if err := validator(input); err != nil {
				return fmt.Errorf("validation failed: %w", err)
			}
			return fn(input)
		}
	}
}

// Logged wraps a function with logging.
func Logged[T any](logFn func(string)) Decorator[func() T] {
	return func(fn func() T) func() T {
		return func() T {
			logFn("entering function")
			result := fn()
			logFn("exiting function")
			return result
		}
	}
}

// Synchronized wraps a function with mutex protection.
func Synchronized[T any](mu *sync.Mutex) Decorator[func() T] {
	return func(fn func() T) func() T {
		return func() T {
			mu.Lock()
			defer mu.Unlock()
			return fn()
		}
	}
}

// BaseComponent provides a base implementation for components.
type BaseComponent struct {
	stereotype api.Stereotype
}

// NewBaseComponent creates a new BaseComponent.
func NewBaseComponent(stereotype api.Stereotype) *BaseComponent {
	return &BaseComponent{stereotype: stereotype}
}

// Stereotype returns the stereotype.
func (c *BaseComponent) Stereotype() api.Stereotype {
	return c.stereotype
}

// BaseService provides a base implementation for services.
type BaseService struct {
	BaseComponent
}

// NewBaseService creates a new BaseService.
func NewBaseService() *BaseService {
	return &BaseService{
		BaseComponent: BaseComponent{stereotype: api.StereotypeService},
	}
}

// BaseRepository provides a base implementation for repositories.
type BaseRepository struct {
	BaseComponent
}

// NewBaseRepository creates a new BaseRepository.
func NewBaseRepository() *BaseRepository {
	return &BaseRepository{
		BaseComponent: BaseComponent{stereotype: api.StereotypeRepository},
	}
}

// BaseController provides a base implementation for controllers.
type BaseController struct {
	BaseComponent
}

// NewBaseController creates a new BaseController.
func NewBaseController() *BaseController {
	return &BaseController{
		BaseComponent: BaseComponent{stereotype: api.StereotypeController},
	}
}

// GetTypeName returns the type name of a value.
func GetTypeName(v any) string {
	t := reflect.TypeOf(v)
	if t.Kind() == reflect.Ptr {
		t = t.Elem()
	}
	return t.PkgPath() + "." + t.Name()
}

// HasMarker checks if a type implements a marker interface.
func HasMarker[T any](v any) bool {
	_, ok := v.(T)
	return ok
}

// GetStereotype gets the stereotype of a value if it implements Marker.
func GetStereotype(v any) (api.Stereotype, bool) {
	if marker, ok := v.(api.Marker); ok {
		return marker.Stereotype(), true
	}
	return "", false
}

// InitializeLifecycle calls OnInit if implemented.
func InitializeLifecycle(v any) error {
	if lc, ok := v.(api.Lifecycle); ok {
		return lc.OnInit()
	}
	if pc, ok := v.(api.PostConstruct); ok {
		return pc.PostConstruct()
	}
	return nil
}

// DestroyLifecycle calls OnDestroy if implemented.
func DestroyLifecycle(v any) error {
	if lc, ok := v.(api.Lifecycle); ok {
		return lc.OnDestroy()
	}
	if pd, ok := v.(api.PreDestroy); ok {
		return pd.PreDestroy()
	}
	return nil
}
