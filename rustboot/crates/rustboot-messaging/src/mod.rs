//! Messaging framework (L4: Core).
//!
//! Publish/subscribe pattern for event-driven architecture.

pub mod bus;

// Re-export main types
pub use bus::{InMemoryBus, Message, MessageStream, MessagingError, MessagingResult, Publisher, Subscriber};
