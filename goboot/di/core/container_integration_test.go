package core

import (
	"sync"
	"testing"

	"dev.engineeringlabs/goboot/di/api"
)

// Service interfaces for integration tests
type Logger interface {
	Log(message string)
}

type Repository interface {
	Get(id string) (string, error)
}

type Service interface {
	Process(id string) (string, error)
}

// Implementations
type consoleLogger struct {
	messages []string
	mu       sync.Mutex
}

func (l *consoleLogger) Log(message string) {
	l.mu.Lock()
	l.messages = append(l.messages, message)
	l.mu.Unlock()
}

type inMemoryRepository struct{}

func (r *inMemoryRepository) Get(id string) (string, error) {
	return "data-" + id, nil
}

type myService struct {
	logger Logger
	repo   Repository
}

func (s *myService) Process(id string) (string, error) {
	s.logger.Log("Processing " + id)
	data, err := s.repo.Get(id)
	if err != nil {
		return "", err
	}
	return "processed:" + data, nil
}

func TestContainer_DependencyInjection(t *testing.T) {
	container := NewContainer()

	// Register Logger as singleton
	logger := &consoleLogger{}
	RegisterInstance(container, logger)

	// Register Repository as singleton
	RegisterFunc(container, func(c api.Container) (*inMemoryRepository, error) {
		return &inMemoryRepository{}, nil
	}, api.Singleton)

	// Register Service with dependencies
	RegisterFunc(container, func(c api.Container) (*myService, error) {
		var log *consoleLogger
		var repo *inMemoryRepository

		if err := c.Resolve(&log); err != nil {
			return nil, err
		}
		if err := c.Resolve(&repo); err != nil {
			return nil, err
		}

		return &myService{logger: log, repo: repo}, nil
	}, api.Transient)

	// Resolve and use the service
	var svc *myService
	if err := container.Resolve(&svc); err != nil {
		t.Fatalf("Failed to resolve service: %v", err)
	}

	result, err := svc.Process("123")
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if result != "processed:data-123" {
		t.Errorf("Unexpected result: %s", result)
	}

	// Verify logger was called
	if len(logger.messages) != 1 {
		t.Errorf("Expected 1 log message, got %d", len(logger.messages))
	}
}

func TestContainer_ScopedDependencies(t *testing.T) {
	container := NewContainer()

	counter := 0
	RegisterFunc(container, func(c api.Container) (int, error) {
		counter++
		return counter, nil
	}, api.Scoped)

	// Create two scopes
	scope1 := container.CreateScope()
	scope2 := container.CreateScope()

	// Resolve in scope1 twice - should get same value
	var val1a, val1b int
	scope1.Resolve(&val1a)
	scope1.Resolve(&val1b)

	if val1a != val1b {
		t.Errorf("Same scope should return same value: %d vs %d", val1a, val1b)
	}

	// Resolve in scope2 - should get different value
	var val2 int
	scope2.Resolve(&val2)

	if val1a == val2 {
		t.Error("Different scopes should return different values")
	}
}

func TestContainer_NestedScopes(t *testing.T) {
	parent := NewContainer()

	parentValue := 100
	RegisterFunc(parent, func(c api.Container) (int, error) {
		return parentValue, nil
	}, api.Singleton)

	// Child scope should be able to access parent registrations
	child := parent.CreateScope()

	var resolved int
	if err := child.Resolve(&resolved); err != nil {
		t.Fatalf("Failed to resolve from child: %v", err)
	}
	if resolved != parentValue {
		t.Errorf("Expected %d, got %d", parentValue, resolved)
	}
}

func TestContainer_ConcurrentResolve(t *testing.T) {
	container := NewContainer()

	callCount := 0
	var mu sync.Mutex

	RegisterFunc(container, func(c api.Container) (string, error) {
		mu.Lock()
		callCount++
		mu.Unlock()
		return "singleton", nil
	}, api.Singleton)

	// Resolve concurrently
	var wg sync.WaitGroup
	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			var val string
			container.Resolve(&val)
		}()
	}
	wg.Wait()

	// Singleton should only be created once
	mu.Lock()
	if callCount != 1 {
		t.Errorf("Singleton factory should be called once, got %d", callCount)
	}
	mu.Unlock()
}

func TestContainer_OverrideRegistration(t *testing.T) {
	container := NewContainer()

	// First registration
	RegisterFunc(container, func(c api.Container) (string, error) {
		return "first", nil
	}, api.Singleton)

	// Override with second registration
	RegisterFunc(container, func(c api.Container) (string, error) {
		return "second", nil
	}, api.Singleton)

	var resolved string
	container.Resolve(&resolved)

	if resolved != "second" {
		t.Errorf("Expected 'second' (override), got '%s'", resolved)
	}
}

func TestContainer_NamedVsTyped(t *testing.T) {
	container := NewContainer()

	// Register by type
	RegisterFunc(container, func(c api.Container) (string, error) {
		return "typed-value", nil
	}, api.Singleton)

	// Register by name
	container.Register(api.Registration{
		Name: "specific-string",
		Factory: func(c api.Container) (any, error) {
			return "named-value", nil
		},
		Scope: api.Singleton,
	})

	// Both should coexist
	var typed string
	container.Resolve(&typed)
	if typed != "typed-value" {
		t.Errorf("Type resolution failed: %s", typed)
	}

	var named string
	container.ResolveNamed("specific-string", &named)
	if named != "named-value" {
		t.Errorf("Named resolution failed: %s", named)
	}
}

// Benchmark tests

func BenchmarkContainer_Resolve_Transient(b *testing.B) {
	container := NewContainer()
	RegisterFunc(container, func(c api.Container) (int, error) {
		return 42, nil
	}, api.Transient)

	var val int
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		container.Resolve(&val)
	}
}

func BenchmarkContainer_Resolve_Singleton(b *testing.B) {
	container := NewContainer()
	RegisterFunc(container, func(c api.Container) (int, error) {
		return 42, nil
	}, api.Singleton)

	var val int
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		container.Resolve(&val)
	}
}

func BenchmarkContainer_Resolve_Scoped(b *testing.B) {
	container := NewContainer()
	RegisterFunc(container, func(c api.Container) (int, error) {
		return 42, nil
	}, api.Scoped)

	scope := container.CreateScope()
	var val int
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		scope.Resolve(&val)
	}
}

func BenchmarkContainer_CreateScope(b *testing.B) {
	container := NewContainer()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		container.CreateScope()
	}
}
