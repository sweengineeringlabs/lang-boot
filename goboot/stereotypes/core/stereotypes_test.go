package core

import (
	"context"
	"errors"
	"sync"
	"testing"
	"time"

	"dev.engineeringlabs/goboot/stereotypes/api"
)

func TestDefaultAnnotationRegistry(t *testing.T) {
	registry := NewAnnotationRegistry()

	annotation := api.NewAnnotation("Transactional").
		With("isolation", "SERIALIZABLE")

	registry.Register("UserService", annotation)

	t.Run("Get", func(t *testing.T) {
		annotations := registry.Get("UserService")
		if len(annotations) != 1 {
			t.Errorf("Expected 1 annotation, got %d", len(annotations))
		}
		if annotations[0].Name != "Transactional" {
			t.Error("Wrong annotation name")
		}
	})

	t.Run("Has", func(t *testing.T) {
		if !registry.Has("UserService", "Transactional") {
			t.Error("Should have Transactional annotation")
		}
		if registry.Has("UserService", "Other") {
			t.Error("Should not have Other annotation")
		}
	})
}

func TestRetryableDecorator(t *testing.T) {
	attempts := 0

	fn := func(ctx context.Context) (string, error) {
		attempts++
		if attempts < 3 {
			return "", errors.New("failed")
		}
		return "success", nil
	}

	wrapped := Retryable[string](5, 10*time.Millisecond)(fn)

	result, err := wrapped(context.Background())
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if result != "success" {
		t.Errorf("Expected 'success', got '%s'", result)
	}
	if attempts != 3 {
		t.Errorf("Expected 3 attempts, got %d", attempts)
	}
}

func TestTimedDecorator(t *testing.T) {
	var loggedName string
	var loggedDuration time.Duration

	fn := func(ctx context.Context) (int, error) {
		time.Sleep(50 * time.Millisecond)
		return 42, nil
	}

	wrapped := Timed[int]("test-function", func(name string, d time.Duration) {
		loggedName = name
		loggedDuration = d
	})(fn)

	result, _ := wrapped(context.Background())

	if result != 42 {
		t.Errorf("Expected 42, got %d", result)
	}
	if loggedName != "test-function" {
		t.Errorf("Expected 'test-function', got '%s'", loggedName)
	}
	if loggedDuration < 50*time.Millisecond {
		t.Error("Duration should be at least 50ms")
	}
}

func TestCachedDecorator(t *testing.T) {
	cache := make(map[string]int)
	var mu sync.RWMutex

	calls := 0
	fn := func(key string) int {
		calls++
		return len(key) * 10
	}

	wrapped := Cached(cache, &mu)(fn)

	// First call - should call original function
	result1 := wrapped("hello")
	if result1 != 50 {
		t.Errorf("Expected 50, got %d", result1)
	}
	if calls != 1 {
		t.Errorf("Expected 1 call, got %d", calls)
	}

	// Second call with same key - should return cached
	result2 := wrapped("hello")
	if result2 != 50 {
		t.Errorf("Expected 50, got %d", result2)
	}
	if calls != 1 {
		t.Error("Should not call original function again")
	}

	// Third call with different key
	result3 := wrapped("world")
	if calls != 2 {
		t.Errorf("Expected 2 calls, got %d", calls)
	}
	_ = result3
}

func TestBaseComponent(t *testing.T) {
	component := NewBaseComponent(api.StereotypeService)

	if component.Stereotype() != api.StereotypeService {
		t.Error("Wrong stereotype")
	}
}

func TestBaseService(t *testing.T) {
	service := NewBaseService()

	if service.Stereotype() != api.StereotypeService {
		t.Error("Should be service stereotype")
	}
}

func TestBaseRepository(t *testing.T) {
	repo := NewBaseRepository()

	if repo.Stereotype() != api.StereotypeRepository {
		t.Error("Should be repository stereotype")
	}
}

func TestBaseController(t *testing.T) {
	controller := NewBaseController()

	if controller.Stereotype() != api.StereotypeController {
		t.Error("Should be controller stereotype")
	}
}

func TestGetStereotype(t *testing.T) {
	service := NewBaseService()

	stereotype, ok := GetStereotype(service)
	if !ok {
		t.Error("Should have stereotype")
	}
	if stereotype != api.StereotypeService {
		t.Error("Wrong stereotype")
	}

	// Non-marker type
	_, ok = GetStereotype("string")
	if ok {
		t.Error("String should not have stereotype")
	}
}

type testLifecycle struct {
	initialized bool
	destroyed   bool
}

func (l *testLifecycle) OnInit() error {
	l.initialized = true
	return nil
}

func (l *testLifecycle) OnDestroy() error {
	l.destroyed = true
	return nil
}

func TestInitializeLifecycle(t *testing.T) {
	lc := &testLifecycle{}

	err := InitializeLifecycle(lc)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if !lc.initialized {
		t.Error("Should be initialized")
	}
}

func TestDestroyLifecycle(t *testing.T) {
	lc := &testLifecycle{}

	err := DestroyLifecycle(lc)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if !lc.destroyed {
		t.Error("Should be destroyed")
	}
}

func TestSynchronizedDecorator(t *testing.T) {
	var mu sync.Mutex
	counter := 0

	fn := func() int {
		counter++
		return counter
	}

	wrapped := Synchronized[int](&mu)(fn)

	var wg sync.WaitGroup
	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			wrapped()
		}()
	}
	wg.Wait()

	if counter != 100 {
		t.Errorf("Expected 100, got %d", counter)
	}
}

func TestWrap(t *testing.T) {
	fn := func() int { return 1 }

	double := func(next func() int) func() int {
		return func() int { return next() * 2 }
	}

	add10 := func(next func() int) func() int {
		return func() int { return next() + 10 }
	}

	// Decorators applied in reverse: double first, then add10
	wrapped := Wrap(fn, add10, double)
	result := wrapped()

	// (1 * 2) + 10 = 12
	if result != 12 {
		t.Errorf("Expected 12, got %d", result)
	}
}
