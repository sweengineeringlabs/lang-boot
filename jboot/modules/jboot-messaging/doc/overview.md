# Messaging Module Overview

## WHAT: Message Queue Abstractions

Event bus, pub/sub patterns, and message queue integrations.

Key capabilities:
- **Event Bus** - In-process event distribution
- **Pub/Sub** - Publish/subscribe patterns
- **Message Queues** - RabbitMQ, Kafka abstractions
- **Dead Letter** - Failed message handling

## WHY: Decoupled Communication

**Problems Solved**:
1. **Tight Coupling** - Event-driven architecture
2. **Async Processing** - Background job queues
3. **Reliability** - Message persistence

**When to Use**: Microservices, async processing

## HOW: Usage Guide

```java
// Event bus
var bus = EventBus.create();

bus.subscribe(UserCreated.class, event -> {
    sendWelcomeEmail(event.user());
});

bus.publish(new UserCreated(user));

// Message queue
var queue = MessageQueue.connect("amqp://...");
queue.send("orders", order);
queue.consume("orders", this::processOrder);
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-serialization | Message encoding |
| jboot-observability | Message tracing |

---

**Status**: Stable
