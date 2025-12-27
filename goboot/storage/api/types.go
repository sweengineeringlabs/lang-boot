// Package api contains the public interfaces and types for the storage module.
package api

import (
	"context"
	"io"
	"time"
)

// FileInfo represents information about a file.
type FileInfo struct {
	Name         string
	Path         string
	Size         int64
	ContentType  string
	ModTime      time.Time
	IsDir        bool
	Metadata     map[string]string
}

// ListOptions configures file listing.
type ListOptions struct {
	Prefix    string
	Delimiter string
	MaxKeys   int
	Cursor    string
}

// ListResult represents the result of a list operation.
type ListResult struct {
	Files         []FileInfo
	Prefixes      []string
	NextCursor    string
	IsTruncated   bool
}

// Storage is the interface for file storage.
type Storage interface {
	// Put stores a file.
	Put(ctx context.Context, path string, reader io.Reader, options ...PutOption) error

	// Get retrieves a file.
	Get(ctx context.Context, path string) (io.ReadCloser, error)

	// Delete deletes a file.
	Delete(ctx context.Context, path string) error

	// Exists checks if a file exists.
	Exists(ctx context.Context, path string) (bool, error)

	// Info returns file information.
	Info(ctx context.Context, path string) (*FileInfo, error)

	// List lists files in a path.
	List(ctx context.Context, options ListOptions) (*ListResult, error)

	// Copy copies a file.
	Copy(ctx context.Context, src, dst string) error

	// Move moves a file.
	Move(ctx context.Context, src, dst string) error
}

// PutOption is a functional option for Put operations.
type PutOption func(*PutOptions)

// PutOptions configures Put operations.
type PutOptions struct {
	ContentType string
	Metadata    map[string]string
	ACL         string
}

// WithContentType sets the content type.
func WithContentType(contentType string) PutOption {
	return func(o *PutOptions) {
		o.ContentType = contentType
	}
}

// WithMetadata sets metadata.
func WithMetadata(metadata map[string]string) PutOption {
	return func(o *PutOptions) {
		o.Metadata = metadata
	}
}

// WithACL sets the ACL.
func WithACL(acl string) PutOption {
	return func(o *PutOptions) {
		o.ACL = acl
	}
}

// ApplyPutOptions applies put options.
func ApplyPutOptions(options []PutOption) PutOptions {
	opts := PutOptions{
		Metadata: make(map[string]string),
	}
	for _, o := range options {
		o(&opts)
	}
	return opts
}

// URLSigner is the interface for generating signed URLs.
type URLSigner interface {
	// SignedURL generates a signed URL for a file.
	SignedURL(ctx context.Context, path string, expiry time.Duration) (string, error)
}

// BucketManager is the interface for bucket/container management.
type BucketManager interface {
	// CreateBucket creates a bucket.
	CreateBucket(ctx context.Context, name string) error

	// DeleteBucket deletes a bucket.
	DeleteBucket(ctx context.Context, name string) error

	// ListBuckets lists all buckets.
	ListBuckets(ctx context.Context) ([]string, error)
}
