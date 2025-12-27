# rustboot-messaging

A flexible messaging abstraction for pub/sub patterns and message queues with support for multiple broker backends.

## Features

- **In-Memory Bus**: Always available, perfect for testing and single-process applications
- **Redis**: Pub/sub and list-based queues with acknowledgments (feature: `redis-broker`)
- **RabbitMQ**: Full AMQP support with exchanges, queues, and routing (feature: `rabbitmq-broker`)
- **Kafka**: Apache Kafka with consumer groups and partitioning (feature: `kafka-broker`)

## Installation

```toml
[dependencies]
dev-engineeringlabs-rustboot-messaging = "0.1.0"

# Enable specific brokers as needed:
dev-engineeringlabs-rustboot-messaging = { version = "0.1.0", features = ["redis-broker"] }
dev-engineeringlabs-rustboot-messaging = { version = "0.1.0", features = ["all-brokers"] }
```

## Usage

### In-Memory Bus (Simple Pub/Sub)

```rust
use dev_engineeringlabs_rustboot_messaging::{InMemoryBus, Message, Publisher, Subscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bus = InMemoryBus::new();

    // Subscribe to a topic
    let mut subscriber = bus.subscribe("events").await?;

    // Publish a message
    let message = Message::new("events", b"Hello World".to_vec());
    bus.publish(message).await?;

    // Receive the message
    if let Some(msg) = subscriber.next().await {
        println!("Received: {:?}", String::from_utf8_lossy(&msg.payload));
    }

    Ok(())
}
```

### Redis Broker

Requires Redis server running and the `redis-broker` feature enabled.

```rust
use dev_engineeringlabs_rustboot_messaging::{
    RedisBroker, MessageBroker, RedisQueueExt, Message,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let broker = RedisBroker::connect("redis://127.0.0.1:6379").await?;

    // Pub/Sub Pattern
    let mut subscriber = broker.subscribe("events").await?;
    broker.publish(Message::new("events", b"data".to_vec())).await?;

    // Queue Pattern with Acknowledgments
    let queue = "tasks";
    broker.push_to_queue(queue, Message::new(queue, b"task".to_vec())).await?;

    let mut consumer = broker.consume(queue).await?;
    if let Some(msg) = consumer.next().await {
        println!("Processing: {:?}", msg.message);
        msg.ack().await?; // Acknowledge successful processing
    }

    Ok(())
}
```

### RabbitMQ Broker

Requires RabbitMQ server running and the `rabbitmq-broker` feature enabled.

```rust
use dev_engineeringlabs_rustboot_messaging::{
    RabbitMQBroker, MessageBroker, RabbitMQExt, Message,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let broker = RabbitMQBroker::connect("amqp://127.0.0.1:5672").await?;

    // Fanout Exchange (Broadcast)
    let mut sub = broker.subscribe("notifications").await?;
    broker.publish(Message::new("notifications", b"alert".to_vec())).await?;

    // Direct Queue
    let queue = "tasks";
    broker.declare_queue(queue, true).await?;
    broker.publish_to_queue(queue, Message::new(queue, b"task".to_vec())).await?;

    let mut consumer = broker.consume(queue).await?;
    if let Some(msg) = consumer.next().await {
        msg.ack().await?;
    }

    // Topic Exchange with Routing
    broker.declare_exchange("logs", "topic", false).await?;
    broker.bind_queue("error_logs", "logs", "*.error").await?;
    broker.publish_to_exchange("logs", "app.error",
        Message::new("logs", b"error message".to_vec())).await?;

    Ok(())
}
```

### Kafka Broker

Requires Kafka broker running and the `kafka-broker` feature enabled.

```rust
use dev_engineeringlabs_rustboot_messaging::{
    KafkaBroker, MessageBroker, KafkaExt, Message,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let broker = KafkaBroker::connect("localhost:9092").await?;

    let topic = "events";

    // Publish
    broker.publish(Message::new(topic, b"event data".to_vec())).await?;

    // Subscribe (each subscriber gets all messages)
    let mut subscriber = broker.subscribe(topic).await?;

    // Consume (consumer group - messages are distributed)
    let mut consumer = broker.consume(topic).await?;

    if let Some(msg) = consumer.next().await {
        println!("Received: {:?}", msg.message);
        msg.ack().await?; // Commit offset
    }

    // Publish with partition key
    broker.publish_with_key(topic, "user-123",
        Message::new(topic, b"user event".to_vec())).await?;

    Ok(())
}
```

### JSON Messages

All brokers support JSON serialization:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Event {
    id: u64,
    name: String,
}

let event = Event { id: 1, name: "test".to_string() };
broker.publish_json("events", &event).await?;

let mut stream = broker.subscribe("events").await?;
if let Some(msg) = stream.next().await {
    let received: Event = msg.message.deserialize()?;
    println!("Received: {:?}", received);
}
```

### Message Acknowledgments

All broker implementations support acknowledgments for reliable message processing:

```rust
let mut consumer = broker.consume("queue").await?;

if let Some(msg) = consumer.next().await {
    match process_message(&msg.message) {
        Ok(_) => msg.ack().await?,      // Success - remove from queue
        Err(_) => msg.nack().await?,    // Failure - requeue for retry
    }
}
```

### Delivery Modes

Configure delivery guarantees:

```rust
use dev_engineeringlabs_rustboot_messaging::{BrokerConfig, DeliveryMode};

let config = BrokerConfig::new("redis://localhost")
    .with_delivery_mode(DeliveryMode::AtLeastOnce)
    .with_timeout(5000);

let broker = RedisBroker::new(config).await?;
```

Available modes:
- `AtMostOnce`: Fire and forget, no acknowledgments
- `AtLeastOnce`: Guaranteed delivery with acknowledgments (default)
- `ExactlyOnce`: Exactly-once semantics (requires deduplication)

## Architecture

The crate provides a unified `MessageBroker` trait that all implementations follow:

```rust
#[async_trait]
pub trait MessageBroker: Send + Sync {
    async fn publish(&self, message: Message) -> MessagingResult<()>;
    async fn subscribe(&self, topic: &str) -> MessagingResult<Box<dyn AckMessageStream>>;
    async fn consume(&self, queue: &str) -> MessagingResult<Box<dyn AckMessageStream>>;
}
```

This allows you to write broker-agnostic code:

```rust
async fn process_messages<B: MessageBroker>(broker: &B, topic: &str) {
    let mut stream = broker.subscribe(topic).await.unwrap();
    while let Some(msg) = stream.next().await {
        println!("Processing: {:?}", msg.message);
    }
}
```

## Examples

Run the examples with:

```bash
# In-memory bus
cargo run --example messaging_basic

# Redis (requires Redis server)
cargo run --example redis_example --features redis-broker

# RabbitMQ (requires RabbitMQ server)
cargo run --example rabbitmq_example --features rabbitmq-broker

# Kafka (requires Kafka broker)
cargo run --example kafka_example --features kafka-broker

# Broker comparison
cargo run --example broker_comparison --features all-brokers
```

## Testing

Run tests:

```bash
# Unit tests (no external dependencies)
cargo test

# Integration tests (requires broker servers)
cargo test --all-features -- --ignored
```

## License

MIT
