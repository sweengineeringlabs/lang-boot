"""Tests for fileio module."""

import pytest
import tempfile
from pathlib import Path
from dev.engineeringlabs.pyboot.fileio import (
    read_file,
    write_file,
    read_json,
    write_json,
    ensure_dir,
    FileError,
)


@pytest.fixture
def temp_dir():
    """Create a temporary directory."""
    import shutil
    path = Path(tempfile.mkdtemp())
    yield path
    shutil.rmtree(path)


class TestReadWriteFile:
    """Tests for read_file and write_file."""
    
    def test_write_and_read(self, temp_dir):
        """Test writing and reading a file."""
        path = temp_dir / "test.txt"
        content = "Hello, World!"
        
        write_file(path, content)
        result = read_file(path)
        
        assert result == content
    
    def test_read_unicode(self, temp_dir):
        """Test reading unicode content."""
        path = temp_dir / "unicode.txt"
        content = "Hello, 世界! 🎉"
        
        write_file(path, content)
        result = read_file(path)
        
        assert result == content
    
    def test_read_nonexistent_raises(self, temp_dir):
        """Test reading nonexistent file raises."""
        path = temp_dir / "nonexistent.txt"
        
        with pytest.raises(FileError):
            read_file(path)
    
    def test_write_multiline(self, temp_dir):
        """Test writing multiline content."""
        path = temp_dir / "lines.txt"
        content = "Line 1\nLine 2\nLine 3"
        
        write_file(path, content)
        result = read_file(path)
        
        assert result == content
        assert len(result.split("\n")) == 3


class TestReadWriteJson:
    """Tests for read_json and write_json."""
    
    def test_write_and_read_json(self, temp_dir):
        """Test writing and reading JSON."""
        path = temp_dir / "data.json"
        data = {"name": "test", "value": 42}
        
        write_json(path, data)
        result = read_json(path)
        
        assert result == data
    
    def test_json_with_list(self, temp_dir):
        """Test JSON with list."""
        path = temp_dir / "list.json"
        data = [1, 2, 3, "four", 5.0]
        
        write_json(path, data)
        result = read_json(path)
        
        assert result == data
    
    def test_json_nested(self, temp_dir):
        """Test nested JSON."""
        path = temp_dir / "nested.json"
        data = {
            "config": {
                "server": {"host": "localhost", "port": 8080},
                "features": ["auth", "cache"],
            }
        }
        
        write_json(path, data)
        result = read_json(path)
        
        assert result["config"]["server"]["port"] == 8080
    
    def test_read_invalid_json_raises(self, temp_dir):
        """Test reading invalid JSON raises."""
        path = temp_dir / "invalid.json"
        write_file(path, "not valid json")
        
        with pytest.raises(FileError):
            read_json(path)


class TestEnsureDir:
    """Tests for ensure_dir."""
    
    def test_creates_directory(self, temp_dir):
        """Test creating a directory."""
        path = temp_dir / "new_dir"
        
        result = ensure_dir(path)
        
        assert result.exists()
        assert result.is_dir()
    
    def test_creates_nested_directories(self, temp_dir):
        """Test creating nested directories."""
        path = temp_dir / "a" / "b" / "c" / "d"
        
        result = ensure_dir(path)
        
        assert result.exists()
    
    def test_existing_directory(self, temp_dir):
        """Test with existing directory."""
        path = temp_dir / "existing"
        path.mkdir()
        
        result = ensure_dir(path)
        
        assert result.exists()
    
    def test_returns_path(self, temp_dir):
        """Test returns Path object."""
        path = temp_dir / "test"
        
        result = ensure_dir(path)
        
        assert isinstance(result, Path)


class TestFileError:
    """Tests for FileError."""
    
    def test_error_message(self):
        """Test error message."""
        error = FileError("Test error", path="/some/path")
        assert "Test error" in error.message
        assert error.path == "/some/path"
    
    def test_error_with_cause(self):
        """Test error with cause."""
        cause = IOError("Original error")
        error = FileError("Wrapped", cause=cause)
        assert error.cause is cause
