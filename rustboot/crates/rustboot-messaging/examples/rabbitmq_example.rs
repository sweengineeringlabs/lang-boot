//! RabbitMQ broker example demonstrating exchanges, queues, and routing.
//!
//! Run with: cargo run --example rabbitmq_example --features rabbitmq-broker
//!
//! Requires RabbitMQ server running on localhost:5672

use dev_engineeringlabs_rustboot_messaging::{
    BrokerConfig, Message, MessageBroker,
    RabbitMQBroker, RabbitMQExt,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct LogEvent {
    level: String,
    message: String,
    timestamp: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot RabbitMQ Messaging Example ===\n");

    // Example 1: Fanout Exchange (Pub/Sub)
    println!("1. Fanout Exchange (broadcast to all subscribers)");
    fanout_example().await?;

    // Example 2: Direct Exchange (Queue)
    println!("\n2. Direct Exchange (message queue)");
    direct_queue_example().await?;

    // Example 3: Topic Exchange (Routing)
    println!("\n3. Topic Exchange (pattern-based routing)");
    topic_example().await?;

    // Example 4: Acknowledgments and Redelivery
    println!("\n4. Message Acknowledgments and Redelivery");
    ack_example().await?;

    Ok(())
}

async fn fanout_example() -> Result<(), Box<dyn std::error::Error>> {
    let broker = RabbitMQBroker::connect("amqp://127.0.0.1:5672").await?;

    // Subscribe creates a fanout exchange automatically
    let mut sub1 = broker.subscribe("notifications").await?;
    let mut sub2 = broker.subscribe("notifications").await?;

    // Publish to the fanout exchange
    let message = Message::new("notifications", b"New user registered".to_vec());
    broker.publish(message).await?;

    // Both subscribers receive the message
    let msg1 = sub1.next().await.expect("No message on subscriber 1");
    let msg2 = sub2.next().await.expect("No message on subscriber 2");

    println!("  Subscriber 1: {:?}", String::from_utf8_lossy(&msg1.message.payload));
    println!("  Subscriber 2: {:?}", String::from_utf8_lossy(&msg2.message.payload));

    msg1.ack().await?;
    msg2.ack().await?;

    Ok(())
}

async fn direct_queue_example() -> Result<(), Box<dyn std::error::Error>> {
    let broker = RabbitMQBroker::connect("amqp://127.0.0.1:5672").await?;

    let queue = "tasks";

    // Declare the queue
    broker.declare_queue(queue, true).await?;

    // Publish directly to queue (using default exchange)
    let message = Message::new(queue, b"Process data".to_vec());
    broker.publish_to_queue(queue, message).await?;

    // Consume from queue
    let mut consumer = broker.consume(queue).await?;
    if let Some(msg) = consumer.next().await {
        println!("  Consumer received task: {:?}", String::from_utf8_lossy(&msg.message.payload));
        msg.ack().await?;
    }

    Ok(())
}

async fn topic_example() -> Result<(), Box<dyn std::error::Error>> {
    let broker = RabbitMQBroker::connect("amqp://127.0.0.1:5672").await?;

    // Create a topic exchange
    broker.declare_exchange("logs", "topic", false).await?;

    // Create queues for different log levels
    broker.declare_queue("error_logs", false).await?;
    broker.declare_queue("all_logs", false).await?;

    // Bind queues with routing patterns
    broker.bind_queue("error_logs", "logs", "*.error").await?;
    broker.bind_queue("all_logs", "logs", "#").await?;

    // Publish messages with different routing keys
    let log1 = LogEvent {
        level: "error".to_string(),
        message: "Database connection failed".to_string(),
        timestamp: "2024-01-01T12:00:00Z".to_string(),
    };

    let log2 = LogEvent {
        level: "info".to_string(),
        message: "Server started".to_string(),
        timestamp: "2024-01-01T12:00:01Z".to_string(),
    };

    let msg1 = Message::new("logs", serde_json::to_vec(&log1)?);
    let msg2 = Message::new("logs", serde_json::to_vec(&log2)?);

    broker.publish_to_exchange("logs", "app.error", msg1).await?;
    broker.publish_to_exchange("logs", "app.info", msg2).await?;

    println!("  Published error and info logs with routing keys");

    // Consumer on error_logs receives only errors
    let mut error_consumer = broker.consume("error_logs").await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    if let Some(msg) = tokio::time::timeout(
        tokio::time::Duration::from_secs(1),
        error_consumer.next()
    ).await.ok().flatten() {
        let log: LogEvent = msg.message.deserialize()?;
        println!("  Error queue received: {} - {}", log.level, log.message);
        msg.ack().await?;
    }

    // Consumer on all_logs receives everything
    let mut all_consumer = broker.consume("all_logs").await?;
    let mut count = 0;
    while let Ok(Some(msg)) = tokio::time::timeout(
        tokio::time::Duration::from_millis(500),
        all_consumer.next()
    ).await {
        let log: LogEvent = msg.message.deserialize()?;
        println!("  All logs queue received: {} - {}", log.level, log.message);
        msg.ack().await?;
        count += 1;
        if count >= 2 {
            break;
        }
    }

    Ok(())
}

async fn ack_example() -> Result<(), Box<dyn std::error::Error>> {
    let broker = RabbitMQBroker::connect("amqp://127.0.0.1:5672").await?;

    let queue = "ack_test";
    broker.declare_queue(queue, false).await?;

    // Publish a message
    let message = Message::new(queue, b"Important task".to_vec());
    broker.publish_to_queue(queue, message).await?;

    // Consumer 1: nack the message
    let mut consumer1 = broker.consume(queue).await?;
    if let Some(msg) = consumer1.next().await {
        println!("  Consumer 1 received: {:?}", String::from_utf8_lossy(&msg.message.payload));
        println!("  Consumer 1 processing failed - sending nack");
        msg.nack().await?;
    }

    // Consumer 2: message should be redelivered
    let mut consumer2 = broker.consume(queue).await?;
    if let Some(msg) = consumer2.next().await {
        println!("  Consumer 2 received redelivered message: {:?}", String::from_utf8_lossy(&msg.message.payload));
        println!("  Consumer 2 processing succeeded - sending ack");
        msg.ack().await?;
    }

    Ok(())
}
