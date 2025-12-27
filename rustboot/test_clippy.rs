// Testing Clippy lint levels
use std::collections::HashMap;

// This will trigger type_complexity lint
pub struct ComplexType {
    // Very complex nested type
    pub data: HashMap<
        String,
        Vec<
            Result<
                Option<Box<dyn Fn(&str) -> Result<i32, String> + Send + Sync>>,
                std::io::Error,
            >,
        >,
    >,
}

fn main() {
    println!("Testing Clippy lint levels");
}
