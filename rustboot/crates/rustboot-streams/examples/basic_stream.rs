//! Basic stream example
//!
//! Demonstrates creating and using event streams.

use dev_engineeringlabs_rustboot_streams::*;
use futures::StreamExt;

#[derive(Debug, Clone)]
enum AppEvent {
    UserLoggedIn(String),
    DataProcessed(usize),
    TaskCompleted,
}

#[tokio::main]
async fn main() {
    println!("=== Basic Stream Example ===\n");

    // Create a stream with default buffer
    let (sender, mut stream) = create_stream::<AppEvent>();

    // Spawn a task to send events
    let send_handle = tokio::spawn(async move {
        println!("Sending events...");
        
        sender.send(AppEvent::UserLoggedIn("alice".to_string())).await.unwrap();
        println!("  ✓ Sent: UserLoggedIn");
        
        sender.send(AppEvent::DataProcessed(100)).await.unwrap();
        println!("  ✓ Sent: DataProcessed(100)");
        
        sender.send(AppEvent::TaskCompleted).await.unwrap();
        println!("  ✓ Sent: TaskCompleted");
        
        println!("\nAll events sent!");
    });

    // Consume the stream
    println!("\nReceiving events:");
    while let Some(event) = stream.next().await {
        match event {
            AppEvent::UserLoggedIn(user) => {
                println!("  ← User logged in: {}", user);
            }
            AppEvent::DataProcessed(count) => {
                println!("  ← Processed {} items", count);
            }
            AppEvent::TaskCompleted => {
                println!("  ← Task completed!");
            }
        }
    }

    send_handle.await.unwrap();
    println!("\n=== Stream Complete ===");
}
