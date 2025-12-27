//! Rustboot State Machine - State machine with transitions and guards

pub mod machine;

pub use machine::{StateMachine, StateMachineError, StateMachineResult};
