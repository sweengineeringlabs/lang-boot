//! Database abstraction (L4: Core).
//!
//! Generic database traits for CRUD operations and transactions.

pub mod traits;

// Re-export main types
pub use traits::{Database, DatabaseError, DatabaseResult, Repository, Row, Transaction, Value};
