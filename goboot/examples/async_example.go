// Example demonstrating async utilities in goboot.
package main

import (
	"context"
	"fmt"
	"time"

	"dev.engineeringlabs/goboot/async"
)

func main() {
	// Example 1: Parallel execution
	fmt.Println("=== Parallel Execution ===")
	results := async.Parallel(
		func() (string, error) {
			time.Sleep(100 * time.Millisecond)
			return "Result 1", nil
		},
		func() (string, error) {
			time.Sleep(50 * time.Millisecond)
			return "Result 2", nil
		},
		func() (string, error) {
			time.Sleep(75 * time.Millisecond)
			return "Result 3", nil
		},
	)

	for i, r := range results {
		if r.Err != nil {
			fmt.Printf("Task %d failed: %v\n", i, r.Err)
		} else {
			fmt.Printf("Task %d: %s\n", i, r.Value)
		}
	}

	// Example 2: Timeout handling
	fmt.Println("\n=== Timeout Handling ===")
	result, err := async.WithTimeout(200*time.Millisecond, func(ctx context.Context) (string, error) {
		select {
		case <-time.After(100 * time.Millisecond):
			return "Completed in time!", nil
		case <-ctx.Done():
			return "", ctx.Err()
		}
	})

	if err != nil {
		fmt.Printf("Timed out: %v\n", err)
	} else {
		fmt.Printf("Result: %s\n", result)
	}

	// Example 3: Worker pool
	fmt.Println("\n=== Worker Pool ===")
	pool := async.NewWorkerPool(4)

	for i := 0; i < 10; i++ {
		taskNum := i
		pool.Submit(func() {
			fmt.Printf("Executing task %d\n", taskNum)
			time.Sleep(50 * time.Millisecond)
		})
	}

	pool.Wait()
	pool.Close()
	fmt.Println("All tasks completed!")

	// Example 4: Rate-limited execution
	fmt.Println("\n=== Rate-Limited Execution ===")
	limiter := async.NewRateLimiter(2)

	for i := 0; i < 5; i++ {
		taskNum := i
		go func() {
			limiter.Execute(func() {
				fmt.Printf("Rate-limited task %d starting\n", taskNum)
				time.Sleep(100 * time.Millisecond)
				fmt.Printf("Rate-limited task %d done\n", taskNum)
			})
		}()
	}

	time.Sleep(500 * time.Millisecond)

	// Example 5: Map parallel
	fmt.Println("\n=== Map Parallel ===")
	numbers := []int{1, 2, 3, 4, 5}
	doubled := async.Map(numbers, func(n int) int {
		return n * 2
	})
	fmt.Printf("Doubled: %v\n", doubled)

	// Example 6: First successful result
	fmt.Println("\n=== First Successful ===")
	fastest, err := async.First(
		func() (string, error) {
			time.Sleep(100 * time.Millisecond)
			return "Slow server", nil
		},
		func() (string, error) {
			time.Sleep(10 * time.Millisecond)
			return "Fast server", nil
		},
	)

	if err == nil {
		fmt.Printf("First result: %s\n", fastest)
	}
}
