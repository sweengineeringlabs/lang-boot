//! Traffic Light State Machine Example
//!
//! This example demonstrates a traffic light control system with:
//! - Simple state transitions
//! - Timed transitions (simulated)
//! - Safety guards to prevent invalid transitions
//! - Integration with external context (traffic sensors)

use dev_engineeringlabs_rustboot_state_machine::StateMachine;
use std::time::{Duration, Instant};

/// Traffic light states
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LightState {
    Red,
    RedYellow,    // European-style: Red + Yellow before Green
    Green,
    Yellow,
}

/// Events that trigger state changes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LightEvent {
    TimerExpired,
    EmergencyStop,
    Resume,
}

/// Traffic light controller with timing and sensor integration
struct TrafficLight {
    state_machine: StateMachine<LightState, LightEvent>,
    state_start_time: Instant,
    emergency_mode: bool,
    vehicle_count: u32,
}

impl TrafficLight {
    fn new() -> Self {
        let mut sm = StateMachine::new(LightState::Red);

        // Define the traffic light cycle transitions
        // Red -> Red+Yellow -> Green -> Yellow -> Red
        sm.add_transition(LightState::Red, LightEvent::TimerExpired, LightState::RedYellow);
        sm.add_transition(LightState::RedYellow, LightEvent::TimerExpired, LightState::Green);
        sm.add_transition(LightState::Green, LightEvent::TimerExpired, LightState::Yellow);
        sm.add_transition(LightState::Yellow, LightEvent::TimerExpired, LightState::Red);

        // Emergency transitions: any state can go to Red
        sm.add_transition(LightState::Green, LightEvent::EmergencyStop, LightState::Red);
        sm.add_transition(LightState::Yellow, LightEvent::EmergencyStop, LightState::Red);
        sm.add_transition(LightState::RedYellow, LightEvent::EmergencyStop, LightState::Red);

        // Resume from emergency
        sm.add_transition(LightState::Red, LightEvent::Resume, LightState::RedYellow);

        // Add guards for safety
        sm.add_guard(LightState::Red, LightEvent::TimerExpired, |_, _| {
            // Safety guard: ensure minimum red light duration
            true // In real system, would check if enough time has passed
        });

        sm.add_guard(LightState::Green, LightEvent::TimerExpired, |_, _| {
            // Safety guard: ensure minimum green light duration
            true // In real system, would check timing
        });

        Self {
            state_machine: sm,
            state_start_time: Instant::now(),
            emergency_mode: false,
            vehicle_count: 0,
        }
    }

    fn current_state(&self) -> &LightState {
        self.state_machine.current_state()
    }

    fn time_in_current_state(&self) -> Duration {
        self.state_start_time.elapsed()
    }

    fn trigger(&mut self, event: LightEvent) -> Result<(), String> {
        let previous_state = self.current_state().clone();

        match self.state_machine.trigger(event.clone()) {
            Ok(new_state) => {
                let new_state_clone = new_state.clone();
                println!("  State change: {:?} -> {:?}", previous_state, new_state);

                // Reset timer when state changes
                self.state_start_time = Instant::now();

                // Handle entry actions for new state
                self.on_enter_state(&new_state_clone);

                Ok(())
            }
            Err(e) => {
                Err(format!("Transition failed: {}", e))
            }
        }
    }

    fn on_enter_state(&mut self, state: &LightState) {
        match state {
            LightState::Red => {
                println!("  🔴 RED LIGHT - Stop!");
                self.emergency_mode = false;
            }
            LightState::RedYellow => {
                println!("  🔴🟡 RED + YELLOW - Prepare to go");
            }
            LightState::Green => {
                println!("  🟢 GREEN LIGHT - Go!");
                self.vehicle_count = 0;
            }
            LightState::Yellow => {
                println!("  🟡 YELLOW LIGHT - Caution, prepare to stop");
            }
        }
    }

    fn emergency_stop(&mut self) {
        println!("\n⚠️  EMERGENCY DETECTED - Switching to RED");
        self.emergency_mode = true;
        let _ = self.trigger(LightEvent::EmergencyStop);
    }

    fn resume_normal(&mut self) {
        if self.emergency_mode {
            println!("\n✓ Emergency cleared - Resuming normal operation");
            let _ = self.trigger(LightEvent::Resume);
        }
    }

    fn detect_vehicle(&mut self) {
        self.vehicle_count += 1;
        println!("  🚗 Vehicle detected (count: {})", self.vehicle_count);
    }

    fn print_status(&self) {
        println!("\n{}", "=".repeat(50));
        println!("Traffic Light Status");
        println!("{}", "=".repeat(50));
        println!("Current State: {:?}", self.current_state());
        println!("Time in state: {:.1}s", self.time_in_current_state().as_secs_f32());
        println!("Emergency Mode: {}", self.emergency_mode);
        println!("Vehicles passed: {}", self.vehicle_count);
        println!("{}", "=".repeat(50));
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║   Traffic Light State Machine Example           ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    let mut light = TrafficLight::new();
    light.print_status();

    // Simulate a complete traffic light cycle
    println!("\n--- Simulating Normal Traffic Light Cycle ---");

    println!("\nPhase 1: Starting from Red");
    std::thread::sleep(Duration::from_millis(500));
    light.trigger(LightEvent::TimerExpired).unwrap();

    println!("\nPhase 2: Red + Yellow transition");
    std::thread::sleep(Duration::from_millis(500));
    light.trigger(LightEvent::TimerExpired).unwrap();

    println!("\nPhase 3: Green light - vehicles can pass");
    std::thread::sleep(Duration::from_millis(200));
    light.detect_vehicle();
    std::thread::sleep(Duration::from_millis(200));
    light.detect_vehicle();
    std::thread::sleep(Duration::from_millis(200));
    light.detect_vehicle();
    std::thread::sleep(Duration::from_millis(400));

    println!("\nPhase 4: Yellow light - prepare to stop");
    light.trigger(LightEvent::TimerExpired).unwrap();
    std::thread::sleep(Duration::from_millis(500));

    println!("\nPhase 5: Back to Red");
    light.trigger(LightEvent::TimerExpired).unwrap();

    light.print_status();

    // Simulate emergency scenario
    println!("\n\n--- Simulating Emergency Scenario ---");

    println!("\nStarting new cycle...");
    light.trigger(LightEvent::TimerExpired).unwrap(); // Red -> RedYellow
    std::thread::sleep(Duration::from_millis(300));

    light.trigger(LightEvent::TimerExpired).unwrap(); // RedYellow -> Green
    std::thread::sleep(Duration::from_millis(300));

    // Emergency occurs during green light
    light.emergency_stop();
    light.print_status();

    std::thread::sleep(Duration::from_millis(500));

    // Resume normal operation
    light.resume_normal();
    light.print_status();

    // Continue cycle
    println!("\nContinuing normal cycle...");
    std::thread::sleep(Duration::from_millis(300));
    light.trigger(LightEvent::TimerExpired).unwrap(); // RedYellow -> Green

    light.print_status();

    // Invalid transition attempt
    println!("\n\n--- Demonstrating Invalid Transition ---");
    println!("\nAttempting to go directly from Green to Red (invalid without emergency):");

    match light.trigger(LightEvent::EmergencyStop) {
        Ok(_) => println!("  Note: EmergencyStop is valid from any state"),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\nAttempting TimerExpired from Red (should go to RedYellow):");
    match light.trigger(LightEvent::TimerExpired) {
        Ok(_) => println!("  Success: Normal transition"),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\nAttempting Resume when not in emergency:");
    match light.trigger(LightEvent::Resume) {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Expected error: {}", e),
    }

    println!("\n\n{}", "═".repeat(50));
    println!("Example demonstrates:");
    println!("  ✓ Cyclic state transitions (traffic light pattern)");
    println!("  ✓ Emergency interrupts to safe state");
    println!("  ✓ Resume from emergency");
    println!("  ✓ Time tracking in each state");
    println!("  ✓ External context (vehicle detection)");
    println!("  ✓ Guard conditions for safety");
    println!("  ✓ Invalid transition handling");
    println!("{}", "═".repeat(50));
}
