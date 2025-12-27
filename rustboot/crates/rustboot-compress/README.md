# rustboot-compress

Gzip and zstd compression.

```rust
use dev_engineeringlabs_rustboot_compress::*;

// Gzip
let compressed = gzip_compress(data)?;
let decompressed = gzip_decompress(&compressed)?;

// Zstd (better compression)
let compressed = zstd_compress(data, 3)?;
let decompressed = zstd_decompress(&compressed)?;
```

See [overview](docs/overview.md).
