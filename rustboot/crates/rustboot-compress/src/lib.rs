//! Rustboot Compress - Compression utilities

pub mod error;
pub mod gzip;
pub mod zstd_compress;

pub use error::{CompressionError, CompressionResult};
pub use gzip::{gzip_compress, gzip_decompress};
pub use zstd_compress::{zstd_compress, zstd_decompress};
