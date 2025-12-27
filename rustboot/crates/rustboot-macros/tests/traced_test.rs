// Test file specifically for the #[traced] macro

#![allow(dead_code)]

use rustboot_macros::traced;

#[test]
fn test_traced_sync_function() {
    #[traced]
    fn simple_sync() -> i32 {
        42
    }

    // Just verify it compiles and runs
    let _result = simple_sync();
}

#[test]
fn test_traced_with_params() {
    #[traced]
    fn with_params(id: u64, name: &str) -> String {
        format!("{}: {}", id, name)
    }

    let _result = with_params(1, "test");
}

#[test]
fn test_traced_with_level() {
    #[traced(level = "DEBUG")]
    fn debug_level() -> bool {
        true
    }

    let _result = debug_level();
}

#[test]
fn test_traced_with_custom_name() {
    #[traced(name = "custom_operation")]
    fn original_name() -> u32 {
        100
    }

    let _result = original_name();
}

#[test]
fn test_traced_with_skip() {
    #[traced(skip = ["password"])]
    fn login(username: &str, password: &str) -> bool {
        !username.is_empty() && !password.is_empty()
    }

    let _result = login("user", "secret");
}

#[tokio::test]
async fn test_traced_async_function() {
    #[traced]
    async fn async_operation() -> Result<String, ()> {
        Ok("success".to_string())
    }

    let _result = async_operation().await;
}

#[tokio::test]
async fn test_traced_async_with_params() {
    #[traced(level = "INFO")]
    async fn fetch_data(id: u64) -> Result<Vec<u8>, ()> {
        Ok(vec![id as u8])
    }

    let _result = fetch_data(42).await;
}
