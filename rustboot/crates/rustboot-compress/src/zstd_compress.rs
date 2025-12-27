//! Zstd compression

use crate::error::CompressionResult;

/// Compress data using zstd
pub fn zstd_compress(data: &[u8], level: i32) -> CompressionResult<Vec<u8>> {
    zstd::encode_all(data, level).map_err(Into::into)
}

/// Decompress zstd data
pub fn zstd_decompress(data: &[u8]) -> CompressionResult<Vec<u8>> {
    zstd::decode_all(data).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zstd_roundtrip() {
        let original = b"Hello, World! This is a test of zstd compression.";
        let compressed = zstd_compress(original, 3).unwrap();
        let decompressed = zstd_decompress(&compressed).unwrap();
        assert_eq!(original.as_slice(), decompressed.as_slice());
    }

    #[test]
    fn test_zstd_compression_reduces_size() {
        let original = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let compressed = zstd_compress(original, 3).unwrap();
        assert!(compressed.len() < original.len());
    }
}
