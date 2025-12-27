//! Gzip compression

use crate::error::CompressionResult;
use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::io::Read;

/// Compress data using gzip
pub fn gzip_compress(data: &[u8]) -> CompressionResult<Vec<u8>> {
    let mut encoder = GzEncoder::new(data, Compression::default());
    let mut compressed = Vec::new();
    encoder.read_to_end(&mut compressed)?;
    Ok(compressed)
}

/// Decompress gzip data
pub fn gzip_decompress(data: &[u8]) -> CompressionResult<Vec<u8>> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gzip_roundtrip() {
        let original = b"Hello, World! This is a test of gzip compression.";
        let compressed = gzip_compress(original).unwrap();
        let decompressed = gzip_decompress(&compressed).unwrap();
        assert_eq!(original.as_slice(), decompressed.as_slice());
    }

    #[test]
    fn test_gzip_compression_reduces_size() {
        let original = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let compressed = gzip_compress(original).unwrap();
        assert!(compressed.len() < original.len());
    }
}
