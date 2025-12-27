//! Kafka broker example demonstrating topics, partitions, and consumer groups.
//!
//! Run with: cargo run --example kafka_example --features kafka-broker
//!
//! Requires Kafka broker running on localhost:9092

use dev_engineeringlabs_rustboot_messaging::{
    AckMessageStream, BrokerConfig, Message, MessageBroker,
    KafkaBroker, KafkaExt,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ClickEvent {
    user_id: String,
    page: String,
    timestamp: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot Kafka Messaging Example ===\n");

    // Example 1: Basic Produce/Consume
    println!("1. Basic Topic Publishing and Consumption");
    basic_example().await?;

    // Example 2: Consumer Groups (Load Balancing)
    println!("\n2. Consumer Groups (competing consumers)");
    consumer_group_example().await?;

    // Example 3: Partitioning with Keys
    println!("\n3. Partition Keys for Ordering");
    partition_key_example().await?;

    // Example 4: Offset Management and Acknowledgments
    println!("\n4. Offset Management and Acknowledgments");
    offset_management_example().await?;

    Ok(())
}

async fn basic_example() -> Result<(), Box<dyn std::error::Error>> {
    let broker = KafkaBroker::connect("localhost:9092").await?;

    let topic = "events";

    // Publish messages
    for i in 1..=3 {
        let message = Message::new(topic, format!("Event {}", i).into_bytes());
        broker.publish(message).await?;
        println!("  Published: Event {}", i);
    }

    // Subscribe (each subscriber gets all messages)
    let mut subscriber = broker.subscribe(topic).await?;

    // Consume messages
    for i in 1..=3 {
        if let Some(msg) = tokio::time::timeout(
            tokio::time::Duration::from_secs(5),
            subscriber.next()
        ).await.ok().flatten() {
            println!("  Consumed: {:?}", String::from_utf8_lossy(&msg.message.payload));
            msg.ack().await?;
        } else {
            println!("  Timeout waiting for message {}", i);
        }
    }

    Ok(())
}

async fn consumer_group_example() -> Result<(), Box<dyn std::error::Error>> {
    let broker = KafkaBroker::connect("localhost:9092").await?;

    let topic = "tasks";

    // Publish multiple messages
    println!("  Publishing 6 tasks...");
    for i in 1..=6 {
        let message = Message::new(topic, format!("Task {}", i).into_bytes());
        broker.publish(message).await?;
    }

    // Create two consumers in the same group
    // They will share the messages (load balancing)
    let mut consumer1 = broker.consume(topic).await?;
    let mut consumer2 = broker.consume(topic).await?;

    println!("  Consumer 1 and 2 competing for tasks...");

    // Simulate consumers processing messages concurrently
    let handle1 = tokio::spawn(async move {
        let mut count = 0;
        while count < 3 {
            if let Some(msg) = tokio::time::timeout(
                tokio::time::Duration::from_secs(2),
                consumer1.next()
            ).await.ok().flatten() {
                println!("  Consumer 1 processing: {:?}", String::from_utf8_lossy(&msg.message.payload));
                msg.ack().await.ok();
                count += 1;
            } else {
                break;
            }
        }
        count
    });

    let handle2 = tokio::spawn(async move {
        let mut count = 0;
        while count < 3 {
            if let Some(msg) = tokio::time::timeout(
                tokio::time::Duration::from_secs(2),
                consumer2.next()
            ).await.ok().flatten() {
                println!("  Consumer 2 processing: {:?}", String::from_utf8_lossy(&msg.message.payload));
                msg.ack().await.ok();
                count += 1;
            } else {
                break;
            }
        }
        count
    });

    let (count1, count2) = tokio::join!(handle1, handle2);
    println!("  Consumer 1 processed: {} tasks", count1.unwrap_or(0));
    println!("  Consumer 2 processed: {} tasks", count2.unwrap_or(0));

    Ok(())
}

async fn partition_key_example() -> Result<(), Box<dyn std::error::Error>> {
    let broker = KafkaBroker::connect("localhost:9092").await?;

    let topic = "user_events";

    // Publish events for different users
    // Events with the same key go to the same partition (ordering guaranteed)
    let users = vec!["alice", "bob", "alice", "bob", "alice"];

    println!("  Publishing events with partition keys...");
    for (i, user) in users.iter().enumerate() {
        let event = ClickEvent {
            user_id: user.to_string(),
            page: format!("/page{}", i),
            timestamp: i as u64,
        };

        let message = Message::new(topic, serde_json::to_vec(&event)?);
        broker.publish_with_key(topic, user, message).await?;
        println!("  Published event for {}: page{}", user, i);
    }

    // Consume events
    let mut consumer = broker.subscribe(topic).await?;

    println!("  Consuming events (ordering preserved per user)...");
    for _ in 0..5 {
        if let Some(msg) = tokio::time::timeout(
            tokio::time::Duration::from_secs(5),
            consumer.next()
        ).await.ok().flatten() {
            let event: ClickEvent = msg.message.deserialize()?;
            println!("  Received: {} clicked {}", event.user_id, event.page);
            msg.ack().await?;
        }
    }

    Ok(())
}

async fn offset_management_example() -> Result<(), Box<dyn std::error::Error>> {
    let broker = KafkaBroker::connect("localhost:9092").await?;

    let topic = "offset_test";

    // Publish messages
    println!("  Publishing messages...");
    for i in 1..=5 {
        let message = Message::new(topic, format!("Message {}", i).into_bytes());
        broker.publish(message).await?;
    }

    // Consumer with manual offset management
    let mut consumer = broker.consume(topic).await?;

    println!("  Processing messages with manual acknowledgment...");

    // Process first 3 messages successfully
    for i in 1..=3 {
        if let Some(msg) = tokio::time::timeout(
            tokio::time::Duration::from_secs(5),
            consumer.next()
        ).await.ok().flatten() {
            println!("  Processing: {:?}", String::from_utf8_lossy(&msg.message.payload));
            msg.ack().await?;
            println!("  Committed offset for message {}", i);
        }
    }

    // Simulate failure on 4th message
    if let Some(msg) = tokio::time::timeout(
        tokio::time::Duration::from_secs(5),
        consumer.next()
    ).await.ok().flatten() {
        println!("  Processing: {:?}", String::from_utf8_lossy(&msg.message.payload));
        println!("  Failed to process - sending nack (will seek back)");
        msg.nack().await?;
    }

    // The message should be reprocessed
    if let Some(msg) = tokio::time::timeout(
        tokio::time::Duration::from_secs(5),
        consumer.next()
    ).await.ok().flatten() {
        println!("  Reprocessing: {:?}", String::from_utf8_lossy(&msg.message.payload));
        msg.ack().await?;
        println!("  Successfully processed on retry");
    }

    Ok(())
}
