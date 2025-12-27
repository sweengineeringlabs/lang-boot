//! State machine implementation (L4: Core - State Machine).
//!
//! Generic state machine with transitions and guards.

use std::collections::HashMap;
use std::hash::Hash;

/// State machine error.
#[derive(Debug, thiserror::Error)]
pub enum StateMachineError {
    /// Invalid transition.
    #[error("Invalid transition from {from:?} to {to:?}")]
    InvalidTransition { from: String, to: String },
    
    /// Guard rejected transition.
    #[error("Guard rejected transition: {0}")]
    GuardRejected(String),
}

/// Result type for state machine operations.
pub type StateMachineResult<T> = Result<T, StateMachineError>;

/// Type alias for guard functions.
type GuardFn<S> = Box<dyn Fn(&S, &S) -> bool + Send + Sync>;

/// State machine.
pub struct StateMachine<S, E>
where
    S: Clone + Eq + Hash + std::fmt::Debug,
    E: Clone + Eq + Hash,
{
    current_state: S,
    transitions: HashMap<(S, E), S>,
    guards: HashMap<(S, E), GuardFn<S>>,
}

impl<S, E> StateMachine<S, E>
where
    S: Clone + Eq + Hash + std::fmt::Debug,
    E: Clone + Eq + Hash,
{
    /// Create a new state machine with initial state.
    pub fn new(initial_state: S) -> Self {
        Self {
            current_state: initial_state,
            transitions: HashMap::new(),
            guards: HashMap::new(),
        }
    }
    
    /// Add a transition.
    pub fn add_transition(&mut self, from: S, event: E, to: S) {
        self.transitions.insert((from, event), to);
    }
    
    /// Add a guard to a transition.
    pub fn add_guard<F>(&mut self, from: S, event: E, guard: F)
    where
        F: Fn(&S, &S) -> bool + Send + Sync + 'static,
    {
        self.guards.insert((from, event), Box::new(guard));
    }
    
    /// Get current state.
    pub fn current_state(&self) -> &S {
        &self.current_state
    }
    
    /// Trigger an event.
    pub fn trigger(&mut self, event: E) -> StateMachineResult<&S> {
        let key = (self.current_state.clone(), event);
        
        if let Some(next_state) = self.transitions.get(&key) {
            // Check guard if exists
            if let Some(guard) = self.guards.get(&key) {
                if !guard(&self.current_state, next_state) {
                    return Err(StateMachineError::GuardRejected(
                        format!("Transition from {:?} rejected", self.current_state)
                    ));
                }
            }
            
            self.current_state = next_state.clone();
            Ok(&self.current_state)
        } else {
            Err(StateMachineError::InvalidTransition {
                from: format!("{:?}", self.current_state),
                to: "unknown".to_string(),
            })
        }
    }
    
    /// Check if transition is valid.
    pub fn can_trigger(&self, event: &E) -> bool {
        let key = (self.current_state.clone(), event.clone());
        self.transitions.contains_key(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum State {
        Idle,
        Running,
        Paused,
        Stopped,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum Event {
        Start,
        Pause,
        Resume,
        Stop,
    }

    #[test]
    fn test_state_machine() {
        let mut sm = StateMachine::new(State::Idle);
        
        sm.add_transition(State::Idle, Event::Start, State::Running);
        sm.add_transition(State::Running, Event::Pause, State::Paused);
        sm.add_transition(State::Paused, Event::Resume, State::Running);
        sm.add_transition(State::Running, Event::Stop, State::Stopped);
        
        assert_eq!(*sm.current_state(), State::Idle);
        
        sm.trigger(Event::Start).unwrap();
        assert_eq!(*sm.current_state(), State::Running);
        
        sm.trigger(Event::Pause).unwrap();
        assert_eq!(*sm.current_state(), State::Paused);
        
        sm.trigger(Event::Resume).unwrap();
        assert_eq!(*sm.current_state(), State::Running);
    }

    #[test]
    fn test_invalid_transition() {
        let mut sm = StateMachine::new(State::Idle);
        sm.add_transition(State::Idle, Event::Start, State::Running);
        
        let result = sm.trigger(Event::Pause);
        assert!(result.is_err());
    }

    #[test]
    fn test_guard() {
        let mut sm = StateMachine::new(State::Idle);
        sm.add_transition(State::Idle, Event::Start, State::Running);
        sm.add_guard(State::Idle, Event::Start, |_, _| false);
        
        let result = sm.trigger(Event::Start);
        assert!(result.is_err());
    }
}
