//! Rustboot FileIO - Safe file operations
//!
//! Atomic writes, temporary files, and safe path utilities.

pub mod atomic;
pub mod error;
pub mod path;

pub use atomic::{write_atomic, write_atomic_with_perms};
pub use error::{FileIoError, FileIoResult};
pub use path::{ensure_dir, safe_join};
