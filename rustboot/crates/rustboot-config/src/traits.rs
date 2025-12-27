//! Configuration traits

/// Trait for types that can be merged
/// 
/// Implement this trait to customize how configuration values are merged.
/// By default, values from `other` completely replace values in `self`.
pub trait Mergeable {
    /// Merge another config into this one
    /// Values from `other` override values in `self`
    fn merge(&mut self, other: Self);
}

// Note: Users should implement Mergeable on their config types.
// For simple replacement semantics, just assign other to self:
// 
// impl Mergeable for MyConfig {
//     fn merge(&mut self, other: Self) {
//         *self = other;
//     }
// }
