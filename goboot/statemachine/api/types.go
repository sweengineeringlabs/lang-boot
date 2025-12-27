// Package api contains the public interfaces and types for the state machine module.
package api

import (
	"context"
	"time"
)

// State represents a state in the state machine.
type State string

// Event represents an event that triggers transitions.
type Event string

// Transition represents a state transition.
type Transition struct {
	From   State
	To     State
	Event  Event
	Guards []Guard
	Actions []Action
}

// Guard is a function that determines if a transition is allowed.
type Guard func(ctx context.Context, data any) bool

// Action is a function executed during a transition.
type Action func(ctx context.Context, from, to State, data any) error

// OnEnter is called when entering a state.
type OnEnter func(ctx context.Context, state State, data any) error

// OnExit is called when exiting a state.
type OnExit func(ctx context.Context, state State, data any) error

// StateMachine is the interface for state machines.
type StateMachine interface {
	// AddState adds a state.
	AddState(state State, onEnter OnEnter, onExit OnExit)

	// AddTransition adds a transition.
	AddTransition(transition Transition)

	// CanFire checks if an event can be fired.
	CanFire(event Event) bool

	// Fire fires an event.
	Fire(ctx context.Context, event Event, data any) error

	// CurrentState returns the current state.
	CurrentState() State

	// History returns the transition history.
	History() []TransitionRecord

	// Reset resets to initial state.
	Reset()
}

// TransitionRecord records a transition.
type TransitionRecord struct {
	From      State
	To        State
	Event     Event
	Timestamp time.Time
	Data      any
}

// StateMachineConfig configures a state machine.
type StateMachineConfig struct {
	InitialState State
	States       []State
	Transitions  []Transition
}

// Builder is a fluent builder for state machines.
type Builder interface {
	// State adds a state.
	State(state State) Builder

	// Initial sets the initial state.
	Initial(state State) Builder

	// Transition adds a transition.
	Transition(from, to State, event Event) TransitionBuilder

	// Build builds the state machine.
	Build() StateMachine
}

// TransitionBuilder builds transitions.
type TransitionBuilder interface {
	// Guard adds a guard condition.
	Guard(guard Guard) TransitionBuilder

	// Action adds an action.
	Action(action Action) TransitionBuilder

	// Done completes the transition definition.
	Done() Builder
}
