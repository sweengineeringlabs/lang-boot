//! Broker comparison example showing the unified interface.
//!
//! Run with: cargo run --example broker_comparison --features all-brokers
//!
//! Demonstrates how all brokers implement the same MessageBroker trait.

use dev_engineeringlabs_rustboot_messaging::{
    AckMessageStream, BrokerConfig, Message, MessageBroker,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Event {
    id: u64,
    name: String,
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot Messaging Broker Comparison ===\n");

    println!("This example demonstrates the unified MessageBroker interface");
    println!("across different backend implementations.\n");

    // In-memory (always available)
    println!("1. In-Memory Bus");
    test_inmemory_broker().await?;

    // Redis (requires redis-broker feature and running Redis)
    #[cfg(feature = "redis-broker")]
    {
        println!("\n2. Redis Broker");
        if let Err(e) = test_redis_broker().await {
            println!("  Redis test skipped: {} (is Redis running?)", e);
        }
    }

    // RabbitMQ (requires rabbitmq-broker feature and running RabbitMQ)
    #[cfg(feature = "rabbitmq-broker")]
    {
        println!("\n3. RabbitMQ Broker");
        if let Err(e) = test_rabbitmq_broker().await {
            println!("  RabbitMQ test skipped: {} (is RabbitMQ running?)", e);
        }
    }

    // Kafka (requires kafka-broker feature and running Kafka)
    #[cfg(feature = "kafka-broker")]
    {
        println!("\n4. Kafka Broker");
        if let Err(e) = test_kafka_broker().await {
            println!("  Kafka test skipped: {} (is Kafka running?)", e);
        }
    }

    Ok(())
}

async fn test_inmemory_broker() -> Result<(), Box<dyn std::error::Error>> {
    use dev_engineeringlabs_rustboot_messaging::{InMemoryBus, Publisher, Subscriber};

    let bus = InMemoryBus::new();

    // Subscribe
    let mut subscriber = bus.subscribe("events").await?;

    // Publish
    let event = Event {
        id: 1,
        name: "in_memory_test".to_string(),
        data: "Hello from in-memory".to_string(),
    };

    bus.publish_json("events", &event).await?;
    println!("  Published event: {:?}", event);

    // Consume
    if let Some(msg) = subscriber.next().await {
        let received: Event = msg.deserialize()?;
        println!("  Received event: {:?}", received);
        assert_eq!(received.name, event.name);
    }

    println!("  ✓ In-memory broker test passed");
    Ok(())
}

#[cfg(feature = "redis-broker")]
async fn test_redis_broker() -> Result<(), Box<dyn std::error::Error>> {
    use dev_engineeringlabs_rustboot_messaging::RedisBroker;

    let broker = RedisBroker::connect("redis://127.0.0.1:6379").await?;

    // Pub/Sub
    let mut subscriber = broker.subscribe("redis_events").await?;

    let event = Event {
        id: 2,
        name: "redis_test".to_string(),
        data: "Hello from Redis".to_string(),
    };

    broker.publish_json("redis_events", &event).await?;
    println!("  Published to Redis: {:?}", event);

    if let Some(msg) = tokio::time::timeout(
        tokio::time::Duration::from_secs(2),
        subscriber.next()
    ).await? {
        let received: Event = msg.message.deserialize()?;
        println!("  Received from Redis: {:?}", received);
        assert_eq!(received.name, event.name);
    }

    println!("  ✓ Redis broker test passed");
    Ok(())
}

#[cfg(feature = "rabbitmq-broker")]
async fn test_rabbitmq_broker() -> Result<(), Box<dyn std::error::Error>> {
    use dev_engineeringlabs_rustboot_messaging::RabbitMQBroker;

    let broker = RabbitMQBroker::connect("amqp://127.0.0.1:5672").await?;

    // Pub/Sub
    let mut subscriber = broker.subscribe("rabbitmq_events").await?;

    let event = Event {
        id: 3,
        name: "rabbitmq_test".to_string(),
        data: "Hello from RabbitMQ".to_string(),
    };

    broker.publish_json("rabbitmq_events", &event).await?;
    println!("  Published to RabbitMQ: {:?}", event);

    if let Some(msg) = tokio::time::timeout(
        tokio::time::Duration::from_secs(2),
        subscriber.next()
    ).await? {
        let received: Event = msg.message.deserialize()?;
        println!("  Received from RabbitMQ: {:?}", received);
        msg.ack().await?;
        assert_eq!(received.name, event.name);
    }

    println!("  ✓ RabbitMQ broker test passed");
    Ok(())
}

#[cfg(feature = "kafka-broker")]
async fn test_kafka_broker() -> Result<(), Box<dyn std::error::Error>> {
    use dev_engineeringlabs_rustboot_messaging::KafkaBroker;

    let broker = KafkaBroker::connect("localhost:9092").await?;

    let topic = "kafka_events";

    let event = Event {
        id: 4,
        name: "kafka_test".to_string(),
        data: "Hello from Kafka".to_string(),
    };

    broker.publish_json(topic, &event).await?;
    println!("  Published to Kafka: {:?}", event);

    // Subscribe
    let mut subscriber = broker.subscribe(topic).await?;

    if let Some(msg) = tokio::time::timeout(
        tokio::time::Duration::from_secs(5),
        subscriber.next()
    ).await? {
        let received: Event = msg.message.deserialize()?;
        println!("  Received from Kafka: {:?}", received);
        msg.ack().await?;
        assert_eq!(received.name, event.name);
    }

    println!("  ✓ Kafka broker test passed");
    Ok(())
}

// Generic function that works with any MessageBroker implementation
async fn generic_publish_subscribe<B: MessageBroker>(
    broker: &B,
    topic: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut subscriber = broker.subscribe(topic).await?;

    let event = Event {
        id: 999,
        name: "generic_test".to_string(),
        data: "Testing generic interface".to_string(),
    };

    broker.publish_json(topic, &event).await?;

    if let Some(msg) = tokio::time::timeout(
        tokio::time::Duration::from_secs(2),
        subscriber.next()
    ).await? {
        let received: Event = msg.message.deserialize()?;
        println!("  Generic test passed: {:?}", received);
    }

    Ok(())
}
