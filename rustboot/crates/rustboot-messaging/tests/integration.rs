//! Integration tests for rustboot-messaging

use dev_engineeringlabs_rustboot_messaging::{
    AckMessageStream, BrokerConfig, DeliveryMode, InMemoryBus, Message, MessageBroker,
    Publisher, Subscriber,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestPayload {
    value: String,
}

// ============================================================================
// In-Memory Bus Tests
// ============================================================================

#[tokio::test]
async fn test_inmemory_pubsub() {
    let bus = InMemoryBus::new();

    let mut subscriber = bus.subscribe("test").await.unwrap();

    let message = Message::new("test", b"hello world".to_vec());
    bus.publish(message).await.unwrap();

    let received = subscriber.next().await.unwrap();
    assert_eq!(received.topic, "test");
    assert_eq!(received.payload, b"hello world");
}

#[tokio::test]
async fn test_inmemory_multiple_subscribers() {
    let bus = InMemoryBus::new();

    let mut sub1 = bus.subscribe("broadcast").await.unwrap();
    let mut sub2 = bus.subscribe("broadcast").await.unwrap();

    let message = Message::new("broadcast", b"message".to_vec());
    bus.publish(message).await.unwrap();

    let msg1 = sub1.next().await.unwrap();
    let msg2 = sub2.next().await.unwrap();

    assert_eq!(msg1.payload, b"message");
    assert_eq!(msg2.payload, b"message");
}

#[tokio::test]
async fn test_inmemory_json_serialization() {
    let bus = InMemoryBus::new();

    let mut subscriber = bus.subscribe("json").await.unwrap();

    let payload = TestPayload {
        value: "test".to_string(),
    };

    bus.publish_json("json", &payload).await.unwrap();

    let received = subscriber.next().await.unwrap();
    let deserialized: TestPayload = received.deserialize().unwrap();

    assert_eq!(deserialized, payload);
}

#[tokio::test]
async fn test_inmemory_topic_isolation() {
    let bus = InMemoryBus::new();

    let mut sub_a = bus.subscribe("topic_a").await.unwrap();
    let mut sub_b = bus.subscribe("topic_b").await.unwrap();

    bus.publish(Message::new("topic_a", b"for_a".to_vec()))
        .await
        .unwrap();
    bus.publish(Message::new("topic_b", b"for_b".to_vec()))
        .await
        .unwrap();

    let msg_a = sub_a.next().await.unwrap();
    let msg_b = sub_b.next().await.unwrap();

    assert_eq!(msg_a.payload, b"for_a");
    assert_eq!(msg_b.payload, b"for_b");
}

// ============================================================================
// Redis Broker Tests
// ============================================================================

#[cfg(feature = "redis-broker")]
mod redis_tests {
    use super::*;
    use dev_engineeringlabs_rustboot_messaging::{RedisBroker, RedisQueueExt};

    #[tokio::test]
    #[ignore] // Requires Redis server
    async fn test_redis_pubsub() {
        let broker = RedisBroker::connect("redis://127.0.0.1:6379")
            .await
            .expect("Failed to connect to Redis");

        let mut subscriber = broker.subscribe("test_topic").await.unwrap();

        let message = Message::new("test_topic", b"redis test".to_vec());
        broker.publish(message).await.unwrap();

        let received = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            subscriber.next(),
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(received.message.payload, b"redis test");
    }

    #[tokio::test]
    #[ignore] // Requires Redis server
    async fn test_redis_queue_with_ack() {
        let broker = RedisBroker::connect("redis://127.0.0.1:6379")
            .await
            .expect("Failed to connect to Redis");

        let queue = format!("test_queue_{}", uuid::Uuid::new_v4());
        let message = Message::new(&queue, b"queue message".to_vec());

        broker.push_to_queue(&queue, message).await.unwrap();

        let mut consumer = broker.consume(&queue).await.unwrap();

        let received = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            consumer.next(),
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(received.message.payload, b"queue message");
        assert!(received.ack_handle.is_some());

        received.ack().await.unwrap();
    }

    #[tokio::test]
    #[ignore] // Requires Redis server
    async fn test_redis_queue_nack_redelivery() {
        let broker = RedisBroker::connect("redis://127.0.0.1:6379")
            .await
            .expect("Failed to connect to Redis");

        let queue = format!("test_nack_{}", uuid::Uuid::new_v4());
        let message = Message::new(&queue, b"nack test".to_vec());

        broker.push_to_queue(&queue, message).await.unwrap();

        let mut consumer = broker.consume(&queue).await.unwrap();

        // First delivery - nack
        let msg1 = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            consumer.next(),
        )
        .await
        .unwrap()
        .unwrap();

        msg1.nack().await.unwrap();

        // Second delivery - should be redelivered
        let msg2 = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            consumer.next(),
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(msg2.message.payload, b"nack test");
        msg2.ack().await.unwrap();
    }

    #[tokio::test]
    #[ignore] // Requires Redis server
    async fn test_redis_delivery_modes() {
        let config = BrokerConfig::new("redis://127.0.0.1:6379")
            .with_delivery_mode(DeliveryMode::AtMostOnce);

        let broker = RedisBroker::new(config).await.unwrap();

        let queue = format!("test_delivery_{}", uuid::Uuid::new_v4());
        broker
            .push_to_queue(&queue, Message::new(&queue, b"test".to_vec()))
            .await
            .unwrap();

        let mut consumer = broker.consume(&queue).await.unwrap();
        let msg = consumer.next().await.unwrap();

        // At-most-once should not have ack handle
        assert!(msg.ack_handle.is_none());
    }
}

// ============================================================================
// RabbitMQ Broker Tests
// ============================================================================

#[cfg(feature = "rabbitmq-broker")]
mod rabbitmq_tests {
    use super::*;
    use dev_engineeringlabs_rustboot_messaging::{RabbitMQBroker, RabbitMQExt};

    #[tokio::test]
    #[ignore] // Requires RabbitMQ server
    async fn test_rabbitmq_pubsub() {
        let broker = RabbitMQBroker::connect("amqp://127.0.0.1:5672")
            .await
            .expect("Failed to connect to RabbitMQ");

        let exchange = format!("test_exchange_{}", uuid::Uuid::new_v4());
        let mut subscriber = broker.subscribe(&exchange).await.unwrap();

        let message = Message::new(&exchange, b"rabbitmq test".to_vec());
        broker.publish(message).await.unwrap();

        let received = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            subscriber.next(),
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(received.message.payload, b"rabbitmq test");
        received.ack().await.unwrap();
    }

    #[tokio::test]
    #[ignore] // Requires RabbitMQ server
    async fn test_rabbitmq_queue() {
        let broker = RabbitMQBroker::connect("amqp://127.0.0.1:5672")
            .await
            .expect("Failed to connect to RabbitMQ");

        let queue = format!("test_queue_{}", uuid::Uuid::new_v4());

        broker.declare_queue(&queue, false).await.unwrap();

        // Publish directly to queue
        let message = Message::new(&queue, b"queue test".to_vec());
        broker.publish_to_queue(&queue, message).await.unwrap();

        let mut consumer = broker.consume(&queue).await.unwrap();

        let received = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            consumer.next(),
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(received.message.payload, b"queue test");
        received.ack().await.unwrap();
    }

    #[tokio::test]
    #[ignore] // Requires RabbitMQ server
    async fn test_rabbitmq_nack_redelivery() {
        let broker = RabbitMQBroker::connect("amqp://127.0.0.1:5672")
            .await
            .expect("Failed to connect to RabbitMQ");

        let queue = format!("test_nack_{}", uuid::Uuid::new_v4());
        broker.declare_queue(&queue, false).await.unwrap();

        let message = Message::new(&queue, b"nack test".to_vec());
        broker.publish_to_queue(&queue, message).await.unwrap();

        let mut consumer = broker.consume(&queue).await.unwrap();

        // Nack first message
        let msg1 = consumer.next().await.unwrap();
        msg1.nack().await.unwrap();

        // Should be redelivered
        let msg2 = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            consumer.next(),
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(msg2.message.payload, b"nack test");
        msg2.ack().await.unwrap();
    }
}

// ============================================================================
// Kafka Broker Tests
// ============================================================================

#[cfg(feature = "kafka-broker")]
mod kafka_tests {
    use super::*;
    use dev_engineeringlabs_rustboot_messaging::{KafkaBroker, KafkaExt};

    #[tokio::test]
    #[ignore] // Requires Kafka broker
    async fn test_kafka_publish_consume() {
        let broker = KafkaBroker::connect("localhost:9092")
            .await
            .expect("Failed to connect to Kafka");

        let topic = format!("test_topic_{}", uuid::Uuid::new_v4());

        let message = Message::new(&topic, b"kafka test".to_vec());
        broker.publish(message).await.unwrap();

        let mut consumer = broker.subscribe(&topic).await.unwrap();

        let received = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            consumer.next(),
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(received.message.payload, b"kafka test");
        received.ack().await.unwrap();
    }

    #[tokio::test]
    #[ignore] // Requires Kafka broker
    async fn test_kafka_with_key() {
        let broker = KafkaBroker::connect("localhost:9092")
            .await
            .expect("Failed to connect to Kafka");

        let topic = format!("test_keyed_{}", uuid::Uuid::new_v4());

        let message = Message::new(&topic, b"keyed message".to_vec());
        broker
            .publish_with_key(&topic, "my-key", message)
            .await
            .unwrap();

        let mut consumer = broker.subscribe(&topic).await.unwrap();

        let received = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            consumer.next(),
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(received.message.payload, b"keyed message");
        received.ack().await.unwrap();
    }

    #[tokio::test]
    #[ignore] // Requires Kafka broker
    async fn test_kafka_consumer_groups() {
        let broker = KafkaBroker::connect("localhost:9092")
            .await
            .expect("Failed to connect to Kafka");

        let topic = format!("test_groups_{}", uuid::Uuid::new_v4());

        // Publish multiple messages
        for i in 0..4 {
            let message = Message::new(&topic, format!("msg{}", i).into_bytes());
            broker.publish(message).await.unwrap();
        }

        // Same group - messages should be distributed
        let _consumer1 = broker.consume(&topic).await.unwrap();
        let _consumer2 = broker.consume(&topic).await.unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Both consumers should receive some messages
        // This is a basic test - exact distribution depends on partitions
    }
}

// ============================================================================
// Broker Config Tests
// ============================================================================

#[test]
fn test_broker_config_builder() {
    let config = BrokerConfig::new("redis://localhost")
        .with_delivery_mode(DeliveryMode::ExactlyOnce)
        .with_timeout(10000)
        .with_auto_reconnect(false);

    assert_eq!(config.url, "redis://localhost");
    assert_eq!(config.delivery_mode, DeliveryMode::ExactlyOnce);
    assert_eq!(config.timeout_ms, 10000);
    assert!(!config.auto_reconnect);
}

#[test]
fn test_broker_config_default() {
    let config = BrokerConfig::default();

    assert_eq!(config.delivery_mode, DeliveryMode::AtLeastOnce);
    assert_eq!(config.timeout_ms, 5000);
    assert!(config.auto_reconnect);
}

#[test]
fn test_delivery_mode_variants() {
    let modes = vec![
        DeliveryMode::AtMostOnce,
        DeliveryMode::AtLeastOnce,
        DeliveryMode::ExactlyOnce,
    ];

    for mode in modes {
        let config = BrokerConfig::new("test").with_delivery_mode(mode);
        assert_eq!(config.delivery_mode, mode);
    }
}
