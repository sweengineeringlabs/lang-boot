//! Message broker abstraction with acknowledgment support.
//!
//! This module provides traits and types for working with message brokers
//! that support reliable delivery with acknowledgments.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{Message, MessagingError, MessagingResult};

/// Message delivery mode for reliable messaging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryMode {
    /// At most once delivery - fire and forget.
    AtMostOnce,
    /// At least once delivery - requires acknowledgment.
    AtLeastOnce,
    /// Exactly once delivery - requires deduplication.
    ExactlyOnce,
}

/// Message acknowledgment handle.
#[async_trait]
pub trait MessageAck: Send {
    /// Acknowledge message processing succeeded.
    async fn ack(&mut self) -> MessagingResult<()>;

    /// Negative acknowledgment - message processing failed.
    /// The message may be redelivered.
    async fn nack(&mut self) -> MessagingResult<()>;

    /// Reject message - will not be redelivered.
    async fn reject(&mut self) -> MessagingResult<()>;
}

/// Message with acknowledgment support.
pub struct AckMessage {
    /// The message content.
    pub message: Message,
    /// Acknowledgment handle (optional for at-most-once delivery).
    pub ack_handle: Option<Box<dyn MessageAck>>,
}

impl AckMessage {
    /// Create a new acknowledgable message.
    pub fn new(message: Message, ack_handle: Option<Box<dyn MessageAck>>) -> Self {
        Self {
            message,
            ack_handle,
        }
    }

    /// Acknowledge the message.
    pub async fn ack(mut self) -> MessagingResult<()> {
        if let Some(ref mut handle) = self.ack_handle {
            handle.ack().await
        } else {
            Ok(())
        }
    }

    /// Negative acknowledge the message.
    pub async fn nack(mut self) -> MessagingResult<()> {
        if let Some(ref mut handle) = self.ack_handle {
            handle.nack().await
        } else {
            Ok(())
        }
    }

    /// Reject the message.
    pub async fn reject(mut self) -> MessagingResult<()> {
        if let Some(ref mut handle) = self.ack_handle {
            handle.reject().await
        } else {
            Ok(())
        }
    }
}

/// Stream of acknowledgable messages.
#[async_trait]
pub trait AckMessageStream: Send {
    /// Get next message with acknowledgment support.
    async fn next(&mut self) -> Option<AckMessage>;
}

/// Message broker with queue support and acknowledgments.
#[async_trait]
pub trait MessageBroker: Send + Sync {
    /// Publish a message to a topic/exchange.
    async fn publish(&self, message: Message) -> MessagingResult<()>;

    /// Publish with JSON payload.
    async fn publish_json<T: Serialize + Send + Sync>(
        &self,
        topic: &str,
        payload: &T,
    ) -> MessagingResult<()> {
        let data = serde_json::to_vec(payload)
            .map_err(|e| MessagingError::Serialization(e.to_string()))?;
        let message = Message::new(topic, data);
        self.publish(message).await
    }

    /// Subscribe to a topic with pub/sub pattern.
    async fn subscribe(&self, topic: &str) -> MessagingResult<Box<dyn AckMessageStream>>;

    /// Consume from a message queue (multiple consumers compete for messages).
    async fn consume(&self, queue: &str) -> MessagingResult<Box<dyn AckMessageStream>>;
}

/// Configuration for broker connections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerConfig {
    /// Connection URL (e.g., redis://localhost, amqp://localhost).
    pub url: String,
    /// Default delivery mode.
    pub delivery_mode: DeliveryMode,
    /// Connection timeout in milliseconds.
    pub timeout_ms: u64,
    /// Enable automatic reconnection.
    pub auto_reconnect: bool,
}

impl Default for BrokerConfig {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            delivery_mode: DeliveryMode::AtLeastOnce,
            timeout_ms: 5000,
            auto_reconnect: true,
        }
    }
}

impl BrokerConfig {
    /// Create a new broker configuration.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    /// Set delivery mode.
    pub fn with_delivery_mode(mut self, mode: DeliveryMode) -> Self {
        self.delivery_mode = mode;
        self
    }

    /// Set connection timeout.
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Set auto-reconnect.
    pub fn with_auto_reconnect(mut self, enabled: bool) -> Self {
        self.auto_reconnect = enabled;
        self
    }
}
