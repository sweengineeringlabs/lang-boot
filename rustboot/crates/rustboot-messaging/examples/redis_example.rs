//! Redis broker example demonstrating pub/sub and queue patterns.
//!
//! Run with: cargo run --example redis_example --features redis-broker
//!
//! Requires Redis server running on localhost:6379

use dev_engineeringlabs_rustboot_messaging::{
    AckMessageStream, BrokerConfig, DeliveryMode, Message, MessageBroker,
    RedisBroker, RedisQueueExt,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct OrderEvent {
    order_id: String,
    customer: String,
    amount: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot Redis Messaging Example ===\n");

    // Example 1: Pub/Sub Pattern
    println!("1. Pub/Sub Pattern (broadcast to multiple subscribers)");
    pubsub_example().await?;

    // Example 2: Queue Pattern with Acknowledgments
    println!("\n2. Queue Pattern (competing consumers with acks)");
    queue_example().await?;

    // Example 3: JSON Messages
    println!("\n3. Structured JSON Messages");
    json_example().await?;

    // Example 4: Delivery Modes
    println!("\n4. Different Delivery Modes");
    delivery_modes_example().await?;

    Ok(())
}

async fn pubsub_example() -> Result<(), Box<dyn std::error::Error>> {
    let broker = RedisBroker::connect("redis://127.0.0.1:6379").await?;

    // Create two subscribers
    let mut subscriber1 = broker.subscribe("events").await?;
    let mut subscriber2 = broker.subscribe("events").await?;

    // Publish a message
    let message = Message::new("events", b"System startup".to_vec());
    broker.publish(message).await?;

    // Both subscribers receive the message
    let msg1 = subscriber1.next().await.expect("No message on subscriber 1");
    let msg2 = subscriber2.next().await.expect("No message on subscriber 2");

    println!("  Subscriber 1 received: {:?}", String::from_utf8_lossy(&msg1.message.payload));
    println!("  Subscriber 2 received: {:?}", String::from_utf8_lossy(&msg2.message.payload));

    Ok(())
}

async fn queue_example() -> Result<(), Box<dyn std::error::Error>> {
    let broker = RedisBroker::connect("redis://127.0.0.1:6379").await?;

    let queue = "orders";

    // Add messages to queue
    for i in 1..=3 {
        let message = Message::new(queue, format!("Order {}", i).into_bytes());
        broker.push_to_queue(queue, message).await?;
    }

    // Create two competing consumers
    let mut consumer1 = broker.consume(queue).await?;
    let mut consumer2 = broker.consume(queue).await?;

    // Consumer 1 processes a message
    if let Some(msg) = consumer1.next().await {
        println!("  Consumer 1 processing: {:?}", String::from_utf8_lossy(&msg.message.payload));
        msg.ack().await?;
        println!("  Consumer 1 acknowledged");
    }

    // Consumer 2 processes a message and nacks it
    if let Some(msg) = consumer2.next().await {
        println!("  Consumer 2 processing: {:?}", String::from_utf8_lossy(&msg.message.payload));
        println!("  Consumer 2 failing - sending nack");
        msg.nack().await?;
    }

    // The nacked message should be redelivered
    if let Some(msg) = consumer1.next().await {
        println!("  Consumer 1 reprocessing nacked message: {:?}", String::from_utf8_lossy(&msg.message.payload));
        msg.ack().await?;
    }

    Ok(())
}

async fn json_example() -> Result<(), Box<dyn std::error::Error>> {
    let broker = RedisBroker::connect("redis://127.0.0.1:6379").await?;

    let mut stream = broker.subscribe("orders").await?;

    // Publish structured data
    let order = OrderEvent {
        order_id: "ORD-001".to_string(),
        customer: "Alice".to_string(),
        amount: 99.99,
    };

    broker.publish_json("orders", &order).await?;

    // Receive and deserialize
    if let Some(msg) = stream.next().await {
        let received_order: OrderEvent = msg.message.deserialize()?;
        println!("  Received order: {:?}", received_order);
        assert_eq!(received_order, order);
    }

    Ok(())
}

async fn delivery_modes_example() -> Result<(), Box<dyn std::error::Error>> {
    // At-most-once delivery (no acknowledgments)
    let config = BrokerConfig::new("redis://127.0.0.1:6379")
        .with_delivery_mode(DeliveryMode::AtMostOnce);

    let broker = RedisBroker::new(config).await?;

    let queue = "fast_queue";
    broker.push_to_queue(queue, Message::new(queue, b"Fast message".to_vec())).await?;

    let mut consumer = broker.consume(queue).await?;
    if let Some(msg) = consumer.next().await {
        println!("  At-most-once message: {:?}", String::from_utf8_lossy(&msg.message.payload));
        // No acknowledgment needed
        assert!(msg.ack_handle.is_none());
    }

    // At-least-once delivery (with acknowledgments)
    let config = BrokerConfig::new("redis://127.0.0.1:6379")
        .with_delivery_mode(DeliveryMode::AtLeastOnce);

    let broker = RedisBroker::new(config).await?;
    let queue2 = "reliable_queue";
    broker.push_to_queue(queue2, Message::new(queue2, b"Reliable message".to_vec())).await?;

    let mut consumer = broker.consume(queue2).await?;
    if let Some(msg) = consumer.next().await {
        println!("  At-least-once message: {:?}", String::from_utf8_lossy(&msg.message.payload));
        // Acknowledgment required
        assert!(msg.ack_handle.is_some());
        msg.ack().await?;
    }

    Ok(())
}
