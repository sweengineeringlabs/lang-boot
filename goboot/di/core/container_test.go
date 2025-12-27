package core

import (
	"reflect"
	"testing"

	"dev.engineeringlabs/goboot/di/api"
)

func TestDefaultContainer_RegisterAndResolve(t *testing.T) {
	container := NewContainer()

	// Use the helper to register
	RegisterFunc(container, func(c api.Container) (string, error) {
		return "hello", nil
	}, api.Transient)

	// Resolve
	var value string
	err := container.Resolve(&value)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if value != "hello" {
		t.Errorf("Expected 'hello', got '%v'", value)
	}
}

func TestDefaultContainer_TransientScope(t *testing.T) {
	container := NewContainer()

	callCount := 0
	RegisterFunc(container, func(c api.Container) (int, error) {
		callCount++
		return callCount, nil
	}, api.Transient)

	// Each resolve should call factory
	var v1, v2 int
	container.Resolve(&v1)
	container.Resolve(&v2)

	if v1 == v2 {
		t.Error("Transient should create new instance each time")
	}
	if callCount != 2 {
		t.Errorf("Expected 2 calls, got %d", callCount)
	}
}

func TestDefaultContainer_SingletonScope(t *testing.T) {
	container := NewContainer()

	callCount := 0
	RegisterFunc(container, func(c api.Container) (int, error) {
		callCount++
		return callCount, nil
	}, api.Singleton)

	// Each resolve should return same instance
	var v1, v2 int
	container.Resolve(&v1)
	container.Resolve(&v2)

	if v1 != v2 {
		t.Error("Singleton should return same instance")
	}
	if callCount != 1 {
		t.Errorf("Expected 1 call, got %d", callCount)
	}
}

func TestDefaultContainer_NamedRegistrations(t *testing.T) {
	container := NewContainer()

	container.Register(api.Registration{
		Type: reflect.TypeOf((*string)(nil)).Elem(),
		Name: "primary",
		Factory: func(c api.Container) (any, error) {
			return "primary-service", nil
		},
		Scope: api.Singleton,
	})

	container.Register(api.Registration{
		Type: reflect.TypeOf((*string)(nil)).Elem(),
		Name: "secondary",
		Factory: func(c api.Container) (any, error) {
			return "secondary-service", nil
		},
		Scope: api.Singleton,
	})

	var primary, secondary string
	container.ResolveNamed("primary", &primary)
	container.ResolveNamed("secondary", &secondary)

	if primary != "primary-service" {
		t.Errorf("Expected 'primary-service', got '%v'", primary)
	}
	if secondary != "secondary-service" {
		t.Errorf("Expected 'secondary-service', got '%v'", secondary)
	}
}

func TestDefaultContainer_ResolveNotRegistered(t *testing.T) {
	container := NewContainer()

	var value string
	err := container.Resolve(&value)
	if err == nil {
		t.Error("Expected error for unregistered type")
	}
}

func TestDefaultContainer_IsRegistered(t *testing.T) {
	container := NewContainer()

	RegisterFunc(container, func(c api.Container) (string, error) {
		return "", nil
	}, api.Transient)

	// Check if type is registered by trying to resolve
	var value string
	err := container.Resolve(&value)
	if err != nil {
		t.Error("Should be able to resolve registered type")
	}
}

func TestRegisterFunc(t *testing.T) {
	container := NewContainer()

	RegisterFunc(container, func(c api.Container) (string, error) {
		return "from-func", nil
	}, api.Transient)

	var value string
	err := container.Resolve(&value)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if value != "from-func" {
		t.Errorf("Expected 'from-func', got '%v'", value)
	}
}

func TestRegisterInstance(t *testing.T) {
	container := NewContainer()

	instance := &testService{name: "pre-created"}
	RegisterInstance(container, instance)

	var resolved *testService
	container.Resolve(&resolved)

	if resolved.name != "pre-created" {
		t.Error("Should return exact instance")
	}
	if resolved != instance {
		t.Error("Should be same pointer")
	}
}

func TestDefaultContainer_CreateScope(t *testing.T) {
	container := NewContainer()

	callCount := 0
	RegisterFunc(container, func(c api.Container) (int, error) {
		callCount++
		return callCount, nil
	}, api.Scoped)

	// Create child scope
	scope := container.CreateScope()

	// Within same scope, should get same instance
	var v1, v2 int
	scope.Resolve(&v1)
	scope.Resolve(&v2)

	if v1 != v2 {
		t.Error("Same scope should return same instance")
	}

	// Different scope should get different instance
	scope2 := container.CreateScope()
	var v3 int
	scope2.Resolve(&v3)

	if v1 == v3 {
		t.Error("Different scopes should have different instances")
	}
}

func TestDefaultContainer_FactoryError(t *testing.T) {
	container := NewContainer()

	container.Register(api.Registration{
		Type: reflect.TypeOf((*string)(nil)).Elem(),
		Factory: func(c api.Container) (any, error) {
			return nil, &testError{"factory error"}
		},
		Scope: api.Transient,
	})

	var value string
	err := container.Resolve(&value)
	if err == nil {
		t.Error("Expected error from factory")
	}
}

type testService struct {
	name string
}

type testError struct {
	msg string
}

func (e *testError) Error() string {
	return e.msg
}
