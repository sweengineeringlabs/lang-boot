//! Message queue abstraction (L4: Core - Messaging).
//!
//! Publish/subscribe pattern for message passing.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Message queue error.
#[derive(Debug, thiserror::Error)]
pub enum MessagingError {
    /// Publishing error.
    #[error("Publish error: {0}")]
    Publish(String),
    
    /// Subscription error.
    #[error("Subscription error: {0}")]
    Subscription(String),
    
    /// Serialization error.
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Result type for messaging operations.
pub type MessagingResult<T> = Result<T, MessagingError>;

/// Message with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message ID.
    pub id: String,
    /// Topic.
    pub topic: String,
    /// Payload.
    pub payload: Vec<u8>,
    /// Timestamp.
    pub timestamp: std::time::SystemTime,
}

impl Message {
    /// Create a new message.
    pub fn new(topic: impl Into<String>, payload: Vec<u8>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            topic: topic.into(),
            payload,
            timestamp: std::time::SystemTime::now(),
        }
    }
    
    /// Deserialize payload.
    pub fn deserialize<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.payload)
    }
}

/// Message publisher.
#[async_trait]
pub trait Publisher: Send + Sync {
    /// Publish a message.
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
}

/// Message subscriber.
#[async_trait]
pub trait Subscriber: Send + Sync {
    /// Subscribe to a topic.
    async fn subscribe(&self, topic: &str) -> MessagingResult<Box<dyn MessageStream>>;
}

/// Message stream.
#[async_trait]
pub trait MessageStream: Send {
    /// Get next message.
    async fn next(&mut self) -> Option<Message>;
}

/// Type alias for the subscriber map.
type SubscriberMap = std::collections::HashMap<String, Vec<tokio::sync::mpsc::UnboundedSender<Message>>>;

/// In-memory message bus.
pub struct InMemoryBus {
    subscribers: std::sync::Arc<std::sync::RwLock<SubscriberMap>>,
}

impl InMemoryBus {
    /// Create a new in-memory bus.
    pub fn new() -> Self {
        Self {
            subscribers: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
}

impl Default for InMemoryBus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Publisher for InMemoryBus {
    async fn publish(&self, message: Message) -> MessagingResult<()> {
        let subscribers = self.subscribers.read().unwrap();
        
        if let Some(subs) = subscribers.get(&message.topic) {
            for sub in subs {
                let _ = sub.send(message.clone());
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl Subscriber for InMemoryBus {
    async fn subscribe(&self, topic: &str) -> MessagingResult<Box<dyn MessageStream>> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        
        let mut subscribers = self.subscribers.write().unwrap();
        subscribers.entry(topic.to_string())
            .or_default()
            .push(tx);
        
        Ok(Box::new(InMemoryStream { rx }))
    }
}

struct InMemoryStream {
    rx: tokio::sync::mpsc::UnboundedReceiver<Message>,
}

#[async_trait]
impl MessageStream for InMemoryStream {
    async fn next(&mut self) -> Option<Message> {
        self.rx.recv().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_bus() {
        let bus = InMemoryBus::new();

        let mut stream = bus.subscribe("test").await.unwrap();

        let message = Message::new("test", b"hello".to_vec());
        bus.publish(message.clone()).await.unwrap();

        let received = stream.next().await.unwrap();
        assert_eq!(received.topic, "test");
        assert_eq!(received.payload, b"hello");
    }

    #[tokio::test]
    async fn test_multiple_subscribers_same_topic() {
        let bus = InMemoryBus::new();

        let mut stream1 = bus.subscribe("events").await.unwrap();
        let mut stream2 = bus.subscribe("events").await.unwrap();

        let message = Message::new("events", b"broadcast".to_vec());
        bus.publish(message).await.unwrap();

        let received1 = stream1.next().await.unwrap();
        let received2 = stream2.next().await.unwrap();

        assert_eq!(received1.payload, b"broadcast");
        assert_eq!(received2.payload, b"broadcast");
    }

    #[tokio::test]
    async fn test_multiple_topics() {
        let bus = InMemoryBus::new();

        let mut stream_a = bus.subscribe("topic_a").await.unwrap();
        let mut stream_b = bus.subscribe("topic_b").await.unwrap();

        bus.publish(Message::new("topic_a", b"msg_a".to_vec())).await.unwrap();
        bus.publish(Message::new("topic_b", b"msg_b".to_vec())).await.unwrap();

        let received_a = stream_a.next().await.unwrap();
        let received_b = stream_b.next().await.unwrap();

        assert_eq!(received_a.topic, "topic_a");
        assert_eq!(received_a.payload, b"msg_a");
        assert_eq!(received_b.topic, "topic_b");
        assert_eq!(received_b.payload, b"msg_b");
    }

    #[tokio::test]
    async fn test_publish_to_nonexistent_topic() {
        let bus = InMemoryBus::new();

        // Should not error when publishing to topic with no subscribers
        let result = bus.publish(Message::new("nobody_listening", b"data".to_vec())).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_publish_json() {
        let bus = InMemoryBus::new();

        let mut stream = bus.subscribe("json_topic").await.unwrap();

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestPayload {
            name: String,
            value: i32,
        }

        let payload = TestPayload { name: "test".to_string(), value: 42 };
        bus.publish_json("json_topic", &payload).await.unwrap();

        let received = stream.next().await.unwrap();
        let deserialized: TestPayload = received.deserialize().unwrap();

        assert_eq!(deserialized, payload);
    }

    #[test]
    fn test_message_new() {
        let msg = Message::new("topic", b"payload".to_vec());

        assert_eq!(msg.topic, "topic");
        assert_eq!(msg.payload, b"payload");
        assert!(!msg.id.is_empty());
    }

    #[test]
    fn test_bus_default() {
        let bus = InMemoryBus::default();
        let subscribers = bus.subscribers.read().unwrap();
        assert!(subscribers.is_empty());
    }
}
