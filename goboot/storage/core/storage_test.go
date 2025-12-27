package core

import (
	"context"
	"io"
	"strings"
	"testing"

	"dev.engineeringlabs/goboot/storage/api"
)

func TestMemoryStorage(t *testing.T) {
	storage := NewMemoryStorage()
	ctx := context.Background()

	t.Run("PutAndGet", func(t *testing.T) {
		content := "hello world"
		err := storage.Put(ctx, "test.txt", strings.NewReader(content))
		if err != nil {
			t.Fatalf("Put failed: %v", err)
		}

		reader, err := storage.Get(ctx, "test.txt")
		if err != nil {
			t.Fatalf("Get failed: %v", err)
		}
		defer reader.Close()

		data, _ := io.ReadAll(reader)
		if string(data) != content {
			t.Errorf("Expected '%s', got '%s'", content, string(data))
		}
	})

	t.Run("Exists", func(t *testing.T) {
		storage.Put(ctx, "exists.txt", strings.NewReader("data"))

		exists, _ := storage.Exists(ctx, "exists.txt")
		if !exists {
			t.Error("File should exist")
		}

		exists, _ = storage.Exists(ctx, "not-exists.txt")
		if exists {
			t.Error("File should not exist")
		}
	})

	t.Run("Delete", func(t *testing.T) {
		storage.Put(ctx, "delete-me.txt", strings.NewReader("data"))
		storage.Delete(ctx, "delete-me.txt")

		exists, _ := storage.Exists(ctx, "delete-me.txt")
		if exists {
			t.Error("File should be deleted")
		}
	})

	t.Run("Info", func(t *testing.T) {
		content := "test content"
		storage.Put(ctx, "info.txt", strings.NewReader(content), 
			api.WithContentType("text/plain"))

		info, err := storage.Info(ctx, "info.txt")
		if err != nil {
			t.Fatalf("Info failed: %v", err)
		}

		if info.Size != int64(len(content)) {
			t.Errorf("Expected size %d, got %d", len(content), info.Size)
		}
		if info.ContentType != "text/plain" {
			t.Errorf("Expected content type 'text/plain', got '%s'", info.ContentType)
		}
	})

	t.Run("List", func(t *testing.T) {
		storage.Put(ctx, "files/a.txt", strings.NewReader("a"))
		storage.Put(ctx, "files/b.txt", strings.NewReader("b"))
		storage.Put(ctx, "other/c.txt", strings.NewReader("c"))

		result, _ := storage.List(ctx, api.ListOptions{Prefix: "files/"})
		if len(result.Files) != 2 {
			t.Errorf("Expected 2 files, got %d", len(result.Files))
		}
	})

	t.Run("Copy", func(t *testing.T) {
		storage.Put(ctx, "original.txt", strings.NewReader("content"))
		storage.Copy(ctx, "original.txt", "copied.txt")

		exists, _ := storage.Exists(ctx, "copied.txt")
		if !exists {
			t.Error("Copied file should exist")
		}
	})

	t.Run("Move", func(t *testing.T) {
		storage.Put(ctx, "moveme.txt", strings.NewReader("content"))
		storage.Move(ctx, "moveme.txt", "moved.txt")

		existsOld, _ := storage.Exists(ctx, "moveme.txt")
		existsNew, _ := storage.Exists(ctx, "moved.txt")

		if existsOld {
			t.Error("Original should not exist")
		}
		if !existsNew {
			t.Error("Moved file should exist")
		}
	})

	t.Run("GetNotFound", func(t *testing.T) {
		_, err := storage.Get(ctx, "nonexistent.txt")
		if err == nil {
			t.Error("Expected error for nonexistent file")
		}
	})
}

func TestPutOptions(t *testing.T) {
	opts := api.ApplyPutOptions([]api.PutOption{
		api.WithContentType("application/json"),
		api.WithMetadata(map[string]string{"author": "john"}),
		api.WithACL("public-read"),
	})

	if opts.ContentType != "application/json" {
		t.Error("ContentType not set")
	}
	if opts.Metadata["author"] != "john" {
		t.Error("Metadata not set")
	}
	if opts.ACL != "public-read" {
		t.Error("ACL not set")
	}
}
