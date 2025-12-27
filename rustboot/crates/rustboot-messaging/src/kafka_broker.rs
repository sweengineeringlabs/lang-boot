//! Kafka message broker implementation.
//!
//! Supports topics, partitions, consumer groups, and offset management.

use async_trait::async_trait;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message as KafkaMessage;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::broker::{AckMessage, AckMessageStream, BrokerConfig, DeliveryMode, MessageAck, MessageBroker};
use crate::{Message, MessagingError, MessagingResult};

/// Kafka message broker.
pub struct KafkaBroker {
    producer: FutureProducer,
    bootstrap_servers: String,
    config: BrokerConfig,
}

impl KafkaBroker {
    /// Create a new Kafka broker.
    pub async fn new(config: BrokerConfig) -> MessagingResult<Self> {
        let bootstrap_servers = Self::extract_bootstrap_servers(&config.url)?;

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &bootstrap_servers)
            .set("message.timeout.ms", config.timeout_ms.to_string())
            .create()
            .map_err(|e| MessagingError::Subscription(format!("Failed to create Kafka producer: {}", e)))?;

        Ok(Self {
            producer,
            bootstrap_servers,
            config,
        })
    }

    /// Create with default configuration.
    pub async fn connect(bootstrap_servers: impl Into<String>) -> MessagingResult<Self> {
        let servers = bootstrap_servers.into();
        let config = BrokerConfig::new(format!("kafka://{}", servers));
        Self::new(config).await
    }

    /// Extract bootstrap servers from URL (kafka://host:port or just host:port).
    fn extract_bootstrap_servers(url: &str) -> MessagingResult<String> {
        let servers = url
            .strip_prefix("kafka://")
            .unwrap_or(url)
            .to_string();

        if servers.is_empty() {
            return Err(MessagingError::Subscription(
                "Invalid Kafka URL: bootstrap servers not specified".to_string(),
            ));
        }

        Ok(servers)
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
}

#[async_trait]
impl MessageBroker for KafkaBroker {
    async fn publish(&self, message: Message) -> MessagingResult<()> {
        let data = Self::serialize_message(&message)?;

        let record = FutureRecord::to(&message.topic)
            .payload(&data)
            .key(&message.id);

        self.producer
            .send(record, Duration::from_millis(self.config.timeout_ms))
            .await
            .map_err(|(e, _)| MessagingError::Publish(format!("Failed to publish to Kafka: {}", e)))?;

        Ok(())
    }

    async fn subscribe(&self, topic: &str) -> MessagingResult<Box<dyn AckMessageStream>> {
        // For Kafka, subscribe and consume are similar
        // Use a unique group ID for each subscriber to get broadcast behavior
        let group_id = format!("subscriber_{}", uuid::Uuid::new_v4());
        self.create_consumer(topic, &group_id).await
    }

    async fn consume(&self, queue: &str) -> MessagingResult<Box<dyn AckMessageStream>> {
        // Use the queue name as the consumer group ID
        // Multiple consumers with the same group ID will share messages
        self.create_consumer(queue, queue).await
    }
}

impl KafkaBroker {
    /// Create a consumer for a topic with a specific group ID.
    async fn create_consumer(&self, topic: &str, group_id: &str) -> MessagingResult<Box<dyn AckMessageStream>> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &self.bootstrap_servers)
            .set("group.id", group_id)
            .set("enable.auto.commit", "false") // Manual commit for acknowledgments
            .set("auto.offset.reset", "earliest")
            .create()
            .map_err(|e| MessagingError::Subscription(format!("Failed to create Kafka consumer: {}", e)))?;

        consumer
            .subscribe(&[topic])
            .map_err(|e| MessagingError::Subscription(format!("Failed to subscribe to topic: {}", e)))?;

        Ok(Box::new(KafkaStream {
            consumer: Arc::new(Mutex::new(consumer)),
            delivery_mode: self.config.delivery_mode,
        }))
    }
}

/// Kafka message stream.
struct KafkaStream {
    consumer: Arc<Mutex<StreamConsumer>>,
    delivery_mode: DeliveryMode,
}

#[async_trait]
impl AckMessageStream for KafkaStream {
    async fn next(&mut self) -> Option<AckMessage> {
        loop {
            let consumer = self.consumer.lock().await;

            let kafka_msg = consumer
                .recv()
                .await
                .ok()?;

            let payload = match kafka_msg.payload() {
                Some(p) => p,
                None => continue,
            };

            match KafkaBroker::deserialize_message(payload) {
                Ok(message) => {
                    let ack_handle = if self.delivery_mode != DeliveryMode::AtMostOnce {
                        Some(Box::new(KafkaAck {
                            consumer: self.consumer.clone(),
                            topic: kafka_msg.topic().to_string(),
                            partition: kafka_msg.partition(),
                            offset: kafka_msg.offset(),
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

/// Kafka acknowledgment handle.
struct KafkaAck {
    consumer: Arc<Mutex<StreamConsumer>>,
    topic: String,
    partition: i32,
    offset: i64,
}

#[async_trait]
impl MessageAck for KafkaAck {
    async fn ack(&mut self) -> MessagingResult<()> {
        let consumer = self.consumer.lock().await;

        // Commit offset
        use rdkafka::TopicPartitionList;
        let mut tpl = TopicPartitionList::new();
        tpl.add_partition_offset(&self.topic, self.partition, rdkafka::Offset::Offset(self.offset + 1))
            .map_err(|e| MessagingError::Publish(format!("Failed to add partition offset: {}", e)))?;

        consumer
            .commit(&tpl, rdkafka::consumer::CommitMode::Sync)
            .map_err(|e| MessagingError::Publish(format!("Failed to commit offset: {}", e)))?;

        Ok(())
    }

    async fn nack(&mut self) -> MessagingResult<()> {
        // Kafka doesn't have native nack - we can seek back to replay
        let consumer = self.consumer.lock().await;

        use rdkafka::consumer::Consumer as ConsumerTrait;
        use rdkafka::TopicPartitionList;

        let mut tpl = TopicPartitionList::new();
        tpl.add_partition_offset(&self.topic, self.partition, rdkafka::Offset::Offset(self.offset))
            .map_err(|e| MessagingError::Publish(format!("Failed to add partition offset: {}", e)))?;

        consumer
            .seek(&self.topic, self.partition, rdkafka::Offset::Offset(self.offset), Duration::from_secs(10))
            .map_err(|e| MessagingError::Publish(format!("Failed to seek offset: {}", e)))?;

        Ok(())
    }

    async fn reject(&mut self) -> MessagingResult<()> {
        // Reject is same as ack for Kafka - just move past the message
        self.ack().await
    }
}

/// Extension trait for Kafka-specific operations.
#[async_trait]
pub trait KafkaExt {
    /// Create a consumer with specific configuration.
    async fn create_consumer_with_config(
        &self,
        topic: &str,
        group_id: &str,
        config: Vec<(&str, &str)>,
    ) -> MessagingResult<Box<dyn AckMessageStream>>;

    /// Publish with partition key.
    async fn publish_with_key(&self, topic: &str, key: &str, message: Message) -> MessagingResult<()>;

    /// Publish with headers.
    async fn publish_with_headers(
        &self,
        topic: &str,
        message: Message,
        headers: Vec<(&str, &[u8])>,
    ) -> MessagingResult<()>;
}

#[async_trait]
impl KafkaExt for KafkaBroker {
    async fn create_consumer_with_config(
        &self,
        topic: &str,
        group_id: &str,
        custom_config: Vec<(&str, &str)>,
    ) -> MessagingResult<Box<dyn AckMessageStream>> {
        let mut client_config = ClientConfig::new();
        client_config
            .set("bootstrap.servers", &self.bootstrap_servers)
            .set("group.id", group_id)
            .set("enable.auto.commit", "false");

        for (key, value) in custom_config {
            client_config.set(key, value);
        }

        let consumer: StreamConsumer = client_config
            .create()
            .map_err(|e| MessagingError::Subscription(format!("Failed to create Kafka consumer: {}", e)))?;

        consumer
            .subscribe(&[topic])
            .map_err(|e| MessagingError::Subscription(format!("Failed to subscribe to topic: {}", e)))?;

        Ok(Box::new(KafkaStream {
            consumer: Arc::new(Mutex::new(consumer)),
            delivery_mode: self.config.delivery_mode,
        }))
    }

    async fn publish_with_key(&self, topic: &str, key: &str, message: Message) -> MessagingResult<()> {
        let data = Self::serialize_message(&message)?;

        let record = FutureRecord::to(topic)
            .payload(&data)
            .key(key);

        self.producer
            .send(record, Duration::from_millis(self.config.timeout_ms))
            .await
            .map_err(|(e, _)| MessagingError::Publish(format!("Failed to publish to Kafka: {}", e)))?;

        Ok(())
    }

    async fn publish_with_headers(
        &self,
        topic: &str,
        message: Message,
        headers: Vec<(&str, &[u8])>,
    ) -> MessagingResult<()> {
        let data = Self::serialize_message(&message)?;

        let mut owned_headers = OwnedHeaders::new();
        for (key, value) in headers {
            owned_headers = owned_headers.insert(rdkafka::message::Header {
                key,
                value: Some(value),
            });
        }

        let record = FutureRecord::to(topic)
            .payload(&data)
            .key(&message.id)
            .headers(owned_headers);

        self.producer
            .send(record, Duration::from_millis(self.config.timeout_ms))
            .await
            .map_err(|(e, _)| MessagingError::Publish(format!("Failed to publish to Kafka: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires Kafka broker running
    async fn test_kafka_publish_consume() {
        let broker = KafkaBroker::connect("localhost:9092")
            .await
            .expect("Failed to connect to Kafka");

        let topic = "test_topic";
        let message = Message::new(topic, b"Hello Kafka".to_vec());

        broker.publish(message.clone()).await.expect("Failed to publish");

        let mut stream = broker.consume(topic).await.expect("Failed to consume");

        let received = tokio::time::timeout(std::time::Duration::from_secs(10), stream.next())
            .await
            .expect("Timeout")
            .expect("No message received");

        assert_eq!(received.message.payload, b"Hello Kafka");
        received.ack().await.expect("Failed to ack");
    }

    #[tokio::test]
    #[ignore] // Requires Kafka broker running
    async fn test_kafka_consumer_groups() {
        let broker = KafkaBroker::connect("localhost:9092")
            .await
            .expect("Failed to connect to Kafka");

        let topic = "consumer_group_test";

        // Two consumers in the same group should compete for messages
        let mut consumer1 = broker.consume(topic).await.expect("Failed to create consumer 1");
        let mut consumer2 = broker.consume(topic).await.expect("Failed to create consumer 2");

        // Publish messages
        for i in 0..4 {
            let message = Message::new(topic, format!("Message {}", i).into_bytes());
            broker.publish(message).await.expect("Failed to publish");
        }

        // Both consumers should receive some messages (load balancing)
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // This test would verify that messages are distributed
        // In practice, exact distribution depends on partition assignment
    }

    #[tokio::test]
    #[ignore] // Requires Kafka broker running
    async fn test_kafka_with_key() {
        let broker = KafkaBroker::connect("localhost:9092")
            .await
            .expect("Failed to connect to Kafka");

        let topic = "keyed_topic";
        let message = Message::new(topic, b"Keyed message".to_vec());

        broker
            .publish_with_key(topic, "my-key", message)
            .await
            .expect("Failed to publish with key");
    }
}
