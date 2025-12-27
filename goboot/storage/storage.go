// Package storage provides file storage utilities for the goboot framework.
//
// This module provides:
//   - API layer: Storage interface, FileInfo, ListOptions
//   - Core layer: FileSystemStorage, MemoryStorage
//   - SPI layer: StorageDriver, Uploader, Versioning interfaces
//
// Example:
//
//	import "dev.engineeringlabs/goboot/storage"
//
//	// Filesystem storage
//	fs, _ := storage.NewFileSystemStorage("/data/files")
//	fs.Put(ctx, "documents/report.pdf", reader)
//	reader, _ := fs.Get(ctx, "documents/report.pdf")
//
//	// Memory storage for testing
//	mem := storage.NewMemoryStorage()
//	mem.Put(ctx, "test.txt", strings.NewReader("hello"))
//
//	// List files
//	result, _ := fs.List(ctx, storage.ListOptions{Prefix: "documents/"})
package storage

import (
	"dev.engineeringlabs/goboot/storage/api"
	"dev.engineeringlabs/goboot/storage/core"
	"dev.engineeringlabs/goboot/storage/spi"
)

// Re-export API types
type (
	// FileInfo represents information about a file.
	FileInfo = api.FileInfo
	// ListOptions configures file listing.
	ListOptions = api.ListOptions
	// ListResult represents the result of a list operation.
	ListResult = api.ListResult
	// Storage is the interface for file storage.
	Storage = api.Storage
	// PutOption is a functional option for Put operations.
	PutOption = api.PutOption
	// PutOptions configures Put operations.
	PutOptions = api.PutOptions
	// URLSigner generates signed URLs.
	URLSigner = api.URLSigner
	// BucketManager manages buckets.
	BucketManager = api.BucketManager
)

// Re-export API functions
var (
	WithContentType  = api.WithContentType
	WithMetadata     = api.WithMetadata
	WithACL          = api.WithACL
	ApplyPutOptions  = api.ApplyPutOptions
)

// Re-export Core types
type (
	// FileSystemStorage uses the local filesystem.
	FileSystemStorage = core.FileSystemStorage
	// MemoryStorage uses in-memory storage.
	MemoryStorage = core.MemoryStorage
)

// Re-export Core functions
var (
	NewFileSystemStorage = core.NewFileSystemStorage
	NewMemoryStorage     = core.NewMemoryStorage
)

// Re-export SPI types
type (
	// StorageDriver is the interface for storage drivers.
	StorageDriver = spi.StorageDriver
	// Uploader is the interface for multipart uploads.
	Uploader = spi.Uploader
	// UploadPart represents a part in a multipart upload.
	UploadPart = spi.UploadPart
	// Versioning is the interface for version-enabled storage.
	Versioning = spi.Versioning
	// FileVersion represents a file version.
	FileVersion = spi.FileVersion
	// Lifecycle is the interface for lifecycle management.
	Lifecycle = spi.Lifecycle
	// LifecycleRule represents a lifecycle rule.
	LifecycleRule = spi.LifecycleRule
)
