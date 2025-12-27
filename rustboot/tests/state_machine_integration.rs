//! Integration tests for Rustboot state machine

use rustboot::state_machine::*;

#[test]
fn test_state_machine_basic_transition() {
    let mut sm = StateMachine::new("idle");
    sm.add_transition("idle", "processing", None);
    
    assert!(sm.transition("processing").is_ok());
    assert_eq!(sm.current_state(), "processing");
}

#[test]
fn test_state_machine_with_guard() {
    let mut sm = StateMachine::new("idle");
    
    let guard = |_: &str| false;
    sm.add_transition("idle", "processing", Some(Box::new(guard)));
    
    assert!(sm.transition("processing").is_err());
    assert_eq!(sm.current_state(), "idle");
}

#[test]
fn test_state_machine_invalid_transition() {
    let mut sm = StateMachine::new("idle");
    sm.add_transition("idle", "processing", None);
    
    let result = sm.transition("completed");
    assert!(result.is_err());
}

#[test]
fn test_state_machine_multiple_transitions() {
    let mut sm = StateMachine::new("idle");
    sm.add_transition("idle", "processing", None);
    sm.add_transition("processing", "completed", None);
    sm.add_transition("completed", "idle", None);
    
    assert!(sm.transition("processing").is_ok());
    assert!(sm.transition("completed").is_ok());
    assert!(sm.transition("idle").is_ok());
}

#[test]
#[should_panic(expected = "InvalidTransition")]
fn test_state_machine_panic_on_invalid() {
    let mut sm = StateMachine::new("idle");
    sm.transition("invalid").expect("Should panic");
}
