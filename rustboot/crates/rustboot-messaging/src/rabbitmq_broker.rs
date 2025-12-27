//! RabbitMQ (AMQP) message broker implementation.
//!
//! Supports exchanges, queues, routing, and message acknowledgments.

use async_trait::async_trait;
use futures::StreamExt;
use lapin::{
    options::*,
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, Consumer,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::broker::{AckMessage, AckMessageStream, BrokerConfig, DeliveryMode, MessageAck, MessageBroker};
use crate::{Message, MessagingError, MessagingResult};

/// RabbitMQ message broker.
pub struct RabbitMQBroker {
    connection: Connection,
    channel: Arc<Mutex<Channel>>,
    config: BrokerConfig,
}

impl RabbitMQBroker {
    /// Create a new RabbitMQ broker.
    pub async fn new(config: BrokerConfig) -> MessagingResult<Self> {
        let connection = Connection::connect(&config.url, ConnectionProperties::default())
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to connect to RabbitMQ: {}", e)))?;

        let channel = connection
            .create_channel()
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to create channel: {}", e)))?;

        Ok(Self {
            connection,
            channel: Arc::new(Mutex::new(channel)),
            config,
        })
    }

    /// Create with default configuration.
    pub async fn connect(url: impl Into<String>) -> MessagingResult<Self> {
        Self::new(BrokerConfig::new(url)).await
    }

    /// Serialize message to JSON.
    fn serialize_message(message: &Message) -> MessagingResult<Vec<u8>> {
        serde_json::to_vec(message)
            .map_err(|e| MessagingError::Serialization(format!("Failed to serialize message: {}", e)))
    }

    /// Deserialize message from JSON.
    fn deserialize_message(data: &[u8]) -> MessagingResult<Message> {
        serde_json::from_slice(data)
            .map_err(|e| MessagingError::Serialization(format!("Failed to deserialize message: {}", e)))
    }

    /// Declare an exchange.
    pub async fn declare_exchange(
        &self,
        name: &str,
        exchange_type: &str,
        durable: bool,
    ) -> MessagingResult<()> {
        let channel = self.channel.lock().await;

        let kind = match exchange_type {
            "fanout" => lapin::ExchangeKind::Fanout,
            "direct" => lapin::ExchangeKind::Direct,
            "topic" => lapin::ExchangeKind::Topic,
            "headers" => lapin::ExchangeKind::Headers,
            _ => lapin::ExchangeKind::Custom(exchange_type.to_string()),
        };

        channel
            .exchange_declare(
                name,
                kind,
                ExchangeDeclareOptions {
                    durable,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to declare exchange: {}", e)))?;

        Ok(())
    }

    /// Declare a queue.
    pub async fn declare_queue(&self, name: &str, durable: bool) -> MessagingResult<()> {
        let channel = self.channel.lock().await;

        channel
            .queue_declare(
                name,
                QueueDeclareOptions {
                    durable,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to declare queue: {}", e)))?;

        Ok(())
    }

    /// Bind a queue to an exchange with a routing key.
    pub async fn bind_queue(&self, queue: &str, exchange: &str, routing_key: &str) -> MessagingResult<()> {
        let channel = self.channel.lock().await;

        channel
            .queue_bind(
                queue,
                exchange,
                routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to bind queue: {}", e)))?;

        Ok(())
    }

    /// Publish directly to a queue (bypassing exchanges).
    pub async fn publish_to_queue(&self, queue: &str, message: Message) -> MessagingResult<()> {
        let channel = self.channel.lock().await;
        let data = Self::serialize_message(&message)?;

        channel
            .basic_publish(
                "",
                queue,
                BasicPublishOptions::default(),
                &data,
                BasicProperties::default(),
            )
            .await
            .map_err(|e| MessagingError::Publish(format!("Failed to publish to queue: {}", e)))?;

        Ok(())
    }
}

#[async_trait]
impl MessageBroker for RabbitMQBroker {
    async fn publish(&self, message: Message) -> MessagingResult<()> {
        let channel = self.channel.lock().await;
        let data = Self::serialize_message(&message)?;

        // Use topic as exchange name, empty routing key for fanout
        channel
            .basic_publish(
                &message.topic,
                "",
                BasicPublishOptions::default(),
                &data,
                BasicProperties::default(),
            )
            .await
            .map_err(|e| MessagingError::Publish(format!("Failed to publish to RabbitMQ: {}", e)))?;

        Ok(())
    }

    async fn subscribe(&self, topic: &str) -> MessagingResult<Box<dyn AckMessageStream>> {
        // Create a new channel for this subscription
        let channel = self
            .connection
            .create_channel()
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to create channel: {}", e)))?;

        // Declare a fanout exchange for the topic
        channel
            .exchange_declare(
                topic,
                lapin::ExchangeKind::Fanout,
                ExchangeDeclareOptions {
                    durable: false,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to declare exchange: {}", e)))?;

        // Create an exclusive queue for this subscriber
        let queue = channel
            .queue_declare(
                "",
                QueueDeclareOptions {
                    exclusive: true,
                    auto_delete: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to declare queue: {}", e)))?;

        // Bind queue to exchange
        channel
            .queue_bind(
                queue.name().as_str(),
                topic,
                "",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to bind queue: {}", e)))?;

        // Start consuming
        let consumer = channel
            .basic_consume(
                queue.name().as_str(),
                "",
                BasicConsumeOptions {
                    no_ack: self.config.delivery_mode == DeliveryMode::AtMostOnce,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to start consumer: {}", e)))?;

        Ok(Box::new(RabbitMQStream {
            consumer,
            channel: Arc::new(Mutex::new(channel)),
            delivery_mode: self.config.delivery_mode,
        }))
    }

    async fn consume(&self, queue: &str) -> MessagingResult<Box<dyn AckMessageStream>> {
        // Create a new channel for this consumer
        let channel = self
            .connection
            .create_channel()
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to create channel: {}", e)))?;

        // Declare the queue (idempotent)
        channel
            .queue_declare(
                queue,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to declare queue: {}", e)))?;

        // Start consuming
        let consumer = channel
            .basic_consume(
                queue,
                "",
                BasicConsumeOptions {
                    no_ack: self.config.delivery_mode == DeliveryMode::AtMostOnce,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| MessagingError::Subscription(format!("Failed to start consumer: {}", e)))?;

        Ok(Box::new(RabbitMQStream {
            consumer,
            channel: Arc::new(Mutex::new(channel)),
            delivery_mode: self.config.delivery_mode,
        }))
    }
}

/// RabbitMQ message stream.
struct RabbitMQStream {
    consumer: Consumer,
    channel: Arc<Mutex<Channel>>,
    delivery_mode: DeliveryMode,
}

#[async_trait]
impl AckMessageStream for RabbitMQStream {
    async fn next(&mut self) -> Option<AckMessage> {
        loop {
            let delivery = self.consumer.next().await?;

            let delivery = match delivery {
                Ok(d) => d,
                Err(_) => continue,
            };

            match RabbitMQBroker::deserialize_message(&delivery.data) {
                Ok(message) => {
                    let ack_handle = if self.delivery_mode != DeliveryMode::AtMostOnce {
                        Some(Box::new(RabbitMQAck {
                            channel: self.channel.clone(),
                            delivery_tag: delivery.delivery_tag,
                        }) as Box<dyn MessageAck>)
                    } else {
                        None
                    };

                    return Some(AckMessage::new(message, ack_handle));
                }
                Err(_) => {
                    // Reject malformed messages
                    let _ = delivery.reject(BasicRejectOptions { requeue: false }).await;
                    continue;
                }
            }
        }
    }
}

/// RabbitMQ acknowledgment handle.
struct RabbitMQAck {
    channel: Arc<Mutex<Channel>>,
    delivery_tag: u64,
}

#[async_trait]
impl MessageAck for RabbitMQAck {
    async fn ack(&mut self) -> MessagingResult<()> {
        let channel = self.channel.lock().await;

        channel
            .basic_ack(self.delivery_tag, BasicAckOptions::default())
            .await
            .map_err(|e| MessagingError::Publish(format!("Failed to ack message: {}", e)))?;

        Ok(())
    }

    async fn nack(&mut self) -> MessagingResult<()> {
        let channel = self.channel.lock().await;

        channel
            .basic_nack(
                self.delivery_tag,
                BasicNackOptions {
                    requeue: true,
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| MessagingError::Publish(format!("Failed to nack message: {}", e)))?;

        Ok(())
    }

    async fn reject(&mut self) -> MessagingResult<()> {
        let channel = self.channel.lock().await;

        channel
            .basic_reject(self.delivery_tag, BasicRejectOptions { requeue: false })
            .await
            .map_err(|e| MessagingError::Publish(format!("Failed to reject message: {}", e)))?;

        Ok(())
    }
}

/// Extension trait for RabbitMQ-specific operations.
#[async_trait]
pub trait RabbitMQExt {
    /// Publish to a specific exchange with routing key.
    async fn publish_to_exchange(
        &self,
        exchange: &str,
        routing_key: &str,
        message: Message,
    ) -> MessagingResult<()>;

    /// Declare an exchange.
    async fn declare_exchange(&self, name: &str, exchange_type: &str, durable: bool) -> MessagingResult<()>;

    /// Declare a queue.
    async fn declare_queue(&self, name: &str, durable: bool) -> MessagingResult<()>;

    /// Bind a queue to an exchange.
    async fn bind_queue(&self, queue: &str, exchange: &str, routing_key: &str) -> MessagingResult<()>;
}

#[async_trait]
impl RabbitMQExt for RabbitMQBroker {
    async fn publish_to_exchange(
        &self,
        exchange: &str,
        routing_key: &str,
        message: Message,
    ) -> MessagingResult<()> {
        let channel = self.channel.lock().await;
        let data = Self::serialize_message(&message)?;

        channel
            .basic_publish(
                exchange,
                routing_key,
                BasicPublishOptions::default(),
                &data,
                BasicProperties::default(),
            )
            .await
            .map_err(|e| MessagingError::Publish(format!("Failed to publish to exchange: {}", e)))?;

        Ok(())
    }

    async fn declare_exchange(&self, name: &str, exchange_type: &str, durable: bool) -> MessagingResult<()> {
        RabbitMQBroker::declare_exchange(self, name, exchange_type, durable).await
    }

    async fn declare_queue(&self, name: &str, durable: bool) -> MessagingResult<()> {
        RabbitMQBroker::declare_queue(self, name, durable).await
    }

    async fn bind_queue(&self, queue: &str, exchange: &str, routing_key: &str) -> MessagingResult<()> {
        RabbitMQBroker::bind_queue(self, queue, exchange, routing_key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires RabbitMQ server running
    async fn test_rabbitmq_pubsub() {
        let broker = RabbitMQBroker::connect("amqp://127.0.0.1:5672")
            .await
            .expect("Failed to connect to RabbitMQ");

        let mut stream = broker
            .subscribe("test_exchange")
            .await
            .expect("Failed to subscribe");

        let message = Message::new("test_exchange", b"Hello RabbitMQ".to_vec());
        broker.publish(message.clone()).await.expect("Failed to publish");

        let received = tokio::time::timeout(std::time::Duration::from_secs(2), stream.next())
            .await
            .expect("Timeout")
            .expect("No message received");

        assert_eq!(received.message.payload, b"Hello RabbitMQ");
    }

    #[tokio::test]
    #[ignore] // Requires RabbitMQ server running
    async fn test_rabbitmq_queue() {
        let broker = RabbitMQBroker::connect("amqp://127.0.0.1:5672")
            .await
            .expect("Failed to connect to RabbitMQ");

        let queue = "test_queue";

        broker
            .declare_queue(queue, true)
            .await
            .expect("Failed to declare queue");

        let message = Message::new(queue, b"Queue message".to_vec());

        // Publish directly to queue
        let channel = broker.channel.lock().await;
        let data = RabbitMQBroker::serialize_message(&message).unwrap();
        channel
            .basic_publish(
                "",
                queue,
                BasicPublishOptions::default(),
                &data,
                BasicProperties::default(),
            )
            .await
            .expect("Failed to publish");
        drop(channel);

        let mut stream = broker.consume(queue).await.expect("Failed to consume");

        let received = tokio::time::timeout(std::time::Duration::from_secs(2), stream.next())
            .await
            .expect("Timeout")
            .expect("No message received");

        assert_eq!(received.message.payload, b"Queue message");
        received.ack().await.expect("Failed to ack");
    }
}
