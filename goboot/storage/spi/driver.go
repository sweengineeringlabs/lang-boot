// Package spi contains the Service Provider Interface for the storage module.
package spi

import (
	"context"
	"io"
	"time"

	"dev.engineeringlabs/goboot/storage/api"
)

// StorageDriver is the interface for storage drivers.
//
// Implement this for S3, GCS, Azure Blob, etc.
//
// Example:
//
//	type S3Driver struct {
//	    client *s3.Client
//	    bucket string
//	}
//
//	func (d *S3Driver) Name() string {
//	    return "s3"
//	}
type StorageDriver interface {
	// Name returns the driver name.
	Name() string

	// Open opens a connection to the storage.
	Open(ctx context.Context, config map[string]string) (api.Storage, error)
}

// Uploader is the interface for multipart uploads.
type Uploader interface {
	// StartUpload starts a multipart upload.
	StartUpload(ctx context.Context, path string) (uploadID string, err error)

	// UploadPart uploads a part.
	UploadPart(ctx context.Context, uploadID string, partNumber int, data io.Reader) (etag string, err error)

	// CompleteUpload completes the upload.
	CompleteUpload(ctx context.Context, uploadID string, parts []UploadPart) error

	// AbortUpload aborts the upload.
	AbortUpload(ctx context.Context, uploadID string) error
}

// UploadPart represents a part in a multipart upload.
type UploadPart struct {
	PartNumber int
	ETag       string
	Size       int64
}

// Versioning is the interface for version-enabled storage.
type Versioning interface {
	// GetVersion gets a specific version of a file.
	GetVersion(ctx context.Context, path, versionID string) (io.ReadCloser, error)

	// ListVersions lists all versions of a file.
	ListVersions(ctx context.Context, path string) ([]FileVersion, error)

	// DeleteVersion deletes a specific version.
	DeleteVersion(ctx context.Context, path, versionID string) error
}

// FileVersion represents a file version.
type FileVersion struct {
	VersionID string
	IsLatest  bool
	ModTime   time.Time
	Size      int64
}

// Lifecycle is the interface for lifecycle management.
type Lifecycle interface {
	// SetLifecycle sets lifecycle rules.
	SetLifecycle(ctx context.Context, rules []LifecycleRule) error

	// GetLifecycle gets lifecycle rules.
	GetLifecycle(ctx context.Context) ([]LifecycleRule, error)
}

// LifecycleRule represents a lifecycle rule.
type LifecycleRule struct {
	ID         string
	Prefix     string
	ExpiryDays int
	Enabled    bool
}
