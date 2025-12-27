// Package core contains the implementation details for the di module.
package core

import (
	"fmt"
	"reflect"
	"sync"

	"dev.engineeringlabs/goboot/di/api"
)

// registrationKey is a composite key for registrations
type registrationKey struct {
	Type reflect.Type
	Name string
}

// DefaultContainer is the default implementation of Container.
type DefaultContainer struct {
	registrations map[reflect.Type]*api.Registration
	namedRegs     map[string]*api.Registration
	singletons    map[registrationKey]any
	parent        *DefaultContainer
	mu            sync.RWMutex
}

// NewContainer creates a new DefaultContainer.
func NewContainer() *DefaultContainer {
	return &DefaultContainer{
		registrations: make(map[reflect.Type]*api.Registration),
		namedRegs:     make(map[string]*api.Registration),
		singletons:    make(map[registrationKey]any),
	}
}

// Register registers a dependency.
func (c *DefaultContainer) Register(registration api.Registration) error {
	c.mu.Lock()
	defer c.mu.Unlock()

	if registration.Name != "" {
		c.namedRegs[registration.Name] = &registration
	} else {
		c.registrations[registration.Type] = &registration
	}
	return nil
}

// Resolve resolves a dependency by type.
func (c *DefaultContainer) Resolve(target any) error {
	targetVal := reflect.ValueOf(target)
	if targetVal.Kind() != reflect.Ptr {
		return fmt.Errorf("target must be a pointer")
	}

	targetType := targetVal.Elem().Type()
	instance, err := c.resolve(targetType)
	if err != nil {
		return err
	}

	targetVal.Elem().Set(reflect.ValueOf(instance))
	return nil
}

// ResolveNamed resolves a named dependency.
func (c *DefaultContainer) ResolveNamed(name string, target any) error {
	targetVal := reflect.ValueOf(target)
	if targetVal.Kind() != reflect.Ptr {
		return fmt.Errorf("target must be a pointer")
	}

	c.mu.RLock()
	reg, ok := c.namedRegs[name]
	c.mu.RUnlock()

	if !ok {
		if c.parent != nil {
			return c.parent.ResolveNamed(name, target)
		}
		return fmt.Errorf("no registration found for name: %s", name)
	}

	instance, err := c.createInstance(reg)
	if err != nil {
		return err
	}

	targetVal.Elem().Set(reflect.ValueOf(instance))
	return nil
}

// CreateScope creates a child scope.
func (c *DefaultContainer) CreateScope() api.Container {
	return &DefaultContainer{
		registrations: make(map[reflect.Type]*api.Registration),
		namedRegs:     make(map[string]*api.Registration),
		singletons:    make(map[registrationKey]any),
		parent:        c,
	}
}

func (c *DefaultContainer) resolve(targetType reflect.Type) (any, error) {
	// First, find the registration (may be in parent)
	reg := c.findRegistration(targetType)
	if reg == nil {
		return nil, fmt.Errorf("no registration found for type: %v", targetType)
	}

	return c.createInstance(reg)
}

func (c *DefaultContainer) findRegistration(targetType reflect.Type) *api.Registration {
	c.mu.RLock()
	reg, ok := c.registrations[targetType]
	c.mu.RUnlock()

	if ok {
		return reg
	}

	if c.parent != nil {
		return c.parent.findRegistration(targetType)
	}

	return nil
}

func (c *DefaultContainer) createInstance(reg *api.Registration) (any, error) {
	// Create a composite key for caching
	key := registrationKey{Type: reg.Type, Name: reg.Name}

	switch reg.Scope {
	case api.Singleton:
		// For singletons, check the root container if we have a parent
		root := c.getRoot()
		
		root.mu.RLock()
		if instance, ok := root.singletons[key]; ok {
			root.mu.RUnlock()
			return instance, nil
		}
		root.mu.RUnlock()

		root.mu.Lock()
		defer root.mu.Unlock()

		// Double-check
		if instance, ok := root.singletons[key]; ok {
			return instance, nil
		}

		instance, err := reg.Factory(c)
		if err != nil {
			return nil, err
		}
		root.singletons[key] = instance
		return instance, nil

	case api.Scoped:
		c.mu.RLock()
		if instance, ok := c.singletons[key]; ok {
			c.mu.RUnlock()
			return instance, nil
		}
		c.mu.RUnlock()

		c.mu.Lock()
		defer c.mu.Unlock()

		if instance, ok := c.singletons[key]; ok {
			return instance, nil
		}

		instance, err := reg.Factory(c)
		if err != nil {
			return nil, err
		}
		c.singletons[key] = instance
		return instance, nil

	default: // Transient
		return reg.Factory(c)
	}
}

// getRoot returns the root container
func (c *DefaultContainer) getRoot() *DefaultContainer {
	if c.parent == nil {
		return c
	}
	return c.parent.getRoot()
}

// RegisterFunc is a helper to register a factory function.
func RegisterFunc[T any](c *DefaultContainer, factory func(api.Container) (T, error), scope api.Scope) error {
	var zero T
	t := reflect.TypeOf(&zero).Elem()

	return c.Register(api.Registration{
		Type: t,
		Factory: func(container api.Container) (any, error) {
			return factory(container)
		},
		Scope: scope,
	})
}

// RegisterInstance registers a pre-created instance as a singleton.
func RegisterInstance[T any](c *DefaultContainer, instance T) error {
	t := reflect.TypeOf(&instance).Elem()

	return c.Register(api.Registration{
		Type: t,
		Factory: func(container api.Container) (any, error) {
			return instance, nil
		},
		Scope: api.Singleton,
	})
}
