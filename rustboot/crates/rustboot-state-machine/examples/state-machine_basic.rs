//! Basic State Machine Example - Order Processing System
//!
//! This example demonstrates all key features of the rustboot state machine:
//! 1. Defining states and events
//! 2. Configuring transitions
//! 3. Adding guards/conditions
//! 4. Entry/exit actions
//! 5. Running the state machine with context
//!
//! Scenario: An e-commerce order lifecycle from creation to completion

use dev_engineeringlabs_rustboot_state_machine::{StateMachine, StateMachineError};

/// Order states representing the lifecycle of an order
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OrderState {
    /// Order created but not yet confirmed
    Pending,
    /// Payment has been processed
    Paid,
    /// Order is being prepared for shipment
    Processing,
    /// Order has been shipped to customer
    Shipped,
    /// Order successfully delivered
    Delivered,
    /// Order cancelled by customer or system
    Cancelled,
}

/// Events that trigger state transitions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OrderEvent {
    /// Customer completes payment
    Pay,
    /// Begin processing the order
    Process,
    /// Ship the order to customer
    Ship,
    /// Confirm delivery to customer
    Deliver,
    /// Cancel the order
    Cancel,
}

/// Order context containing business data
#[derive(Debug, Clone)]
struct OrderContext {
    order_id: String,
    customer_name: String,
    total_amount: f64,
    payment_received: bool,
    inventory_reserved: bool,
    tracking_number: Option<String>,
}

impl OrderContext {
    fn new(order_id: String, customer_name: String, total_amount: f64) -> Self {
        Self {
            order_id,
            customer_name,
            total_amount,
            payment_received: false,
            inventory_reserved: false,
            tracking_number: None,
        }
    }

    fn process_payment(&mut self) {
        self.payment_received = true;
        println!("  💳 Payment of ${:.2} processed successfully", self.total_amount);
    }

    fn reserve_inventory(&mut self) {
        self.inventory_reserved = true;
        println!("  📦 Inventory reserved for order {}", self.order_id);
    }

    fn release_inventory(&mut self) {
        self.inventory_reserved = false;
        println!("  ↩️  Inventory released for order {}", self.order_id);
    }

    fn generate_tracking(&mut self) {
        self.tracking_number = Some(format!("TRACK-{}-{}", self.order_id, chrono::Utc::now().timestamp()));
        println!("  🚚 Tracking number generated: {}", self.tracking_number.as_ref().unwrap());
    }
}

/// Order processor that wraps the state machine with business logic
struct OrderProcessor {
    state_machine: StateMachine<OrderState, OrderEvent>,
    context: OrderContext,
    transition_count: usize,
}

impl OrderProcessor {
    fn new(order_id: String, customer_name: String, total_amount: f64) -> Self {
        let mut sm = StateMachine::new(OrderState::Pending);

        // 2. CONFIGURE TRANSITIONS - Define valid state transitions
        // Happy path: Pending -> Paid -> Processing -> Shipped -> Delivered
        sm.add_transition(OrderState::Pending, OrderEvent::Pay, OrderState::Paid);
        sm.add_transition(OrderState::Paid, OrderEvent::Process, OrderState::Processing);
        sm.add_transition(OrderState::Processing, OrderEvent::Ship, OrderState::Shipped);
        sm.add_transition(OrderState::Shipped, OrderEvent::Deliver, OrderState::Delivered);

        // Cancellation paths - can cancel at any time (but guard will prevent after shipping)
        sm.add_transition(OrderState::Pending, OrderEvent::Cancel, OrderState::Cancelled);
        sm.add_transition(OrderState::Paid, OrderEvent::Cancel, OrderState::Cancelled);
        sm.add_transition(OrderState::Processing, OrderEvent::Cancel, OrderState::Cancelled);
        sm.add_transition(OrderState::Shipped, OrderEvent::Cancel, OrderState::Cancelled);

        // 3. ADD GUARDS/CONDITIONS - Business rules that must be satisfied

        // Guard: Can only pay if amount is positive
        sm.add_guard(OrderState::Pending, OrderEvent::Pay, |_from, _to| {
            println!("  🔍 Guard: Validating payment amount...");
            // In real system, would check payment gateway, fraud detection, etc.
            true
        });

        // Guard: Can only process if payment is verified
        sm.add_guard(OrderState::Paid, OrderEvent::Process, |_from, _to| {
            println!("  🔍 Guard: Verifying inventory availability...");
            // In real system, would check inventory levels
            true
        });

        // Guard: Can only ship if items are packed
        sm.add_guard(OrderState::Processing, OrderEvent::Ship, |_from, _to| {
            println!("  🔍 Guard: Confirming items are packed and ready...");
            // In real system, would verify warehouse confirmation
            true
        });

        // Guard: Cannot cancel after shipping
        sm.add_guard(OrderState::Shipped, OrderEvent::Cancel, |_from, _to| {
            println!("  🔍 Guard: Checking if cancellation is allowed...");
            false // Cannot cancel after shipping
        });

        Self {
            state_machine: sm,
            context: OrderContext::new(order_id, customer_name, total_amount),
            transition_count: 0,
        }
    }

    fn current_state(&self) -> &OrderState {
        self.state_machine.current_state()
    }

    fn can_trigger(&self, event: &OrderEvent) -> bool {
        self.state_machine.can_trigger(event)
    }

    /// 5. RUNNING THE STATE MACHINE - Execute state transitions with full lifecycle
    fn trigger(&mut self, event: OrderEvent) -> Result<(), StateMachineError> {
        println!("\n▶️  Triggering event: {:?}", event);

        let from_state = self.current_state().clone();

        // 4. ENTRY ACTIONS - Execute before state transition
        self.execute_entry_action(&event);

        // Execute the state transition
        match self.state_machine.trigger(event.clone()) {
            Ok(to_state) => {
                let to_state_clone = to_state.clone();
                self.transition_count += 1;

                println!("  ✅ Transition successful: {:?} → {:?}", from_state, to_state);

                // 4. EXIT ACTIONS - Execute after state transition
                self.execute_exit_action(&from_state, &to_state_clone);

                Ok(())
            }
            Err(e) => {
                println!("  ❌ Transition failed: {}", e);
                Err(e)
            }
        }
    }

    /// Execute actions when entering a state (triggered by event)
    fn execute_entry_action(&mut self, event: &OrderEvent) {
        match event {
            OrderEvent::Pay => {
                self.context.process_payment();
            }
            OrderEvent::Process => {
                self.context.reserve_inventory();
            }
            OrderEvent::Ship => {
                self.context.generate_tracking();
            }
            OrderEvent::Deliver => {
                println!("  📧 Sending delivery confirmation email to {}", self.context.customer_name);
            }
            OrderEvent::Cancel => {
                if self.context.inventory_reserved {
                    self.context.release_inventory();
                }
                println!("  🔔 Sending cancellation notification to {}", self.context.customer_name);
            }
        }
    }

    /// Execute actions when exiting a state (after successful transition)
    fn execute_exit_action(&self, from: &OrderState, to: &OrderState) {
        // Log the state transition for audit trail
        println!("  📝 Logging: Order {} transitioned from {:?} to {:?}",
                 self.context.order_id, from, to);

        // Notify relevant systems
        match to {
            OrderState::Paid => println!("  📨 Notification sent to warehouse"),
            OrderState::Shipped => println!("  📨 Notification sent to customer with tracking"),
            OrderState::Delivered => println!("  📨 Notification sent to accounting for final processing"),
            OrderState::Cancelled => println!("  📨 Notification sent to refund system"),
            _ => {}
        }
    }

    fn print_status(&self) {
        println!("\n{}", "═".repeat(70));
        println!("📊 ORDER STATUS");
        println!("{}", "═".repeat(70));
        println!("Order ID:          {}", self.context.order_id);
        println!("Customer:          {}", self.context.customer_name);
        println!("Current State:     {:?}", self.current_state());
        println!("Total Amount:      ${:.2}", self.context.total_amount);
        println!("Payment Received:  {}", if self.context.payment_received { "✓" } else { "✗" });
        println!("Inventory Reserved: {}", if self.context.inventory_reserved { "✓" } else { "✗" });
        if let Some(tracking) = &self.context.tracking_number {
            println!("Tracking Number:   {}", tracking);
        }
        println!("Transitions Made:  {}", self.transition_count);
        println!("{}", "═".repeat(70));
    }

    fn print_available_actions(&self) {
        println!("\n📋 Available actions from {:?} state:", self.current_state());

        let all_events = vec![
            OrderEvent::Pay,
            OrderEvent::Process,
            OrderEvent::Ship,
            OrderEvent::Deliver,
            OrderEvent::Cancel,
        ];

        for event in all_events {
            if self.can_trigger(&event) {
                println!("  ✓ {:?}", event);
            }
        }
    }
}

// Chrono is used for timestamp generation
mod chrono {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub struct Utc;

    impl Utc {
        pub fn now() -> DateTime {
            DateTime
        }
    }

    pub struct DateTime;

    impl DateTime {
        pub fn timestamp(&self) -> u64 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        }
    }
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("║        Rustboot State Machine - Order Processing Example          ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");

    // Create a new order
    let mut order = OrderProcessor::new(
        "ORD-2024-12345".to_string(),
        "Alice Johnson".to_string(),
        249.99,
    );

    order.print_status();
    order.print_available_actions();

    // SCENARIO 1: Successful Order Flow
    println!("\n\n{}", "▬".repeat(70));
    println!("🎯 SCENARIO 1: Successful Order Flow");
    println!("{}", "▬".repeat(70));

    // Step 1: Customer pays for the order
    order.trigger(OrderEvent::Pay).unwrap();
    order.print_status();

    // Step 2: Start processing the order
    order.trigger(OrderEvent::Process).unwrap();

    // Step 3: Ship the order
    order.trigger(OrderEvent::Ship).unwrap();
    order.print_status();

    // Step 4: Confirm delivery
    order.trigger(OrderEvent::Deliver).unwrap();
    order.print_status();

    // SCENARIO 2: Invalid Transition
    println!("\n\n{}", "▬".repeat(70));
    println!("🎯 SCENARIO 2: Invalid Transition - Already Delivered");
    println!("{}", "▬".repeat(70));

    println!("\nAttempting to ship an already delivered order (should fail):");
    match order.trigger(OrderEvent::Ship) {
        Ok(_) => println!("  ⚠️  Unexpected success!"),
        Err(e) => println!("  ✓ Expected error: {}", e),
    }

    // SCENARIO 3: Order Cancellation
    println!("\n\n{}", "▬".repeat(70));
    println!("🎯 SCENARIO 3: Order Cancellation Before Shipping");
    println!("{}", "▬".repeat(70));

    let mut order2 = OrderProcessor::new(
        "ORD-2024-67890".to_string(),
        "Bob Smith".to_string(),
        149.99,
    );

    order2.print_status();

    // Customer pays
    order2.trigger(OrderEvent::Pay).unwrap();

    // Start processing
    order2.trigger(OrderEvent::Process).unwrap();

    // Customer changes mind and cancels
    println!("\nCustomer decides to cancel before shipping...");
    order2.trigger(OrderEvent::Cancel).unwrap();

    order2.print_status();

    // SCENARIO 4: Guard Rejection
    println!("\n\n{}", "▬".repeat(70));
    println!("🎯 SCENARIO 4: Guard Rejection - Cannot Cancel After Shipping");
    println!("{}", "▬".repeat(70));

    let mut order3 = OrderProcessor::new(
        "ORD-2024-11111".to_string(),
        "Charlie Davis".to_string(),
        399.99,
    );

    // Complete the order flow to shipped state
    order3.trigger(OrderEvent::Pay).unwrap();
    order3.trigger(OrderEvent::Process).unwrap();
    order3.trigger(OrderEvent::Ship).unwrap();

    println!("\nAttempting to cancel after shipping (should be blocked by guard):");
    match order3.trigger(OrderEvent::Cancel) {
        Ok(_) => println!("  ⚠️  Unexpected success!"),
        Err(e) => println!("  ✓ Expected guard rejection: {}", e),
    }

    order3.print_status();

    // SCENARIO 5: Checking Available Transitions
    println!("\n\n{}", "▬".repeat(70));
    println!("🎯 SCENARIO 5: Checking Available Transitions");
    println!("{}", "▬".repeat(70));

    let mut order4 = OrderProcessor::new(
        "ORD-2024-22222".to_string(),
        "Diana Prince".to_string(),
        99.99,
    );

    println!("\nInitial state - Pending:");
    order4.print_available_actions();

    order4.trigger(OrderEvent::Pay).unwrap();
    println!("\nAfter payment - Paid:");
    order4.print_available_actions();

    order4.trigger(OrderEvent::Process).unwrap();
    println!("\nDuring processing - Processing:");
    order4.print_available_actions();

    order4.trigger(OrderEvent::Ship).unwrap();
    println!("\nAfter shipping - Shipped:");
    order4.print_available_actions();

    // Summary
    println!("\n\n{}", "═".repeat(70));
    println!("✨ EXAMPLE COMPLETE - All Features Demonstrated:");
    println!("{}", "═".repeat(70));
    println!("1. ✓ Defining states (OrderState enum) and events (OrderEvent enum)");
    println!("2. ✓ Configuring transitions (add_transition)");
    println!("3. ✓ Adding guards/conditions (add_guard for business rules)");
    println!("4. ✓ Entry/exit actions (execute_entry_action, execute_exit_action)");
    println!("5. ✓ Running the state machine (trigger method with full lifecycle)");
    println!("{}", "═".repeat(70));
    println!("\nAdditional features demonstrated:");
    println!("  • Context management alongside state transitions");
    println!("  • Error handling for invalid transitions");
    println!("  • Guard rejection for business rule violations");
    println!("  • Checking available transitions (can_trigger)");
    println!("  • Multiple workflow scenarios (success, cancellation, errors)");
    println!("{}", "═".repeat(70));
    println!();
}
