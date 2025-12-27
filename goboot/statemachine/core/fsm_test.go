package core

import (
	"context"
	"testing"

	"dev.engineeringlabs/goboot/statemachine/api"
)

func TestFSM_BasicTransition(t *testing.T) {
	fsm := NewFSM("draft")
	fsm.AddTransition(api.Transition{
		From:  "draft",
		To:    "published",
		Event: "publish",
	})

	err := fsm.Fire(context.Background(), "publish", nil)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	if fsm.CurrentState() != "published" {
		t.Errorf("Expected 'published', got '%s'", fsm.CurrentState())
	}
}

func TestFSM_InvalidTransition(t *testing.T) {
	fsm := NewFSM("draft")
	fsm.AddTransition(api.Transition{
		From:  "draft",
		To:    "published",
		Event: "publish",
	})

	err := fsm.Fire(context.Background(), "delete", nil)
	if err == nil {
		t.Error("Expected error for invalid transition")
	}
}

func TestFSM_Guard(t *testing.T) {
	fsm := NewFSM("draft")
	fsm.AddTransition(api.Transition{
		From:  "draft",
		To:    "published",
		Event: "publish",
		Guards: []api.Guard{
			func(ctx context.Context, data any) bool {
				validated, _ := data.(bool)
				return validated
			},
		},
	})

	// Guard rejects
	err := fsm.Fire(context.Background(), "publish", false)
	if err == nil {
		t.Error("Expected error when guard rejects")
	}

	// Guard accepts
	err = fsm.Fire(context.Background(), "publish", true)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
}

func TestFSM_History(t *testing.T) {
	fsm := NewFSM("a")
	fsm.AddTransition(api.Transition{From: "a", To: "b", Event: "next"})
	fsm.AddTransition(api.Transition{From: "b", To: "c", Event: "next"})

	fsm.Fire(context.Background(), "next", nil)
	fsm.Fire(context.Background(), "next", nil)

	history := fsm.History()
	if len(history) != 2 {
		t.Errorf("Expected 2 history records, got %d", len(history))
	}
}

func TestFSM_Reset(t *testing.T) {
	fsm := NewFSM("initial")
	fsm.AddTransition(api.Transition{From: "initial", To: "running", Event: "start"})

	fsm.Fire(context.Background(), "start", nil)
	fsm.Reset()

	if fsm.CurrentState() != "initial" {
		t.Error("Reset should return to initial state")
	}
	if len(fsm.History()) != 0 {
		t.Error("Reset should clear history")
	}
}

func TestFSM_CanFire(t *testing.T) {
	fsm := NewFSM("draft")
	fsm.AddTransition(api.Transition{From: "draft", To: "published", Event: "publish"})

	if !fsm.CanFire("publish") {
		t.Error("Should be able to fire 'publish' from 'draft'")
	}
	if fsm.CanFire("delete") {
		t.Error("Should not be able to fire 'delete' from 'draft'")
	}
}

func TestFSMBuilder(t *testing.T) {
	fsm := NewBuilder().
		Initial("pending").
		State("pending").
		State("approved").
		State("rejected").
		Transition("pending", "approved", "approve").Done().
		Transition("pending", "rejected", "reject").Done().
		Build()

	if fsm.CurrentState() != "pending" {
		t.Error("Should start in pending state")
	}

	fsm.Fire(context.Background(), "approve", nil)
	if fsm.CurrentState() != "approved" {
		t.Error("Should be approved after approve event")
	}
}
