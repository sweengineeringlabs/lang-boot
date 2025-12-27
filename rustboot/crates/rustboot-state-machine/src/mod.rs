//! State machine framework (L4: Core).
//!
//! Generic state machine with transitions, guards, and event handling.

pub mod machine;

// Re-export main types
pub use machine::{StateMachine, StateMachineError, StateMachineResult};
