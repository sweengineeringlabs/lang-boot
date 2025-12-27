package core

import (
	"context"
	"sync"
	"sync/atomic"
	"testing"

	"dev.engineeringlabs/goboot/statemachine/api"
)

// Integration tests for state machine

func TestFSM_DocumentWorkflow(t *testing.T) {
	// Create a document workflow state machine
	fsm := NewBuilder().
		Initial("draft").
		// States
		State("draft").
		State("review").
		State("approved").
		State("rejected").
		State("published").
		State("archived").
		// Transitions
		Transition("draft", "review", "submit").Done().
		Transition("review", "approved", "approve").Done().
		Transition("review", "rejected", "reject").Done().
		Transition("rejected", "draft", "revise").Done().
		Transition("approved", "published", "publish").Done().
		Transition("published", "archived", "archive").Done().
		Build()

	ctx := context.Background()

	// Simulate workflow
	steps := []struct {
		event     api.Event
		expected  api.State
		shouldErr bool
	}{
		{"submit", "review", false},
		{"reject", "rejected", false},
		{"revise", "draft", false},
		{"submit", "review", false},
		{"approve", "approved", false},
		{"publish", "published", false},
		{"archive", "archived", false},
	}

	for _, step := range steps {
		err := fsm.Fire(ctx, step.event, nil)
		if step.shouldErr && err == nil {
			t.Errorf("Expected error for event '%s'", step.event)
		}
		if !step.shouldErr && err != nil {
			t.Errorf("Unexpected error for event '%s': %v", step.event, err)
		}
		if fsm.CurrentState() != step.expected {
			t.Errorf("After '%s': expected '%s', got '%s'",
				step.event, step.expected, fsm.CurrentState())
		}
	}

	// Verify history
	history := fsm.History()
	if len(history) != len(steps) {
		t.Errorf("Expected %d history records, got %d", len(steps), len(history))
	}
}

func TestFSM_OrderProcessing(t *testing.T) {
	type OrderContext struct {
		IsPaid    bool
		IsInStock bool
		Amount    float64
	}

	fsm := NewBuilder().
		Initial("pending").
		State("pending").
		State("paid").
		State("processing").
		State("shipped").
		State("delivered").
		State("cancelled").
		// Transitions with guards
		Transition("pending", "paid", "pay").
			Guard(func(ctx context.Context, data any) bool {
				order := data.(*OrderContext)
				return order.Amount > 0
			}).Done().
		Transition("paid", "processing", "process").
			Guard(func(ctx context.Context, data any) bool {
				order := data.(*OrderContext)
				return order.IsInStock
			}).Done().
		Transition("processing", "shipped", "ship").Done().
		Transition("shipped", "delivered", "deliver").Done().
		Transition("pending", "cancelled", "cancel").Done().
		Transition("paid", "cancelled", "cancel").Done().
		Build()

	ctx := context.Background()

	t.Run("SuccessfulOrder", func(t *testing.T) {
		fsm.Reset()
		order := &OrderContext{Amount: 99.99, IsInStock: true}

		fsm.Fire(ctx, "pay", order)
		fsm.Fire(ctx, "process", order)
		fsm.Fire(ctx, "ship", order)
		fsm.Fire(ctx, "deliver", order)

		if fsm.CurrentState() != "delivered" {
			t.Errorf("Expected 'delivered', got '%s'", fsm.CurrentState())
		}
	})

	t.Run("OutOfStock", func(t *testing.T) {
		fsm.Reset()
		order := &OrderContext{Amount: 99.99, IsInStock: false}

		fsm.Fire(ctx, "pay", order)
		err := fsm.Fire(ctx, "process", order)

		if err == nil {
			t.Error("Expected error when processing out-of-stock order")
		}
		if fsm.CurrentState() != "paid" {
			t.Errorf("Should remain in 'paid', got '%s'", fsm.CurrentState())
		}
	})

	t.Run("ZeroAmount", func(t *testing.T) {
		fsm.Reset()
		order := &OrderContext{Amount: 0, IsInStock: true}

		err := fsm.Fire(ctx, "pay", order)

		if err == nil {
			t.Error("Expected error when paying zero amount")
		}
		if fsm.CurrentState() != "pending" {
			t.Errorf("Should remain in 'pending', got '%s'", fsm.CurrentState())
		}
	})

	t.Run("CancelFromPending", func(t *testing.T) {
		fsm.Reset()
		fsm.Fire(ctx, "cancel", nil)

		if fsm.CurrentState() != "cancelled" {
			t.Errorf("Expected 'cancelled', got '%s'", fsm.CurrentState())
		}
	})
}

func TestFSM_WithActions(t *testing.T) {
	var actionsExecuted []string
	var mu sync.Mutex

	recordAction := func(action string) {
		mu.Lock()
		actionsExecuted = append(actionsExecuted, action)
		mu.Unlock()
	}

	fsm := NewBuilder().
		Initial("idle").
		State("idle").
		State("running").
		State("paused").
		State("stopped").
		Transition("idle", "running", "start").
			Action(func(ctx context.Context, from, to api.State, data any) error {
				recordAction("onStart")
				return nil
			}).Done().
		Transition("running", "paused", "pause").
			Action(func(ctx context.Context, from, to api.State, data any) error {
				recordAction("onPause")
				return nil
			}).Done().
		Transition("paused", "running", "resume").
			Action(func(ctx context.Context, from, to api.State, data any) error {
				recordAction("onResume")
				return nil
			}).Done().
		Transition("running", "stopped", "stop").
			Action(func(ctx context.Context, from, to api.State, data any) error {
				recordAction("onStop")
				return nil
			}).Done().
		Build()

	ctx := context.Background()

	fsm.Fire(ctx, "start", nil)
	fsm.Fire(ctx, "pause", nil)
	fsm.Fire(ctx, "resume", nil)
	fsm.Fire(ctx, "stop", nil)

	expected := []string{"onStart", "onPause", "onResume", "onStop"}
	if len(actionsExecuted) != len(expected) {
		t.Errorf("Expected %d actions, got %d", len(expected), len(actionsExecuted))
	}
	for i, action := range expected {
		if i >= len(actionsExecuted) || actionsExecuted[i] != action {
			t.Errorf("Action[%d]: expected '%s', got '%s'", i, action, actionsExecuted[i])
		}
	}
}

func TestFSM_ConcurrentFire(t *testing.T) {
	fsm := NewFSM("count_0")
	for i := 0; i < 10; i++ {
		from := api.State("count_" + string(rune('0'+i)))
		to := api.State("count_" + string(rune('0'+i+1)))
		fsm.AddTransition(api.Transition{
			From:  from,
			To:    to,
			Event: "increment",
		})
	}

	ctx := context.Background()

	// Try concurrent fires - only one should succeed at a time
	var successCount int32
	var wg sync.WaitGroup

	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			if err := fsm.Fire(ctx, "increment", nil); err == nil {
				atomic.AddInt32(&successCount, 1)
			}
		}()
	}

	wg.Wait()

	// Due to the sequential nature of states, only specific transitions should succeed
	if successCount < 1 {
		t.Error("At least one transition should succeed")
	}
}

func TestFSM_AvailableEvents(t *testing.T) {
	fsm := NewBuilder().
		Initial("a").
		State("a").
		State("b").
		State("c").
		Transition("a", "b", "go_b").Done().
		Transition("a", "c", "go_c").Done().
		Transition("b", "c", "go_c").Done().
		Build()

	// From state "a", both "go_b" and "go_c" should be available
	if !fsm.CanFire("go_b") {
		t.Error("Should be able to fire 'go_b' from 'a'")
	}
	if !fsm.CanFire("go_c") {
		t.Error("Should be able to fire 'go_c' from 'a'")
	}

	fsm.Fire(context.Background(), "go_b", nil)

	// From state "b", only "go_c" should be available
	if fsm.CanFire("go_b") {
		t.Error("Should not be able to fire 'go_b' from 'b'")
	}
	if !fsm.CanFire("go_c") {
		t.Error("Should be able to fire 'go_c' from 'b'")
	}
}

func TestFSM_MultipleGuards(t *testing.T) {
	fsm := NewFSM("start")
	fsm.AddTransition(api.Transition{
		From:  "start",
		To:    "end",
		Event: "complete",
		Guards: []api.Guard{
			func(ctx context.Context, data any) bool {
				m := data.(map[string]bool)
				return m["guard1"]
			},
			func(ctx context.Context, data any) bool {
				m := data.(map[string]bool)
				return m["guard2"]
			},
		},
	})

	ctx := context.Background()

	t.Run("AllGuardsPass", func(t *testing.T) {
		fsm.Reset()
		data := map[string]bool{"guard1": true, "guard2": true}
		err := fsm.Fire(ctx, "complete", data)
		if err != nil {
			t.Errorf("Should succeed when all guards pass: %v", err)
		}
	})

	t.Run("FirstGuardFails", func(t *testing.T) {
		fsm.Reset()
		data := map[string]bool{"guard1": false, "guard2": true}
		err := fsm.Fire(ctx, "complete", data)
		if err == nil {
			t.Error("Should fail when first guard fails")
		}
	})

	t.Run("SecondGuardFails", func(t *testing.T) {
		fsm.Reset()
		data := map[string]bool{"guard1": true, "guard2": false}
		err := fsm.Fire(ctx, "complete", data)
		if err == nil {
			t.Error("Should fail when second guard fails")
		}
	})
}

// Benchmark tests

func BenchmarkFSM_Fire(b *testing.B) {
	fsm := NewFSM("a")
	fsm.AddTransition(api.Transition{From: "a", To: "b", Event: "next"})
	fsm.AddTransition(api.Transition{From: "b", To: "a", Event: "back"})

	ctx := context.Background()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		fsm.Fire(ctx, "next", nil)
		fsm.Fire(ctx, "back", nil)
	}
}

func BenchmarkFSM_CanFire(b *testing.B) {
	fsm := NewFSM("a")
	fsm.AddTransition(api.Transition{From: "a", To: "b", Event: "next"})

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		fsm.CanFire("next")
	}
}

func BenchmarkFSM_WithGuard(b *testing.B) {
	fsm := NewFSM("a")
	fsm.AddTransition(api.Transition{
		From:  "a",
		To:    "b",
		Event: "next",
		Guards: []api.Guard{
			func(ctx context.Context, data any) bool { return true },
		},
	})
	fsm.AddTransition(api.Transition{From: "b", To: "a", Event: "back"})

	ctx := context.Background()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		fsm.Fire(ctx, "next", nil)
		fsm.Fire(ctx, "back", nil)
	}
}

func BenchmarkFSMBuilder(b *testing.B) {
	for i := 0; i < b.N; i++ {
		NewBuilder().
			Initial("a").
			State("a").
			State("b").
			State("c").
			Transition("a", "b", "to_b").Done().
			Transition("b", "c", "to_c").Done().
			Transition("c", "a", "to_a").Done().
			Build()
	}
}
