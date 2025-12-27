# Storage Module Overview

## WHAT: File Storage Abstractions

Local and cloud file storage.

Key capabilities:
- **Local** - File system
- **Cloud** - S3, GCS, Azure
- **Streams** - Streaming I/O
- **Metadata** - File info

## WHY: Portable Storage

**Problems Solved**: Vendor lock-in, configuration

**When to Use**: File uploads, documents

## HOW: Usage Guide

```go
store := storage.NewS3(storage.S3Config{
    Bucket: "my-bucket",
    Region: "us-east-1",
})

// Upload
store.Put(ctx, "files/doc.pdf", reader)

// Download
reader, _ := store.Get(ctx, "files/doc.pdf")

// List
files, _ := store.List(ctx, "files/")
```

---

**Status**: Stable
