// Package core contains the implementation details for the storage module.
package core

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"dev.engineeringlabs/goboot/storage/api"
)

// FileSystemStorage implements Storage using the local filesystem.
type FileSystemStorage struct {
	basePath string
}

// NewFileSystemStorage creates a new FileSystemStorage.
func NewFileSystemStorage(basePath string) (*FileSystemStorage, error) {
	// Ensure base path exists
	if err := os.MkdirAll(basePath, 0755); err != nil {
		return nil, fmt.Errorf("failed to create base path: %w", err)
	}
	return &FileSystemStorage{basePath: basePath}, nil
}

// Put stores a file.
func (s *FileSystemStorage) Put(ctx context.Context, path string, reader io.Reader, options ...api.PutOption) error {
	fullPath := filepath.Join(s.basePath, path)

	// Ensure directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	file, err := os.Create(fullPath)
	if err != nil {
		return fmt.Errorf("failed to create file: %w", err)
	}
	defer file.Close()

	if _, err := io.Copy(file, reader); err != nil {
		return fmt.Errorf("failed to write file: %w", err)
	}

	return nil
}

// Get retrieves a file.
func (s *FileSystemStorage) Get(ctx context.Context, path string) (io.ReadCloser, error) {
	fullPath := filepath.Join(s.basePath, path)
	file, err := os.Open(fullPath)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, fmt.Errorf("file not found: %s", path)
		}
		return nil, fmt.Errorf("failed to open file: %w", err)
	}
	return file, nil
}

// Delete deletes a file.
func (s *FileSystemStorage) Delete(ctx context.Context, path string) error {
	fullPath := filepath.Join(s.basePath, path)
	if err := os.Remove(fullPath); err != nil {
		if os.IsNotExist(err) {
			return nil
		}
		return fmt.Errorf("failed to delete file: %w", err)
	}
	return nil
}

// Exists checks if a file exists.
func (s *FileSystemStorage) Exists(ctx context.Context, path string) (bool, error) {
	fullPath := filepath.Join(s.basePath, path)
	_, err := os.Stat(fullPath)
	if err != nil {
		if os.IsNotExist(err) {
			return false, nil
		}
		return false, err
	}
	return true, nil
}

// Info returns file information.
func (s *FileSystemStorage) Info(ctx context.Context, path string) (*api.FileInfo, error) {
	fullPath := filepath.Join(s.basePath, path)
	stat, err := os.Stat(fullPath)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, fmt.Errorf("file not found: %s", path)
		}
		return nil, fmt.Errorf("failed to stat file: %w", err)
	}

	return &api.FileInfo{
		Name:    stat.Name(),
		Path:    path,
		Size:    stat.Size(),
		ModTime: stat.ModTime(),
		IsDir:   stat.IsDir(),
	}, nil
}

// List lists files in a path.
func (s *FileSystemStorage) List(ctx context.Context, options api.ListOptions) (*api.ListResult, error) {
	searchPath := filepath.Join(s.basePath, options.Prefix)
	
	var files []api.FileInfo
	err := filepath.Walk(searchPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil // Skip errors
		}
		
		relPath, _ := filepath.Rel(s.basePath, path)
		if relPath == "." {
			return nil
		}

		files = append(files, api.FileInfo{
			Name:    info.Name(),
			Path:    relPath,
			Size:    info.Size(),
			ModTime: info.ModTime(),
			IsDir:   info.IsDir(),
		})

		if options.MaxKeys > 0 && len(files) >= options.MaxKeys {
			return filepath.SkipAll
		}
		return nil
	})

	if err != nil && !os.IsNotExist(err) {
		return nil, fmt.Errorf("failed to list files: %w", err)
	}

	return &api.ListResult{
		Files: files,
	}, nil
}

// Copy copies a file.
func (s *FileSystemStorage) Copy(ctx context.Context, src, dst string) error {
	reader, err := s.Get(ctx, src)
	if err != nil {
		return err
	}
	defer reader.Close()

	return s.Put(ctx, dst, reader)
}

// Move moves a file.
func (s *FileSystemStorage) Move(ctx context.Context, src, dst string) error {
	srcPath := filepath.Join(s.basePath, src)
	dstPath := filepath.Join(s.basePath, dst)

	// Ensure destination directory exists
	if err := os.MkdirAll(filepath.Dir(dstPath), 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.Rename(srcPath, dstPath)
}

// MemoryStorage implements Storage using in-memory storage.
type MemoryStorage struct {
	files map[string]*memoryFile
	mu    sync.RWMutex
}

type memoryFile struct {
	data        []byte
	contentType string
	metadata    map[string]string
	modTime     time.Time
}

// NewMemoryStorage creates a new MemoryStorage.
func NewMemoryStorage() *MemoryStorage {
	return &MemoryStorage{
		files: make(map[string]*memoryFile),
	}
}

// Put stores a file.
func (s *MemoryStorage) Put(ctx context.Context, path string, reader io.Reader, options ...api.PutOption) error {
	opts := api.ApplyPutOptions(options)

	data, err := io.ReadAll(reader)
	if err != nil {
		return fmt.Errorf("failed to read data: %w", err)
	}

	s.mu.Lock()
	s.files[path] = &memoryFile{
		data:        data,
		contentType: opts.ContentType,
		metadata:    opts.Metadata,
		modTime:     time.Now(),
	}
	s.mu.Unlock()

	return nil
}

// Get retrieves a file.
func (s *MemoryStorage) Get(ctx context.Context, path string) (io.ReadCloser, error) {
	s.mu.RLock()
	file, ok := s.files[path]
	s.mu.RUnlock()

	if !ok {
		return nil, fmt.Errorf("file not found: %s", path)
	}

	return io.NopCloser(bytes.NewReader(file.data)), nil
}

// Delete deletes a file.
func (s *MemoryStorage) Delete(ctx context.Context, path string) error {
	s.mu.Lock()
	delete(s.files, path)
	s.mu.Unlock()
	return nil
}

// Exists checks if a file exists.
func (s *MemoryStorage) Exists(ctx context.Context, path string) (bool, error) {
	s.mu.RLock()
	_, ok := s.files[path]
	s.mu.RUnlock()
	return ok, nil
}

// Info returns file information.
func (s *MemoryStorage) Info(ctx context.Context, path string) (*api.FileInfo, error) {
	s.mu.RLock()
	file, ok := s.files[path]
	s.mu.RUnlock()

	if !ok {
		return nil, fmt.Errorf("file not found: %s", path)
	}

	return &api.FileInfo{
		Name:        filepath.Base(path),
		Path:        path,
		Size:        int64(len(file.data)),
		ContentType: file.contentType,
		ModTime:     file.modTime,
		Metadata:    file.metadata,
	}, nil
}

// List lists files.
func (s *MemoryStorage) List(ctx context.Context, options api.ListOptions) (*api.ListResult, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	var files []api.FileInfo
	for path, file := range s.files {
		if options.Prefix != "" && !strings.HasPrefix(path, options.Prefix) {
			continue
		}

		files = append(files, api.FileInfo{
			Name:        filepath.Base(path),
			Path:        path,
			Size:        int64(len(file.data)),
			ContentType: file.contentType,
			ModTime:     file.modTime,
			Metadata:    file.metadata,
		})

		if options.MaxKeys > 0 && len(files) >= options.MaxKeys {
			break
		}
	}

	return &api.ListResult{Files: files}, nil
}

// Copy copies a file.
func (s *MemoryStorage) Copy(ctx context.Context, src, dst string) error {
	reader, err := s.Get(ctx, src)
	if err != nil {
		return err
	}
	defer reader.Close()

	return s.Put(ctx, dst, reader)
}

// Move moves a file.
func (s *MemoryStorage) Move(ctx context.Context, src, dst string) error {
	if err := s.Copy(ctx, src, dst); err != nil {
		return err
	}
	return s.Delete(ctx, src)
}
