// Package core contains the implementation details for the state machine module.
package core

import (
	"context"
	"fmt"
	"sync"
	"time"

	"dev.engineeringlabs/goboot/statemachine/api"
)

// FSM is a finite state machine implementation.
type FSM struct {
	current     api.State
	initial     api.State
	states      map[api.State]*stateConfig
	transitions []api.Transition
	history     []api.TransitionRecord
	mu          sync.RWMutex
}

type stateConfig struct {
	onEnter api.OnEnter
	onExit  api.OnExit
}

// NewFSM creates a new FSM.
func NewFSM(initial api.State) *FSM {
	return &FSM{
		current:     initial,
		initial:     initial,
		states:      make(map[api.State]*stateConfig),
		transitions: make([]api.Transition, 0),
		history:     make([]api.TransitionRecord, 0),
	}
}

// AddState adds a state.
func (f *FSM) AddState(state api.State, onEnter api.OnEnter, onExit api.OnExit) {
	f.mu.Lock()
	f.states[state] = &stateConfig{
		onEnter: onEnter,
		onExit:  onExit,
	}
	f.mu.Unlock()
}

// AddTransition adds a transition.
func (f *FSM) AddTransition(transition api.Transition) {
	f.mu.Lock()
	f.transitions = append(f.transitions, transition)
	f.mu.Unlock()
}

// CanFire checks if an event can be fired.
func (f *FSM) CanFire(event api.Event) bool {
	f.mu.RLock()
	defer f.mu.RUnlock()

	for _, t := range f.transitions {
		if t.From == f.current && t.Event == event {
			return true
		}
	}
	return false
}

// Fire fires an event.
func (f *FSM) Fire(ctx context.Context, event api.Event, data any) error {
	f.mu.Lock()
	defer f.mu.Unlock()

	// Find transition
	var transition *api.Transition
	for i := range f.transitions {
		if f.transitions[i].From == f.current && f.transitions[i].Event == event {
			transition = &f.transitions[i]
			break
		}
	}

	if transition == nil {
		return fmt.Errorf("no transition from %s on event %s", f.current, event)
	}

	// Check guards
	for _, guard := range transition.Guards {
		if !guard(ctx, data) {
			return fmt.Errorf("guard rejected transition from %s to %s", transition.From, transition.To)
		}
	}

	// Execute exit handler
	if config, ok := f.states[f.current]; ok && config.onExit != nil {
		if err := config.onExit(ctx, f.current, data); err != nil {
			return fmt.Errorf("exit handler failed: %w", err)
		}
	}

	// Execute transition actions
	for _, action := range transition.Actions {
		if err := action(ctx, transition.From, transition.To, data); err != nil {
			return fmt.Errorf("action failed: %w", err)
		}
	}

	// Record history
	f.history = append(f.history, api.TransitionRecord{
		From:      transition.From,
		To:        transition.To,
		Event:     event,
		Timestamp: time.Now(),
		Data:      data,
	})

	// Execute enter handler
	if config, ok := f.states[transition.To]; ok && config.onEnter != nil {
		if err := config.onEnter(ctx, transition.To, data); err != nil {
			return fmt.Errorf("enter handler failed: %w", err)
		}
	}

	// Update state
	f.current = transition.To

	return nil
}

// CurrentState returns the current state.
func (f *FSM) CurrentState() api.State {
	f.mu.RLock()
	defer f.mu.RUnlock()
	return f.current
}

// History returns the transition history.
func (f *FSM) History() []api.TransitionRecord {
	f.mu.RLock()
	defer f.mu.RUnlock()
	result := make([]api.TransitionRecord, len(f.history))
	copy(result, f.history)
	return result
}

// Reset resets to initial state.
func (f *FSM) Reset() {
	f.mu.Lock()
	f.current = f.initial
	f.history = make([]api.TransitionRecord, 0)
	f.mu.Unlock()
}

// FSMBuilder is a fluent builder for state machines.
type FSMBuilder struct {
	initial     api.State
	states      []api.State
	transitions []api.Transition
	stateEnter  map[api.State]api.OnEnter
	stateExit   map[api.State]api.OnExit
}

// NewBuilder creates a new FSMBuilder.
func NewBuilder() *FSMBuilder {
	return &FSMBuilder{
		states:      make([]api.State, 0),
		transitions: make([]api.Transition, 0),
		stateEnter:  make(map[api.State]api.OnEnter),
		stateExit:   make(map[api.State]api.OnExit),
	}
}

// State adds a state.
func (b *FSMBuilder) State(state api.State) *FSMBuilder {
	b.states = append(b.states, state)
	return b
}

// Initial sets the initial state.
func (b *FSMBuilder) Initial(state api.State) *FSMBuilder {
	b.initial = state
	return b
}

// OnEnter sets the enter handler for a state.
func (b *FSMBuilder) OnEnter(state api.State, handler api.OnEnter) *FSMBuilder {
	b.stateEnter[state] = handler
	return b
}

// OnExit sets the exit handler for a state.
func (b *FSMBuilder) OnExit(state api.State, handler api.OnExit) *FSMBuilder {
	b.stateExit[state] = handler
	return b
}

// Transition adds a transition.
func (b *FSMBuilder) Transition(from, to api.State, event api.Event) *TransitionBuilder {
	return &TransitionBuilder{
		builder: b,
		transition: api.Transition{
			From:  from,
			To:    to,
			Event: event,
		},
	}
}

// Build builds the state machine.
func (b *FSMBuilder) Build() api.StateMachine {
	fsm := NewFSM(b.initial)

	for _, state := range b.states {
		fsm.AddState(state, b.stateEnter[state], b.stateExit[state])
	}

	for _, t := range b.transitions {
		fsm.AddTransition(t)
	}

	return fsm
}

// TransitionBuilder builds transitions.
type TransitionBuilder struct {
	builder    *FSMBuilder
	transition api.Transition
}

// Guard adds a guard condition.
func (t *TransitionBuilder) Guard(guard api.Guard) *TransitionBuilder {
	t.transition.Guards = append(t.transition.Guards, guard)
	return t
}

// Action adds an action.
func (t *TransitionBuilder) Action(action api.Action) *TransitionBuilder {
	t.transition.Actions = append(t.transition.Actions, action)
	return t
}

// Done completes the transition definition.
func (t *TransitionBuilder) Done() *FSMBuilder {
	t.builder.transitions = append(t.builder.transitions, t.transition)
	return t.builder
}
