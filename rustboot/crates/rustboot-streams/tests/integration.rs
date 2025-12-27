//! Integration tests for rustboot-streams

use dev_engineeringlabs_rustboot_streams::*;
use futures::StreamExt;

#[tokio::test]
async fn test_multiple_senders() {
    let (sender, mut stream) = create_stream::<u32>();

    let sender1 = sender.clone();
    let sender2 = sender.clone();

    // Multiple senders sending concurrently
    tokio::spawn(async move {
        for i in 0..5 {
            sender1.send(i).await.unwrap();
        }
    });

    tokio::spawn(async move {
        for i in 5..10 {
            sender2.send(i).await.unwrap();
        }
    });

    drop(sender); // Drop original

    let mut events = Vec::new();
    while let Some(event) = stream.next().await {
        events.push(event);
    }

    events.sort(); // Events may arrive in any order
    assert_eq!(events, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[tokio::test]
async fn test_stream_builder_custom_buffer() {
    let (sender, mut stream) = StreamBuilder::<String>::new()
        .buffer_size(5)
        .build();

    for i in 0..5 {
        sender.try_send(format!("msg{}", i)).unwrap();
    }

    drop(sender);

    let messages: Vec<_> = stream.collect().await;
    assert_eq!(messages.len(), 5);
}

#[tokio::test]
async fn test_sender_drops_notify_stream() {
    let (sender, mut stream) = create_stream::<i32>();

    sender.send(1).await.unwrap();
    drop(sender);

    // Should get the one message then None
    assert_eq!(stream.next().await, Some(1));
    assert_eq!(stream.next().await, None);
}
