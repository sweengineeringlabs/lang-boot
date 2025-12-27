//! State machine visualization utilities.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

/// State machine visualizer for text-based diagrams.
pub struct StateMachineVisualizer<S, E>
where
    S: Clone + Eq + Hash + fmt::Debug,
    E: Clone + Eq + Hash + fmt::Debug,
{
    current_state: Option<S>,
    transitions: HashMap<(S, E), S>,
    state_descriptions: HashMap<S, String>,
}

impl<S, E> StateMachineVisualizer<S, E>
where
    S: Clone + Eq + Hash + fmt::Debug,
    E: Clone + Eq + Hash + fmt::Debug,
{
    /// Create a new visualizer.
    pub fn new() -> Self {
        Self {
            current_state: None,
            transitions: HashMap::new(),
            state_descriptions: HashMap::new(),
        }
    }

    /// Set the current state (will be highlighted in visualization).
    pub fn set_current_state(&mut self, state: S) {
        self.current_state = Some(state);
    }

    /// Add a transition to visualize.
    pub fn add_transition(&mut self, from: S, event: E, to: S) {
        self.transitions.insert((from, event), to);
    }

    /// Add a description for a state.
    pub fn add_state_description(&mut self, state: S, description: impl Into<String>) {
        self.state_descriptions.insert(state, description.into());
    }

    /// Generate a text-based state diagram.
    pub fn generate_diagram(&self) -> String {
        let mut output = String::new();
        output.push_str("State Machine Diagram\n");
        output.push_str("====================\n\n");

        // Collect all states
        let mut states = HashSet::new();
        for ((from, _), to) in &self.transitions {
            states.insert(from);
            states.insert(to);
        }

        // Show current state
        if let Some(ref current) = self.current_state {
            output.push_str(&format!("Current State: {:?} (*)\n\n", current));
        }

        // List all states
        output.push_str("States:\n");
        for state in &states {
            let marker = if Some(*state) == self.current_state.as_ref() {
                " (*)"
            } else {
                ""
            };

            if let Some(description) = self.state_descriptions.get(state) {
                output.push_str(&format!("  {:?}{} - {}\n", state, marker, description));
            } else {
                output.push_str(&format!("  {:?}{}\n", state, marker));
            }
        }

        output.push_str("\nTransitions:\n");

        // Group transitions by source state
        let mut transitions_by_state: HashMap<&S, Vec<(&E, &S)>> = HashMap::new();
        for ((from, event), to) in &self.transitions {
            transitions_by_state
                .entry(from)
                .or_default()
                .push((event, to));
        }

        for (from, transitions) in transitions_by_state {
            output.push_str(&format!("  {:?}\n", from));
            for (event, to) in transitions {
                output.push_str(&format!("    --[ {:?} ]--> {:?}\n", event, to));
            }
        }

        output
    }

    /// Generate a simplified ASCII art diagram.
    pub fn generate_ascii_diagram(&self) -> String {
        let mut output = String::new();

        // Collect all states
        let mut states: Vec<&S> = HashSet::<&S>::new()
            .into_iter()
            .chain(
                self.transitions
                    .iter()
                    .flat_map(|((from, _), to)| vec![from, to]),
            )
            .collect();

        // Sort states for consistent output
        states.sort_by_key(|s| format!("{:?}", s));

        for ((from, event), to) in &self.transitions {
            let from_marker = if Some(from) == self.current_state.as_ref() {
                "(*)"
            } else {
                ""
            };

            let to_marker = if Some(to) == self.current_state.as_ref() {
                "(*)"
            } else {
                ""
            };

            output.push_str(&format!(
                "[{:?}]{} --{:?}--> [{:?}]{}\n",
                from, from_marker, event, to, to_marker
            ));
        }

        output
    }

    /// Generate a DOT graph format (for Graphviz).
    pub fn generate_dot_graph(&self) -> String {
        let mut output = String::new();
        output.push_str("digraph StateMachine {\n");
        output.push_str("  rankdir=LR;\n");
        output.push_str("  node [shape=circle];\n\n");

        // Highlight current state
        if let Some(ref current) = self.current_state {
            output.push_str(&format!(
                "  \"{:?}\" [style=filled, fillcolor=lightblue];\n",
                current
            ));
        }

        // Add transitions
        for ((from, event), to) in &self.transitions {
            output.push_str(&format!(
                "  \"{:?}\" -> \"{:?}\" [label=\"{:?}\"];\n",
                from, to, event
            ));
        }

        output.push_str("}\n");
        output
    }

    /// Generate a markdown table of transitions.
    pub fn generate_markdown_table(&self) -> String {
        let mut output = String::new();
        output.push_str("| From State | Event | To State |\n");
        output.push_str("|------------|-------|----------|\n");

        let mut transitions: Vec<_> = self.transitions.iter().collect();
        transitions.sort_by_key(|((from, _), _)| format!("{:?}", from));

        for ((from, event), to) in transitions {
            let from_str = format!("{:?}", from);
            let event_str = format!("{:?}", event);
            let to_str = format!("{:?}", to);

            output.push_str(&format!("| {} | {} | {} |\n", from_str, event_str, to_str));
        }

        output
    }
}

impl<S, E> Default for StateMachineVisualizer<S, E>
where
    S: Clone + Eq + Hash + fmt::Debug,
    E: Clone + Eq + Hash + fmt::Debug,
{
    fn default() -> Self {
        Self::new()
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
    fn test_visualizer_basic() {
        let mut viz = StateMachineVisualizer::new();
        viz.set_current_state(State::Idle);
        viz.add_transition(State::Idle, Event::Start, State::Running);
        viz.add_transition(State::Running, Event::Pause, State::Paused);
        viz.add_transition(State::Paused, Event::Resume, State::Running);
        viz.add_transition(State::Running, Event::Stop, State::Stopped);

        let diagram = viz.generate_diagram();
        assert!(diagram.contains("State Machine Diagram"));
        assert!(diagram.contains("Idle"));
        assert!(diagram.contains("(*)"));
    }

    #[test]
    fn test_ascii_diagram() {
        let mut viz = StateMachineVisualizer::new();
        viz.set_current_state(State::Running);
        viz.add_transition(State::Idle, Event::Start, State::Running);
        viz.add_transition(State::Running, Event::Stop, State::Stopped);

        let diagram = viz.generate_ascii_diagram();
        assert!(diagram.contains("Idle"));
        assert!(diagram.contains("Running"));
        assert!(diagram.contains("(*)"));
    }

    #[test]
    fn test_dot_graph() {
        let mut viz = StateMachineVisualizer::new();
        viz.add_transition(State::Idle, Event::Start, State::Running);

        let dot = viz.generate_dot_graph();
        assert!(dot.contains("digraph StateMachine"));
        assert!(dot.contains("->"));
    }

    #[test]
    fn test_markdown_table() {
        let mut viz = StateMachineVisualizer::new();
        viz.add_transition(State::Idle, Event::Start, State::Running);
        viz.add_transition(State::Running, Event::Pause, State::Paused);

        let markdown = viz.generate_markdown_table();
        assert!(markdown.contains("| From State | Event | To State |"));
        assert!(markdown.contains("Idle"));
        assert!(markdown.contains("Start"));
    }

    #[test]
    fn test_state_descriptions() {
        let mut viz = StateMachineVisualizer::new();
        viz.add_state_description(State::Idle, "Waiting for work");
        viz.add_state_description(State::Running, "Processing tasks");
        viz.add_transition(State::Idle, Event::Start, State::Running);

        let diagram = viz.generate_diagram();
        assert!(diagram.contains("Waiting for work"));
        assert!(diagram.contains("Processing tasks"));
    }
}
