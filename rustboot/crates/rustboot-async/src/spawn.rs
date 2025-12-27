//! Task spawning utilities

use std::future::Future;

/// Spawn an async task on the runtime
pub fn spawn_task<F, T>(future: F) -> tokio::task::JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    tokio::spawn(future)
}

/// Spawn a blocking task on a dedicated thread pool
pub fn spawn_blocking_task<F, T>(f: F) -> tokio::task::JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_spawn_task() {
        let handle = spawn_task(async { 42 });
        let result = handle.await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_spawn_blocking() {
        let handle = spawn_blocking_task(|| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            "done"
        });
        let result = handle.await.unwrap();
        assert_eq!(result, "done");
    }
}
