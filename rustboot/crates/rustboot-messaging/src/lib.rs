//! Rustboot Messaging - Pub/sub messaging abstraction
//!
//! This crate provides messaging abstractions for publish/subscribe patterns
//! and message queues with support for multiple broker backends:
//!
//! - In-memory bus (always available)
//! - Redis (feature: `redis-broker`)
//! - RabbitMQ (feature: `rabbitmq-broker`)
//! - Kafka (feature: `kafka-broker`)
//!
//! # Features
//!
//! - `redis-broker`: Enable Redis pub/sub and queue support
//! - `rabbitmq-broker`: Enable RabbitMQ AMQP support
//! - `kafka-broker`: Enable Apache Kafka support
//! - `all-brokers`: Enable all broker implementations
//!
//! # Example
//!
//! ```no_run
//! use dev_engineeringlabs_rustboot_messaging::{Message, Publisher, InMemoryBus};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let bus = InMemoryBus::new();
//!
//! let message = Message::new("events", b"Hello".to_vec());
//! bus.publish(message).await?;
//! # Ok(())
//! # }
//! ```

pub mod bus;
pub mod broker;

// Conditional broker modules
#[cfg(feature = "redis-broker")]
pub mod redis_broker;

#[cfg(feature = "rabbitmq-broker")]
pub mod rabbitmq_broker;

#[cfg(feature = "kafka-broker")]
pub mod kafka_broker;

// Re-export main types
pub use bus::{InMemoryBus, Message, MessageStream, MessagingError, MessagingResult, Publisher, Subscriber};

pub use broker::{
    AckMessage, AckMessageStream, BrokerConfig, DeliveryMode, MessageAck, MessageBroker,
};

// Re-export broker implementations when features are enabled
#[cfg(feature = "redis-broker")]
pub use redis_broker::{RedisBroker, RedisQueueExt};

#[cfg(feature = "rabbitmq-broker")]
pub use rabbitmq_broker::{RabbitMQBroker, RabbitMQExt};

#[cfg(feature = "kafka-broker")]
pub use kafka_broker::{KafkaBroker, KafkaExt};
