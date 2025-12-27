//! Cache Usage Example

use dev_engineeringlabs_rustboot_cache::*;
use std::time::Duration;

fn main() {
    println!("=== Rustboot Cache Example ===\n");
    
    let cache: InMemoryCache<String, String> = InMemoryCache::new();
    
    // Cache user data
    println!("Caching user data...");
    cache.set_with_ttl(
        "user:123".to_string(), 
        "John Doe <john@example.com>".to_string(),
        Duration::from_secs(300)
    ).unwrap();
    
    // Retrieve from cache
    if let Some(user) = cache.get(&"user:123".to_string()).unwrap() {
        println!("✓ Cache hit: {}", user);
    }
    
    // Cache API response
    println!("\nCaching API response...");
    cache.set_with_ttl(
        "api:weather".to_string(), 
        "72.5°F, Sunny".to_string(),
        Duration::from_secs(600)
    ).unwrap();
    
    if let Some(weather) = cache.get(&"api:weather".to_string()).unwrap() {
        println!("✓ Weather: {}", weather);
    }
    
    // Test simple set (no TTL)
    println!("\nCaching session...");
    cache.set("session:abc123".to_string(), "active".to_string()).unwrap();
    
    if cache.contains(&"session:abc123".to_string()) {
        println!("✓ Session is cached");
    }
    
    // Clear cache
    cache.clear().unwrap();
    println!("\n✓ Cache cleared");
    
    println!("\n=== Cache Complete ===");
}
