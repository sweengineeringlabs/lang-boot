# Storage Module Overview

## WHAT: File Storage Abstractions

Local and cloud file storage with unified API.

Key capabilities:
- **Local** - File system storage
- **Cloud** - S3, GCS, Azure Blob
- **Streams** - Streaming upload/download
- **Metadata** - File metadata handling

## WHY: Portable Storage

**Problems Solved**:
1. **Vendor Lock-in** - Unified storage API
2. **Configuration** - Environment-based backends
3. **Large Files** - Streaming support

**When to Use**: File uploads, document storage

## HOW: Usage Guide

```java
var storage = Storage.s3(S3Config.builder()
    .bucket("my-bucket")
    .region("us-east-1")
    .build());

// Upload
storage.put("files/doc.pdf", inputStream);

// Download
InputStream data = storage.get("files/doc.pdf");

// List
List<FileInfo> files = storage.list("files/");
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-config | Storage configuration |
| jboot-security | Access control |

---

**Status**: Stable
