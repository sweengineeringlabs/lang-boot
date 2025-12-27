// Package statemachine provides state machine utilities for the goboot framework.
//
// This module provides:
//   - API layer: State, Event, Transition, StateMachine interface
//   - Core layer: FSM, FSMBuilder implementations
//
// Example:
//
//	import "dev.engineeringlabs/goboot/statemachine"
//
//	const (
//	    StateDraft    = statemachine.State("draft")
//	    StateReview   = statemachine.State("review")
//	    StateApproved = statemachine.State("approved")
//	)
//
//	const (
//	    EventSubmit  = statemachine.Event("submit")
//	    EventApprove = statemachine.Event("approve")
//	    EventReject  = statemachine.Event("reject")
//	)
//
//	fsm := statemachine.NewBuilder().
//	    Initial(StateDraft).
//	    State(StateDraft).
//	    State(StateReview).
//	    State(StateApproved).
//	    Transition(StateDraft, StateReview, EventSubmit).Done().
//	    Transition(StateReview, StateApproved, EventApprove).Done().
//	    Transition(StateReview, StateDraft, EventReject).Done().
//	    Build()
//
//	fsm.Fire(ctx, EventSubmit, nil)
package statemachine

import (
	"dev.engineeringlabs/goboot/statemachine/api"
	"dev.engineeringlabs/goboot/statemachine/core"
)

// Re-export API types
type (
	// State represents a state in the state machine.
	State = api.State
	// Event represents an event that triggers transitions.
	Event = api.Event
	// Transition represents a state transition.
	Transition = api.Transition
	// Guard is a function that determines if a transition is allowed.
	Guard = api.Guard
	// Action is a function executed during a transition.
	Action = api.Action
	// OnEnter is called when entering a state.
	OnEnter = api.OnEnter
	// OnExit is called when exiting a state.
	OnExit = api.OnExit
	// StateMachine is the interface for state machines.
	StateMachine = api.StateMachine
	// TransitionRecord records a transition.
	TransitionRecord = api.TransitionRecord
	// StateMachineConfig configures a state machine.
	StateMachineConfig = api.StateMachineConfig
)

// Re-export Core types
type (
	// FSM is a finite state machine implementation.
	FSM = core.FSM
	// FSMBuilder is a fluent builder for state machines.
	FSMBuilder = core.FSMBuilder
	// TransitionBuilder builds transitions.
	TransitionBuilder = core.TransitionBuilder
)

// Re-export Core functions
var (
	NewFSM     = core.NewFSM
	NewBuilder = core.NewBuilder
)
