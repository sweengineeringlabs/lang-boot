//! # Rustboot Streams
//!
//! Async stream utilities for event-driven architectures.
//!
//! This crate provides type-safe wrappers around tokio streams and channels,
//! making it easier to work with async event streams in a clean, functional way.
//!
//! ## Features
//!
//! - **EventStream**: Type alias for boxed async streams
//! - **EventSender**: Wrapper around tokio mpsc sender with ergonomic API
//! - **StreamBuilder**: Builder pattern for creating event streams
//! - **EventStreamExt**: Extension trait for stream operations
//!
//! ## Quick Start
//!
//! ```rust
//! use dev_engineeringlabs_rustboot_streams::*;
//!
//! # async fn example() {
//! // Create a stream with default buffer
//! let (sender, mut stream) = create_stream::<String>();
//!
//! // Send events
//! sender.send("Hello".to_string()).await.unwrap();
//! sender.send("World".to_string()).await.unwrap();
//!
//! // Or use the builder for custom configuration
//! let (sender, stream) = StreamBuilder::<u32>::new()
//!     .buffer_size(1000)
//!     .build();
//! # }
//! ```

use std::pin::Pin;

use futures_core::Stream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

/// Type alias for a boxed async stream of events.
///
/// This is the standard return type for event-producing operations.
/// It allows returning different stream implementations without
/// exposing implementation details.
///
/// # Example
///
/// ```rust
/// use dev_engineeringlabs_rustboot_streams::EventStream;
/// use futures_core::Stream;
///
/// fn create_number_stream() -> EventStream<i32> {
///     let (_, stream) = dev_engineeringlabs_rustboot_streams::create_stream();
///     stream
/// }
/// ```
pub type EventStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;

/// A sender for events in an async stream.
///
/// This wraps a tokio mpsc sender and provides convenience methods
/// for sending events with better ergonomics.
///
/// # Example
///
/// ```rust
/// use dev_engineeringlabs_rustboot_streams::create_stream;
///
/// # async fn example() {
/// let (sender, _stream) = create_stream::<String>();
///
/// // Send events
/// sender.send("event1".to_string()).await.unwrap();
///
/// // Try send without waiting
/// sender.try_send("event2".to_string()).unwrap();
///
/// // Check if receiver is still alive
/// if !sender.is_closed() {
///     sender.send("event3".to_string()).await.unwrap();
/// }
/// # }
/// ```
#[derive(Debug)]
pub struct EventSender<T> {
    tx: mpsc::Sender<T>,
}

impl<T> EventSender<T> {
    /// Create a new event sender from an mpsc sender.
    pub fn new(tx: mpsc::Sender<T>) -> Self {
        Self { tx }
    }

    /// Send an event asynchronously.
    ///
    /// Returns `Ok(())` if the event was sent, or `Err(event)` if the
    /// receiver was dropped.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use dev_engineeringlabs_rustboot_streams::create_stream;
    /// # async fn example() {
    /// let (sender, _stream) = create_stream::<u32>();
    ///
    /// match sender.send(42).await {
    ///     Ok(()) => println!("Sent successfully"),
    ///     Err(val) => println!("Failed to send: {}", val),
    /// }
    /// # }
    /// ```
    #[must_use = "send operation may fail if receiver is dropped"]
    pub async fn send(&self, event: T) -> Result<(), T> {
        self.tx.send(event).await.map_err(|e| e.0)
    }

    /// Try to send an event without waiting.
    ///
    /// Returns `Ok(())` if the event was sent, or `Err(event)` if the
    /// channel is full or closed.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use dev_engineeringlabs_rustboot_streams::create_stream;
    /// let (sender, _stream) = create_stream::<u32>();
    ///
    /// match sender.try_send(42) {
    ///     Ok(()) => println!("Sent immediately"),
    ///     Err(val) => println!("Could not send: {}", val),
    /// }
    /// ```
    #[must_use = "send operation may fail if channel is full or closed"]
    pub fn try_send(&self, event: T) -> Result<(), T> {
        self.tx.try_send(event).map_err(|e| match e {
            mpsc::error::TrySendError::Full(v) => v,
            mpsc::error::TrySendError::Closed(v) => v,
        })
    }

    /// Check if the receiver has been dropped.
    ///
    /// Returns `true` if the receiver is no longer available.
    pub fn is_closed(&self) -> bool {
        self.tx.is_closed()
    }

    /// Get the current capacity of the underlying channel.
    ///
    /// Returns the number of additional messages the channel can hold.
    pub fn capacity(&self) -> usize {
        self.tx.capacity()
    }

    /// Get access to the underlying mpsc sender.
    ///
    /// This is useful for advanced use cases where you need direct
    /// access to the tokio mpsc sender.
    pub fn inner(&self) -> &mpsc::Sender<T> {
        &self.tx
    }
}

impl<T> Clone for EventSender<T> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

/// Builder for creating event streams with custom configuration.
///
/// # Example
///
/// ```rust
/// use dev_engineeringlabs_rustboot_streams::StreamBuilder;
///
/// #[derive(Debug, Clone)]
/// enum MyEvent {
///     Started,
///     Progress(u32),
///     Complete,
/// }
///
/// # async fn example() {
/// let (sender, stream) = StreamBuilder::<MyEvent>::new()
///     .buffer_size(1000)
///     .build();
///
/// // Send events
/// sender.send(MyEvent::Started).await.unwrap();
/// sender.send(MyEvent::Progress(50)).await.unwrap();
/// sender.send(MyEvent::Complete).await.unwrap();
/// # }
/// ```
pub struct StreamBuilder<T> {
    buffer_size: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Send + 'static> StreamBuilder<T> {
    /// Create a new stream builder with default settings.
    ///
    /// Default buffer size is 100.
    pub fn new() -> Self {
        Self {
            buffer_size: 100,
            _marker: std::marker::PhantomData,
        }
    }

    /// Set the buffer size for the underlying channel.
    ///
    /// This determines how many events can be queued before
    /// senders start blocking.
    ///
    /// Default is 100.
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// Build the stream and sender.
    ///
    /// Returns a tuple of (sender, stream) ready to use.
    pub fn build(self) -> (EventSender<T>, EventStream<T>) {
        let (tx, rx) = mpsc::channel(self.buffer_size);
        let sender = EventSender::new(tx);
        let stream: EventStream<T> = Box::pin(ReceiverStream::new(rx));
        (sender, stream)
    }
}

impl<T: Send + 'static> Default for StreamBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Create an event stream with the default buffer size (100).
///
/// This is a convenience function for simple use cases where
/// you don't need custom configuration.
///
/// # Example
///
/// ```rust
/// use dev_engineeringlabs_rustboot_streams::create_stream;
///
/// # async fn example() {
/// let (sender, stream) = create_stream::<String>();
/// sender.send("Hello".to_string()).await.unwrap();
/// # }
/// ```
pub fn create_stream<T: Send + 'static>() -> (EventSender<T>, EventStream<T>) {
    StreamBuilder::<T>::new().build()
}

/// Create an event stream with a specific buffer size.
///
/// # Example
///
/// ```rust
/// use dev_engineeringlabs_rustboot_streams::create_stream_with_buffer;
///
/// # async fn example() {
/// // Create stream with large buffer for high-throughput scenarios
/// let (sender, stream) = create_stream_with_buffer::<u32>(10000);
/// # }
/// ```
pub fn create_stream_with_buffer<T: Send + 'static>(
    buffer_size: usize,
) -> (EventSender<T>, EventStream<T>) {
    StreamBuilder::<T>::new().buffer_size(buffer_size).build()
}

/// Extension trait for working with event streams.
///
/// Provides convenient methods for converting streams into
/// the standard EventStream type.
pub trait EventStreamExt<T>: Stream<Item = T> + Send + 'static {
    /// Convert this stream into a boxed EventStream.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dev_engineeringlabs_rustboot_streams::EventStreamExt;
    /// use tokio_stream::StreamExt;
    ///
    /// # async fn example() {
    /// let stream = tokio_stream::iter(vec![1, 2, 3]);
    /// let boxed = stream.boxed_stream();
    /// # }
    /// ```
    fn boxed_stream(self) -> EventStream<T>
    where
        Self: Sized,
    {
        Box::pin(self)
    }
}

impl<S, T> EventStreamExt<T> for S where S: Stream<Item = T> + Send + 'static {}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[derive(Debug, Clone, PartialEq)]
    enum TestEvent {
        Start,
        Progress(u32),
        Complete,
    }

    #[tokio::test]
    async fn test_stream_builder() {
        let (sender, mut stream) = StreamBuilder::<TestEvent>::new().buffer_size(10).build();

        sender.send(TestEvent::Start).await.unwrap();
        sender.send(TestEvent::Progress(50)).await.unwrap();
        sender.send(TestEvent::Complete).await.unwrap();
        drop(sender);

        let events: Vec<_> = stream.collect().await;
        assert_eq!(
            events,
            vec![
                TestEvent::Start,
                TestEvent::Progress(50),
                TestEvent::Complete,
            ]
        );
    }

    #[tokio::test]
    async fn test_create_stream() {
        let (sender, mut stream) = create_stream::<String>();

        sender.send("Hello".to_string()).await.unwrap();
        sender.send("World".to_string()).await.unwrap();
        drop(sender);

        let events: Vec<_> = stream.collect().await;
        assert_eq!(events, vec!["Hello", "World"]);
    }

    #[tokio::test]
    async fn test_sender_clone() {
        let (sender, mut stream) = create_stream::<u32>();

        let sender2 = sender.clone();
        sender.send(1).await.unwrap();
        sender2.send(2).await.unwrap();
        drop(sender);
        drop(sender2);

        let events: Vec<_> = stream.collect().await;
        assert_eq!(events, vec![1, 2]);
    }

    #[tokio::test]
    async fn test_try_send() {
        let (sender, _stream) = create_stream_with_buffer::<u32>(1);

        // First send should succeed
        assert!(sender.try_send(1).is_ok());
        // Second might fail if buffer is full
        let _ = sender.try_send(2);
    }

    #[tokio::test]
    async fn test_is_closed() {
        let (sender, stream) = create_stream::<u32>();

        assert!(!sender.is_closed());
        drop(stream);
        // Give tokio time to process the drop
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        assert!(sender.is_closed());
    }

    #[tokio::test]
    async fn test_capacity() {
        let buffer_size = 50;
        let (sender, _stream) = create_stream_with_buffer::<u32>(buffer_size);

        assert_eq!(sender.capacity(), buffer_size);
    }
}
