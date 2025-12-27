//! Redis-based message broker implementation.
//!
//! Supports both pub/sub and list-based queues with acknowledgments.

use async_trait::async_trait;
use futures::StreamExt;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::broker::{AckMessage, AckMessageStream, BrokerConfig, DeliveryMode, MessageAck, MessageBroker};
use crate::{Message, MessagingError, MessagingResult};

/// Redis message broker.
pub struct RedisBroker {
    client: Client,
    config: BrokerConfig,
}

impl RedisBroker {
    /// Create a new Redis broker.
    pub async fn new(config: BrokerConfig) -> MessagingResult<Self> {
        let client = Client::open(config.url.as_str())
            .map_err(|e| MessagingError::Subscription(format!("Failed to create Redis client: {}", e)))?;

        // Test connection
        let mut conn = client
            .get_connection_manager()
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to connect to Redis: {}", e)))?;

        // Ping to verify connection
        let _: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .map_err(|e| MessagingError::Subscription(format!("Redis connection test failed: {}", e)))?;

        Ok(Self { client, config })
    }

    /// Create with default configuration.
    pub async fn connect(url: impl Into<String>) -> MessagingResult<Self> {
        Self::new(BrokerConfig::new(url)).await
    }

    async fn get_connection(&self) -> MessagingResult<ConnectionManager> {
        self.client
            .get_connection_manager()
            .await
            .map_err(|e| MessagingError::Publish(format!("Failed to get Redis connection: {}", e)))
    }

    /// Serialize message to JSON.
    fn serialize_message(message: &Message) -> MessagingResult<String> {
        serde_json::to_string(message)
            .map_err(|e| MessagingError::Serialization(format!("Failed to serialize message: {}", e)))
    }

    /// Deserialize message from JSON.
    fn deserialize_message(data: &str) -> MessagingResult<Message> {
        serde_json::from_str(data)
            .map_err(|e| MessagingError::Serialization(format!("Failed to deserialize message: {}", e)))
    }
}

#[async_trait]
impl MessageBroker for RedisBroker {
    async fn publish(&self, message: Message) -> MessagingResult<()> {
        let mut conn = self.get_connection().await?;
        let data = Self::serialize_message(&message)?;

        // Publish to pub/sub channel
        conn.publish::<_, _, ()>(&message.topic, &data)
            .await
            .map_err(|e| MessagingError::Publish(format!("Failed to publish to Redis: {}", e)))?;

        Ok(())
    }

    async fn subscribe(&self, topic: &str) -> MessagingResult<Box<dyn AckMessageStream>> {
        let mut pubsub = self
            .client
            .get_async_pubsub()
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to create pub/sub: {}", e)))?;

        pubsub
            .subscribe(topic)
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to subscribe to topic: {}", e)))?;

        Ok(Box::new(RedisPubSubStream {
            pubsub,
        }))
    }

    async fn consume(&self, queue: &str) -> MessagingResult<Box<dyn AckMessageStream>> {
        let conn = self.get_connection().await?;

        Ok(Box::new(RedisQueueStream {
            conn,
            queue: queue.to_string(),
            processing_queue: format!("{}:processing", queue),
            delivery_mode: self.config.delivery_mode,
        }))
    }
}

/// Redis pub/sub stream (no acknowledgments needed).
struct RedisPubSubStream {
    pubsub: redis::aio::PubSub,
}

#[async_trait]
impl AckMessageStream for RedisPubSubStream {
    async fn next(&mut self) -> Option<AckMessage> {
        loop {
            let msg = self.pubsub.on_message().next().await?;
            let payload: String = match msg.get_payload() {
                Ok(p) => p,
                Err(_) => continue,
            };

            match RedisBroker::deserialize_message(&payload) {
                Ok(message) => {
                    // Pub/sub doesn't support acks
                    return Some(AckMessage::new(message, None));
                }
                Err(_) => continue,
            }
        }
    }
}

/// Redis list-based queue stream with acknowledgments.
struct RedisQueueStream {
    conn: ConnectionManager,
    queue: String,
    processing_queue: String,
    delivery_mode: DeliveryMode,
}

#[async_trait]
impl AckMessageStream for RedisQueueStream {
    async fn next(&mut self) -> Option<AckMessage> {
        loop {
            // Use BRPOPLPUSH for reliable queue consumption
            // Move from main queue to processing queue atomically
            let result: Result<Option<String>, _> = redis::cmd("BRPOPLPUSH")
                .arg(&self.queue)
                .arg(&self.processing_queue)
                .arg(0) // Block indefinitely
                .query_async(&mut self.conn)
                .await;

            let payload = match result {
                Ok(Some(p)) => p,
                Ok(None) => continue,
                Err(_) => continue,
            };

            match RedisBroker::deserialize_message(&payload) {
                Ok(message) => {
                    let ack_handle = if self.delivery_mode != DeliveryMode::AtMostOnce {
                        Some(Box::new(RedisAck {
                            conn: Arc::new(Mutex::new(self.conn.clone())),
                            processing_queue: self.processing_queue.clone(),
                            main_queue: self.queue.clone(),
                            message_data: payload,
                        }) as Box<dyn MessageAck>)
                    } else {
                        None
                    };

                    return Some(AckMessage::new(message, ack_handle));
                }
                Err(_) => continue,
            }
        }
    }
}

/// Redis acknowledgment handle.
struct RedisAck {
    conn: Arc<Mutex<ConnectionManager>>,
    processing_queue: String,
    main_queue: String,
    message_data: String,
}

#[async_trait]
impl MessageAck for RedisAck {
    async fn ack(&mut self) -> MessagingResult<()> {
        let mut conn = self.conn.lock().await;

        // Remove from processing queue
        conn.lrem::<_, _, ()>(&self.processing_queue, 1, &self.message_data)
            .await
            .map_err(|e| MessagingError::Publish(format!("Failed to ack message: {}", e)))?;

        Ok(())
    }

    async fn nack(&mut self) -> MessagingResult<()> {
        let mut conn = self.conn.lock().await;

        // Move back to main queue for redelivery
        let _: () = redis::pipe()
            .atomic()
            .lrem(&self.processing_queue, 1, &self.message_data)
            .ignore()
            .lpush(&self.main_queue, &self.message_data)
            .ignore()
            .query_async(&mut *conn)
            .await
            .map_err(|e| MessagingError::Publish(format!("Failed to nack message: {}", e)))?;

        Ok(())
    }

    async fn reject(&mut self) -> MessagingResult<()> {
        let mut conn = self.conn.lock().await;

        // Remove from processing queue without redelivery
        conn.lrem::<_, _, ()>(&self.processing_queue, 1, &self.message_data)
            .await
            .map_err(|e| MessagingError::Publish(format!("Failed to reject message: {}", e)))?;

        Ok(())
    }
}

/// Helper to push messages to a Redis queue.
pub async fn push_to_queue(
    conn: &mut ConnectionManager,
    queue: &str,
    message: &Message,
) -> MessagingResult<()> {
    let data = RedisBroker::serialize_message(message)?;

    conn.lpush::<_, _, ()>(queue, data)
        .await
        .map_err(|e| MessagingError::Publish(format!("Failed to push to queue: {}", e)))?;

    Ok(())
}

/// Extension trait for Redis-specific queue operations.
#[async_trait]
pub trait RedisQueueExt {
    /// Push a message to a queue (for use with consume()).
    async fn push_to_queue(&self, queue: &str, message: Message) -> MessagingResult<()>;
}

#[async_trait]
impl RedisQueueExt for RedisBroker {
    async fn push_to_queue(&self, queue: &str, message: Message) -> MessagingResult<()> {
        let mut conn = self.get_connection().await?;
        push_to_queue(&mut conn, queue, &message).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires Redis server running
    async fn test_redis_pubsub() {
        let broker = RedisBroker::connect("redis://127.0.0.1:6379")
            .await
            .expect("Failed to connect to Redis");

        let mut stream = broker
            .subscribe("test_topic")
            .await
            .expect("Failed to subscribe");

        let message = Message::new("test_topic", b"Hello Redis".to_vec());
        broker.publish(message.clone()).await.expect("Failed to publish");

        let received = tokio::time::timeout(std::time::Duration::from_secs(1), stream.next())
            .await
            .expect("Timeout")
            .expect("No message received");

        assert_eq!(received.message.topic, "test_topic");
        assert_eq!(received.message.payload, b"Hello Redis");
    }

    #[tokio::test]
    #[ignore] // Requires Redis server running
    async fn test_redis_queue_with_ack() {
        let broker = RedisBroker::connect("redis://127.0.0.1:6379")
            .await
            .expect("Failed to connect to Redis");

        let queue = "test_queue";
        let message = Message::new(queue, b"Queue message".to_vec());

        broker
            .push_to_queue(queue, message.clone())
            .await
            .expect("Failed to push to queue");

        let mut stream = broker.consume(queue).await.expect("Failed to consume");

        let received = tokio::time::timeout(std::time::Duration::from_secs(1), stream.next())
            .await
            .expect("Timeout")
            .expect("No message received");

        assert_eq!(received.message.payload, b"Queue message");

        // Acknowledge the message
        received.ack().await.expect("Failed to ack");
    }

    #[tokio::test]
    #[ignore] // Requires Redis server running
    async fn test_redis_queue_nack_redelivery() {
        let broker = RedisBroker::connect("redis://127.0.0.1:6379")
            .await
            .expect("Failed to connect to Redis");

        let queue = "test_nack_queue";
        let message = Message::new(queue, b"Nack test".to_vec());

        broker
            .push_to_queue(queue, message.clone())
            .await
            .expect("Failed to push to queue");

        let mut stream = broker.consume(queue).await.expect("Failed to consume");

        // First delivery
        let received1 = tokio::time::timeout(std::time::Duration::from_secs(1), stream.next())
            .await
            .expect("Timeout")
            .expect("No message received");

        // Nack to trigger redelivery
        received1.nack().await.expect("Failed to nack");

        // Second delivery (redelivered)
        let received2 = tokio::time::timeout(std::time::Duration::from_secs(1), stream.next())
            .await
            .expect("Timeout")
            .expect("No message received");

        assert_eq!(received2.message.payload, b"Nack test");
        received2.ack().await.expect("Failed to ack");
    }
}
