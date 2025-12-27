package async

import (
	"context"
	"errors"
	"sync/atomic"
	"testing"
	"time"
)

func TestParallel(t *testing.T) {
	results := Parallel(
		func() (int, error) { return 1, nil },
		func() (int, error) { return 2, nil },
		func() (int, error) { return 3, nil },
	)

	if len(results) != 3 {
		t.Errorf("expected 3 results, got %d", len(results))
	}

	for i, r := range results {
		if r.Err != nil {
			t.Errorf("result %d has error: %v", i, r.Err)
		}
		if r.Value != i+1 {
			t.Errorf("result %d has value %d, expected %d", i, r.Value, i+1)
		}
	}
}

func TestParallelWithError(t *testing.T) {
	expectedErr := errors.New("test error")
	results := Parallel(
		func() (int, error) { return 1, nil },
		func() (int, error) { return 0, expectedErr },
		func() (int, error) { return 3, nil },
	)

	if results[1].Err != expectedErr {
		t.Errorf("expected error, got %v", results[1].Err)
	}
}

func TestWithTimeout_Success(t *testing.T) {
	result, err := WithTimeout(time.Second, func(ctx context.Context) (string, error) {
		return "success", nil
	})

	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if result != "success" {
		t.Errorf("expected 'success', got %s", result)
	}
}

func TestWithTimeout_Timeout(t *testing.T) {
	_, err := WithTimeout(10*time.Millisecond, func(ctx context.Context) (string, error) {
		time.Sleep(100 * time.Millisecond)
		return "too late", nil
	})

	if err == nil {
		t.Error("expected timeout error")
	}
	if err != context.DeadlineExceeded {
		t.Errorf("expected DeadlineExceeded, got %v", err)
	}
}

func TestWorkerPool(t *testing.T) {
	pool := NewWorkerPool(4)
	defer pool.Close()

	var counter int32
	for i := 0; i < 100; i++ {
		pool.Submit(func() {
			atomic.AddInt32(&counter, 1)
		})
	}

	pool.Wait()

	if counter != 100 {
		t.Errorf("expected counter 100, got %d", counter)
	}
}

func TestRateLimiter(t *testing.T) {
	limiter := NewRateLimiter(2)
	var running int32
	var maxRunning int32

	for i := 0; i < 10; i++ {
		go func() {
			limiter.Execute(func() {
				current := atomic.AddInt32(&running, 1)
				
				// Track max concurrent
				for {
					max := atomic.LoadInt32(&maxRunning)
					if current <= max || atomic.CompareAndSwapInt32(&maxRunning, max, current) {
						break
					}
				}
				
				time.Sleep(10 * time.Millisecond)
				atomic.AddInt32(&running, -1)
			})
		}()
	}

	time.Sleep(200 * time.Millisecond)

	if maxRunning > 2 {
		t.Errorf("rate limiter allowed %d concurrent, expected max 2", maxRunning)
	}
}

func TestMap(t *testing.T) {
	items := []int{1, 2, 3, 4, 5}
	results := Map(items, func(x int) int {
		return x * 2
	})

	expected := []int{2, 4, 6, 8, 10}
	for i, r := range results {
		if r != expected[i] {
			t.Errorf("index %d: expected %d, got %d", i, expected[i], r)
		}
	}
}

func TestForEach(t *testing.T) {
	items := []int{1, 2, 3, 4, 5}
	var sum int32

	ForEach(items, func(x int) {
		atomic.AddInt32(&sum, int32(x))
	})

	if sum != 15 {
		t.Errorf("expected sum 15, got %d", sum)
	}
}

func TestFirst(t *testing.T) {
	result, err := First(
		func() (string, error) {
			time.Sleep(100 * time.Millisecond)
			return "slow", nil
		},
		func() (string, error) {
			return "fast", nil
		},
		func() (string, error) {
			time.Sleep(50 * time.Millisecond)
			return "medium", nil
		},
	)

	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if result != "fast" {
		t.Errorf("expected 'fast', got %s", result)
	}
}
