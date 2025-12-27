//! Order Processing State Machine Example
//!
//! This example demonstrates a comprehensive state machine implementation
//! for an e-commerce order processing system. It shows:
//! - Defining states and events
//! - Creating a state machine with transitions
//! - Using guard conditions for business logic
//! - Handling state transitions and errors
//! - A realistic use case with context data

use dev_engineeringlabs_rustboot_state_machine::{StateMachine, StateMachineError};

/// Order states in the processing lifecycle
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OrderState {
    /// Order created but not yet submitted
    Draft,
    /// Order submitted and awaiting payment
    Pending,
    /// Payment received, awaiting confirmation
    PaymentReceived,
    /// Payment confirmed, order ready for processing
    Confirmed,
    /// Order is being prepared/packed
    Processing,
    /// Order has been shipped
    Shipped,
    /// Order delivered to customer
    Delivered,
    /// Order cancelled
    Cancelled,
    /// Order returned by customer
    Returned,
    /// Order refunded
    Refunded,
}

/// Events that trigger state transitions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OrderEvent {
    /// Customer submits the order
    Submit,
    /// Payment is received
    ReceivePayment,
    /// Payment is confirmed by payment processor
    ConfirmPayment,
    /// Payment confirmation fails
    RejectPayment,
    /// Start processing the order
    StartProcessing,
    /// Ship the order
    Ship,
    /// Confirm delivery
    ConfirmDelivery,
    /// Customer cancels the order
    Cancel,
    /// Customer initiates a return
    InitiateReturn,
    /// Process refund
    ProcessRefund,
}

/// Order context with business data
#[derive(Debug, Clone)]
struct OrderContext {
    order_id: String,
    customer_id: String,
    total_amount: f64,
    items_count: usize,
    payment_verified: bool,
    inventory_reserved: bool,
    shipping_address: Option<String>,
}

impl OrderContext {
    fn new(order_id: String, customer_id: String, total_amount: f64, items_count: usize) -> Self {
        Self {
            order_id,
            customer_id,
            total_amount,
            items_count,
            payment_verified: false,
            inventory_reserved: false,
            shipping_address: None,
        }
    }

    fn set_shipping_address(&mut self, address: String) {
        self.shipping_address = Some(address);
    }

    fn verify_payment(&mut self) {
        self.payment_verified = true;
    }

    fn reserve_inventory(&mut self) {
        self.inventory_reserved = true;
    }
}

/// Order processor that manages state machine and context
struct OrderProcessor {
    state_machine: StateMachine<OrderState, OrderEvent>,
    context: OrderContext,
    event_history: Vec<(OrderEvent, OrderState)>,
}

impl OrderProcessor {
    fn new(order_id: String, customer_id: String, total_amount: f64, items_count: usize) -> Self {
        let mut sm = StateMachine::new(OrderState::Draft);

        // Define all valid state transitions
        Self::setup_transitions(&mut sm);

        // Add guard conditions
        Self::setup_guards(&mut sm);

        Self {
            state_machine: sm,
            context: OrderContext::new(order_id, customer_id, total_amount, items_count),
            event_history: Vec::new(),
        }
    }

    fn setup_transitions(sm: &mut StateMachine<OrderState, OrderEvent>) {
        // Draft -> Pending (submit order)
        sm.add_transition(OrderState::Draft, OrderEvent::Submit, OrderState::Pending);

        // Pending -> PaymentReceived (receive payment)
        sm.add_transition(OrderState::Pending, OrderEvent::ReceivePayment, OrderState::PaymentReceived);

        // PaymentReceived -> Confirmed (confirm payment)
        sm.add_transition(OrderState::PaymentReceived, OrderEvent::ConfirmPayment, OrderState::Confirmed);

        // PaymentReceived -> Pending (reject payment, retry)
        sm.add_transition(OrderState::PaymentReceived, OrderEvent::RejectPayment, OrderState::Pending);

        // Confirmed -> Processing (start processing)
        sm.add_transition(OrderState::Confirmed, OrderEvent::StartProcessing, OrderState::Processing);

        // Processing -> Shipped (ship order)
        sm.add_transition(OrderState::Processing, OrderEvent::Ship, OrderState::Shipped);

        // Shipped -> Delivered (confirm delivery)
        sm.add_transition(OrderState::Shipped, OrderEvent::ConfirmDelivery, OrderState::Delivered);

        // Cancellation paths
        sm.add_transition(OrderState::Draft, OrderEvent::Cancel, OrderState::Cancelled);
        sm.add_transition(OrderState::Pending, OrderEvent::Cancel, OrderState::Cancelled);
        sm.add_transition(OrderState::PaymentReceived, OrderEvent::Cancel, OrderState::Cancelled);
        sm.add_transition(OrderState::Confirmed, OrderEvent::Cancel, OrderState::Cancelled);

        // Return and refund paths
        sm.add_transition(OrderState::Delivered, OrderEvent::InitiateReturn, OrderState::Returned);
        sm.add_transition(OrderState::Returned, OrderEvent::ProcessRefund, OrderState::Refunded);
        sm.add_transition(OrderState::Cancelled, OrderEvent::ProcessRefund, OrderState::Refunded);
    }

    fn setup_guards(sm: &mut StateMachine<OrderState, OrderEvent>) {
        // Guard: Can only submit if we have valid order data
        // In a real system, this would check inventory, pricing, etc.
        sm.add_guard(OrderState::Draft, OrderEvent::Submit, |_, _| {
            // Always allow for this example
            true
        });

        // Guard: Can only confirm payment if payment was actually verified
        // This would integrate with payment processor in real system
        sm.add_guard(OrderState::PaymentReceived, OrderEvent::ConfirmPayment, |_, _| {
            // Always allow for this example - in reality would check payment status
            true
        });

        // Guard: Can only start processing if inventory is available
        sm.add_guard(OrderState::Confirmed, OrderEvent::StartProcessing, |_, _| {
            // Always allow for this example - in reality would check inventory
            true
        });

        // Guard: Can only ship if items are packed and ready
        sm.add_guard(OrderState::Processing, OrderEvent::Ship, |_, _| {
            // Always allow for this example - in reality would verify packing complete
            true
        });
    }

    fn current_state(&self) -> &OrderState {
        self.state_machine.current_state()
    }

    fn can_trigger(&self, event: &OrderEvent) -> bool {
        self.state_machine.can_trigger(event)
    }

    fn trigger(&mut self, event: OrderEvent) -> Result<(), StateMachineError> {
        println!("\n>>> Triggering event: {:?}", event);

        // Perform side effects based on event (entry actions)
        self.perform_entry_actions(&event);

        // Get current state before transition
        let previous_state = self.current_state().clone();

        // Attempt state transition
        match self.state_machine.trigger(event.clone()) {
            Ok(new_state) => {
                let new_state_clone = new_state.clone();
                println!("✓ Transition successful: {:?} -> {:?}", previous_state, new_state);

                // Perform exit actions from previous state
                self.perform_exit_actions(&previous_state, &new_state_clone);

                // Record in history
                self.event_history.push((event, new_state_clone));

                Ok(())
            }
            Err(e) => {
                println!("✗ Transition failed: {}", e);
                Err(e)
            }
        }
    }

    fn perform_entry_actions(&mut self, event: &OrderEvent) {
        match event {
            OrderEvent::Submit => {
                println!("  → Setting shipping address...");
                self.context.set_shipping_address("123 Main St, City, State 12345".to_string());
            }
            OrderEvent::ReceivePayment => {
                println!("  → Processing payment of ${:.2}...", self.context.total_amount);
            }
            OrderEvent::ConfirmPayment => {
                println!("  → Verifying payment...");
                self.context.verify_payment();
            }
            OrderEvent::StartProcessing => {
                println!("  → Reserving inventory for {} items...", self.context.items_count);
                self.context.reserve_inventory();
            }
            OrderEvent::Ship => {
                println!("  → Generating shipping label...");
                println!("  → Notifying customer about shipment...");
            }
            OrderEvent::ConfirmDelivery => {
                println!("  → Sending delivery confirmation email...");
            }
            OrderEvent::Cancel => {
                println!("  → Releasing reserved inventory...");
                self.context.inventory_reserved = false;
            }
            OrderEvent::InitiateReturn => {
                println!("  → Creating return authorization...");
            }
            OrderEvent::ProcessRefund => {
                println!("  → Processing refund of ${:.2}...", self.context.total_amount);
            }
            OrderEvent::RejectPayment => {
                println!("  → Payment verification failed, resetting...");
            }
        }
    }

    fn perform_exit_actions(&self, from_state: &OrderState, to_state: &OrderState) {
        println!("  → Exiting state: {:?}", from_state);
        println!("  → Entering state: {:?}", to_state);

        // Log state change to audit trail
        println!("  → Logged state change in audit trail");
    }

    fn print_status(&self) {
        println!("\n{}", "=".repeat(60));
        println!("Order Status Report");
        println!("{}", "=".repeat(60));
        println!("Order ID: {}", self.context.order_id);
        println!("Customer ID: {}", self.context.customer_id);
        println!("Current State: {:?}", self.current_state());
        println!("Total Amount: ${:.2}", self.context.total_amount);
        println!("Items Count: {}", self.context.items_count);
        println!("Payment Verified: {}", self.context.payment_verified);
        println!("Inventory Reserved: {}", self.context.inventory_reserved);
        if let Some(addr) = &self.context.shipping_address {
            println!("Shipping Address: {}", addr);
        }
        println!("{}", "=".repeat(60));
    }

    fn print_history(&self) {
        println!("\nEvent History:");
        println!("{}", "-".repeat(60));
        for (i, (event, state)) in self.event_history.iter().enumerate() {
            println!("{}. {:?} -> {:?}", i + 1, event, state);
        }
        println!("{}", "-".repeat(60));
    }

    fn print_available_actions(&self) {
        println!("\nAvailable Actions from {:?}:", self.current_state());
        let events = vec![
            OrderEvent::Submit,
            OrderEvent::ReceivePayment,
            OrderEvent::ConfirmPayment,
            OrderEvent::RejectPayment,
            OrderEvent::StartProcessing,
            OrderEvent::Ship,
            OrderEvent::ConfirmDelivery,
            OrderEvent::Cancel,
            OrderEvent::InitiateReturn,
            OrderEvent::ProcessRefund,
        ];

        for event in events {
            if self.can_trigger(&event) {
                println!("  ✓ {:?}", event);
            }
        }
    }
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║   Rustboot State Machine - Order Processing Example       ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    // Create a new order
    let mut order = OrderProcessor::new(
        "ORD-2024-001".to_string(),
        "CUST-12345".to_string(),
        299.99,
        3,
    );

    order.print_status();
    order.print_available_actions();

    // Scenario 1: Successful order flow
    println!("\n\n{}", "█".repeat(60));
    println!("SCENARIO 1: Successful Order Flow");
    println!("{}", "█".repeat(60));

    // Submit order
    order.trigger(OrderEvent::Submit).unwrap();
    order.print_status();

    // Receive payment
    order.trigger(OrderEvent::ReceivePayment).unwrap();

    // Confirm payment
    order.trigger(OrderEvent::ConfirmPayment).unwrap();
    order.print_status();

    // Start processing
    order.trigger(OrderEvent::StartProcessing).unwrap();

    // Ship order
    order.trigger(OrderEvent::Ship).unwrap();

    // Confirm delivery
    order.trigger(OrderEvent::ConfirmDelivery).unwrap();

    order.print_status();
    order.print_history();

    // Scenario 2: Invalid transition attempt
    println!("\n\n{}", "█".repeat(60));
    println!("SCENARIO 2: Invalid Transition Attempt");
    println!("{}", "█".repeat(60));

    println!("\nAttempting to ship an already delivered order (should fail):");
    match order.trigger(OrderEvent::Ship) {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Expected error: {}", e),
    }

    // Scenario 3: Return and refund flow
    println!("\n\n{}", "█".repeat(60));
    println!("SCENARIO 3: Return and Refund Flow");
    println!("{}", "█".repeat(60));

    println!("\nCustomer initiates return...");
    order.trigger(OrderEvent::InitiateReturn).unwrap();
    order.print_status();

    println!("\nProcessing refund...");
    order.trigger(OrderEvent::ProcessRefund).unwrap();

    order.print_status();
    order.print_history();

    // Scenario 4: Cancellation flow
    println!("\n\n{}", "█".repeat(60));
    println!("SCENARIO 4: Order Cancellation");
    println!("{}", "█".repeat(60));

    let mut order2 = OrderProcessor::new(
        "ORD-2024-002".to_string(),
        "CUST-67890".to_string(),
        149.99,
        2,
    );

    order2.trigger(OrderEvent::Submit).unwrap();
    order2.trigger(OrderEvent::ReceivePayment).unwrap();

    println!("\nCustomer decides to cancel before payment confirmation...");
    order2.trigger(OrderEvent::Cancel).unwrap();

    order2.print_status();
    order2.print_history();

    // Scenario 5: Payment rejection and retry
    println!("\n\n{}", "█".repeat(60));
    println!("SCENARIO 5: Payment Rejection and Retry");
    println!("{}", "█".repeat(60));

    let mut order3 = OrderProcessor::new(
        "ORD-2024-003".to_string(),
        "CUST-11111".to_string(),
        499.99,
        5,
    );

    order3.trigger(OrderEvent::Submit).unwrap();
    order3.trigger(OrderEvent::ReceivePayment).unwrap();

    println!("\nPayment processor rejects the payment...");
    order3.trigger(OrderEvent::RejectPayment).unwrap();
    order3.print_status();

    println!("\nCustomer retries with different payment method...");
    order3.trigger(OrderEvent::ReceivePayment).unwrap();
    order3.trigger(OrderEvent::ConfirmPayment).unwrap();
    order3.trigger(OrderEvent::StartProcessing).unwrap();

    order3.print_status();
    order3.print_history();

    // Summary
    println!("\n\n{}{}{}", "╔", "═".repeat(58), "╗");
    println!("║{:^58}║", "Example Complete");
    println!("{}{}{}", "╚", "═".repeat(58), "╝");

    println!("\nThis example demonstrated:");
    println!("  ✓ Defining custom states and events");
    println!("  ✓ Setting up state machine transitions");
    println!("  ✓ Using guard conditions for validation");
    println!("  ✓ Handling entry and exit actions");
    println!("  ✓ Error handling for invalid transitions");
    println!("  ✓ Multiple workflow scenarios (success, failure, cancellation)");
    println!("  ✓ Context management alongside state transitions");
    println!("  ✓ Event history tracking");
    println!();
}
