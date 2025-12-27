//! Integration tests for rustboot-state-machine
//!
//! Comprehensive tests covering:
//! 1. State transitions (valid and invalid)
//! 2. Guard conditions
//! 3. Entry/exit actions (via wrapper pattern)
//! 4. Event handling
//! 5. State history tracking
//! 6. Error cases
//! 7. Edge cases

use dev_engineeringlabs_rustboot_state_machine::*;
use std::cell::RefCell;
use std::hash::Hash;
use std::rc::Rc;

// ============================================================================
// Basic State Machine - Simple On/Off Device
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DeviceState {
    Off,
    On,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DeviceEvent {
    PowerOn,
    PowerOff,
}

#[test]
fn test_simple_state_machine_creation() {
    let sm = StateMachine::<DeviceState, DeviceEvent>::new(DeviceState::Off);
    assert_eq!(*sm.current_state(), DeviceState::Off);
}

#[test]
fn test_simple_state_transitions() {
    let mut sm = StateMachine::new(DeviceState::Off);

    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);
    sm.add_transition(DeviceState::On, DeviceEvent::PowerOff, DeviceState::Off);

    // Initial state
    assert_eq!(*sm.current_state(), DeviceState::Off);

    // Power on
    let result = sm.trigger(DeviceEvent::PowerOn);
    assert!(result.is_ok());
    assert_eq!(*sm.current_state(), DeviceState::On);

    // Power off
    let result = sm.trigger(DeviceEvent::PowerOff);
    assert!(result.is_ok());
    assert_eq!(*sm.current_state(), DeviceState::Off);
}

#[test]
fn test_multiple_state_transitions_sequence() {
    let mut sm = StateMachine::new(DeviceState::Off);

    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);
    sm.add_transition(DeviceState::On, DeviceEvent::PowerOff, DeviceState::Off);

    // Test multiple on/off cycles
    for _ in 0..5 {
        assert_eq!(*sm.current_state(), DeviceState::Off);
        sm.trigger(DeviceEvent::PowerOn).unwrap();
        assert_eq!(*sm.current_state(), DeviceState::On);
        sm.trigger(DeviceEvent::PowerOff).unwrap();
    }
    assert_eq!(*sm.current_state(), DeviceState::Off);
}

#[test]
fn test_invalid_transition_error() {
    let mut sm = StateMachine::new(DeviceState::Off);
    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);

    // Try to power off when already off (invalid)
    let result = sm.trigger(DeviceEvent::PowerOff);
    assert!(result.is_err());

    match result {
        Err(StateMachineError::InvalidTransition { from, to }) => {
            assert!(from.contains("Off"));
            assert_eq!(to, "unknown");
        }
        _ => panic!("Expected InvalidTransition error"),
    }

    // State should remain unchanged
    assert_eq!(*sm.current_state(), DeviceState::Off);
}

#[test]
fn test_can_trigger_method() {
    let mut sm = StateMachine::new(DeviceState::Off);
    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);
    sm.add_transition(DeviceState::On, DeviceEvent::PowerOff, DeviceState::Off);

    // From Off state
    assert!(sm.can_trigger(&DeviceEvent::PowerOn));
    assert!(!sm.can_trigger(&DeviceEvent::PowerOff));

    // After transitioning to On
    sm.trigger(DeviceEvent::PowerOn).unwrap();
    assert!(!sm.can_trigger(&DeviceEvent::PowerOn));
    assert!(sm.can_trigger(&DeviceEvent::PowerOff));
}

// ============================================================================
// Complex State Machine - Media Player
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PlayerState {
    Stopped,
    Playing,
    Paused,
    Buffering,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PlayerEvent {
    Play,
    Pause,
    Stop,
    Buffer,
    Resume,
}

#[test]
fn test_complex_state_transitions() {
    let mut player = StateMachine::new(PlayerState::Stopped);

    // Define valid transitions
    player.add_transition(PlayerState::Stopped, PlayerEvent::Play, PlayerState::Playing);
    player.add_transition(PlayerState::Playing, PlayerEvent::Pause, PlayerState::Paused);
    player.add_transition(PlayerState::Playing, PlayerEvent::Stop, PlayerState::Stopped);
    player.add_transition(PlayerState::Playing, PlayerEvent::Buffer, PlayerState::Buffering);
    player.add_transition(PlayerState::Paused, PlayerEvent::Resume, PlayerState::Playing);
    player.add_transition(PlayerState::Paused, PlayerEvent::Stop, PlayerState::Stopped);
    player.add_transition(PlayerState::Buffering, PlayerEvent::Resume, PlayerState::Playing);
    player.add_transition(PlayerState::Buffering, PlayerEvent::Stop, PlayerState::Stopped);

    // Test complete workflow
    assert_eq!(*player.current_state(), PlayerState::Stopped);

    player.trigger(PlayerEvent::Play).unwrap();
    assert_eq!(*player.current_state(), PlayerState::Playing);

    player.trigger(PlayerEvent::Buffer).unwrap();
    assert_eq!(*player.current_state(), PlayerState::Buffering);

    player.trigger(PlayerEvent::Resume).unwrap();
    assert_eq!(*player.current_state(), PlayerState::Playing);

    player.trigger(PlayerEvent::Pause).unwrap();
    assert_eq!(*player.current_state(), PlayerState::Paused);

    player.trigger(PlayerEvent::Resume).unwrap();
    assert_eq!(*player.current_state(), PlayerState::Playing);

    player.trigger(PlayerEvent::Stop).unwrap();
    assert_eq!(*player.current_state(), PlayerState::Stopped);
}

#[test]
fn test_multiple_invalid_transitions() {
    let mut player = StateMachine::new(PlayerState::Stopped);
    player.add_transition(PlayerState::Stopped, PlayerEvent::Play, PlayerState::Playing);

    // Cannot pause when stopped
    assert!(player.trigger(PlayerEvent::Pause).is_err());
    assert_eq!(*player.current_state(), PlayerState::Stopped);

    // Cannot buffer when stopped
    assert!(player.trigger(PlayerEvent::Buffer).is_err());
    assert_eq!(*player.current_state(), PlayerState::Stopped);

    // Can play
    assert!(player.trigger(PlayerEvent::Play).is_ok());
    assert_eq!(*player.current_state(), PlayerState::Playing);
}

// ============================================================================
// Guard Conditions - Authentication & Authorization
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AuthState {
    LoggedOut,
    LoggedIn,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AuthEvent {
    Login,
    Logout,
    Elevate,
}

#[test]
fn test_guard_rejection() {
    let mut auth = StateMachine::new(AuthState::LoggedOut);

    auth.add_transition(AuthState::LoggedOut, AuthEvent::Login, AuthState::LoggedIn);

    // Add guard that always rejects
    auth.add_guard(AuthState::LoggedOut, AuthEvent::Login, |_, _| false);

    let result = auth.trigger(AuthEvent::Login);
    assert!(result.is_err());

    match result {
        Err(StateMachineError::GuardRejected(msg)) => {
            assert!(msg.contains("LoggedOut"));
        }
        _ => panic!("Expected GuardRejected error"),
    }

    // State should remain unchanged
    assert_eq!(*auth.current_state(), AuthState::LoggedOut);
}

#[test]
fn test_guard_acceptance() {
    let mut auth = StateMachine::new(AuthState::LoggedOut);

    auth.add_transition(AuthState::LoggedOut, AuthEvent::Login, AuthState::LoggedIn);

    // Add guard that always accepts
    auth.add_guard(AuthState::LoggedOut, AuthEvent::Login, |_, _| true);

    let result = auth.trigger(AuthEvent::Login);
    assert!(result.is_ok());
    assert_eq!(*auth.current_state(), AuthState::LoggedIn);
}

#[test]
fn test_conditional_guard() {
    let mut auth = StateMachine::new(AuthState::LoggedIn);

    auth.add_transition(AuthState::LoggedIn, AuthEvent::Elevate, AuthState::Admin);

    // Guard checks if current state is LoggedIn
    auth.add_guard(AuthState::LoggedIn, AuthEvent::Elevate, |current, _next| {
        *current == AuthState::LoggedIn
    });

    // Should succeed since we're in LoggedIn state
    let result = auth.trigger(AuthEvent::Elevate);
    assert!(result.is_ok());
    assert_eq!(*auth.current_state(), AuthState::Admin);
}

#[test]
fn test_multiple_guards_scenario() {
    // Simulate a system where elevation requires being logged in
    let mut auth1 = StateMachine::new(AuthState::LoggedIn);
    auth1.add_transition(AuthState::LoggedIn, AuthEvent::Elevate, AuthState::Admin);
    auth1.add_guard(AuthState::LoggedIn, AuthEvent::Elevate, |_, _| true);
    assert!(auth1.trigger(AuthEvent::Elevate).is_ok());

    // Another guard that rejects
    let mut auth2 = StateMachine::new(AuthState::LoggedIn);
    auth2.add_transition(AuthState::LoggedIn, AuthEvent::Elevate, AuthState::Admin);
    auth2.add_guard(AuthState::LoggedIn, AuthEvent::Elevate, |_, _| false);
    assert!(auth2.trigger(AuthEvent::Elevate).is_err());
}

// ============================================================================
// State Machine with Numeric States - Workflow Steps
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum WorkflowStep {
    Step1,
    Step2,
    Step3,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum WorkflowAction {
    Next,
    Previous,
    Fail,
    Reset,
}

#[test]
fn test_linear_workflow() {
    let mut workflow = StateMachine::new(WorkflowStep::Step1);

    workflow.add_transition(WorkflowStep::Step1, WorkflowAction::Next, WorkflowStep::Step2);
    workflow.add_transition(WorkflowStep::Step2, WorkflowAction::Next, WorkflowStep::Step3);
    workflow.add_transition(WorkflowStep::Step3, WorkflowAction::Next, WorkflowStep::Completed);
    workflow.add_transition(WorkflowStep::Step2, WorkflowAction::Previous, WorkflowStep::Step1);
    workflow.add_transition(WorkflowStep::Step3, WorkflowAction::Previous, WorkflowStep::Step2);

    // Forward progression
    assert_eq!(*workflow.current_state(), WorkflowStep::Step1);
    workflow.trigger(WorkflowAction::Next).unwrap();
    assert_eq!(*workflow.current_state(), WorkflowStep::Step2);
    workflow.trigger(WorkflowAction::Next).unwrap();
    assert_eq!(*workflow.current_state(), WorkflowStep::Step3);
    workflow.trigger(WorkflowAction::Next).unwrap();
    assert_eq!(*workflow.current_state(), WorkflowStep::Completed);
}

#[test]
fn test_workflow_with_failure() {
    let mut workflow = StateMachine::new(WorkflowStep::Step1);

    workflow.add_transition(WorkflowStep::Step1, WorkflowAction::Next, WorkflowStep::Step2);
    workflow.add_transition(WorkflowStep::Step2, WorkflowAction::Next, WorkflowStep::Step3);
    workflow.add_transition(WorkflowStep::Step1, WorkflowAction::Fail, WorkflowStep::Failed);
    workflow.add_transition(WorkflowStep::Step2, WorkflowAction::Fail, WorkflowStep::Failed);
    workflow.add_transition(WorkflowStep::Step3, WorkflowAction::Fail, WorkflowStep::Failed);
    workflow.add_transition(WorkflowStep::Failed, WorkflowAction::Reset, WorkflowStep::Step1);

    // Progress then fail
    workflow.trigger(WorkflowAction::Next).unwrap();
    assert_eq!(*workflow.current_state(), WorkflowStep::Step2);

    workflow.trigger(WorkflowAction::Fail).unwrap();
    assert_eq!(*workflow.current_state(), WorkflowStep::Failed);

    // Reset
    workflow.trigger(WorkflowAction::Reset).unwrap();
    assert_eq!(*workflow.current_state(), WorkflowStep::Step1);
}

#[test]
fn test_workflow_backward_navigation() {
    let mut workflow = StateMachine::new(WorkflowStep::Step1);

    workflow.add_transition(WorkflowStep::Step1, WorkflowAction::Next, WorkflowStep::Step2);
    workflow.add_transition(WorkflowStep::Step2, WorkflowAction::Next, WorkflowStep::Step3);
    workflow.add_transition(WorkflowStep::Step2, WorkflowAction::Previous, WorkflowStep::Step1);
    workflow.add_transition(WorkflowStep::Step3, WorkflowAction::Previous, WorkflowStep::Step2);

    // Go forward then backward
    workflow.trigger(WorkflowAction::Next).unwrap();
    workflow.trigger(WorkflowAction::Next).unwrap();
    assert_eq!(*workflow.current_state(), WorkflowStep::Step3);

    workflow.trigger(WorkflowAction::Previous).unwrap();
    assert_eq!(*workflow.current_state(), WorkflowStep::Step2);

    workflow.trigger(WorkflowAction::Previous).unwrap();
    assert_eq!(*workflow.current_state(), WorkflowStep::Step1);
}

// ============================================================================
// Event Handling - Order Processing
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OrderState {
    New,
    PaymentPending,
    Paid,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OrderEvent {
    Submit,
    Pay,
    Ship,
    Deliver,
    Cancel,
}

#[test]
fn test_order_lifecycle() {
    let mut order = StateMachine::new(OrderState::New);

    order.add_transition(OrderState::New, OrderEvent::Submit, OrderState::PaymentPending);
    order.add_transition(OrderState::PaymentPending, OrderEvent::Pay, OrderState::Paid);
    order.add_transition(OrderState::Paid, OrderEvent::Ship, OrderState::Shipped);
    order.add_transition(OrderState::Shipped, OrderEvent::Deliver, OrderState::Delivered);
    order.add_transition(OrderState::New, OrderEvent::Cancel, OrderState::Cancelled);
    order.add_transition(OrderState::PaymentPending, OrderEvent::Cancel, OrderState::Cancelled);

    // Happy path
    assert_eq!(*order.current_state(), OrderState::New);

    order.trigger(OrderEvent::Submit).unwrap();
    assert_eq!(*order.current_state(), OrderState::PaymentPending);

    order.trigger(OrderEvent::Pay).unwrap();
    assert_eq!(*order.current_state(), OrderState::Paid);

    order.trigger(OrderEvent::Ship).unwrap();
    assert_eq!(*order.current_state(), OrderState::Shipped);

    order.trigger(OrderEvent::Deliver).unwrap();
    assert_eq!(*order.current_state(), OrderState::Delivered);
}

#[test]
fn test_order_cancellation() {
    let mut order = StateMachine::new(OrderState::New);

    order.add_transition(OrderState::New, OrderEvent::Submit, OrderState::PaymentPending);
    order.add_transition(OrderState::PaymentPending, OrderEvent::Cancel, OrderState::Cancelled);

    order.trigger(OrderEvent::Submit).unwrap();
    assert_eq!(*order.current_state(), OrderState::PaymentPending);

    order.trigger(OrderEvent::Cancel).unwrap();
    assert_eq!(*order.current_state(), OrderState::Cancelled);
}

#[test]
fn test_order_invalid_operations() {
    let mut order = StateMachine::new(OrderState::New);

    order.add_transition(OrderState::New, OrderEvent::Submit, OrderState::PaymentPending);
    order.add_transition(OrderState::PaymentPending, OrderEvent::Pay, OrderState::Paid);

    // Cannot ship before submitting
    assert!(order.trigger(OrderEvent::Ship).is_err());

    // Cannot pay before submitting
    assert!(order.trigger(OrderEvent::Pay).is_err());

    order.trigger(OrderEvent::Submit).unwrap();

    // Cannot ship before paying
    assert!(order.trigger(OrderEvent::Ship).is_err());
}

#[test]
fn test_order_with_payment_guard() {
    let mut order = StateMachine::new(OrderState::PaymentPending);

    order.add_transition(OrderState::PaymentPending, OrderEvent::Pay, OrderState::Paid);

    // Guard simulates payment validation (e.g., checking funds)
    order.add_guard(OrderState::PaymentPending, OrderEvent::Pay, |current, next| {
        // Simulate payment validation logic
        *current == OrderState::PaymentPending && *next == OrderState::Paid
    });

    let result = order.trigger(OrderEvent::Pay);
    assert!(result.is_ok());
    assert_eq!(*order.current_state(), OrderState::Paid);
}

// ============================================================================
// Guard Conditions - Complex Business Logic
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AccountState {
    Active,
    Suspended,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AccountEvent {
    Suspend,
    Reactivate,
    Close,
}

#[test]
fn test_account_state_with_guards() {
    let mut account = StateMachine::new(AccountState::Active);

    account.add_transition(AccountState::Active, AccountEvent::Suspend, AccountState::Suspended);
    account.add_transition(AccountState::Suspended, AccountEvent::Reactivate, AccountState::Active);
    account.add_transition(AccountState::Active, AccountEvent::Close, AccountState::Closed);
    account.add_transition(AccountState::Suspended, AccountEvent::Close, AccountState::Closed);

    // Guard to prevent closing active accounts with outstanding balance (simulated)
    account.add_guard(AccountState::Active, AccountEvent::Close, |current, _| {
        // Simulate checking for zero balance
        *current == AccountState::Active // In real scenario, check balance == 0
    });

    // Should allow closing
    let result = account.trigger(AccountEvent::Close);
    assert!(result.is_ok());
    assert_eq!(*account.current_state(), AccountState::Closed);
}

#[test]
fn test_guard_prevents_invalid_state_change() {
    let mut account = StateMachine::new(AccountState::Active);

    account.add_transition(AccountState::Active, AccountEvent::Suspend, AccountState::Suspended);

    // Guard that checks business rules
    account.add_guard(AccountState::Active, AccountEvent::Suspend, |_, _| {
        // Simulate business rule: cannot suspend if user has active premium subscription
        false // Simulate user has premium subscription
    });

    let result = account.trigger(AccountEvent::Suspend);
    assert!(result.is_err());
    assert_eq!(*account.current_state(), AccountState::Active);
}

// ============================================================================
// State Machine Reusability - Multiple Instances
// ============================================================================

#[test]
fn test_multiple_state_machine_instances() {
    // Create multiple independent traffic light state machines
    let mut light1 = StateMachine::new(DeviceState::Off);
    let mut light2 = StateMachine::new(DeviceState::Off);

    light1.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);
    light2.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);

    // Change only light1
    light1.trigger(DeviceEvent::PowerOn).unwrap();

    assert_eq!(*light1.current_state(), DeviceState::On);
    assert_eq!(*light2.current_state(), DeviceState::Off);

    // Change light2
    light2.trigger(DeviceEvent::PowerOn).unwrap();
    assert_eq!(*light2.current_state(), DeviceState::On);
}

#[test]
fn test_state_machines_with_different_transitions() {
    let mut sm1 = StateMachine::new(WorkflowStep::Step1);
    let mut sm2: StateMachine<WorkflowStep, WorkflowAction> = StateMachine::new(WorkflowStep::Step1);

    // sm1 has linear progression
    sm1.add_transition(WorkflowStep::Step1, WorkflowAction::Next, WorkflowStep::Step2);

    // sm2 has different path - add a different transition to it
    sm2.add_transition(WorkflowStep::Step1, WorkflowAction::Fail, WorkflowStep::Failed);

    sm1.trigger(WorkflowAction::Next).unwrap();
    assert_eq!(*sm1.current_state(), WorkflowStep::Step2);

    // sm2 should still be at Step1
    assert_eq!(*sm2.current_state(), WorkflowStep::Step1);
}

// ============================================================================
// Error Handling
// ============================================================================

#[test]
fn test_error_message_format() {
    let mut sm = StateMachine::new(DeviceState::Off);
    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);

    let result = sm.trigger(DeviceEvent::PowerOff);

    match result {
        Err(StateMachineError::InvalidTransition { from, to }) => {
            assert!(!from.is_empty());
            assert_eq!(to, "unknown");
        }
        _ => panic!("Expected InvalidTransition error"),
    }
}

#[test]
fn test_guard_error_message() {
    let mut sm = StateMachine::new(AuthState::LoggedOut);
    sm.add_transition(AuthState::LoggedOut, AuthEvent::Login, AuthState::LoggedIn);
    sm.add_guard(AuthState::LoggedOut, AuthEvent::Login, |_, _| false);

    let result = sm.trigger(AuthEvent::Login);

    match result {
        Err(StateMachineError::GuardRejected(msg)) => {
            assert!(!msg.is_empty());
            assert!(msg.contains("rejected"));
        }
        _ => panic!("Expected GuardRejected error"),
    }
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_self_transition() {
    let mut sm = StateMachine::new(DeviceState::On);

    // Add transition from On to On (refresh/reload action)
    sm.add_transition(DeviceState::On, DeviceEvent::PowerOn, DeviceState::On);

    sm.trigger(DeviceEvent::PowerOn).unwrap();
    assert_eq!(*sm.current_state(), DeviceState::On);
}

#[test]
fn test_same_event_different_sources() {
    let mut player = StateMachine::new(PlayerState::Stopped);

    // Same event (Stop) from different states goes to Stopped
    player.add_transition(PlayerState::Stopped, PlayerEvent::Play, PlayerState::Playing);
    player.add_transition(PlayerState::Playing, PlayerEvent::Pause, PlayerState::Paused);
    player.add_transition(PlayerState::Playing, PlayerEvent::Stop, PlayerState::Stopped);
    player.add_transition(PlayerState::Paused, PlayerEvent::Stop, PlayerState::Stopped);

    // From Playing
    player.trigger(PlayerEvent::Play).unwrap();
    assert_eq!(*player.current_state(), PlayerState::Playing);
    player.trigger(PlayerEvent::Stop).unwrap();
    assert_eq!(*player.current_state(), PlayerState::Stopped);

    // From Paused
    player.trigger(PlayerEvent::Play).unwrap();
    player.trigger(PlayerEvent::Pause).unwrap();
    assert_eq!(*player.current_state(), PlayerState::Paused);
    player.trigger(PlayerEvent::Stop).unwrap();
    assert_eq!(*player.current_state(), PlayerState::Stopped);
}

#[test]
fn test_overriding_transition() {
    let mut sm = StateMachine::new(DeviceState::Off);

    // Add initial transition
    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);

    // Override with same transition (should replace)
    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);

    sm.trigger(DeviceEvent::PowerOn).unwrap();
    assert_eq!(*sm.current_state(), DeviceState::On);
}

#[test]
fn test_guard_with_state_comparison() {
    let mut sm = StateMachine::new(PlayerState::Playing);

    sm.add_transition(PlayerState::Playing, PlayerEvent::Pause, PlayerState::Paused);

    // Guard that checks both current and next state
    sm.add_guard(PlayerState::Playing, PlayerEvent::Pause, |current, next| {
        *current == PlayerState::Playing && *next == PlayerState::Paused
    });

    assert!(sm.trigger(PlayerEvent::Pause).is_ok());
    assert_eq!(*sm.current_state(), PlayerState::Paused);
}

// ============================================================================
// String-based States - Document Workflow
// ============================================================================

#[test]
fn test_string_state_machine() {
    let mut doc_sm = StateMachine::new("draft".to_string());

    doc_sm.add_transition("draft".to_string(), "submit".to_string(), "review".to_string());
    doc_sm.add_transition("review".to_string(), "approve".to_string(), "published".to_string());
    doc_sm.add_transition("review".to_string(), "reject".to_string(), "draft".to_string());

    assert_eq!(*doc_sm.current_state(), "draft");

    doc_sm.trigger("submit".to_string()).unwrap();
    assert_eq!(*doc_sm.current_state(), "review");

    doc_sm.trigger("approve".to_string()).unwrap();
    assert_eq!(*doc_sm.current_state(), "published");
}

// ============================================================================
// Complex Guard Logic
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CustomState {
    name: String,
    value: i32,
}

impl CustomState {
    fn new(name: &str, value: i32) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CustomEvent {
    Increment,
    Decrement,
}

#[test]
fn test_state_machine_with_complex_state_type() {
    let initial = CustomState::new("counter", 0);
    let mut sm = StateMachine::new(initial.clone());

    let next = CustomState::new("counter", 1);
    sm.add_transition(initial.clone(), CustomEvent::Increment, next.clone());

    // Guard checks state value
    sm.add_guard(initial.clone(), CustomEvent::Increment, |current, next| {
        current.value < next.value
    });

    sm.trigger(CustomEvent::Increment).unwrap();
    assert_eq!(sm.current_state().value, 1);
}

// ============================================================================
// Entry/Exit Actions - Via Wrapper Pattern
// ============================================================================

/// Wrapper around StateMachine to track entry/exit actions
struct StateMachineWithActions<S, E>
where
    S: Clone + Eq + Hash + std::fmt::Debug,
    E: Clone + Eq + Hash,
{
    machine: StateMachine<S, E>,
    action_log: Rc<RefCell<Vec<String>>>,
}

impl<S, E> StateMachineWithActions<S, E>
where
    S: Clone + Eq + Hash + std::fmt::Debug,
    E: Clone + Eq + Hash,
{
    fn new(initial_state: S) -> Self {
        Self {
            machine: StateMachine::new(initial_state),
            action_log: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn add_transition(&mut self, from: S, event: E, to: S) {
        self.machine.add_transition(from, event, to);
    }

    fn add_guard<F>(&mut self, from: S, event: E, guard: F)
    where
        F: Fn(&S, &S) -> bool + Send + Sync + 'static,
    {
        self.machine.add_guard(from, event, guard);
    }

    fn trigger_with_actions(&mut self, event: E) -> StateMachineResult<()>
    where
        S: std::fmt::Display,
    {
        let old_state = self.machine.current_state().clone();

        // Log exit action
        self.action_log.borrow_mut().push(format!("exit: {}", old_state));

        match self.machine.trigger(event) {
            Ok(new_state) => {
                // Log entry action
                self.action_log.borrow_mut().push(format!("enter: {}", new_state));
                Ok(())
            }
            Err(e) => {
                // Rollback exit action on error
                self.action_log.borrow_mut().pop();
                Err(e)
            }
        }
    }

    fn current_state(&self) -> &S {
        self.machine.current_state()
    }

    fn get_action_log(&self) -> Vec<String> {
        self.action_log.borrow().clone()
    }
}

impl std::fmt::Display for DeviceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceState::Off => write!(f, "Off"),
            DeviceState::On => write!(f, "On"),
        }
    }
}

#[test]
fn test_entry_exit_actions() {
    let mut sm = StateMachineWithActions::new(DeviceState::Off);

    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);
    sm.add_transition(DeviceState::On, DeviceEvent::PowerOff, DeviceState::Off);

    // Trigger transitions and verify entry/exit actions
    sm.trigger_with_actions(DeviceEvent::PowerOn).unwrap();
    sm.trigger_with_actions(DeviceEvent::PowerOff).unwrap();

    let log = sm.get_action_log();
    assert_eq!(log.len(), 4);
    assert_eq!(log[0], "exit: Off");
    assert_eq!(log[1], "enter: On");
    assert_eq!(log[2], "exit: On");
    assert_eq!(log[3], "enter: Off");
}

#[test]
fn test_entry_exit_actions_on_error() {
    let mut sm = StateMachineWithActions::new(DeviceState::Off);

    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);

    // Attempt invalid transition
    let result = sm.trigger_with_actions(DeviceEvent::PowerOff);
    assert!(result.is_err());

    // No actions should be logged on error
    let log = sm.get_action_log();
    assert_eq!(log.len(), 0);
}

#[test]
fn test_entry_exit_actions_with_guard() {
    let mut sm = StateMachineWithActions::new(DeviceState::Off);

    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);
    sm.add_guard(DeviceState::Off, DeviceEvent::PowerOn, |_, _| false);

    // Guard should reject transition
    let result = sm.trigger_with_actions(DeviceEvent::PowerOn);
    assert!(result.is_err());

    // Exit action logged, but entry action not executed (rolled back)
    let log = sm.get_action_log();
    assert_eq!(log.len(), 0); // Rollback should have removed the exit action
}

// ============================================================================
// State History Tracking
// ============================================================================

struct StateMachineWithHistory<S, E>
where
    S: Clone + Eq + Hash + std::fmt::Debug,
    E: Clone + Eq + Hash,
{
    machine: StateMachine<S, E>,
    history: Vec<S>,
}

impl<S, E> StateMachineWithHistory<S, E>
where
    S: Clone + Eq + Hash + std::fmt::Debug,
    E: Clone + Eq + Hash,
{
    fn new(initial_state: S) -> Self {
        let history = vec![initial_state.clone()];
        Self {
            machine: StateMachine::new(initial_state),
            history,
        }
    }

    fn add_transition(&mut self, from: S, event: E, to: S) {
        self.machine.add_transition(from, event, to);
    }

    fn add_guard<F>(&mut self, from: S, event: E, guard: F)
    where
        F: Fn(&S, &S) -> bool + Send + Sync + 'static,
    {
        self.machine.add_guard(from, event, guard);
    }

    fn trigger(&mut self, event: E) -> StateMachineResult<&S> {
        match self.machine.trigger(event) {
            Ok(new_state) => {
                self.history.push(new_state.clone());
                Ok(new_state)
            }
            Err(e) => Err(e),
        }
    }

    fn current_state(&self) -> &S {
        self.machine.current_state()
    }

    fn get_history(&self) -> &[S] {
        &self.history
    }

    fn previous_state(&self) -> Option<&S> {
        if self.history.len() > 1 {
            self.history.get(self.history.len() - 2)
        } else {
            None
        }
    }

    fn can_rollback(&self) -> bool {
        self.history.len() > 1
    }
}

#[test]
fn test_state_history_tracking() {
    let mut sm = StateMachineWithHistory::new(DeviceState::Off);

    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);
    sm.add_transition(DeviceState::On, DeviceEvent::PowerOff, DeviceState::Off);

    // Initial state in history
    assert_eq!(sm.get_history().len(), 1);
    assert_eq!(sm.get_history()[0], DeviceState::Off);

    // Transition and check history
    sm.trigger(DeviceEvent::PowerOn).unwrap();
    assert_eq!(sm.get_history().len(), 2);
    assert_eq!(sm.get_history()[1], DeviceState::On);

    sm.trigger(DeviceEvent::PowerOff).unwrap();
    assert_eq!(sm.get_history().len(), 3);
    assert_eq!(sm.get_history()[2], DeviceState::Off);
}

#[test]
fn test_previous_state_retrieval() {
    let mut sm = StateMachineWithHistory::new(DeviceState::Off);

    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);

    // No previous state initially
    assert_eq!(sm.previous_state(), None);

    sm.trigger(DeviceEvent::PowerOn).unwrap();

    // Previous state should be Off
    assert_eq!(sm.previous_state(), Some(&DeviceState::Off));
    assert_eq!(*sm.current_state(), DeviceState::On);
}

#[test]
fn test_history_not_updated_on_error() {
    let mut sm = StateMachineWithHistory::new(DeviceState::Off);

    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);

    // Attempt invalid transition
    let result = sm.trigger(DeviceEvent::PowerOff);
    assert!(result.is_err());

    // History should remain unchanged
    assert_eq!(sm.get_history().len(), 1);
    assert_eq!(sm.get_history()[0], DeviceState::Off);
}

#[test]
fn test_complex_state_history() {
    let mut player = StateMachineWithHistory::new(PlayerState::Stopped);

    player.add_transition(PlayerState::Stopped, PlayerEvent::Play, PlayerState::Playing);
    player.add_transition(PlayerState::Playing, PlayerEvent::Pause, PlayerState::Paused);
    player.add_transition(PlayerState::Paused, PlayerEvent::Resume, PlayerState::Playing);
    player.add_transition(PlayerState::Playing, PlayerEvent::Stop, PlayerState::Stopped);

    // Execute a series of transitions
    player.trigger(PlayerEvent::Play).unwrap();
    player.trigger(PlayerEvent::Pause).unwrap();
    player.trigger(PlayerEvent::Resume).unwrap();
    player.trigger(PlayerEvent::Stop).unwrap();

    let history = player.get_history();
    assert_eq!(history.len(), 5);
    assert_eq!(history[0], PlayerState::Stopped);
    assert_eq!(history[1], PlayerState::Playing);
    assert_eq!(history[2], PlayerState::Paused);
    assert_eq!(history[3], PlayerState::Playing);
    assert_eq!(history[4], PlayerState::Stopped);
}

// ============================================================================
// Advanced Guard Conditions
// ============================================================================

#[test]
fn test_guard_with_external_state() {
    use std::sync::{Arc, Mutex};

    let balance = Arc::new(Mutex::new(100));
    let balance_clone = balance.clone();

    let mut account = StateMachine::new(AccountState::Active);
    account.add_transition(AccountState::Active, AccountEvent::Close, AccountState::Closed);

    // Guard checks external state (balance)
    account.add_guard(AccountState::Active, AccountEvent::Close, move |_, _| {
        *balance_clone.lock().unwrap() == 0
    });

    // Should fail - balance is not zero
    assert!(account.trigger(AccountEvent::Close).is_err());
    assert_eq!(*account.current_state(), AccountState::Active);

    // Set balance to zero
    *balance.lock().unwrap() = 0;

    // Create new state machine with updated guard
    let balance_clone2 = balance.clone();
    let mut account2 = StateMachine::new(AccountState::Active);
    account2.add_transition(AccountState::Active, AccountEvent::Close, AccountState::Closed);
    account2.add_guard(AccountState::Active, AccountEvent::Close, move |_, _| {
        *balance_clone2.lock().unwrap() == 0
    });

    // Should succeed now
    assert!(account2.trigger(AccountEvent::Close).is_ok());
    assert_eq!(*account2.current_state(), AccountState::Closed);
}

#[test]
fn test_guard_with_state_validation() {
    let mut workflow = StateMachine::new(WorkflowStep::Step1);

    workflow.add_transition(WorkflowStep::Step1, WorkflowAction::Next, WorkflowStep::Step2);
    workflow.add_transition(WorkflowStep::Step2, WorkflowAction::Next, WorkflowStep::Step3);

    // Guard that validates sequential progression
    workflow.add_guard(WorkflowStep::Step1, WorkflowAction::Next, |current, next| {
        matches!(
            (current, next),
            (WorkflowStep::Step1, WorkflowStep::Step2)
        )
    });

    assert!(workflow.trigger(WorkflowAction::Next).is_ok());
    assert_eq!(*workflow.current_state(), WorkflowStep::Step2);
}

#[test]
fn test_multiple_guards_evaluation() {
    use std::sync::{Arc, Mutex};

    // Test that guard is properly evaluated each time
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();

    let mut sm = StateMachine::new(DeviceState::Off);
    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);

    // Guard that passes only when counter >= 3
    sm.add_guard(DeviceState::Off, DeviceEvent::PowerOn, move |_, _| {
        let mut guard = counter_clone.lock().unwrap();
        let current = *guard;
        *guard += 1;
        current >= 3
    });

    // First 3 attempts should fail
    for _ in 0..3 {
        assert!(sm.trigger(DeviceEvent::PowerOn).is_err());
        assert_eq!(*sm.current_state(), DeviceState::Off);
    }

    // 4th attempt should succeed
    assert!(sm.trigger(DeviceEvent::PowerOn).is_ok());
    assert_eq!(*sm.current_state(), DeviceState::On);
}

// ============================================================================
// Concurrent State Machines
// ============================================================================

#[test]
fn test_independent_state_machines() {
    let mut sm1 = StateMachine::new(DeviceState::Off);
    let mut sm2 = StateMachine::new(DeviceState::Off);
    let mut sm3 = StateMachine::new(DeviceState::On);

    sm1.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);
    sm2.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);
    sm3.add_transition(DeviceState::On, DeviceEvent::PowerOff, DeviceState::Off);

    // Operate on sm1
    sm1.trigger(DeviceEvent::PowerOn).unwrap();
    assert_eq!(*sm1.current_state(), DeviceState::On);
    assert_eq!(*sm2.current_state(), DeviceState::Off);
    assert_eq!(*sm3.current_state(), DeviceState::On);

    // Operate on sm3
    sm3.trigger(DeviceEvent::PowerOff).unwrap();
    assert_eq!(*sm1.current_state(), DeviceState::On);
    assert_eq!(*sm2.current_state(), DeviceState::Off);
    assert_eq!(*sm3.current_state(), DeviceState::Off);
}

// ============================================================================
// Error Message Validation
// ============================================================================

#[test]
fn test_invalid_transition_error_contains_state_info() {
    let mut sm = StateMachine::new(PlayerState::Stopped);
    sm.add_transition(PlayerState::Stopped, PlayerEvent::Play, PlayerState::Playing);

    let result = sm.trigger(PlayerEvent::Pause);

    match result {
        Err(StateMachineError::InvalidTransition { from, to }) => {
            assert!(from.contains("Stopped"));
            assert_eq!(to, "unknown");
        }
        _ => panic!("Expected InvalidTransition error"),
    }
}

#[test]
fn test_guard_rejected_error_message() {
    let mut sm = StateMachine::new(AuthState::LoggedOut);
    sm.add_transition(AuthState::LoggedOut, AuthEvent::Login, AuthState::LoggedIn);
    sm.add_guard(AuthState::LoggedOut, AuthEvent::Login, |_, _| false);

    let result = sm.trigger(AuthEvent::Login);

    match result {
        Err(StateMachineError::GuardRejected(msg)) => {
            assert!(msg.contains("LoggedOut"));
            assert!(msg.contains("rejected"));
        }
        _ => panic!("Expected GuardRejected error"),
    }
}

// ============================================================================
// Edge Cases and Boundary Conditions
// ============================================================================

#[test]
fn test_transition_to_same_state() {
    let mut sm = StateMachine::new(DeviceState::On);

    // Self-loop: On -> On
    sm.add_transition(DeviceState::On, DeviceEvent::PowerOn, DeviceState::On);

    let initial_state = sm.current_state().clone();
    sm.trigger(DeviceEvent::PowerOn).unwrap();
    assert_eq!(*sm.current_state(), initial_state);
}

#[test]
fn test_can_trigger_without_modifying_state() {
    let mut sm = StateMachine::new(DeviceState::Off);
    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);

    // Check multiple times without triggering
    for _ in 0..10 {
        assert!(sm.can_trigger(&DeviceEvent::PowerOn));
        assert_eq!(*sm.current_state(), DeviceState::Off);
    }
}

#[test]
fn test_guard_does_not_affect_can_trigger() {
    let mut sm = StateMachine::new(DeviceState::Off);
    sm.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);
    sm.add_guard(DeviceState::Off, DeviceEvent::PowerOn, |_, _| false);

    // can_trigger only checks if transition exists, not guards
    assert!(sm.can_trigger(&DeviceEvent::PowerOn));

    // But actual trigger should fail due to guard
    assert!(sm.trigger(DeviceEvent::PowerOn).is_err());
}

#[test]
fn test_replacing_transition() {
    let mut sm = StateMachine::new(WorkflowStep::Step1);

    // Add transition to Step2
    sm.add_transition(WorkflowStep::Step1, WorkflowAction::Next, WorkflowStep::Step2);

    // Replace with transition to Completed
    sm.add_transition(WorkflowStep::Step1, WorkflowAction::Next, WorkflowStep::Completed);

    sm.trigger(WorkflowAction::Next).unwrap();
    assert_eq!(*sm.current_state(), WorkflowStep::Completed);
}

#[test]
fn test_complex_workflow_with_multiple_paths() {
    let mut workflow = StateMachine::new(WorkflowStep::Step1);

    // Normal path
    workflow.add_transition(WorkflowStep::Step1, WorkflowAction::Next, WorkflowStep::Step2);
    workflow.add_transition(WorkflowStep::Step2, WorkflowAction::Next, WorkflowStep::Step3);
    workflow.add_transition(WorkflowStep::Step3, WorkflowAction::Next, WorkflowStep::Completed);

    // Failure paths
    workflow.add_transition(WorkflowStep::Step1, WorkflowAction::Fail, WorkflowStep::Failed);
    workflow.add_transition(WorkflowStep::Step2, WorkflowAction::Fail, WorkflowStep::Failed);
    workflow.add_transition(WorkflowStep::Step3, WorkflowAction::Fail, WorkflowStep::Failed);

    // Reset from failed
    workflow.add_transition(WorkflowStep::Failed, WorkflowAction::Reset, WorkflowStep::Step1);

    // Test normal path
    workflow.trigger(WorkflowAction::Next).unwrap();
    assert_eq!(*workflow.current_state(), WorkflowStep::Step2);

    // Test failure path
    workflow.trigger(WorkflowAction::Fail).unwrap();
    assert_eq!(*workflow.current_state(), WorkflowStep::Failed);

    // Test reset
    workflow.trigger(WorkflowAction::Reset).unwrap();
    assert_eq!(*workflow.current_state(), WorkflowStep::Step1);
}

// ============================================================================
// State Machine Lifecycle Tests
// ============================================================================

#[test]
fn test_state_machine_reuse() {
    let mut sm = StateMachine::new(OrderState::New);

    sm.add_transition(OrderState::New, OrderEvent::Submit, OrderState::PaymentPending);
    sm.add_transition(OrderState::PaymentPending, OrderEvent::Pay, OrderState::Paid);
    sm.add_transition(OrderState::Paid, OrderEvent::Ship, OrderState::Shipped);
    sm.add_transition(OrderState::Shipped, OrderEvent::Deliver, OrderState::Delivered);

    // Complete first order
    sm.trigger(OrderEvent::Submit).unwrap();
    sm.trigger(OrderEvent::Pay).unwrap();
    sm.trigger(OrderEvent::Ship).unwrap();
    sm.trigger(OrderEvent::Deliver).unwrap();
    assert_eq!(*sm.current_state(), OrderState::Delivered);

    // State machine stays in Delivered - cannot process another order
    // (would need to reset to New state or create new instance)
    assert!(sm.trigger(OrderEvent::Submit).is_err());
}

#[test]
fn test_numeric_state_values() {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum Level {
        L0,
        L1,
        L2,
        L3,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum LevelEvent {
        Up,
        Down,
    }

    let mut sm = StateMachine::new(Level::L0);

    sm.add_transition(Level::L0, LevelEvent::Up, Level::L1);
    sm.add_transition(Level::L1, LevelEvent::Up, Level::L2);
    sm.add_transition(Level::L2, LevelEvent::Up, Level::L3);
    sm.add_transition(Level::L1, LevelEvent::Down, Level::L0);
    sm.add_transition(Level::L2, LevelEvent::Down, Level::L1);
    sm.add_transition(Level::L3, LevelEvent::Down, Level::L2);

    // Climb up
    for expected in [Level::L1, Level::L2, Level::L3] {
        sm.trigger(LevelEvent::Up).unwrap();
        assert_eq!(*sm.current_state(), expected);
    }

    // Climb down
    for expected in [Level::L2, Level::L1, Level::L0] {
        sm.trigger(LevelEvent::Down).unwrap();
        assert_eq!(*sm.current_state(), expected);
    }
}

// ============================================================================
// Documentation Examples Validation
// ============================================================================

#[test]
fn test_readme_example_device() {
    // Validate that example from documentation works correctly
    let mut device = StateMachine::new(DeviceState::Off);

    device.add_transition(DeviceState::Off, DeviceEvent::PowerOn, DeviceState::On);
    device.add_transition(DeviceState::On, DeviceEvent::PowerOff, DeviceState::Off);

    assert_eq!(*device.current_state(), DeviceState::Off);
    assert!(device.can_trigger(&DeviceEvent::PowerOn));

    device.trigger(DeviceEvent::PowerOn).unwrap();
    assert_eq!(*device.current_state(), DeviceState::On);
}

#[test]
fn test_readme_example_with_guard() {
    // Validate guard example from documentation
    let mut auth = StateMachine::new(AuthState::LoggedOut);

    auth.add_transition(AuthState::LoggedOut, AuthEvent::Login, AuthState::LoggedIn);
    auth.add_guard(AuthState::LoggedOut, AuthEvent::Login, |current, next| {
        *current == AuthState::LoggedOut && *next == AuthState::LoggedIn
    });

    assert!(auth.trigger(AuthEvent::Login).is_ok());
    assert_eq!(*auth.current_state(), AuthState::LoggedIn);
}
